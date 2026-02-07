use polars::prelude::*;
use serde_json::{json, Value};
use std::io::Cursor;

/// Process employee data with salary adjustment
///
/// # Arguments
/// * `csv_data` - CSV string with employee data (columns: name, age, department, salary)
/// * `raise_percent` - Percentage to increase salary (e.g., 10 for 10% raise)
/// * `min_age` - Optional: Only apply raise to employees older than this age
fn main(
    csv_data: String,
    raise_percent: f64,
    min_age: Option<i32>,
) -> anyhow::Result<Value> {
    println!("Parameters:");
    println!("  Raise: {}%", raise_percent);
    println!("  Min Age Filter: {:?}", min_age.unwrap_or(0));
    println!();

    // Parse CSV from string
    let df = CsvReadOptions::default()
        .with_has_header(true)
        .try_into_reader_with_file_handle(Cursor::new(csv_data.as_bytes()))?
        .finish()?;

    println!("Original Data ({} rows):", df.height());
    println!("{}", df);
    println!();

    // Build the transformation
    let mut lazy_df = df.lazy();

    // Apply age filter if specified
    if let Some(min) = min_age {
        println!("Filtering: age > {}", min);
        lazy_df = lazy_df.filter(col("age").gt(lit(min)));
    }

    // Calculate raise
    let raise_multiplier = 1.0 + (raise_percent / 100.0);

    // Apply transformations
    let result = lazy_df
        .with_columns([
            col("salary").alias("old_salary"),
        ])
        .with_columns([
            (col("salary") * lit(raise_multiplier)).alias("new_salary"),
            (col("salary") * lit(raise_percent / 100.0)).alias("raise_amount"),
        ])
        .collect()?;

    println!("Transformed Data ({} rows):", result.height());
    println!("{}", result);
    println!();

    // Calculate summary statistics
    let total_old = result
        .column("old_salary")?
        .cast(&DataType::Float64)?
        .sum::<f64>()
        .unwrap_or(0.0);

    let total_new = result
        .column("new_salary")?
        .cast(&DataType::Float64)?
        .sum::<f64>()
        .unwrap_or(0.0);

    let total_raise = total_new - total_old;

    println!("Summary:");
    println!("  Total Old Salary: ${:.2}", total_old);
    println!("  Total New Salary: ${:.2}", total_new);
    println!("  Total Raise Cost: ${:.2}", total_raise);

    // Convert to JSON for Windmill output
    let columns: Vec<String> = result
        .get_column_names()
        .iter()
        .map(|s| s.to_string())
        .collect();

    Ok(json!({
        "summary": {
            "total_employees": result.height(),
            "raise_percent": raise_percent,
            "total_old_salary": total_old,
            "total_new_salary": total_new,
            "total_raise_cost": total_raise,
        },
        "columns": columns,
        "data": format!("{}", result),
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_salary_raise() {
        let csv = r#"name,age,department,salary
Alice,28,Engineering,75000
Bob,34,Sales,65000
Charlie,29,Engineering,82000"#;

        let result = main(csv.to_string(), 10.0, None).unwrap();
        println!("{}", serde_json::to_string_pretty(&result).unwrap());
    }
}
