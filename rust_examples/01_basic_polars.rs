//! Basic Polars Example - Create and Transform DataFrame
//!
//! To use in Windmill:
//! 1. Create a new Script
//! 2. Select "Rust" as language
//! 3. Paste this code
//! 4. Windmill will automatically handle dependencies
//!
//! Dependencies (add in Windmill UI):
//! polars = { version = "0.44", features = ["lazy", "json"] }
//! serde_json = "1.0"

use polars::prelude::*;
use serde_json::{json, Value};

/// Basic Polars operations
pub fn main() -> Result<Value, String> {
    // Create a simple DataFrame
    let df = df! {
        "name" => ["Alice", "Bob", "Charlie", "Diana", "Eve"],
        "age" => [25, 30, 35, 28, 42],
        "department" => ["Engineering", "Sales", "Engineering", "HR", "Sales"],
        "salary" => [75000, 65000, 85000, 60000, 90000],
    }
    .map_err(|e| e.to_string())?;

    println!("Original DataFrame:");
    println!("{}", df);

    // Transform: Filter employees over 25 and group by department
    let result = df
        .lazy()
        .filter(col("age").gt(25))
        .group_by([col("department")])
        .agg([
            col("name").count().alias("employee_count"),
            col("salary").mean().alias("avg_salary"),
            col("age").mean().alias("avg_age"),
        ])
        .sort("avg_salary", SortOptions::default().with_order_descending(true))
        .collect()
        .map_err(|e| e.to_string())?;

    println!("\nAggregated Results:");
    println!("{}", result);

    // Convert to JSON for Windmill output
    let json_str = format!("{}", result);

    Ok(json!({
        "status": "success",
        "row_count": result.height(),
        "columns": result.get_column_names(),
        "preview": json_str,
    }))
}
