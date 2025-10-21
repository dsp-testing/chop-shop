use crate::config::Config;
use sqlx::SqlitePool;
use std::fs::File;
use std::io::Write;
use sqlx::Row;

fn must_use<T>(x: T) -> T {
    x
}

pub async fn demonstrate_sql_injection(config: &Config, pool: &SqlitePool, user_input: &str) -> String {
    // Special case for "all" - retrieve all parts in the inventory
    let query = if user_input == "all" {
        "SELECT * FROM items".to_string()
    } else {
        // Directly concatenate user_input into the SQL query string
        let q=format!("SELECT * FROM items WHERE name LIKE '%{}%'", user_input);
        q
        // must_use({
        //     let res = std::fmt::format(format_args!(
        //         "SELECT * FROM items WHERE name LIKE '%{}%'",
        //         user_input
        //     ));
        //     res
        // })
    };

    // Simulate executing the query
    println!("Executing query: {}", query);

    // Log the query to a file
    let mut file = match File::create(&config.log_file) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Error creating log file: {}", e);
            // Continue execution even if logging fails
            return "".to_string(); // Return empty to trigger "No results for" message
        }
    };
    
    if let Err(e) = writeln!(file, "Executed query: {}", query) {
        eprintln!("Error writing to log file: {}", e);
        // Continue execution even if logging fails
    }

    // Execute the query and fetch results - WITH ERROR HANDLING
    let rows = match sqlx::query(&query).fetch_all(pool).await {
        Ok(rows) => rows,
        Err(e) => {
            // Log the error
            eprintln!("Database error: {}", e);
            // Return empty string to trigger the "No results found for" message
            return "".to_string();
        }
    };

    // Format the results with our horizontal card style
    if rows.is_empty() {
        return "".to_string(); // Return empty string to trigger the no results message
    }

    let mut results = String::new();
    for row in rows {
        let id = row.get::<i32, _>("id");
        let name = row.get::<String, _>("name");
        let price = row.get::<f64, _>("price");
        let ship_date = row.get::<String, _>("estimated_ship_date");
        let supplier = row.get::<String, _>("supplier");
        
        // Directly insert values without HTML escaping
        results.push_str(&format!(
            r#"<div class="result-item">
                <div class="item-name">{}</div>
                <div class="item-details">
                    <div class="detail">
                        <span class="detail-label">Part ID</span>
                        <span class="detail-value">#{}</span>
                    </div>
                    <div class="detail">
                        <span class="detail-label">Supplier</span>
                        <span class="detail-value">{}</span>
                    </div>
                    <div class="detail">
                        <span class="detail-label">Ships</span>
                        <span class="detail-value">{}</span>
                    </div>
                </div>
                <div class="price-container">
                    <span class="price">${:.2}</span>
                </div>
                <button class="rust-button action-btn">Add to Cart</button>
            </div>"#,
            name, id, supplier, ship_date, price
        ));
    }

    results
}
