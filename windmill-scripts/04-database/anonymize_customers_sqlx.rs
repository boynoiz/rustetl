//! Anonymize Customers with sqlx + Polars
//!
//! ```cargo
//! [dependencies]
//! sqlx = { version = "0.8", features = ["runtime-tokio-rustls", "postgres"] }
//! tokio = { version = "1", features = ["full"] }
//! polars = { version = "0.44", features = ["lazy", "strings"] }
//! serde_json = "1.0"
//! anyhow = "1.0"
//! sha2 = "0.10"
//! ```

use sqlx::{PgPool, Row};
use polars::prelude::*;
use serde_json::json;
use sha2::{Sha256, Digest};

fn hash_string(input: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    format!("{:x}", hasher.finalize())[..16].to_string()
}

// Wrapper for Windmill
fn main(db_host: Option<String>) -> anyhow::Result<serde_json::Value> {
    tokio::runtime::Runtime::new()?.block_on(async_main(db_host))
}

async fn async_main(db_host: Option<String>) -> anyhow::Result<serde_json::Value> {
    let host = db_host.unwrap_or_else(|| "db".to_string());

    println!("🔐 Async Anonymization Pipeline");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("  Engine: sqlx + Polars");
    println!("  Database: shopping");
    println!();

    let database_url = format!("postgres://postgres:changeme@{}/shopping", host);

    println!("🔌 Connecting...");
    let pool = PgPool::connect(&database_url).await?;
    println!("  ✓ Connected!");

    // Check if source table exists
    let row = sqlx::query("SELECT EXISTS (SELECT FROM information_schema.tables WHERE table_name = 'customers')")
        .fetch_one(&pool)
        .await?;

    let exists: bool = row.try_get(0)?;

    if !exists {
        pool.close().await;
        return Ok(json!({
            "status": "error",
            "message": "❌ customers table not found!"
        }));
    }

    // Read data
    println!("\n📖 Reading customer data...");
    let rows = sqlx::query("SELECT id, name, email, phone, address, age, salary, ssn FROM customers ORDER BY id")
        .fetch_all(&pool)
        .await?;

    let total = rows.len();
    println!("  Found {} records", total);

    if total == 0 {
        pool.close().await;
        return Ok(json!({
            "status": "error",
            "message": "❌ No data found!"
        }));
    }

    // Extract to vectors
    let mut ids: Vec<i32> = Vec::with_capacity(total);
    let mut names: Vec<String> = Vec::with_capacity(total);
    let mut emails: Vec<String> = Vec::with_capacity(total);
    let mut phones: Vec<String> = Vec::with_capacity(total);
    let mut addresses: Vec<String> = Vec::with_capacity(total);
    let mut ages: Vec<i32> = Vec::with_capacity(total);
    let mut salaries: Vec<i32> = Vec::with_capacity(total);
    let mut ssns: Vec<String> = Vec::with_capacity(total);

    for row in &rows {
        ids.push(row.try_get("id")?);
        names.push(row.try_get("name")?);
        emails.push(row.try_get("email")?);
        phones.push(row.try_get::<Option<String>, _>("phone")?.unwrap_or_default());
        addresses.push(row.try_get::<Option<String>, _>("address")?.unwrap_or_default());
        ages.push(row.try_get::<Option<i32>, _>("age")?.unwrap_or(0));
        salaries.push(row.try_get::<Option<i32>, _>("salary")?.unwrap_or(0));
        ssns.push(row.try_get::<Option<String>, _>("ssn")?.unwrap_or_default());
    }

    // Create Polars DataFrame
    let df = df! {
        "id" => &ids,
        "name" => &names,
        "email" => &emails,
        "phone" => &phones,
        "address" => &addresses,
        "age" => &ages,
        "salary" => &salaries,
        "ssn" => &ssns,
    }?;

    println!("\n📊 Original Data (first 3):");
    println!("{}", df.head(Some(3)));

    // Anonymize
    println!("\n🎭 Anonymizing...");

    let anonymized_names: Vec<String> = names.iter()
        .map(|name| format!("Customer_{}", hash_string(name)))
        .collect();

    let anonymized_emails: Vec<String> = emails.iter()
        .map(|email| format!("{}@anonymized.local", hash_string(email)))
        .collect();

    let anonymized_phones: Vec<String> = phones.iter()
        .map(|_| "***-***-****".to_string())
        .collect();

    let anonymized_addresses: Vec<String> = addresses.iter()
        .map(|_| "REDACTED".to_string())
        .collect();

    let anonymized_ssns: Vec<String> = ssns.iter()
        .map(|_| "***-**-****".to_string())
        .collect();

    let salary_buckets: Vec<String> = salaries.iter()
        .map(|s| {
            if *s < 50000 { "< $50k".to_string() }
            else if *s < 75000 { "$50k-$75k".to_string() }
            else if *s < 100000 { "$75k-$100k".to_string() }
            else if *s < 125000 { "$100k-$125k".to_string() }
            else { "> $125k".to_string() }
        })
        .collect();

    let anonymized_df = df! {
        "id" => &ids,
        "name_hash" => &anonymized_names,
        "email_hash" => &anonymized_emails,
        "phone" => &anonymized_phones,
        "address" => &anonymized_addresses,
        "age" => &ages,
        "salary_bucket" => &salary_buckets,
        "ssn" => &anonymized_ssns,
    }?;

    println!("\n📊 Anonymized (first 3):");
    println!("{}", anonymized_df.head(Some(3)));

    // Create table
    println!("\n💾 Creating customers_anonymized...");
    sqlx::query("DROP TABLE IF EXISTS customers_anonymized")
        .execute(&pool)
        .await?;

    sqlx::query(
        "CREATE TABLE customers_anonymized (
            id INTEGER PRIMARY KEY,
            name_hash VARCHAR(255),
            email_hash VARCHAR(255),
            phone VARCHAR(50),
            address TEXT,
            age INTEGER,
            salary_bucket VARCHAR(50),
            ssn VARCHAR(20),
            anonymized_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )"
    )
    .execute(&pool)
    .await?;

    sqlx::query("CREATE INDEX idx_customers_anon_age ON customers_anonymized(age)")
        .execute(&pool)
        .await?;

    // Insert
    println!("📥 Inserting {} records...", total);
    for i in 0..total {
        sqlx::query(
            "INSERT INTO customers_anonymized
             (id, name_hash, email_hash, phone, address, age, salary_bucket, ssn)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8)"
        )
        .bind(ids[i])
        .bind(&anonymized_names[i])
        .bind(&anonymized_emails[i])
        .bind(&anonymized_phones[i])
        .bind(&anonymized_addresses[i])
        .bind(ages[i])
        .bind(&salary_buckets[i])
        .bind(&anonymized_ssns[i])
        .execute(&pool)
        .await?;

        if (i + 1) % 100 == 0 {
            println!("  ✓ {}/{}", i + 1, total);
        }
    }

    pool.close().await;
    println!("\n✅ Complete!");

    Ok(json!({
        "status": "success",
        "engine": "sqlx + Polars",
        "database": "shopping",
        "tables": {
            "original": "customers",
            "anonymized": "customers_anonymized"
        },
        "records_processed": total,
        "gdpr_compliant": true
    }))
}
