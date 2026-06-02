use std::sync::Arc;
use std::sync::RwLock;

use axum::{routing::get, Router};
use dotenvy::dotenv;
use property_analysis::services::csv::load_sales_history_csv_files;
use tracing::{debug, info};

use property_analysis::models::app::AppState;
use property_analysis::routes::{
    health::get_health,
    v1::{
        analysis::{aggregate::get_suburb_aggregate_analysis, trends::get_suburb_trend_analysis},
        sales_history::{
            properties::get_property_sales_history, suburbs::get_suburb_sales_history,
        },
        properties::listings::get_listings,
    },
};
use property_analysis::services::csv::{load_listings, load_sales_history};


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let port = std::env::var("PORT").expect("PORT must be set in .env");

    info!("Welcome to the Property API");
    debug!("listening on port {port}");

    //load_sales_history_csv_files(r"C:\Users\User\source\repos\rust\rust_projects\property_analysis\property_data\sales_history");

    let sales_history_path = std::env::var("SALES_HISTORY_PATH")
        .expect("Failed to load path. SALES_HISTORY_PATH must be set in .env ");
    let property_listings_path = std::env::var("PROPERTY_LISTINGS_PATH")
        .expect("Failed to load path. PROPERTY_LISTINGS_PATH must be set in .env ");

    let sales_history = load_sales_history_csv_files(&sales_history_path)?;
    let property_listings = load_listings(&property_listings_path)?;

    let shared_app_state = AppState {
        sales_history: Arc::new(RwLock::new(sales_history)),
        property_listings: Arc::new(RwLock::new(property_listings)),
    };

    let v1_routes = Router::new()
        .route("/sales-history/properties", get(get_property_sales_history))
        .route(
            "/sales-history/suburbs/{suburb}",
            get(get_suburb_sales_history),
        )
        .route(
            "/analysis/suburbs/{suburb}/trends",
            get(get_suburb_trend_analysis),
        )
        .route(
            "/analysis/suburbs/{suburb}/aggregate",
            get(get_suburb_aggregate_analysis),
        )
        .route("/listings", get(get_listings));

    let app_router = Router::new()
        .route("/health", get(get_health))
        .nest("/v1", v1_routes)
        .with_state(shared_app_state);

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await?;
    axum::serve(listener, app_router).await?;

    Ok(())
}
