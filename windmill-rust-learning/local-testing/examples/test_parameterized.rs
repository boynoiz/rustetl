use std::fs;

fn main() {
    // Read test CSV
    let csv_data = fs::read_to_string("test_employees.csv")
        .expect("Failed to read test CSV file");

    println!("Test 1: 10% raise for everyone");
    println!("================================");
    test_salary_raise(csv_data.clone(), 10.0, None);

    println!("\n\n");

    println!("Test 2: 15% raise for employees over 30");
    println!("=========================================");
    test_salary_raise(csv_data.clone(), 15.0, Some(30));

    println!("\n\n");

    println!("Test 3: 5% raise for everyone");
    println!("==============================");
    test_salary_raise(csv_data, 5.0, None);
}

// Copy the main function from parameterized.rs
fn test_salary_raise(csv_data: String, raise_percent: f64, min_age: Option<i32>) {
    use polars::prelude::*;
    use std::io::Cursor;

    // Parse CSV
    let df = CsvReadOptions::default()
        .with_has_header(true)
        .into_reader_with_file_handle(Cursor::new(csv_data.as_bytes()))
        .finish()
        .unwrap();

    println!("Original Data ({} rows):", df.height());
    println!("{}", df);
    println!();

    // Build transformation
    let mut lazy_df = df.lazy();

    if let Some(min) = min_age {
        println!("Filtering: age > {}", min);
        lazy_df = lazy_df.filter(col("age").gt(lit(min)));
    }

    let raise_multiplier = 1.0 + (raise_percent / 100.0);

    let result = lazy_df
        .with_columns([col("salary").alias("old_salary")])
        .with_columns([
            (col("salary") * lit(raise_multiplier)).alias("new_salary"),
            (col("salary") * lit(raise_percent / 100.0)).alias("raise_amount"),
        ])
        .collect()
        .unwrap();

    println!("Transformed Data ({} rows):", result.height());
    println!("{}", result);
    println!();

    let total_old: f64 = result
        .column("old_salary")
        .unwrap()
        .as_materialized_series()
        .cast(&DataType::Float64)
        .unwrap()
        .sum()
        .unwrap();

    let total_new: f64 = result
        .column("new_salary")
        .unwrap()
        .as_materialized_series()
        .cast(&DataType::Float64)
        .unwrap()
        .sum()
        .unwrap();

    println!("Summary:");
    println!("  Total Old Salary: ${:.2}", total_old);
    println!("  Total New Salary: ${:.2}", total_new);
    println!("  Total Raise Cost: ${:.2}", total_new - total_old);
}
