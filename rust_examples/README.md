# Rust + Polars Examples for Windmill

These examples demonstrate how to use Polars (Rust version) in Windmill for ETL tasks.

## How to Use in Windmill

### Method 1: Direct in Windmill UI (Recommended)

1. Open Windmill: http://localhost:80
2. Create a new Script (+ button)
3. Select **"Rust"** as the language
4. Copy one of the example files
5. Add dependencies in the Windmill UI:

```toml
[dependencies]
polars = { version = "0.44", features = ["lazy", "csv", "json"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

6. Click "Save" and then "Run"

### Method 2: Local Development

```bash
# Create a new Rust project
cargo new my-etl
cd my-etl

# Add dependencies
cargo add polars --features lazy,csv,json
cargo add serde --features derive
cargo add serde_json

# Copy examples and test locally
cargo run
```

## Examples Overview

### 1. Basic Polars (`01_basic_polars.rs`)

**What it does:**
- Creates a DataFrame in memory
- Performs filtering and aggregation
- Groups by department
- Returns JSON results

**Learn:**
- DataFrame creation with `df!` macro
- Lazy evaluation with `.lazy()`
- Aggregation functions
- JSON serialization for Windmill

**Try it:** Perfect first example to paste into Windmill!

---

### 2. CSV ETL (`02_csv_etl.rs`)

**What it does:**
- Accepts CSV content as input parameter
- Parses CSV into DataFrame
- Applies transformations
- Returns structured results

**Learn:**
- CSV parsing from string
- Parameter handling in Windmill
- ETL pattern: Extract → Transform → Load
- Error handling with `Result`

**Try it:**
In Windmill, pass this CSV as parameter:
```csv
name,age,salary
Alice,25,50000
Bob,17,30000
Charlie,30,60000
```

---

### 3. Advanced Transformations (`03_advanced_transformations.rs`)

**What it does:**
- Complex multi-column calculations
- Multi-level grouping
- Window functions for analytics
- Percentage calculations

**Learn:**
- Creating computed columns
- Window functions with `.over()`
- Multiple aggregations
- Real-world analytics patterns

**Use case:** Sales analytics, business intelligence

---

### 4. Lazy Query Optimization (`04_lazy_query_optimization.rs`)

**What it does:**
- Processes 1 million rows
- Demonstrates query optimization
- Shows execution plan
- Measures performance

**Learn:**
- Why Polars is fast
- Query plan optimization
- Performance profiling
- Handling large datasets

**Important:** This creates a large dataset in memory - perfect for understanding Polars' performance!

---

## Key Polars Concepts

### Lazy vs Eager

```rust
// Eager: Executes immediately
let result = df.filter(...).group_by(...);

// Lazy: Builds query plan, optimizes, then executes
let result = df.lazy()
    .filter(...)
    .group_by(...)
    .collect()?;  // Execute here
```

**Always prefer lazy** for better performance!

### Common Operations

```rust
// Selection
.select([col("a"), col("b")])

// Filtering
.filter(col("age").gt(25))
.filter(col("name").str().contains("Alice"))

// Computed columns
.with_column((col("a") + col("b")).alias("sum"))

// Aggregation
.group_by([col("category")])
.agg([
    col("value").sum(),
    col("count").count(),
])

// Sorting
.sort("value", SortOptions::default().with_order_descending(true))

// Joins
.join(other_df, [col("id")], [col("id")], JoinType::Inner)
```

### Expressions

Polars uses expressions for transformations:

```rust
col("age")                    // Column reference
lit(100)                      // Literal value
col("a") + col("b")          // Math operations
col("age").gt(25)            // Comparisons
col("name").str().contains() // String operations
col("date").dt().year()      // Date operations
```

## Windmill-Specific Tips

### 1. Dependencies

In Windmill UI, click "Dependencies" and add:

```toml
polars = { version = "0.44", features = ["lazy", "csv", "json", "parquet"] }
```

**Useful features:**
- `lazy` - Lazy evaluation (always include)
- `csv` - CSV reading/writing
- `json` - JSON support
- `parquet` - Parquet format
- `dtype-date` - Date/time operations
- `strings` - String operations

### 2. Return Types

Windmill works best with these return types:

```rust
// Simple string
pub fn main() -> Result<String, String>

// JSON for complex data
use serde_json::Value;
pub fn main() -> Result<Value, String>

// Custom struct (must derive Serialize)
#[derive(Serialize)]
pub struct MyResult { ... }
pub fn main() -> Result<MyResult, String>
```

### 3. Parameters

```rust
// Simple parameters
pub fn main(name: String, age: i32) -> Result<String, String>

// Optional parameters with defaults
pub fn main(limit: Option<usize>) -> Result<String, String> {
    let limit = limit.unwrap_or(100);
    // ...
}

// Complex types
pub fn main(csv_data: String, filters: Vec<String>) -> Result<Value, String>
```

### 4. Logging

```rust
// Use println! for debugging (shows in Windmill logs)
println!("Processing {} rows", df.height());
println!("Result: {}", result);
```

## Performance Tips

1. **Always use lazy evaluation** for datasets > 1000 rows
2. **Filter early** in the pipeline to reduce data
3. **Use appropriate types** - don't use String when i32 works
4. **Leverage parallelism** - Polars automatically parallelizes operations
5. **Use Parquet** for storage - much faster than CSV

## Next Steps

1. **Start with `01_basic_polars.rs`** - Get familiar with syntax
2. **Try `02_csv_etl.rs`** - Learn parameter passing
3. **Build a real pipeline:**
   - Extract data from an API
   - Transform with Polars
   - Load into PostgreSQL
4. **Chain scripts** - Use Windmill flows to connect multiple scripts
5. **Schedule** - Set up periodic runs

## Common Patterns

### Read from PostgreSQL

```rust
// Add dependency: postgres = "0.19"
use postgres::{Client, NoTls};
use polars::prelude::*;

pub fn main() -> Result<String, String> {
    // Connect (use Windmill resources for connection strings)
    let mut client = Client::connect("postgresql://...", NoTls)
        .map_err(|e| e.to_string())?;

    // Query
    let rows = client.query("SELECT * FROM users", &[])
        .map_err(|e| e.to_string())?;

    // Convert to DataFrame
    // ... (manual conversion or use polars-sql)

    Ok("Success".to_string())
}
```

### Write to PostgreSQL

Use the `postgres` crate to insert DataFrame rows.

### Chain with other scripts

In Windmill, create a Flow that:
1. Extracts data (Rust script)
2. Transforms (Rust + Polars)
3. Loads (Rust script)
4. Sends notification (Python/Bash)

## Resources

- Polars Rust Docs: https://docs.rs/polars/latest/polars/
- Polars Guide: https://docs.pola.rs/
- Windmill Docs: https://www.windmill.dev/docs/getting_started/scripts_quickstart/rust
- Rust Book: https://doc.rust-lang.org/book/

## Questions?

Check the Windmill logs for detailed error messages, and remember:
- Polars errors are usually type-related
- Use `.map_err(|e| e.to_string())?` to convert errors
- Test locally first with `cargo run`

Happy ETL-ing! 🦀
