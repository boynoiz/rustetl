# Parameterized Scripts

Scripts that accept input parameters, making them reusable for different scenarios.

## Scripts

### `salary_calculator.rs`
Calculate salary raises with parameters:
- **csv_data**: Employee data (name, age, department, salary)
- **raise_percent**: Percentage increase (e.g., 10 for 10%)
- **min_age**: Optional age filter

**Features**:
- CSV parsing from string input
- Conditional filtering
- Calculated columns
- Summary statistics
- JSON output

## Example Usage

```
Parameters in Windmill UI:

csv_data:
name,age,department,salary
Alice,28,Engineering,75000
Bob,34,Sales,65000
Charlie,29,Engineering,82000

raise_percent: 10
min_age: 30 (optional)
```

**Output**:
- Transformed data with new salary columns
- Total raise cost
- Average raise per employee
- Filtered employee count

## Benefits

- Reusable across different datasets
- No code changes needed
- Easy to schedule with different parameters
- Type-safe inputs

## Use Cases

- Salary planning
- Budget forecasting
- What-if analysis
- Reporting with different filters
