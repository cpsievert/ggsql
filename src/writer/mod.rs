//! Output writer abstraction layer for ggsql
//!
//! The writer module provides a pluggable interface for generating visualization
//! outputs from Prepared specifications.
//!
//! # Architecture
//!
//! All writers implement the `Writer` trait, which provides:
//! - Prepared → Output conversion via `render()`
//! - Low-level Plot + Data → Output via `write()`
//! - Format-specific rendering logic
//!
//! # Example
//!
//! ```rust,ignore
//! use ggsql::writer::{Writer, VegaLiteWriter};
//!
//! let writer = VegaLiteWriter::new();
//! let json = writer.render(&prepared)?;
//! println!("{}", json);
//! ```

use crate::api::Prepared;
use crate::{DataFrame, Plot, Result};
use std::collections::HashMap;

#[cfg(feature = "vegalite")]
pub mod vegalite;

#[cfg(feature = "vegalite")]
pub use vegalite::VegaLiteWriter;

/// Trait for visualization output writers
///
/// Writers take a Prepared specification and produce formatted output
/// (JSON, R code, PNG bytes, etc.).
pub trait Writer {
    /// Render a prepared visualization to output format
    ///
    /// This is the primary rendering method. It extracts the plot and data
    /// from the Prepared object and generates the output.
    ///
    /// # Arguments
    ///
    /// * `prepared` - The prepared visualization (from `reader.execute()`)
    ///
    /// # Returns
    ///
    /// A string containing the formatted output (JSON, code, etc.)
    ///
    /// # Errors
    ///
    /// Returns `GgsqlError::WriterError` if rendering fails
    fn render(&self, prepared: &Prepared) -> Result<String> {
        self.write(prepared.plot(), prepared.data_map())
    }

    /// Generate output from a visualization specification and data sources
    ///
    /// This is a lower-level method that takes the plot and data separately.
    /// Most callers should use `render()` instead.
    ///
    /// # Arguments
    ///
    /// * `spec` - The parsed ggsql specification
    /// * `data` - A map of data source names to DataFrames. The writer decides
    ///   how to use these based on the spec's layer configurations.
    ///
    /// # Returns
    ///
    /// A string containing the formatted output (JSON, code, etc.)
    ///
    /// # Errors
    ///
    /// Returns `GgsqlError::WriterError` if:
    /// - The spec is incompatible with this writer
    /// - The data doesn't match the spec's requirements
    /// - Output generation fails
    fn write(&self, spec: &Plot, data: &HashMap<String, DataFrame>) -> Result<String>;

    /// Validate that a spec is compatible with this writer
    ///
    /// Checks whether the spec can be rendered by this writer without
    /// actually generating output.
    ///
    /// # Arguments
    ///
    /// * `spec` - The visualization specification to validate
    ///
    /// # Returns
    ///
    /// Ok(()) if the spec is compatible, otherwise an error
    fn validate(&self, spec: &Plot) -> Result<()>;
}
