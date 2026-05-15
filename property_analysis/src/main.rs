use std::sync::Arc;
use std::sync::RwLock;

use axum::{routing::get, Router};
use dotenvy::dotenv;

use property_analysis::models::app::AppState;
use property_analysis::routes::{health::get_health, suburbs::get_suburb_sales_history};
use property_analysis::services::csv::load_sales_history;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    println!("Welcome to the Property API\n");

    let sales_history_path = std::env::var("SALES_HISTORY_PATH")
        .expect("Failed to load path from .env, ensure SALES_HISTORY_PATH is set. ");

    let sales_history = load_sales_history(&sales_history_path)?;

    let shared_sales_history = AppState {
        sales_history: Arc::new(RwLock::new(sales_history)),
    };

    let app = Router::new()
        .route("/health", get(get_health))
        .route(
            "/suburbs/{suburb}/sales-history",
            get(get_suburb_sales_history),
        )
        .with_state(shared_sales_history);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3030").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
