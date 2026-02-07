//! Generate and Insert Fake Customer Data
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
    ssn: String,  // Sensitive data to anonymize later
}

fn main(
    num_records: Option<i32>,
    db_host: Option<String>,
) -> anyhow::Result<serde_json::Value> {
    let num = num_records.unwrap_or(1000);
    let host = db_host.unwrap_or_else(|| "db".to_string());

    println!("🎲 Generating {} fake customer records...", num);

    // Connect to Windmill's PostgreSQL
    let connection_string = format!(
        "host={} user=postgres password=changeme dbname=windmill",
        host
    );

    let mut client = Client::connect(&connection_string, NoTls)?;

    // Create customers table
    println!("📋 Creating customers table...");
    client.execute("DROP TABLE IF EXISTS customers", &[])?;
    client.execute(
        "CREATE TABLE customers (
            id SERIAL PRIMARY KEY,
            name VARCHAR(255) NOT NULL,
            email VARCHAR(255) NOT NULL,
            phone VARCHAR(50),
            address TEXT,
            age INTEGER,
            salary INTEGER,
            ssn VARCHAR(20),
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )",
        &[],
    )?;

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

    // Get some stats
    let row = client.query_one(
        "SELECT COUNT(*), AVG(age), AVG(salary) FROM customers",
        &[],
    )?;

    let count: i64 = row.get(0);
    let avg_age: Option<f64> = row.get(1);
    let avg_salary: Option<f64> = row.get(2);

    Ok(json!({
        "status": "success",
        "records_inserted": inserted,
        "total_records": count,
        "avg_age": avg_age.unwrap_or(0.0),
        "avg_salary": avg_salary.unwrap_or(0.0),
        "table": "customers",
        "note": "Data contains sensitive information (SSN) - use anonymization script next!"
    }))
}
