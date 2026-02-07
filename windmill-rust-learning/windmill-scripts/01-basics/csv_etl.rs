//! CSV ETL Pipeline - Read, Transform, Export
//!
//! Dependencies:
//! polars = { version = "0.44", features = ["lazy", "csv", "json"] }
//! serde = { version = "1.0", features = ["derive"] }
//! serde_json = "1.0"

use polars::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct EtlResult {
    total_rows: usize,
    filtered_rows: usize,
    summary: String,
}

/// ETL Pipeline: Read CSV data, transform, and return results
///
/// In Windmill, you can pass CSV content as a parameter
pub fn main(csv_content: String) -> Result<EtlResult, String> {
    // Extract: Parse CSV from string
    let df = CsvReadOptions::default()
        .try_into_reader_with_file_handle(std::io::Cursor::new(csv_content.as_bytes()))
        .map_err(|e| e.to_string())?
        .finish()
        .map_err(|e| e.to_string())?;

    println!("Loaded {} rows", df.height());
    let total_rows = df.height();

    // Transform: Apply business logic
    let transformed = df
        .lazy()
        // Example transformations:
        .filter(col("age").gt(18)) // Filter adults only
        .with_column(
            // Add a new computed column
            (col("salary") * lit(1.10)).alias("salary_with_raise")
        )
        .collect()
        .map_err(|e| e.to_string())?;

    let filtered_rows = transformed.height();
    println!("After filtering: {} rows", filtered_rows);

    // Load: In Windmill, you can return the data or store it
    // Here we'll just return a summary
    let summary = format!("{}", transformed);

    Ok(EtlResult {
        total_rows,
        filtered_rows,
        summary,
    })
}

// Default example for testing
#[cfg(not(target_env = "windmill"))]
fn main() {
    let sample_csv = r#"name,age,salary
Alice,25,50000
Bob,17,30000
Charlie,30,60000
Diana,22,45000"#;

    match main(sample_csv.to_string()) {
        Ok(result) => println!("{:#?}", result),
        Err(e) => eprintln!("Error: {}", e),
    }
}
