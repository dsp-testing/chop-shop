use crate::config::Config;
use sqlx::SqlitePool;
use std::fs::File;
use std::io::{self, Write};
use sqlx::Row;

pub async fn demonstrate_sql_injection(config: &Config, pool: &SqlitePool, user_input: &str) -> String {
    // Directly concatenate user_input into the SQL query string
    let query = format!("SELECT * FROM items WHERE name LIKE '%{}%'", user_input);

    // Simulate executing the query
    println!("Executing query: {}", query);

    // Log the query to a file
    let mut file = File::create(&config.log_file).expect("Unable to create log file");
    writeln!(file, "Executed query: {}", query).expect("Unable to write to log file");

    // Execute the query and fetch results
    let rows = sqlx::query(&query)
        .fetch_all(pool)
        .await
        .expect("Failed to fetch items");

    // Format the results
    let mut results = String::new();
    for row in rows {
        results.push_str(&format!(
            "ID: {}, Name: {}, Price: {}, Estimated Ship Date: {}, Supplier: {}<br>",
            row.get::<i32, _>("id"),
            row.get::<String, _>("name"),
            row.get::<f64, _>("price"),
            row.get::<String, _>("estimated_ship_date"),
            row.get::<String, _>("supplier")
        ));
    }

    results
}
