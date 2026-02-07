use polars::prelude::*;
use serde_json::json;

fn main() -> Result<(), Box<dyn std::error::Error>> {
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

    // Show as JSON (like Windmill output)
    let columns: Vec<String> = result
        .get_column_names()
        .iter()
        .map(|s| s.to_string())
        .collect();

    let output = json!({
        "rows": result.height(),
        "columns": columns,
        "preview": format!("{}", result)
    });

    println!("JSON Output:");
    println!("{}", serde_json::to_string_pretty(&output)?);

    Ok(())
}
