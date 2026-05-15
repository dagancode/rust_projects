use std::sync::Arc;
use std::sync::RwLock;

use axum::{routing::get, Router};
use dotenvy::dotenv;
use tracing::{debug, info};

use property_analysis::models::app::AppState;
use property_analysis::routes::{
    health::get_health,
    sales_history::{properties::get_property_sales_history, suburbs::get_suburb_sales_history},
};
use property_analysis::services::csv::load_sales_history;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let port = std::env::var("PORT").expect("PORT must be set in .env");

    info!("Welcome to the Property API");
    debug!("listening on port {port}");

    let sales_history_path = std::env::var("SALES_HISTORY_PATH")
        .expect("Failed to load path. SALES_HISTORY_PATH must be set in .env ");

    let sales_history = load_sales_history(&sales_history_path)?;

    let shared_sales_history = AppState {
        sales_history: Arc::new(RwLock::new(sales_history)),
    };

    let app = Router::new()
        .route("/health", get(get_health))
        .route("/sales-history/properties", get(get_property_sales_history))
        .route(
            "/sales-history/suburbs/{suburb}",
            get(get_suburb_sales_history),
        )
        .with_state(shared_sales_history);

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
