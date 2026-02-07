//! Lazy Query Optimization - Polars' Secret Weapon
//!
//! This demonstrates why Polars is so fast: lazy evaluation
//! allows it to optimize the entire query plan before execution
//!
//! Dependencies:
//! polars = { version = "0.44", features = ["lazy", "csv"] }

use polars::prelude::*;

pub fn main() -> Result<String, String> {
    // Create a large-ish dataset
    let n = 1_000_000;
    let df = df! {
        "id" => (1..=n).collect::<Vec<_>>(),
        "value" => (1..=n).map(|x| x as f64 * 1.5).collect::<Vec<_>>(),
        "category" => (1..=n).map(|x| format!("Cat{}", x % 100)).collect::<Vec<_>>(),
    }
    .map_err(|e| e.to_string())?;

    println!("Created DataFrame with {} rows", df.height());

    // Build a lazy query
    let lazy_query = df
        .lazy()
        .filter(col("value").gt(500_000.0))
        .select([
            col("category"),
            col("value"),
        ])
        .group_by([col("category")])
        .agg([
            col("value").sum().alias("total_value"),
            col("value").count().alias("count"),
        ])
        .sort("total_value", SortOptions::default().with_order_descending(true))
        .limit(10);

    // Show the optimized query plan
    let plan = format!("{}", lazy_query.describe_optimized_plan().map_err(|e| e.to_string())?);
    println!("\nOptimized Query Plan:\n{}", plan);

    // Execute the query
    let start = std::time::Instant::now();
    let result = lazy_query.collect().map_err(|e| e.to_string())?;
    let elapsed = start.elapsed();

    println!("\nQuery executed in {:?}", elapsed);
    println!("\nTop 10 Results:");
    println!("{}", result);

    Ok(format!(
        "Processed {} rows in {:?}\nReturned {} results",
        n,
        elapsed,
        result.height()
    ))
}
