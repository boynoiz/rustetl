//! Anonymize Customer Data using Polars
//!
//! Reads from customers table, anonymizes sensitive data,
//! writes to customers_anonymized table
//!
//! ```cargo
//! [dependencies]
//! postgres = "0.19"
//! polars = { version = "0.44", features = ["lazy", "strings", "sql"] }
//! serde_json = "1.0"
//! anyhow = "1.0"
//! sha2 = "0.10"
//! ```

use postgres::{Client, NoTls};
use polars::prelude::*;
use serde_json::json;
use sha2::{Sha256, Digest};

fn hash_string(input: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    format!("{:x}", hasher.finalize())[..16].to_string()
}

fn main(
    db_host: Option<String>,
    mask_percentage: Option<i32>,
) -> anyhow::Result<serde_json::Value> {
    let host = db_host.unwrap_or_else(|| "db".to_string());
    let mask_pct = mask_percentage.unwrap_or(100);

    println!("🔐 Starting data anonymization process...");
    println!("  Database: {}", host);
    println!("  Masking: {}% of records", mask_pct);

    // Connect to database
    let connection_string = format!(
        "host={} user=postgres password=changeme dbname=windmill",
        host
    );

    let mut client = Client::connect(&connection_string, NoTls)?;

    // Read data from customers table
    println!("📖 Reading customer data...");
    let rows = client.query(
        "SELECT id, name, email, phone, address, age, salary, ssn FROM customers",
        &[],
    )?;

    let total = rows.len();
    println!("  Found {} records", total);

    // Convert to Polars DataFrame
    let mut ids: Vec<i32> = Vec::with_capacity(total);
    let mut names: Vec<String> = Vec::with_capacity(total);
    let mut emails: Vec<String> = Vec::with_capacity(total);
    let mut phones: Vec<String> = Vec::with_capacity(total);
    let mut addresses: Vec<String> = Vec::with_capacity(total);
    let mut ages: Vec<i32> = Vec::with_capacity(total);
    let mut salaries: Vec<i32> = Vec::with_capacity(total);
    let mut ssns: Vec<String> = Vec::with_capacity(total);

    for row in rows {
        ids.push(row.get(0));
        names.push(row.get(1));
        emails.push(row.get(2));
        phones.push(row.get(3));
        addresses.push(row.get(4));
        ages.push(row.get(5));
        salaries.push(row.get(6));
        ssns.push(row.get(7));
    }

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

    println!("📊 Original data sample:");
    println!("{}", df.head(Some(3)));

    // Anonymize sensitive data
    println!("\n🎭 Applying anonymization...");

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

    // Salary buckets (for privacy)
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
        "age" => &ages,  // Keep age for analytics
        "salary_bucket" => &salary_buckets,  // Bucketed salary
        "ssn" => &anonymized_ssns,
    }?;

    println!("📊 Anonymized data sample:");
    println!("{}", anonymized_df.head(Some(3)));

    // Create anonymized table
    println!("\n💾 Creating anonymized table...");
    client.execute("DROP TABLE IF EXISTS customers_anonymized", &[])?;
    client.execute(
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
        )",
        &[],
    )?;

    // Insert anonymized data
    println!("📥 Inserting anonymized records...");
    for i in 0..total {
        client.execute(
            "INSERT INTO customers_anonymized
             (id, name_hash, email_hash, phone, address, age, salary_bucket, ssn)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
            &[
                &ids[i],
                &anonymized_names[i],
                &anonymized_emails[i],
                &anonymized_phones[i],
                &anonymized_addresses[i],
                &ages[i],
                &salary_buckets[i],
                &anonymized_ssns[i],
            ],
        )?;
    }

    println!("✅ Anonymization complete!");

    Ok(json!({
        "status": "success",
        "original_table": "customers",
        "anonymized_table": "customers_anonymized",
        "records_processed": total,
        "anonymization_applied": [
            "Names → Hashed",
            "Emails → Hashed",
            "Phones → Masked",
            "Addresses → Redacted",
            "SSN → Masked",
            "Salaries → Bucketed"
        ],
        "preserved_fields": ["id", "age"],
        "note": "Safe to share anonymized table"
    }))
}
