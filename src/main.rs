mod config;
mod sql_injection;
mod clear_text_logging;
mod sell_parts;
mod contact_form;

use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use actix_web::web::Query;
use actix_files as fs;
use sqlx::SqlitePool;
use dotenv::dotenv;
use std::env;

async fn greet() -> impl Responder {
    let html_content = include_str!("../templates/greet.html");
    HttpResponse::Ok().content_type("text/html").body(html_content)
}

async fn search_page(Query(user_input): Query<String>, config: web::Data<config::Config>, pool: web::Data<SqlitePool>) -> impl Responder {
    let mut results = String::new();
    let mut search_performed = false;
    let mut search_term = String::new();
    
    
    if !user_input.trim().is_empty() {
        search_performed = true;
        // No HTML escaping here - pass the raw input directly
        search_term = user_input.clone();
        results = sql_injection::demonstrate_sql_injection(&config, pool.get_ref(), &user_input).await;
    }
    

    // Create response with direct string replacement (no HTML escaping)
    let html_content = include_str!("../templates/search.html");
    let response_body = html_content
        .replace("{{results}}", &results)
        .replace("{{search_performed}}", &search_performed.to_string())
        .replace("{{search_term}}", &search_term);
    
    HttpResponse::Ok().content_type("text/html").body(response_body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let config = config::Config::new();
    let pool = SqlitePool::connect(&env::var("DATABASE_URL").expect("DATABASE_URL must be set")).await.expect("Failed to create pool");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(config.clone()))
            .app_data(web::Data::new(pool.clone()))
            // Add this line to properly handle URL-encoded form data
            .app_data(web::FormConfig::default().limit(32 * 1024))
            .service(fs::Files::new("/css", "./templates/css").show_files_listing())
            .route("/", web::get().to(greet))
            .route("/search", web::get().to(search_page))
            .route("/sell", web::get().to(sell_parts::sell_page))
            .route("/sell", web::post().to(sell_parts::handle_sell_form))
            .route("/contact", web::get().to(contact_form::contact_page))
            .route("/contact", web::post().to(contact_form::handle_contact_form))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
