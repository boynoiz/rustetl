# Database ETL Workflow with Windmill

## Overview

This workflow demonstrates:
1. **Generate fake data** → Insert to PostgreSQL
2. **Anonymize sensitive data** → Using Polars + Rust
3. **Create Windmill Flow** → Chain scripts as DAG

## Architecture

```
┌─────────────────────────────────────────────────┐
│              Windmill PostgreSQL                │
│                                                 │
│  ┌──────────────┐      ┌────────────────────┐  │
│  │  customers   │  →   │customers_anonymized│  │
│  │              │      │                    │  │
│  │ - name       │      │ - name_hash        │  │
│  │ - email      │      │ - email_hash       │  │
│  │ - phone      │      │ - phone (masked)   │  │
│  │ - ssn        │      │ - ssn (masked)     │  │
│  │ - salary     │      │ - salary_bucket    │  │
│  └──────────────┘      └────────────────────┘  │
└─────────────────────────────────────────────────┘
         ↑                        ↑
         │                        │
    Script 1               Script 2
  (Generate)             (Anonymize)
```

## Step 1: Create Scripts in Windmill

### Script 1: Generate Fake Data

1. Open Windmill UI
2. Create new Rust script: `generate_customers`
3. Paste code from `01_generate_fake_data.rs`
4. Parameters:
   - `num_records`: 1000 (default)
   - `db_host`: "db" (default - Windmill's postgres)

### Script 2: Anonymize Data

1. Create new Rust script: `anonymize_customers`
2. Paste code from `02_anonymize_data.rs`
3. Parameters:
   - `db_host`: "db"
   - `mask_percentage`: 100

## Step 2: Test Scripts Individually

### Test Script 1:
```
Run generate_customers
Parameters:
  num_records: 100  (start small)
  db_host: db

Expected output:
  ✅ 100 records inserted
  📊 Statistics: avg age, salary
```

### Test Script 2:
```
Run anonymize_customers
Parameters:
  db_host: db
  mask_percentage: 100

Expected output:
  ✅ 100 records anonymized
  🔐 Anonymization report
```

## Step 3: Create Windmill Flow (DAG)

Windmill Flows = Airflow DAGs

### Create Flow:

1. Click **"+ Flow"** in Windmill
2. Name it: `customer_data_pipeline`
3. Add steps:

```
Step 1: Generate Data
  ├─ Script: generate_customers
  ├─ Input: { num_records: 1000 }
  └─ Output: → results.generation

Step 2: Anonymize Data
  ├─ Script: anonymize_customers
  ├─ Input: { db_host: "db" }
  ├─ Depends on: Step 1
  └─ Output: → results.anonymization

Step 3: Verify (optional)
  ├─ Script: verify_anonymization
  └─ Depends on: Step 2
```

### Flow Configuration:

- **Trigger**: Manual / Scheduled / Webhook
- **Concurrency**: 1 (don't run multiple instances)
- **Timeout**: 5 minutes
- **Retry**: 2 times on failure

## Step 4: Verify Results

### Connect to PostgreSQL:

```bash
# From host machine
docker exec -it windmill-db-1 psql -U postgres -d windmill

# Or use any PostgreSQL client
Host: localhost
Port: 5432 (if exposed, check docker-compose.yml)
User: postgres
Password: changeme
Database: windmill
```

### Check Tables:

```sql
-- Original data (sensitive)
SELECT * FROM customers LIMIT 5;

-- Anonymized data (safe)
SELECT * FROM customers_anonymized LIMIT 5;

-- Compare counts
SELECT
  (SELECT COUNT(*) FROM customers) as original_count,
  (SELECT COUNT(*) FROM customers_anonymized) as anonymized_count;

-- Age distribution (preserved)
SELECT age, COUNT(*)
FROM customers_anonymized
GROUP BY age
ORDER BY age;

-- Salary buckets (anonymized)
SELECT salary_bucket, COUNT(*)
FROM customers_anonymized
GROUP BY salary_bucket
ORDER BY salary_bucket;
```

## Anonymization Examples

### Before (Sensitive):
```
name: John Smith
email: john.smith@gmail.com
phone: 555-123-4567
address: 123 Main St, New York
ssn: 123-45-6789
salary: 85000
```

### After (Anonymized):
```
name_hash: Customer_a1b2c3d4e5f6
email_hash: a1b2c3d4e5f6@anonymized.local
phone: ***-***-****
address: REDACTED
ssn: ***-**-****
salary_bucket: $75k-$100k
```

## Real-World Use Cases

### Use Case 1: Customer Data Migration
```
Old System → Generate → Anonymize → New System
```

### Use Case 2: Test Data Generation
```
Generate → Anonymize → Dev/Staging Environment
```

### Use Case 3: Data Sharing (GDPR Compliant)
```
Production DB → Anonymize → Share with Partners
```

### Use Case 4: Analytics
```
Generate → Anonymize → Analytics Platform
(Preserve: age, location, patterns)
(Remove: PII, identifiers)
```

## Scheduling the Flow

### Option 1: Cron Schedule
```
Schedule: 0 2 * * * (daily at 2 AM)
Generate 1000 new records daily
Anonymize immediately
```

### Option 2: Webhook Trigger
```
POST /api/w/workspace/jobs/run/f/customer_data_pipeline
Trigger on: customer signup, data refresh, etc.
```

### Option 3: Manual
```
Run on-demand for testing or one-time migrations
```

## Performance Expectations

### Generate 1,000 records:
- First run: ~20-30 seconds (compile + insert)
- Cached: ~5-10 seconds (insert only)

### Anonymize 1,000 records:
- First run: ~20-30 seconds (compile + process)
- Cached: ~2-5 seconds (process only)

### Total pipeline: ~10-15 seconds (cached)

### Scale test:
- 10,000 records: ~30-60 seconds
- 100,000 records: ~5-10 minutes
- 1,000,000 records: Consider chunking

## Next Steps

1. **Add more transformations**:
   - Data validation
   - Enrichment
   - Deduplication

2. **Add notifications**:
   - Email on completion
   - Slack alerts on failure

3. **Add monitoring**:
   - Track record counts
   - Monitor memory usage
   - Alert on anomalies

4. **Production hardening**:
   - Add error handling
   - Implement retry logic
   - Add data quality checks

## Comparing to Airflow

### Airflow DAG:
```python
@dag(schedule="@daily")
def customer_pipeline():
    generate = PythonOperator(...)
    anonymize = PythonOperator(...)

    generate >> anonymize
```

### Windmill Flow:
```
Visual UI:
  Step 1 (generate) → Step 2 (anonymize)

Or Code:
  - Rust scripts (faster, safer)
  - Built-in dependency management
  - Automatic caching
  - Better resource usage
```

**Benefits over Airflow:**
- ✅ No Python environment issues
- ✅ 10x less memory usage
- ✅ Faster execution
- ✅ Type safety (Rust)
- ✅ Easier deployment

Ready to build your first ETL pipeline! 🚀
