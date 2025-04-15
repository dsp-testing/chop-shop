mod config;
mod sql_injection;
mod clear_text_logging;

use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use actix_web::web::Query;
use sqlx::SqlitePool;
use dotenv::dotenv;
use std::env;

async fn greet() -> impl Responder {
    let html_content = include_str!("../templates/greet.html");
    HttpResponse::Ok().body(html_content)
}

async fn search_page(query: Query<std::collections::HashMap<String, String>>, config: web::Data<config::Config>, pool: web::Data<SqlitePool>) -> impl Responder {
    let mut results = String::new();
    if let Some(user_input) = query.get("query") {
        if !user_input.trim().is_empty() {
            results = sql_injection::demonstrate_sql_injection(&config, pool.get_ref(), user_input).await;
        }
    }

    let html_content = include_str!("../templates/search.html");
    let response_body = html_content.replace("{{results}}", &results);
    HttpResponse::Ok().body(response_body)
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
            .route("/", web::get().to(greet))
            .route("/search", web::get().to(search_page))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
