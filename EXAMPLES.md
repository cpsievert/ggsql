# vvSQL Examples

This document provides a collection of basic examples demonstrating how to use vvSQL for data visualization.

## Table of Contents

- [Basic Visualizations](#basic-visualizations)
- [Multiple Layers](#multiple-layers)
- [Scales and Transformations](#scales-and-transformations)
- [Coordinate Systems](#coordinate-systems)
- [Labels and Themes](#labels-and-themes)
- [Faceting](#faceting)
- [Advanced Examples](#advanced-examples)

---

## Basic Visualizations

### Simple Scatter Plot

```sql
SELECT x, y FROM data
VISUALISE AS PLOT
WITH point USING x = x, y = y
```

### Line Chart

```sql
SELECT date, revenue FROM sales
WHERE year = 2024
VISUALISE AS PLOT
WITH line USING x = date, y = revenue
```

### Bar Chart

```sql
SELECT category, total FROM sales
GROUP BY category
VISUALISE AS PLOT
WITH bar USING x = category, y = total
```

### Area Chart

```sql
SELECT date, cumulative FROM metrics
VISUALISE AS PLOT
WITH area USING x = date, y = cumulative
```

---

## Multiple Layers

### Line with Points

```sql
SELECT date, value FROM timeseries
VISUALISE AS PLOT
WITH line USING x = date, y = value
WITH point USING x = date, y = value
```

### Bar Chart with Colored Regions

```sql
SELECT category, revenue, region FROM sales
GROUP BY category, region
VISUALISE AS PLOT
WITH bar USING x = category, y = revenue, fill = region
```

### Multiple Lines by Group

```sql
SELECT date, value, category FROM metrics
VISUALISE AS PLOT
WITH line USING x = date, y = value, color = category
```

---

## Scales and Transformations

### Date Scale

```sql
SELECT sale_date, revenue FROM sales
VISUALISE AS PLOT
WITH line USING x = sale_date, y = revenue
SCALE x USING type = 'date'
```

### Logarithmic Scale

```sql
SELECT x, y FROM exponential_data
VISUALISE AS PLOT
WITH point USING x = x, y = y
SCALE y USING type = 'log10'
```

### Color Palette

```sql
SELECT date, temperature, station FROM weather
VISUALISE AS PLOT
WITH line USING x = date, y = temperature, color = station
SCALE color USING palette = 'viridis'
```

### Custom Domain

```sql
SELECT category, value FROM data
VISUALISE AS PLOT
WITH bar USING x = category, y = value, fill = category
SCALE fill USING domain = ['A', 'B', 'C', 'D']
```

---

## Coordinate Systems

### Cartesian with Limits

```sql
SELECT x, y FROM data
VISUALISE AS PLOT
WITH point USING x = x, y = y
COORD cartesian USING xlim = [0, 100], ylim = [0, 50]
```

### Flipped Coordinates (Horizontal Bar Chart)

```sql
SELECT category, value FROM data
ORDER BY value DESC
VISUALISE AS PLOT
WITH bar USING x = category, y = value
COORD flip
```

### Polar Coordinates (Pie Chart)

```sql
SELECT category, SUM(value) as total FROM data
GROUP BY category
VISUALISE AS PLOT
WITH bar USING x = category, y = total
COORD polar
```

### Polar with Theta Specification

```sql
SELECT category, value FROM data
VISUALISE AS PLOT
WITH bar USING x = category, y = value
COORD polar USING theta = y
```

---

## Labels and Themes

### Chart with Title and Axis Labels

```sql
SELECT date, revenue FROM sales
VISUALISE AS PLOT
WITH line USING x = date, y = revenue
LABEL title = 'Monthly Revenue Trends',
      x = 'Date',
      y = 'Revenue ($)'
```

### Multiple Labels

```sql
SELECT date, value FROM metrics
VISUALISE AS PLOT
WITH area USING x = date, y = value
LABEL title = 'Performance Metrics',
      subtitle = 'Q4 2024',
      x = 'Date',
      y = 'Metric Value',
      caption = 'Data source: Analytics DB'
```

### Themed Visualization

```sql
SELECT category, value FROM data
VISUALISE AS PLOT
WITH bar USING x = category, y = value
THEME minimal
```

### Theme with Custom Properties

```sql
SELECT x, y FROM data
VISUALISE AS PLOT
WITH point USING x = x, y = y
THEME dark USING background = '#1a1a1a'
```

---

## Faceting

### Facet Wrap

```sql
SELECT date, value, region FROM sales
VISUALISE AS PLOT
WITH line USING x = date, y = value
FACET WRAP region
```

### Facet Grid

```sql
SELECT date, value, region, product FROM sales
VISUALISE AS PLOT
WITH line USING x = date, y = value
FACET region BY product
```

### Facet with Free Scales

```sql
SELECT date, value, category FROM metrics
VISUALISE AS PLOT
WITH line USING x = date, y = value
FACET WRAP category USING scales = 'free_y'
```

---

## Advanced Examples

### Complete Regional Sales Analysis

```sql
SELECT
    sale_date,
    region,
    SUM(quantity) as total_quantity
FROM sales
WHERE sale_date >= '2024-01-01'
GROUP BY sale_date, region
ORDER BY sale_date
VISUALISE AS PLOT
WITH line USING x = sale_date, y = total_quantity, color = region
WITH point USING x = sale_date, y = total_quantity, color = region
SCALE x USING type = 'date'
FACET WRAP region
LABEL title = 'Sales Trends by Region',
      x = 'Date',
      y = 'Total Quantity'
THEME minimal
```

### Time Series with Multiple Aesthetics

```sql
SELECT
    timestamp,
    temperature,
    humidity,
    station
FROM weather_data
WHERE timestamp >= NOW() - INTERVAL '7 days'
VISUALISE AS PLOT
WITH line USING x = timestamp, y = temperature, color = station, linetype = station
SCALE x USING type = 'datetime'
SCALE color USING palette = 'viridis'
LABEL title = 'Temperature Trends',
      x = 'Time',
      y = 'Temperature (°C)'
```

### Categorical Analysis with Flipped Coordinates

```sql
SELECT
    product_name,
    SUM(revenue) as total_revenue
FROM sales
GROUP BY product_name
ORDER BY total_revenue DESC
LIMIT 10
VISUALISE AS PLOT
WITH bar USING x = product_name, y = total_revenue, fill = product_name
COORD flip USING color = ['red', 'orange', 'yellow', 'green', 'blue',
                          'indigo', 'violet', 'pink', 'brown', 'gray']
LABEL title = 'Top 10 Products by Revenue',
      x = 'Product',
      y = 'Revenue ($)'
THEME classic
```

### Distribution with Custom Domain

```sql
SELECT
    date,
    value,
    category
FROM measurements
WHERE category IN ('A', 'B', 'C')
VISUALISE AS PLOT
WITH point USING x = date, y = value, color = category, size = value
SCALE x USING type = 'date'
SCALE color USING domain = ['A', 'B', 'C']
SCALE size USING limits = [0, 100]
COORD cartesian USING ylim = [0, 150]
LABEL title = 'Measurement Distribution',
      x = 'Date',
      y = 'Value'
```

### Multi-Layer Visualization with Annotations

```sql
SELECT
    x,
    y,
    category,
    label
FROM data_points
VISUALISE AS PLOT
WITH point USING x = x, y = y, color = category, size = 5
WITH text USING x = x, y = y, label = label
SCALE color USING palette = 'viridis'
COORD cartesian USING xlim = [0, 100], ylim = [0, 100]
LABEL title = 'Annotated Scatter Plot',
      x = 'X Axis',
      y = 'Y Axis'
```

```sql
SELECT
    cyl,
    COUNT(*) as vehicle_count
FROM data_points
WHERE cyl IN (4, 6, 8)
GROUP BY cyl
ORDER BY cyl
VISUALISE AS PLOT
WITH bar USING x = cyl, y = vehicle_count
SCALE x USING domain = [4, 6, 8]
LABEL title = 'Distribution of Vehicles by Number of Cylinders',
      x = 'Number of Cylinders',
      y = 'Number of Vehicles'"
```

---

## Case Insensitivity

vvSQL keywords are case-insensitive. All of the following are valid:

```sql
-- Uppercase (traditional)
VISUALISE AS PLOT
WITH line USING x = date, y = value

-- Lowercase
visualise as plot
with line using x = date, y = value

-- Mixed case
Visualise As Plot
With Line Using x = date, y = value
```

---

## Tips and Best Practices

1. **Date Handling**: Always use `SCALE x USING type = 'date'` for date columns to ensure proper axis formatting.

2. **Color Mappings**: Use `color` for continuous data and `fill` for categorical data in bars/areas.

3. **Coordinate Limits**: Set explicit limits with `COORD cartesian USING xlim = [min, max]` to control axis ranges.

4. **Faceting**: Use faceting to create small multiples when comparing across categories.

5. **Multiple Layers**: Combine layers (e.g., line + point) for richer visualizations.

6. **Themes**: Apply themes last in your specification for consistent styling.

7. **Labels**: Always provide meaningful titles and axis labels for clarity.

8. **Domain Specification**: Use either SCALE or COORD for domain/limit specification, but not both for the same aesthetic.

---

## Running Examples

### Using the CLI

```bash
# Parse and validate
vvsql parse query.sql

# Execute and generate Vega-Lite JSON
vvsql exec query.sql --writer vegalite --output chart.vl.json

# Execute from file
vvsql run query.sql
```

### Using the REST API

```bash
# Start the server with sample data
vvsql-rest --load-sample-data --port 3334

# Execute a query via HTTP
curl -X POST http://localhost:3334/api/v1/query \
  -H "Content-Type: application/json" \
  -d '{"query": "SELECT * FROM products VISUALISE AS PLOT WITH bar USING x = name, y = price"}'
```

### Using the Test Application

```bash
cd test-app
npm install
npm run dev
# Open http://localhost:5173
```

---

## Further Reading

- See [CLAUDE.md](CLAUDE.md) for full system architecture and implementation details
- See [README.md](README.md) for installation and setup instructions
- See the `tree-sitter-vvsql/test/corpus/` directory for more grammar examples
