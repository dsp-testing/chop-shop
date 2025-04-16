use sqlx::SqlitePool;
use actix_web::{web, Responder, HttpResponse};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

// Define the structure for form data
#[derive(Deserialize, Serialize)]
pub struct NewPart {
    name: String,
    price: f64,
    ship_date: String,
    supplier: String,
}

// Server-side validation
pub fn validate_part(part: &NewPart) -> Result<(), String> {
    // Validate name
    if part.name.trim().len() < 3 || part.name.trim().len() > 50 {
        return Err("Part name must be between 3 and 50 characters".to_string());
    }

    // Validate price
    if part.price <= 0.0 {
        return Err("Price must be greater than zero".to_string());
    }

    // Validate date
    match NaiveDate::parse_from_str(&part.ship_date, "%Y-%m-%d") {
        Ok(date) => {
            let today = chrono::Local::now().naive_local().date();
            if date < today {
                return Err("Ship date cannot be in the past".to_string());
            }
        },
        Err(_) => {
            return Err("Invalid date format".to_string());
        }
    }

    // Validate supplier
    if part.supplier.trim().len() < 3 || part.supplier.trim().len() > 50 {
        return Err("Supplier name must be between 3 and 50 characters".to_string());
    }

    Ok(())
}

// Insert part into database
pub async fn insert_part(part: &NewPart, pool: &SqlitePool) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO items (name, price, estimated_ship_date, supplier) VALUES (?, ?, ?, ?)"
    )
    .bind(&part.name)
    .bind(part.price)
    .bind(&part.ship_date)
    .bind(&part.supplier)
    .execute(pool)
    .await?;

    Ok(())
}

// Handler for the GET request to display the sell form
pub async fn sell_page() -> impl Responder {
    let html_content = include_str!("../templates/sell.html");
    let response_body = html_content
        .replace("{{message_type}}", "")
        .replace("{{status_message}}", "");
    
    HttpResponse::Ok().content_type("text/html").body(response_body)
}

// Handler for the POST request to process the form submission
pub async fn handle_sell_form(
    form: web::Form<NewPart>,
    pool: web::Data<SqlitePool>
) -> impl Responder {
    let html_content = include_str!("../templates/sell.html");
    let part = form.into_inner();
    
    // Validate the part data
    match validate_part(&part) {
        Ok(_) => {
            // Insert into database
            match insert_part(&part, pool.get_ref()).await {
                Ok(_) => {
                    // Success response
                    let response_body = html_content
                        .replace("{{message_type}}", "success")
                        .replace("{{status_message}}", "Part successfully added to inventory!");

                    // Show success message and display: block
                    let response_body = response_body.replace(
                        "class=\"message {{message_type}}\"", 
                        "class=\"message success\" style=\"display: block;\""
                    );
                    
                    HttpResponse::Ok().content_type("text/html").body(response_body)
                },
                Err(e) => {
                    // Database error response
                    let response_body = html_content
                        .replace("{{message_type}}", "error")
                        .replace("{{status_message}}", &format!("Database error: {}", e));

                    // Show error message and display: block
                    let response_body = response_body.replace(
                        "class=\"message {{message_type}}\"", 
                        "class=\"message error\" style=\"display: block;\""
                    );
                    
                    HttpResponse::InternalServerError().content_type("text/html").body(response_body)
                }
            }
        },
        Err(validation_error) => {
            // Validation error response
            let response_body = html_content
                .replace("{{message_type}}", "error")
                .replace("{{status_message}}", &validation_error);

            // Show error message and display: block
            let response_body = response_body.replace(
                "class=\"message {{message_type}}\"", 
                "class=\"message error\" style=\"display: block;\""
            );
            
            HttpResponse::BadRequest().content_type("text/html").body(response_body)
        }
    }
}