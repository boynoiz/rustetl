# Quick Start: Rust + Polars in Windmill

Yes! You can absolutely use Polars.rs (the Rust version) directly in Windmill. Here's how to get started right now.

## 🚀 Your First Rust Script (5 minutes)

### Step 1: Open Windmill

```bash
# Make sure Windmill is running
cd ~/Projects/windmill
docker compose up -d

# Open browser
xdg-open http://localhost:80  # or just go to http://localhost:80
```

### Step 2: Create a Rust Script

1. Click the **"+ Script"** button
2. Select **"Rust"** as the language
3. Name it: `my_first_polars`

### Step 3: Paste This Code

```rust
//! My First Polars Script
//!
//! Add dependencies in the following partial Cargo.toml manifest
//!
//! ```cargo
//! [dependencies]
//! polars = { version = "0.44", features = ["lazy"] }
//! serde_json = "1.0"
//! anyhow = "1.0"
//! ```

use polars::prelude::*;
use serde_json::{json, Value};

fn main() -> anyhow::Result<Value> {
   // Create a DataFrame
   let df = df! {
          "name" => &["Alice", "Bob", "Charlie"],
          "age" => &[25, 30, 35],
          "salary" => &[70000, 80000, 90000],
      }?;

   println!("Original DataFrame:");
   println!("{}", df);
   println!();

   // Transform
   let result = df
           .lazy()
           .filter(col("age").gt(lit(25)))
           .with_column((col("salary") * lit(1.1)).alias("salary_raise"))
           .collect()?;

   println!("Transformed:");
   println!("{}", result);
   println!();

   // Convert column names to String for serialization
   let columns: Vec<String> = result
           .get_column_names()
           .iter()
           .map(|s| s.to_string())
           .collect();

   // Return JSON output
   Ok(json!({
          "rows": result.height(),
          "columns": columns,
          "preview": format!("{}", result)
      }))
}
```

### Step 4: Add Dependencies

In the Windmill UI:
1. Click **"Dependencies"** tab
2. Paste:
```toml
polars = { version = "0.44", features = ["lazy"] }
serde_json = "1.0"
```

### Step 5: Run It!

Click **"Run"** button and watch it execute. You'll see:
- Build logs (first run takes ~30-60 seconds to compile)
- Execution output
- JSON result

**Subsequent runs are cached and much faster!**

## 📊 What You Get with Rust + Polars

### Performance

```rust
// Process millions of rows efficiently
let df = df! {
    "id" => (1..=1_000_000).collect::<Vec<_>>(),
    "value" => (1..=1_000_000).map(|x| x as f64).collect::<Vec<_>>(),
}
.unwrap();

// Lazy evaluation optimizes the entire query
let result = df
    .lazy()
    .filter(col("value").gt(500_000.0))
    .group_by([col("id").cast(DataType::String)])
    .agg([col("value").sum()])
    .collect()
    .unwrap();
```

### Type Safety

```rust
// Compile-time checks prevent bugs
let result = df
    .lazy()
    .filter(col("age").gt(25))  // Type-safe operations
    .collect()?;

// Error handling with Result<T, E>
pub fn main() -> Result<Value, String> {
    let df = load_data()?;  // ? operator for error propagation
    let result = transform(df)?;
    Ok(result)
}
```

### Zero-Copy Operations

Polars uses Apache Arrow internally - operations happen in-place when possible.

## 🎯 Comparison: Python vs Rust

### Python (Polars-py)

```python
import polars as pl

def main():
    df = pl.DataFrame({
        "name": ["Alice", "Bob"],
        "age": [25, 30]
    })

    result = (
        df
        .filter(pl.col("age") > 25)
        .group_by("name")
        .count()
    )

    return result.to_dicts()
```

### Rust (Polars-rs)

```rust
use polars::prelude::*;

pub fn main() -> Result<String, String> {
    let df = df! {
        "name" => ["Alice", "Bob"],
        "age" => [25, 30]
    }
    .map_err(|e| e.to_string())?;

    let result = df
        .lazy()
        .filter(col("age").gt(25))
        .group_by([col("name")])
        .count()
        .collect()
        .map_err(|e| e.to_string())?;

    Ok(format!("{}", result))
}
```

**Key Differences:**
- Python: Duck typing, runtime errors, easier syntax
- Rust: Strong typing, compile-time errors, explicit error handling
- API is ~95% the same!

## 🔥 When to Use Rust + Polars

**Choose Rust when:**
- Processing > 100K rows regularly
- Performance is critical (Rust is 2-10x faster)
- You want compile-time guarantees
- Memory efficiency matters
- You're learning Rust (great practice!)

**Choose Python when:**
- Quick prototyping
- One-off analysis
- Team prefers Python
- Integrating with Python-only libraries

**Pro tip:** Start in Python, port to Rust when it matters!

## 📚 Learning Path for Rust + Polars

### Week 1: Basics

**Day 1-2:** Copy example scripts into Windmill
- Run `01_basic_polars.rs`
- Modify values, see what happens
- Break things intentionally to learn

**Day 3-4:** Understand the syntax
- What is `.lazy()`?
- Why use `col("name")` instead of `"name"`?
- Error handling with `?` and `Result`

**Day 5-7:** Build something real
- Read actual CSV file
- Transform data
- Export results

### Week 2: Intermediate

- Window functions
- Joins between DataFrames
- Date/time operations
- String manipulation

### Week 3-4: Production

- Error handling strategies
- Performance optimization
- Integration with databases
- Scheduled pipelines

## 🛠️ Common Operations

### Reading Data

```rust
// From CSV string (in Windmill parameters)
let df = CsvReadOptions::default()
    .try_into_reader_with_file_handle(std::io::Cursor::new(csv_string.as_bytes()))
    .map_err(|e| e.to_string())?
    .finish()
    .map_err(|e| e.to_string())?;

// From file (if mounted in Windmill)
let df = CsvReadOptions::default()
    .try_into_reader_with_file_path(Some("data.csv".into()))
    .map_err(|e| e.to_string())?
    .finish()
    .map_err(|e| e.to_string())?;
```

### Filtering

```rust
// Simple filter
.filter(col("age").gt(25))

// Multiple conditions
.filter(
    col("age").gt(25)
        .and(col("salary").lt(100000))
)

// String operations
.filter(col("name").str().contains(lit("Alice")))
```

### Aggregations

```rust
.group_by([col("department")])
.agg([
    col("salary").sum().alias("total_salary"),
    col("salary").mean().alias("avg_salary"),
    col("salary").min().alias("min_salary"),
    col("salary").max().alias("max_salary"),
    col("employee_id").count().alias("employee_count"),
])
```

### Adding Columns

```rust
// Simple calculation
.with_column(
    (col("salary") * lit(1.1)).alias("salary_with_raise")
)

// Conditional logic
.with_column(
    when(col("age").gt(30))
        .then(lit("Senior"))
        .otherwise(lit("Junior"))
        .alias("level")
)
```

### Sorting

```rust
// Ascending
.sort("salary", SortOptions::default())

// Descending
.sort("salary", SortOptions::default().with_order_descending(true))

// Multiple columns
.sort_by_exprs(
    &[col("department"), col("salary")],
    &[false, true], // ascending department, descending salary
)
```

## 🔧 Troubleshooting

### "Feature not enabled"

Add the feature to dependencies:
```toml
polars = { version = "0.44", features = ["lazy", "csv", "strings", "dtype-date"] }
```

### "Type mismatch"

Polars is strict about types:
```rust
// Bad
col("age") + "5"

// Good
col("age") + lit(5)
```

### "Cannot move out of borrowed content"

Use `.clone()` when needed:
```rust
let df_copy = df.clone();
```

### Build takes forever

First build compiles everything (~1-2 minutes). Subsequent runs use cache and are fast.

## 🎓 Next Steps

1. **Run the examples**: Go through all 4 examples in `rust_examples/`
2. **Build a real ETL**:
   - Fetch data from an API (use `reqwest` crate)
   - Transform with Polars
   - Store in PostgreSQL
3. **Create a Flow**: Chain multiple Rust scripts together
4. **Learn Rust**: Focus on chapters relevant to data processing

## 📖 Resources

- **Examples**: Check `rust_examples/` directory
- **Polars Docs**: https://docs.rs/polars/latest/polars/
- **Polars Guide**: https://docs.pola.rs/
- **Windmill Rust Docs**: https://www.windmill.dev/docs/getting_started/scripts_quickstart/rust
- **This project**: See `LEARNING_PATH.md` for full curriculum

## 💡 Pro Tips

1. **Use `.lazy()` everywhere** - It's faster and more flexible
2. **Print intermediate steps** - Use `println!("{}", df)` for debugging
3. **Start simple** - Don't try to build everything at once
4. **Test locally first** - Use `cargo new` to test before Windmill
5. **Read error messages** - Rust errors are detailed and helpful

## Your First Real Project (Next 30 minutes)

Let's build a CSV transformer:

**Goal**: Upload CSV → Transform → Return results

1. Copy `02_csv_etl.rs` to Windmill
2. Modify the transformation logic
3. Test with your own CSV data
4. Add more transformations (sorting, filtering, etc.)
5. Return the results as JSON

Then try:
- Adding parameters (filter value, column names)
- Reading from a URL
- Writing to a file
- Scheduling it to run every hour

You're ready to go! 🚀

Open http://localhost:80 and start coding!
