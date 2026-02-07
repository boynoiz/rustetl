//! Advanced Polars Transformations
//!
//! Dependencies:
//! polars = { version = "0.44", features = ["lazy", "dtype-date", "strings"] }
//! serde_json = "1.0"

use polars::prelude::*;
use serde_json::{json, Value};

/// Advanced data transformations with Polars
pub fn main() -> Result<Value, String> {
    // Create sample sales data
    let df = df! {
        "product" => ["Laptop", "Mouse", "Keyboard", "Laptop", "Mouse", "Monitor"],
        "category" => ["Electronics", "Accessories", "Accessories", "Electronics", "Accessories", "Electronics"],
        "quantity" => [2, 10, 5, 1, 15, 3],
        "price" => [1200.0, 25.0, 75.0, 1200.0, 25.0, 450.0],
        "region" => ["North", "South", "North", "East", "West", "North"],
    }
    .map_err(|e| e.to_string())?;

    println!("Original Data:");
    println!("{}", df);

    // Complex transformation pipeline
    let result = df
        .lazy()
        // Add calculated columns
        .with_column((col("quantity") * col("price")).alias("revenue"))
        // Group by multiple columns
        .group_by([col("category"), col("region")])
        .agg([
            col("revenue").sum().alias("total_revenue"),
            col("quantity").sum().alias("total_quantity"),
            col("product").n_unique().alias("unique_products"),
            col("price").mean().alias("avg_price"),
        ])
        // Sort by revenue descending
        .sort("total_revenue", SortOptions::default().with_order_descending(true))
        .collect()
        .map_err(|e| e.to_string())?;

    println!("\nTransformed Data:");
    println!("{}", result);

    // Window functions example
    let with_windows = result
        .lazy()
        .with_column(
            col("total_revenue")
                .sum()
                .over([col("category")])
                .alias("category_total_revenue")
        )
        .with_column(
            (col("total_revenue") / col("category_total_revenue") * lit(100))
                .alias("revenue_percentage")
        )
        .collect()
        .map_err(|e| e.to_string())?;

    println!("\nWith Window Functions:");
    println!("{}", with_windows);

    Ok(json!({
        "status": "success",
        "rows": with_windows.height(),
        "preview": format!("{}", with_windows),
    }))
}
