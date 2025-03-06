use sqlx::SqlitePool;
use std::env;
use dotenv::dotenv;
use std::fs;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv().ok();

    // Ensure the database file exists
    if !std::path::Path::new("chop_shop.db").exists() {
        fs::File::create("chop_shop.db").expect("Failed to create database file");
    }

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = SqlitePool::connect(&database_url).await.expect("Failed to create pool");

    // Create the items table and insert sample data
    sqlx::query(include_str!("../migrations/20231010_create_items_table.sql"))
        .execute(&pool)
        .await
        .expect("Failed to create items table");

    println!("Database setup completed successfully.");
    Ok(())
}
