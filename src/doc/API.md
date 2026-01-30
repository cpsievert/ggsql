# ggsql API Reference

This document provides a comprehensive reference for the ggsql public API.

## Overview

- **Stage 1: `prepare()`** - Parse query, execute SQL, resolve mappings, prepare data
- **Stage 2: `writer.render()`** - Generate output (Vega-Lite JSON, etc.)

### API Functions

| Function     | Use Case                                             |
| ------------ | ---------------------------------------------------- |
| `prepare()`  | Main entry point - full visualization pipeline       |
| `writer.render()` | Generate output from prepared data              |
| `validate()` | Validate syntax + semantics, inspect query structure |

---

## Core Functions

### `prepare`

```rust
pub fn prepare(query: &str, reader: &dyn Reader) -> Result<Prepared>
```

Prepare a ggsql query for visualization. This is the main entry point for the two-stage API.

**What happens during preparation:**

1. Parses the query (SQL + VISUALISE portions)
2. Executes the main SQL query using the provided reader
3. Resolves wildcards (`VISUALISE *`) against actual columns
4. Merges global mappings into each layer
5. Executes layer-specific queries (filters, stats)
6. Injects constant values as synthetic columns
7. Computes aesthetic labels from column names

**Arguments:**

- `query` - The full ggsql query string
- `reader` - A reader implementing the `Reader` trait

**Returns:**

- `Ok(Prepared)` - Ready for rendering
- `Err(GgsqlError)` - Parse, validation, or execution error

**Example:**

```rust
use ggsql::{prepare, reader::DuckDBReader, writer::{VegaLiteWriter, Writer}};

let reader = DuckDBReader::from_connection_string("duckdb://memory")?;
let prepared = prepare(
    "SELECT x, y FROM data VISUALISE x, y DRAW point",
    &reader
)?;

// Access metadata
println!("Rows: {}", prepared.metadata().rows);
println!("Columns: {:?}", prepared.metadata().columns);

// Render to Vega-Lite
let writer = VegaLiteWriter::new();
let result = writer.render(&prepared)?;
```

**Error Conditions:**

- Parse error in SQL or VISUALISE portion
- SQL execution failure
- Missing required aesthetics
- Invalid geom type
- Multiple VISUALISE statements (not yet supported)

---

### `validate`

```rust
pub fn validate(query: &str) -> Result<Validated>
```

Validate query syntax and semantics without executing SQL. This function combines query parsing and validation into a single operation.

**What is validated:**

- Syntax (parsing)
- Required aesthetics for each geom type
- Valid scale types (linear, log10, date, etc.)
- Valid coord types and properties
- Valid geom types
- Valid aesthetic names
- Valid SETTING parameters

**Arguments:**

- `query` - The full ggsql query string (SQL + VISUALISE)

**Returns:**

- `Ok(Validated)` - Validation results with query inspection methods
- `Err(GgsqlError)` - Internal error

**Example:**

```rust
use ggsql::validate;

let validated = validate("SELECT x, y FROM data VISUALISE x, y DRAW point")?;

// Check validity
if !validated.valid() {
    for error in validated.errors() {
        eprintln!("Error: {}", error.message);
    }
}

// Inspect query structure
if validated.has_visual() {
    println!("SQL: {}", validated.sql());
    println!("Visual: {}", validated.visual());
}
```

**Notes:**

- Does not execute SQL
- Does not resolve wildcards or global mappings
- Cannot validate column existence (requires data)
- Returns all errors, not just the first one
- CST available via `tree()` for advanced inspection

---

## Type Reference

### `Validated`

Result of validating a query (syntax + semantics, no SQL execution).

```rust
pub struct Validated {
    // All fields private
}
```

**Methods:**

| Method       | Signature                                    | Description                        |
| ------------ | -------------------------------------------- | ---------------------------------- |
| `has_visual` | `fn has_visual(&self) -> bool`               | Whether query contains VISUALISE   |
| `sql`        | `fn sql(&self) -> &str`                      | The SQL portion (before VISUALISE) |
| `visual`     | `fn visual(&self) -> &str`                   | The VISUALISE portion (raw text)   |
| `tree`       | `fn tree(&self) -> Option<&Tree>`            | CST for advanced inspection        |
| `valid`      | `fn valid(&self) -> bool`                    | Whether query is valid             |
| `errors`     | `fn errors(&self) -> &[ValidationError]`     | Validation errors                  |
| `warnings`   | `fn warnings(&self) -> &[ValidationWarning]` | Validation warnings                |

**Example:**

```rust
let validated = ggsql::validate("SELECT 1 as x VISUALISE DRAW point MAPPING x AS x, y AS y")?;

// Check validity
if !validated.valid() {
    for error in validated.errors() {
        eprintln!("Error: {}", error.message);
    }
}

// Inspect query structure
assert!(validated.has_visual());
assert_eq!(validated.sql(), "SELECT 1 as x");
assert!(validated.visual().starts_with("VISUALISE"));

// CST access for advanced use cases
if let Some(tree) = validated.tree() {
    println!("Root node: {}", tree.root_node().kind());
}
```

---

### `Prepared`

Result of preparing a visualization, ready for rendering.

#### Plot Access Methods

| Method        | Signature                        | Description                     |
| ------------- | -------------------------------- | ------------------------------- |
| `plot`        | `fn plot(&self) -> &Plot`        | Get resolved plot specification |
| `layer_count` | `fn layer_count(&self) -> usize` | Number of layers                |

**Example:**

```rust
println!("Layers: {}", prepared.layer_count());

let plot = prepared.plot();
for (i, layer) in plot.layers.iter().enumerate() {
    println!("Layer {}: {:?}", i, layer.geom);
}
```

#### Metadata Methods

| Method     | Signature                         | Description                |
| ---------- | --------------------------------- | -------------------------- |
| `metadata` | `fn metadata(&self) -> &Metadata` | Get visualization metadata |

**Example:**

```rust
let meta = prepared.metadata();
println!("Rows: {}", meta.rows);
println!("Columns: {:?}", meta.columns);
println!("Layer count: {}", meta.layer_count);
```

#### Data Access Methods

| Method       | Signature                                              | Description                     |
| ------------ | ------------------------------------------------------ | ------------------------------- |
| `data`       | `fn data(&self) -> Option<&DataFrame>`                 | Global data (main query result) |
| `layer_data` | `fn layer_data(&self, i: usize) -> Option<&DataFrame>` | Layer-specific data             |
| `stat_data`  | `fn stat_data(&self, i: usize) -> Option<&DataFrame>`  | Stat transform results          |
| `data_map`   | `fn data_map(&self) -> &HashMap<String, DataFrame>`    | Raw data map access             |

**Example:**

```rust
// Global data
if let Some(df) = prepared.data() {
    println!("Global data: {} rows", df.height());
}

// Layer-specific data (from FILTER or FROM clause)
if let Some(df) = prepared.layer_data(0) {
    println!("Layer 0 has filtered data: {} rows", df.height());
}

// Stat data (histogram bins, density estimates, etc.)
if let Some(df) = prepared.stat_data(1) {
    println!("Layer 1 stat data: {} rows", df.height());
}
```

#### Query Introspection Methods

| Method      | Signature                                       | Description                      |
| ----------- | ----------------------------------------------- | -------------------------------- |
| `sql`       | `fn sql(&self) -> &str`                         | Main SQL query that was executed |
| `visual`    | `fn visual(&self) -> &str`                      | Raw VISUALISE text               |
| `layer_sql` | `fn layer_sql(&self, i: usize) -> Option<&str>` | Layer filter/source query        |
| `stat_sql`  | `fn stat_sql(&self, i: usize) -> Option<&str>`  | Stat transform query             |

**Example:**

```rust
// Main query
println!("SQL: {}", prepared.sql());
println!("Visual: {}", prepared.visual());

// Per-layer queries
for i in 0..prepared.layer_count() {
    if let Some(sql) = prepared.layer_sql(i) {
        println!("Layer {} filter: {}", i, sql);
    }
    if let Some(sql) = prepared.stat_sql(i) {
        println!("Layer {} stat: {}", i, sql);
    }
}
```

#### Warnings Method

| Method     | Signature                                    | Description                          |
| ---------- | -------------------------------------------- | ------------------------------------ |
| `warnings` | `fn warnings(&self) -> &[ValidationWarning]` | Validation warnings from preparation |

**Example:**

```rust
let prepared = ggsql::prepare(query, &reader)?;

// Check for warnings
if !prepared.warnings().is_empty() {
    for warning in prepared.warnings() {
        eprintln!("Warning: {}", warning.message);
    }
}

// Continue with rendering
let writer = VegaLiteWriter::new();
let json = writer.render(&prepared)?;
```

---

### `Metadata`

Information about the prepared visualization.

```rust
pub struct Metadata {
    pub rows: usize,           // Rows in primary data source
    pub columns: Vec<String>,  // Column names
    pub layer_count: usize,    // Number of layers in the plot
}
```

---

### `ValidationError`

A validation error (fatal issue).

```rust
pub struct ValidationError {
    pub message: String,
    pub location: Option<Location>,
}
```

---

### `ValidationWarning`

A validation warning (non-fatal issue).

```rust
pub struct ValidationWarning {
    pub message: String,
    pub location: Option<Location>,
}
```

---

### `Location`

Location within a query string.

```rust
pub struct Location {
    pub line: usize,    // 0-based line number
    pub column: usize,  // 0-based column number
}
```

---

## Reader Trait & Implementations

### `Reader` Trait

```rust
pub trait Reader {
    /// Execute a SQL query and return a DataFrame
    fn execute_sql(&self, sql: &str) -> Result<DataFrame>;

    /// Register a DataFrame as a queryable table
    fn register(&mut self, name: &str, df: DataFrame) -> Result<()>;

    /// Unregister a table (fails silently if not found)
    fn unregister(&mut self, name: &str);

    /// Check if this reader supports DataFrame registration
    fn supports_register(&self) -> bool;
}
```

---

## Writer Trait & Implementations

### `Writer` Trait

```rust
pub trait Writer {
    /// Render a prepared visualization to output format
    fn render(&self, prepared: &Prepared) -> Result<String>;

    /// Lower-level: render from plot specification and data map
    fn write(&self, spec: &Plot, data: &HashMap<String, DataFrame>) -> Result<String>;

    /// Validate that a spec is compatible with this writer
    fn validate(&self, spec: &Plot) -> Result<()>;
}
```

**Example:**

```rust
use ggsql::writer::{VegaLiteWriter, Writer};

let writer = VegaLiteWriter::new();
let json = writer.render(&prepared)?;
println!("{}", json);
```

---

## Python Bindings

The Python bindings provide the same two-stage API with Pythonic conventions.

### Module Structure

- `ggsql.readers` - Reader classes (`DuckDB`)
- `ggsql.writers` - Writer classes (`VegaLite`)
- `ggsql` - Types (`Validated`, `Prepared`), exceptions (`NoVisualiseError`), and functions (`validate`)

### Classes

#### `ggsql.readers.DuckDB`

```python
class DuckDB:
    def __init__(self, connection: str) -> None:
        """Create a DuckDB reader.

        Args:
            connection: Connection string (e.g., "duckdb://memory")
        """

    def execute(
        self,
        query: str,
        data: dict[str, polars.DataFrame] | None = None
    ) -> Prepared:
        """Execute a ggsql query with optional DataFrame registration.

        DataFrames are registered before execution and automatically
        unregistered afterward (even on error).

        Args:
            query: The ggsql query (must contain VISUALISE clause)
            data: DataFrames to register as tables (keys are table names)

        Returns:
            Prepared visualization ready for rendering

        Raises:
            NoVisualiseError: If query has no VISUALISE clause
            ValueError: If parsing or execution fails
        """

    def execute_sql(self, sql: str) -> polars.DataFrame:
        """Execute plain SQL and return a Polars DataFrame."""

    def register(self, name: str, df: polars.DataFrame) -> None:
        """Manually register a DataFrame as a queryable table."""

    def unregister(self, name: str) -> None:
        """Unregister a table (fails silently if not found)."""
```

#### `ggsql.writers.VegaLite`

```python
class VegaLite:
    def __init__(self) -> None:
        """Create a Vega-Lite writer."""

    def render(self, spec: Prepared) -> str:
        """Render to Vega-Lite JSON string."""

    def render_chart(self, spec: Prepared, **kwargs) -> AltairChart:
        """Render to Altair chart object.

        Args:
            spec: Prepared visualization from reader.execute()
            **kwargs: Additional args for altair.Chart.from_json()

        Returns:
            Altair chart (Chart, LayerChart, FacetChart, etc.)
        """
```

#### `Validated`

```python
class Validated:
    def has_visual(self) -> bool:
        """Check if query has VISUALISE clause."""

    def sql(self) -> str:
        """Get the SQL portion."""

    def visual(self) -> str:
        """Get the VISUALISE portion."""

    def valid(self) -> bool:
        """Check if query is valid."""

    def errors(self) -> list[dict]:
        """Get validation errors as list of dicts with 'message', 'location'."""

    def warnings(self) -> list[dict]:
        """Get validation warnings as list of dicts with 'message', 'location'."""

    # Note: tree() not exposed (tree-sitter nodes are Rust-only)
```

#### `Prepared`

```python
class Prepared:
    def metadata(self) -> dict:
        """Get metadata as dict with keys: rows, columns, layer_count."""

    def sql(self) -> str:
        """Get the main SQL query."""

    def visual(self) -> str:
        """Get the VISUALISE text."""

    def layer_count(self) -> int:
        """Get number of layers."""

    def warnings(self) -> list[dict]:
        """Get validation warnings as list of dicts with 'message', 'location'."""

    def data(self) -> polars.DataFrame | None:
        """Get global data."""

    def layer_data(self, index: int) -> polars.DataFrame | None:
        """Get layer-specific data."""

    def stat_data(self, index: int) -> polars.DataFrame | None:
        """Get stat transform data."""

    def layer_sql(self, index: int) -> str | None:
        """Get layer filter query."""

    def stat_sql(self, index: int) -> str | None:
        """Get stat transform query."""
```

### Exceptions

```python
class NoVisualiseError(Exception):
    """Raised when execute() is called on a query without VISUALISE clause."""
```

### Functions

```python
def validate(query: str) -> Validated:
    """Validate query syntax and semantics.

    Returns Validated object with query inspection and validation methods.
    """
```

### Usage Example

```python
import polars as pl
from ggsql.readers import DuckDB
from ggsql.writers import VegaLite

# Create data
df = pl.DataFrame({"x": [1, 2, 3], "y": [10, 20, 30]})

# Execute with inline data registration
reader = DuckDB("duckdb://memory")
spec = reader.execute(
    "SELECT * FROM data VISUALISE x, y DRAW point",
    {"data": df}
)

# Render to Altair chart
writer = VegaLite()
chart = writer.render_chart(spec)
```
