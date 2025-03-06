mod config;
mod sql_injection;
mod clear_text_logging;

use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use actix_web::web::Query;
use sqlx::SqlitePool;
use dotenv::dotenv;
use std::env;

async fn greet() -> impl Responder {
    HttpResponse::Ok().body(r#"
        <html>
            <body>
                <h1>Hello, world!</h1>
                <a href="/search">Search for items</a>
            </body>
        </html>
    "#)
}

async fn search_page(query: Query<std::collections::HashMap<String, String>>, config: web::Data<config::Config>, pool: web::Data<SqlitePool>) -> impl Responder {
    let mut results = String::new();
    if let Some(user_input) = query.get("query") {
        results = sql_injection::demonstrate_sql_injection(&config, pool.get_ref(), user_input).await;
    }

    HttpResponse::Ok().body(format!(r#"
        <html>
            <body>
                <h1>Search for Items</h1>
                <form action="/search" method="get">
                    <input type="text" name="query" placeholder="Search...">
                    <button type="submit">Search</button>
                </form>
                <h2>Results</h2>
                <p>{}</p>
            </body>
        </html>
    "#, results))
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
