//! Generate Fake Customer Data - Shopping Database
//!
//! ```cargo
//! [dependencies]
//! postgres = "0.19"
//! fake = { version = "2.9", features = ["derive"] }
//! rand = "0.8"
//! serde = { version = "1.0", features = ["derive"] }
//! serde_json = "1.0"
//! anyhow = "1.0"
//! ```

use fake::{Fake, Faker};
use fake::faker::name::en::*;
use fake::faker::internet::en::*;
use fake::faker::phone_number::en::*;
use fake::faker::address::en::*;
use postgres::{Client, NoTls};
use serde::{Serialize, Deserialize};
use serde_json::json;

#[derive(Debug, Serialize, Deserialize)]
struct Customer {
    name: String,
    email: String,
    phone: String,
    address: String,
    age: i32,
    salary: i32,
    ssn: String,
}

fn main(
    num_records: Option<i32>,
    db_host: Option<String>,
) -> anyhow::Result<serde_json::Value> {
    let num = num_records.unwrap_or(1000);
    let host = db_host.unwrap_or_else(|| "db".to_string());

    println!("🎲 Generating {} fake customer records...", num);
    println!("💾 Database: shopping (separate from Windmill)");

    // Connect to shopping database
    let connection_string = format!(
        "host={} user=postgres password=changeme dbname=shopping",
        host
    );

    let mut client = Client::connect(&connection_string, NoTls)?;

    // Create customers table
    println!("📋 Creating customers table...");
    client.execute("DROP TABLE IF EXISTS customers CASCADE", &[])?;
    client.execute(
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
        )",
        &[],
    )?;

    // Create index for better query performance
    client.execute("CREATE INDEX idx_customers_age ON customers(age)", &[])?;
    client.execute("CREATE INDEX idx_customers_created_at ON customers(created_at)", &[])?;

    println!("📥 Inserting {} records...", num);

    let mut inserted = 0;
    for i in 0..num {
        let customer = Customer {
            name: Name().fake(),
            email: SafeEmail().fake(),
            phone: PhoneNumber().fake(),
            address: format!("{}, {}", StreetAddress().fake::<String>(), CityName().fake::<String>()),
            age: (25..65).fake(),
            salary: (30000..150000).fake(),
            ssn: format!("{:03}-{:02}-{:04}",
                (100..999).fake::<i32>(),
                (10..99).fake::<i32>(),
                (1000..9999).fake::<i32>()
            ),
        };

        client.execute(
            "INSERT INTO customers (name, email, phone, address, age, salary, ssn)
             VALUES ($1, $2, $3, $4, $5, $6, $7)",
            &[
                &customer.name,
                &customer.email,
                &customer.phone,
                &customer.address,
                &customer.age,
                &customer.salary,
                &customer.ssn,
            ],
        )?;

        inserted += 1;

        if (i + 1) % 100 == 0 {
            println!("  ✓ Inserted {}/{} records", i + 1, num);
        }
    }

    println!("✅ Successfully inserted {} customers!", inserted);

    // Get statistics
    let row = client.query_one(
        "SELECT
            COUNT(*) as total,
            ROUND(AVG(age)) as avg_age,
            ROUND(AVG(salary)) as avg_salary,
            MIN(age) as min_age,
            MAX(age) as max_age,
            MIN(salary) as min_salary,
            MAX(salary) as max_salary
         FROM customers",
        &[],
    )?;

    let count: i64 = row.get(0);
    let avg_age: Option<f64> = row.get(1);
    let avg_salary: Option<f64> = row.get(2);
    let min_age: i32 = row.get(3);
    let max_age: i32 = row.get(4);
    let min_salary: i32 = row.get(5);
    let max_salary: i32 = row.get(6);

    Ok(json!({
        "status": "success",
        "database": "shopping",
        "table": "customers",
        "records_inserted": inserted,
        "total_records": count,
        "statistics": {
            "age": {
                "average": avg_age.unwrap_or(0.0),
                "min": min_age,
                "max": max_age
            },
            "salary": {
                "average": avg_salary.unwrap_or(0.0),
                "min": min_salary,
                "max": max_salary
            }
        },
        "warning": "⚠️  Contains sensitive PII (SSN, email, phone)",
        "next_step": "Run anonymize script to create GDPR-compliant copy"
    }))
}
