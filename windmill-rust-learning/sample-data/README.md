# Sample Data

Test CSV files for learning and experimentation.

## Files

### `employees.csv`
Sample employee data with 15 records:
- name, age, department, salary, city
- Use for basic transformations
- Good for testing filters and aggregations

### `test_employees.csv`
Smaller dataset with 10 records:
- name, age, department, salary
- Perfect for quick tests
- Used in parameterized examples

## Usage

### In Windmill Scripts
Paste CSV content as parameter:
```rust
fn main(csv_data: String) -> anyhow::Result<Value> {
    let df = CsvReadOptions::default()
        .with_has_header(true)
        .into_reader_with_file_handle(Cursor::new(csv_data.as_bytes()))
        .finish()?;
    // ...
}
```

### In Local Testing
Read from file:
```rust
let csv_data = fs::read_to_string("sample-data/employees.csv")?;
let df = CsvReadOptions::default()
    .with_has_header(true)
    .try_into_reader_with_file_path(Some("sample-data/employees.csv".into()))?
    .finish()?;
```

## Creating Your Own

Generate custom test data:
1. Use the `generate_fake_data.rs` script
2. Export from database to CSV
3. Create manually for specific test cases

## Tips

- Keep test files small (<100 rows) for fast iteration
- Include edge cases (nulls, duplicates, outliers)
- Use realistic data formats
- Document column meanings
