//! Generate Fake Customers with sqlx (Async + Fast)
//!
//! ```cargo
//! [dependencies]
//! sqlx = { version = "0.8", features = ["runtime-tokio-rustls", "postgres"] }
//! tokio = { version = "1", features = ["full"] }
//! fake = { version = "2.9", features = ["derive"] }
//! rand = "0.8"
//! serde = { version = "1.0", features = ["derive"] }
//! serde_json = "1.0"
//! anyhow = "1.0"
//! ```

use fake::Fake;
use fake::faker::name::en::*;
use fake::faker::internet::en::*;
use fake::faker::phone_number::en::*;
use fake::faker::address::en::*;
use sqlx::{PgPool, Row};
use serde_json::json;

// Wrapper to make it work with Windmill parameters
fn main(
    num_records: Option<i32>,
    db_host: Option<String>,
) -> anyhow::Result<serde_json::Value> {
    // Run async code in tokio runtime
    tokio::runtime::Runtime::new()?.block_on(async_main(num_records, db_host))
}

async fn async_main(
    num_records: Option<i32>,
    db_host: Option<String>,
) -> anyhow::Result<serde_json::Value> {
    let num = num_records.unwrap_or(1000);
    let host = db_host.unwrap_or_else(|| "db".to_string());

    println!("🚀 Async Customer Generator (sqlx)");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("  Records: {}", num);
    println!("  Database: shopping");
    println!();

    // Connection string
    let database_url = format!(
        "postgres://postgres:changeme@{}/shopping",
        host
    );

    // Create connection pool
    println!("🔌 Connecting to database...");
    let pool = PgPool::connect(&database_url).await?;
    println!("  ✓ Connected!");

    // Create table
    println!("\n📋 Creating customers table...");
    sqlx::query("DROP TABLE IF EXISTS customers CASCADE")
        .execute(&pool)
        .await?;

    sqlx::query(
        "CREATE TABLE customers (
            id SERIAL PRIMARY KEY,
            name VARCHAR(255) NOT NULL,
            email VARCHAR(255) NOT NULL,
            phone VARCHAR(50),
            address TEXT,
            age INTEGER CHECK (age >= 18 AND age <= 100),
            salary INTEGER CHECK (salary >= 0),
            ssn VARCHAR(20),
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )"
    )
    .execute(&pool)
    .await?;

    // Create indexes
    sqlx::query("CREATE INDEX idx_customers_age ON customers(age)")
        .execute(&pool)
        .await?;
    sqlx::query("CREATE INDEX idx_customers_created_at ON customers(created_at)")
        .execute(&pool)
        .await?;

    println!("  ✓ Table created with indexes");

    // Generate and insert data
    println!("\n📥 Inserting {} records...", num);

    let mut inserted = 0;
    for i in 0..num {
        let name: String = Name().fake();
        let email: String = SafeEmail().fake();
        let phone: String = PhoneNumber().fake();
        let street: String = StreetAddress().fake();
        let city: String = CityName().fake();
        let address = format!("{}, {}", street, city);
        let age: i32 = (25..65).fake();
        let salary: i32 = (30000..150000).fake();
        let ssn = format!("{:03}-{:02}-{:04}",
            (100..999).fake::<i32>(),
            (10..99).fake::<i32>(),
            (1000..9999).fake::<i32>()
        );

        sqlx::query(
            "INSERT INTO customers (name, email, phone, address, age, salary, ssn)
             VALUES ($1, $2, $3, $4, $5, $6, $7)"
        )
        .bind(&name)
        .bind(&email)
        .bind(&phone)
        .bind(&address)
        .bind(age)
        .bind(salary)
        .bind(&ssn)
        .execute(&pool)
        .await?;

        inserted += 1;

        if (i + 1) % 100 == 0 {
            println!("  ✓ Inserted {}/{}", i + 1, num);
        }
    }

    println!("\n✅ Successfully inserted {} customers!", inserted);

    // Get statistics (runtime query, not compile-time checked)
    println!("\n📊 Calculating statistics...");

    let row = sqlx::query(
        "SELECT
            COUNT(*) as count,
            ROUND(AVG(age)) as avg_age,
            ROUND(AVG(salary)) as avg_salary,
            MIN(age) as min_age,
            MAX(age) as max_age,
            MIN(salary) as min_salary,
            MAX(salary) as max_salary
         FROM customers"
    )
    .fetch_one(&pool)
    .await?;

    let count: i64 = row.try_get("count")?;
    let avg_age: Option<f64> = row.try_get("avg_age").ok();
    let avg_salary: Option<f64> = row.try_get("avg_salary").ok();
    let min_age: Option<i32> = row.try_get("min_age").ok();
    let max_age: Option<i32> = row.try_get("max_age").ok();
    let min_salary: Option<i32> = row.try_get("min_salary").ok();
    let max_salary: Option<i32> = row.try_get("max_salary").ok();

    // Close pool
    pool.close().await;

    Ok(json!({
        "status": "success",
        "engine": "sqlx (async + rustls)",
        "database": "shopping",
        "table": "customers",
        "records_inserted": inserted,
        "total_records": count,
        "statistics": {
            "age": {
                "average": avg_age.unwrap_or(0.0),
                "min": min_age.unwrap_or(0),
                "max": max_age.unwrap_or(0)
            },
            "salary": {
                "average": avg_salary.unwrap_or(0.0),
                "min": min_salary.unwrap_or(0),
                "max": max_salary.unwrap_or(0)
            }
        },
        "features": [
            "✅ Async/await",
            "✅ Connection pooling",
            "✅ Rustls (no OpenSSL)",
            "✅ Lower memory"
        ],
        "warning": "⚠️  Contains PII"
    }))
}
