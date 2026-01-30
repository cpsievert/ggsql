# ggsql

Python bindings for [ggsql](https://github.com/georgestagg/ggsql), a SQL extension for declarative data visualization.

This package provides Python bindings to the Rust `ggsql` crate, enabling Python users to create visualizations using ggsql's VISUALISE syntax with native Altair chart output.

## Installation

### From PyPI (when published)

```bash
pip install ggsql
```

### From source

Building from source requires:

- Rust toolchain (install via [rustup](https://rustup.rs/))
- Python 3.10+
- [maturin](https://github.com/PyO3/maturin)

```bash
# Clone the monorepo
git clone https://github.com/georgestagg/ggsql.git
cd ggsql/ggsql-python

# Create a virtual environment
python -m venv .venv
source .venv/bin/activate  # or `.venv\Scripts\activate` on Windows

# Install build dependencies
pip install maturin

# Build and install in development mode
maturin develop

# Or build a wheel
maturin build --release
pip install target/wheels/ggsql-*.whl
```

## Quick Start

```python
import polars as pl
import ggsql

# Create a DataFrame
df = pl.DataFrame({
    "x": [1, 2, 3, 4, 5],
    "y": [10, 20, 15, 30, 25],
    "category": ["A", "B", "A", "B", "A"]
})

# Create reader and execute query with inline data registration
reader = ggsql.readers.DuckDB("duckdb://memory")
spec = reader.execute(
    "SELECT * FROM data VISUALISE x, y, category AS color DRAW point",
    {"data": df}
)

# Render to Vega-Lite JSON
writer = ggsql.writers.VegaLite()
json_str = writer.render_json(spec)

# Or render to Altair chart
chart = writer.render_chart(spec)
chart.display()  # In Jupyter
```

## API Reference

### Modules

#### `ggsql.readers`

Database reader classes.

##### `DuckDB(connection: str)`

Database reader that executes SQL and manages DataFrames.

```python
import ggsql

reader = ggsql.readers.DuckDB("duckdb://memory")  # In-memory database
reader = ggsql.readers.DuckDB("duckdb:///path/to/file.db")  # File database
```

**Methods:**

- `execute(query: str, data: dict[str, DataFrame] | None = None) -> Prepared` - Execute a ggsql query with optional DataFrame registration. DataFrames are automatically registered before execution and unregistered afterward. Raises `NoVisualiseError` if query has no VISUALISE clause.
- `execute_sql(sql: str) -> pl.DataFrame` - Execute plain SQL and return results (no VISUALISE clause needed)
- `register(name: str, df: DataFrame) -> None` - Manually register a DataFrame as a queryable table
- `unregister(name: str) -> None` - Unregister a table (fails silently if not found)

**Context manager:** DuckDB supports the context manager protocol for use with `with` statements:

```python
with ggsql.readers.DuckDB("duckdb://memory") as reader:
    spec = reader.execute(query, {"data": df})
```

**DataFrame support:** Accepts any [narwhals](https://narwhals-dev.github.io/narwhals/)-compatible DataFrame (polars, pandas, pyarrow, etc.).

#### `ggsql.writers`

Output writer classes.

##### `VegaLite()`

Writer that generates Vega-Lite v6 JSON specifications.

```python
import ggsql

writer = ggsql.writers.VegaLite()
json_str = writer.render_json(spec)
chart = writer.render_chart(spec)
```

**Methods:**

- `render_json(spec: Prepared) -> str` - Render to Vega-Lite JSON string
- `render_chart(spec: Prepared, **kwargs) -> AltairChart` - Render to Altair chart object

#### `ggsql.types`

Type classes returned by ggsql functions.

##### `Validated`

Result of `validate()` containing query analysis without SQL execution.

**Methods:**

- `valid() -> bool` - Whether the query is syntactically and semantically valid
- `has_visual() -> bool` - Whether the query contains a VISUALISE clause
- `sql() -> str` - The SQL portion (before VISUALISE)
- `visual() -> str` - The VISUALISE portion
- `errors() -> list[dict]` - Validation errors with messages and locations
- `warnings() -> list[dict]` - Validation warnings

##### `Prepared`

Result of `reader.execute()`, containing resolved visualization ready for rendering.

**Methods:**

- `metadata() -> dict` - Get `{"rows": int, "columns": list[str], "layer_count": int}`
- `sql() -> str` - The executed SQL query
- `visual() -> str` - The VISUALISE clause
- `layer_count() -> int` - Number of DRAW layers
- `data() -> pl.DataFrame | None` - Main query result DataFrame
- `layer_data(index: int) -> pl.DataFrame | None` - Layer-specific data (if filtered)
- `stat_data(index: int) -> pl.DataFrame | None` - Statistical transform data
- `layer_sql(index: int) -> str | None` - Layer filter SQL
- `stat_sql(index: int) -> str | None` - Stat transform SQL
- `warnings() -> list[dict]` - Validation warnings from preparation

### Exceptions

All ggsql exceptions inherit from `GgsqlError`, allowing you to catch all ggsql-specific errors:

```python
try:
    spec = reader.execute(query)
except ggsql.types.GgsqlError as e:
    print(f"ggsql error: {e}")
```

#### Exception Hierarchy

- `GgsqlError` - Base exception for all ggsql errors
  - `ParseError` - Query parsing failed
  - `ValidationError` - Query validation failed (e.g., missing required aesthetics)
  - `ReaderError` - Database/SQL execution failed
  - `WriterError` - Output generation failed
  - `NoVisualiseError` - Query has no VISUALISE clause

#### `NoVisualiseError`

Raised when `reader.execute()` is called on a query without a VISUALISE clause. Use `reader.execute_sql()` for plain SQL queries.

```python
try:
    spec = reader.execute("SELECT * FROM data")  # No VISUALISE
except ggsql.types.NoVisualiseError:
    df = reader.execute_sql("SELECT * FROM data")  # Use this instead
```

### Functions

#### `validate(query: str) -> Validated`

Validate query syntax and semantics without executing SQL.

```python
validated = ggsql.validate("SELECT x, y FROM data VISUALISE x, y DRAW point")
if validated.valid():
    print("Query is valid!")
else:
    for error in validated.errors():
        print(f"Error: {error['message']}")
```

## Examples

### Basic Usage

```python
import polars as pl
import ggsql

df = pl.DataFrame({"x": [1, 2, 3], "y": [10, 20, 30]})

reader = ggsql.readers.DuckDB("duckdb://memory")
spec = reader.execute("SELECT * FROM data VISUALISE x, y DRAW point", {"data": df})

writer = ggsql.writers.VegaLite()
chart = writer.render_chart(spec)
```

### Multiple Tables

```python
sales = pl.DataFrame({"id": [1, 2], "product_id": [1, 1], "amount": [100, 200]})
products = pl.DataFrame({"id": [1], "name": ["Widget"]})

spec = reader.execute(
    """
    SELECT s.id, s.amount, p.name
    FROM sales s JOIN products p ON s.product_id = p.id
    VISUALISE id AS x, amount AS y, name AS color
    DRAW bar
    """,
    {"sales": sales, "products": products}
)
```

### VISUALISE FROM Shorthand

```python
spec = reader.execute(
    "VISUALISE FROM data DRAW point MAPPING x AS x, y AS y",
    {"data": df}
)
```

### Mapping Styles

```python
df = pl.DataFrame({"x": [1, 2, 3], "y": [10, 20, 30], "category": ["A", "B", "A"]})

# Explicit mapping
spec = reader.execute("SELECT * FROM df VISUALISE x AS x, y AS y DRAW point", {"df": df})

# Implicit mapping (column name = aesthetic name)
spec = reader.execute("SELECT * FROM df VISUALISE x, y DRAW point", {"df": df})

# Wildcard mapping (map all matching columns)
spec = reader.execute("SELECT * FROM df VISUALISE * DRAW point", {"df": df})

# With color encoding
spec = reader.execute("SELECT * FROM df VISUALISE x, y, category AS color DRAW point", {"df": df})
```

### Using Pandas DataFrames

```python
import pandas as pd
import ggsql

# Works with pandas DataFrames (via narwhals)
df = pd.DataFrame({"x": [1, 2, 3], "y": [10, 20, 30]})

reader = ggsql.readers.DuckDB("duckdb://memory")
spec = reader.execute("SELECT * FROM data VISUALISE x, y DRAW point", {"data": df})

writer = ggsql.writers.VegaLite()
chart = writer.render_chart(spec)
```

### Handling Plain SQL

```python
import ggsql

try:
    spec = reader.execute("SELECT * FROM data", {"data": df})
except ggsql.types.NoVisualiseError:
    # Use execute_sql() for queries without VISUALISE
    result_df = reader.execute_sql("SELECT * FROM data")
```

## Development

### Keeping in sync with the monorepo

The `ggsql-python` package is part of the [ggsql monorepo](https://github.com/posit-dev/ggsql) and depends on the Rust `ggsql` crate via a path dependency. When the Rust crate is updated, you may need to rebuild:

```bash
cd ggsql-python

# Rebuild after Rust changes
maturin develop

# If tree-sitter grammar changed, clean and rebuild
cd .. && cargo clean -p tree-sitter-ggsql && cd ggsql-python
maturin develop
```

### Running tests

```bash
# Install test dependencies
pip install pytest

# Run all tests
pytest tests/ -v
```

## Requirements

- Python >= 3.10
- altair >= 5.0
- narwhals >= 1.0
- polars >= 1.0

## License

MIT
