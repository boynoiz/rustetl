# How to Use the Parameterized Salary Calculator in Windmill

## Step 1: Create Script in Windmill

1. Open Windmill UI: http://localhost:80
2. Click **"+ Script"**
3. Select **"Rust"**
4. Name it: `salary_calculator`

## Step 2: Paste the Code

Copy the entire content from `windmill_parameterized.rs` file.

## Step 3: Test with Sample Data

When you run the script, Windmill will ask for parameters:

### Parameter 1: csv_data (String)
Paste this CSV:
```csv
name,age,department,salary
Alice Johnson,28,Engineering,75000
Bob Smith,34,Sales,65000
Charlie Davis,29,Engineering,82000
Diana Prince,31,Marketing,70000
Eve Wilson,26,Sales,58000
Frank Miller,45,Engineering,95000
Grace Lee,33,Marketing,72000
Henry Brown,27,Sales,61000
Ivy Chen,30,Engineering,85000
Jack Robinson,38,Sales,78000
```

### Parameter 2: raise_percent (Number)
Enter: `10` (for 10% raise)

### Parameter 3: min_age (Optional Number)
Leave empty, or enter: `30` (only raise for employees > 30)

## Step 4: Run It!

Click "Run" and watch it:
1. Parse the CSV
2. Apply filters (if any)
3. Calculate new salaries
4. Show summary statistics

## Example Scenarios

### Scenario 1: Everyone Gets 10% Raise
```
csv_data: [paste CSV above]
raise_percent: 10
min_age: [leave empty]
```

**Result:**
- 10 employees affected
- Total raise cost: $74,100

### Scenario 2: Only Senior Staff (30+) Get 15% Raise
```
csv_data: [paste CSV above]
raise_percent: 15
min_age: 30
```

**Result:**
- 5 employees affected (over 30)
- Total raise cost: $57,000

### Scenario 3: Small 5% Raise for All
```
csv_data: [paste CSV above]
raise_percent: 5
min_age: [leave empty]
```

**Result:**
- 10 employees affected
- Total raise cost: $37,050

## Output Format

The script returns JSON with:
```json
{
  "summary": {
    "total_employees": 10,
    "raise_percent": 10.0,
    "min_age_filter": null,
    "total_old_salary": 741000.0,
    "total_new_salary": 815100.0,
    "total_raise_cost": 74100.0,
    "average_raise": 7410.0
  },
  "columns": ["name", "age", "department", "salary", "old_salary", "new_salary", "raise_amount"],
  "preview": "[formatted table]"
}
```

## Tips

1. **CSV Format**: Must have header row with columns: name, age, department, salary
2. **Raise Percent**: Use whole numbers (10 = 10%, not 0.10)
3. **Min Age**: Optional - leave empty to apply raise to everyone
4. **Error Messages**: Check logs if CSV format is wrong

## Advanced: Use Your Own Data

Replace the sample CSV with your actual employee data:
- Must have: name, age, department, salary columns
- Can have additional columns (they'll be preserved)
- Salary should be numeric (no $ or commas)

Example with your data:
```csv
name,age,department,salary,employee_id,location
John Doe,35,IT,95000,EMP001,NYC
Jane Smith,29,Marketing,75000,EMP002,LA
...
```

## Schedule It

Once it works:
1. Click "Schedule"
2. Set frequency (e.g., monthly)
3. Configure input parameters
4. Windmill will run it automatically

## Next Steps

- Add email notifications for results
- Export to CSV/Excel
- Connect to HR database
- Create approval workflow
- Build dashboard with results
