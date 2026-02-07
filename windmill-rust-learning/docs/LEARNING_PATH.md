# Learning Rust through ETL with Windmill & Polars

## Overview

This learning path helps you master Rust by building practical ETL (Extract, Transform, Load) pipelines using:
- **Windmill**: Workflow orchestration platform (Rust-based)
- **Polars**: Fast DataFrame library (Rust core, Python bindings)

## Your Current Setup

You have Windmill running locally with:
- Windmill Server (API & UI) on `localhost:80`
- PostgreSQL database
- 3 worker instances for job execution
- 1 native worker for lightweight tasks

## Learning Path: From Beginner to ETL Master

### Phase 1: Understanding the Tools (Week 1-2)

#### **1.1 Get Familiar with Windmill**

Start Windmill:
```bash
cd ~/Projects/windmill
docker compose up -d
```

Access UI: http://localhost:80

**First Tasks:**
- [ ] Create an account and workspace
- [ ] Create a simple Python script in Windmill UI
- [ ] Create a simple Bash script
- [ ] Understand the concept of "flows" (workflows)
- [ ] Try scheduling a script to run periodically

**Learn:**
- How Windmill executes scripts in isolated environments
- How to pass parameters between scripts
- How to view logs and job history

#### **1.2 Explore Polars (Python first)**

Why start with Python Polars?
- Faster to learn the API
- Better documentation
- Easy prototyping before moving to Rust

Create a simple Polars script in Windmill:
```python
import polars as pl

def main():
    # Create a sample DataFrame
    df = pl.DataFrame({
        "name": ["Alice", "Bob", "Charlie"],
        "age": [25, 30, 35],
        "city": ["NYC", "LA", "Chicago"]
    })

    # Transform: filter and aggregate
    result = df.filter(pl.col("age") > 25).group_by("city").count()

    return result.to_dicts()
```

**Practice Tasks:**
- [ ] Read CSV data
- [ ] Filter, transform, aggregate data
- [ ] Join multiple datasets
- [ ] Write results to different formats

### Phase 2: Rust Fundamentals (Week 3-4)

Before building ETL in Rust, learn the basics:

#### **2.1 Rust Basics to Focus On**

**Essential concepts for data processing:**
1. **Ownership & Borrowing** - Critical for efficient data handling
2. **Result & Option types** - Error handling in ETL
3. **Iterators** - Processing data streams
4. **Structs & Enums** - Modeling data structures
5. **Traits** - Working with Polars APIs
6. **Serde** - Serialization/deserialization (JSON, CSV)

**Learning Resources:**
- The Rust Book: Chapters 1-10
- Rust By Example (https://doc.rust-lang.org/rust-by-example/)
- Focus on practical examples, not theory

#### **2.2 First Rust Programs**

Start with simple CLI tools:

```rust
// Example: CSV reader
use std::error::Error;
use std::fs::File;
use csv::Reader;

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("data.csv")?;
    let mut rdr = Reader::from_reader(file);

    for result in rdr.records() {
        let record = result?;
        println!("{:?}", record);
    }

    Ok(())
}
```

**Practice Projects:**
- [ ] CSV parser
- [ ] JSON transformer
- [ ] Log file analyzer
- [ ] Simple HTTP API client

### Phase 3: Polars in Rust (Week 5-6)

Now combine Rust + Polars!

#### **3.1 Setup Rust Project**

```bash
cd ~/Projects
cargo new etl-learning
cd etl-learning
```

Add to `Cargo.toml`:
```toml
[dependencies]
polars = { version = "0.44", features = ["lazy", "json", "csv-file", "parquet"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
anyhow = "1.0"  # Better error handling
```

#### **3.2 First Polars ETL Script**

```rust
use polars::prelude::*;
use anyhow::Result;

fn main() -> Result<()> {
    // Extract: Read CSV
    let df = CsvReader::from_path("data.csv")?
        .infer_schema(None)
        .has_header(true)
        .finish()?;

    // Transform: Filter and aggregate
    let result = df
        .lazy()
        .filter(col("age").gt(25))
        .group_by([col("city")])
        .agg([col("name").count().alias("count")])
        .collect()?;

    // Load: Write to Parquet
    ParquetWriter::new(std::fs::File::create("output.parquet")?)
        .finish(&mut result.clone())?;

    println!("{}", result);
    Ok(())
}
```

**Key Polars Concepts:**
- **Lazy API**: Build query plan, execute efficiently
- **Expressions**: `col()`, `lit()`, operations
- **Aggregations**: `group_by()`, `agg()`
- **File formats**: CSV, Parquet, JSON

### Phase 4: ETL Projects with Windmill (Week 7-10)

Now integrate everything!

#### **4.1 Project Ideas (Easy → Advanced)**

**Project 1: Log File ETL Pipeline**
- Extract: Read log files from API/S3
- Transform: Parse, filter errors, aggregate by hour
- Load: Store in PostgreSQL
- Schedule: Run every hour

**Project 2: CSV to Database Pipeline**
- Extract: Upload CSV via Windmill UI
- Transform: Clean data, validate, enrich
- Load: Insert into PostgreSQL with proper types
- Add: Error handling and notifications

**Project 3: API Data Aggregator**
- Extract: Fetch data from multiple REST APIs
- Transform: Join datasets, calculate metrics
- Load: Generate Parquet files + summary dashboard
- Schedule: Daily runs with retry logic

**Project 4: Real-time Data Processor**
- Extract: Poll webhook or message queue
- Transform: Streaming aggregations with Polars
- Load: Time-series database (TimescaleDB)
- Monitor: Alerts on anomalies

**Project 5: Multi-source Data Warehouse**
- Extract: PostgreSQL + CSV + API + S3
- Transform: Complex joins, window functions
- Load: Star schema in data warehouse
- Orchestrate: Multi-step Windmill flow

#### **4.2 Running Rust Scripts in Windmill**

Windmill supports Rust natively! Two approaches:

**Approach A: Direct Rust Script**
In Windmill UI, create a Rust script:

```rust
//! Requires:
//! polars = { version = "0.44", features = ["lazy"] }

use polars::prelude::*;

pub fn main() -> String {
    let df = df! {
        "name" => ["Alice", "Bob"],
        "age" => [25, 30],
    }.unwrap();

    format!("{}", df)
}
```

**Approach B: Compiled Binary**
Build locally and upload as binary:
```bash
cargo build --release
# Upload binary to Windmill, execute via Bash script
```

### Phase 5: Advanced Patterns (Week 11+)

#### **5.1 Performance Optimization**

- **Lazy evaluation**: Always use `.lazy()` for query planning
- **Streaming**: Process large files without loading into memory
- **Parallel processing**: Use Polars' built-in parallelism
- **Memory profiling**: Learn to optimize memory usage

#### **5.2 Testing Your ETL Code**

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transform() {
        let df = create_test_data();
        let result = transform_data(df).unwrap();
        assert_eq!(result.height(), 10);
    }
}
```

#### **5.3 Error Handling & Monitoring**

```rust
use anyhow::{Context, Result};

fn process_pipeline() -> Result<()> {
    let df = read_csv("data.csv")
        .context("Failed to read input CSV")?;

    let transformed = transform(df)
        .context("Transformation failed")?;

    write_output(transformed)
        .context("Failed to write output")?;

    Ok(())
}
```

## Practical Weekly Plan

### Week 1-2: Setup & Basics
- Mon-Wed: Learn Windmill UI, create Python scripts
- Thu-Fri: Python Polars basics (read, filter, transform)
- Weekend: Build a simple CSV transformer in Windmill

### Week 3-4: Rust Fundamentals
- Mon-Wed: Rust basics (ownership, types, error handling)
- Thu-Fri: Practice with small CLI tools
- Weekend: Port a Python script to Rust

### Week 5-6: Polars + Rust
- Mon-Wed: Polars Rust API (lazy, expressions)
- Thu-Fri: Build standalone ETL tool
- Weekend: Compare Python vs Rust performance

### Week 7-8: First Windmill Project
- Mon-Tue: Design pipeline architecture
- Wed-Thu: Implement Extract & Transform
- Fri: Implement Load & testing
- Weekend: Deploy to Windmill, add monitoring

### Week 9-10: Advanced Projects
- Build one of the complex projects above
- Focus on: error handling, retry logic, monitoring

### Week 11+: Production-Ready
- Add comprehensive testing
- Optimize performance
- Set up CI/CD for Windmill scripts
- Document your pipelines

## Resources & References

### Documentation
- Windmill Docs: https://www.windmill.dev/docs
- Polars Rust API: https://docs.rs/polars/latest/polars/
- Polars Guide: https://docs.pola.rs/

### Code Examples
- Windmill Hub (script templates): https://hub.windmill.dev/
- Polars examples: https://github.com/pola-rs/polars-examples

### Community
- Windmill Discord
- Polars Discord
- /r/rust subreddit

## Tips for Success

1. **Start Small**: Don't jump to complex ETL immediately
2. **Iterate**: Python prototype → Rust implementation
3. **Test Early**: Write tests while building, not after
4. **Profile**: Use Polars' performance tools to optimize
5. **Read Code**: Study Windmill's Rust codebase on GitHub
6. **Track Progress**: Use `CONTEXT_NEXT_SESSION.md` for continuity

## Next Steps

1. Start Windmill: `docker compose up -d`
2. Open UI: http://localhost:80
3. Create your first Python + Polars script
4. Follow Phase 1 tasks above

Happy learning! 🦀
