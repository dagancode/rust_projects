use std::sync::Arc;
use std::sync::RwLock;

use axum::middleware;
use axum::{routing::{get, post}, Router};
use dotenvy::dotenv;
use property_analysis::models::app::AppData;
use property_analysis::routes::auth::jwt::post_create_access_token;
use property_analysis::routes::auth::jwt::validate_token;
use property_analysis::services::csv::load_sales_history_csv_files;
use tracing::{debug, info};

use property_analysis::models::app::AppState;
use property_analysis::routes::{
    health::get_health,
    v1::{
        analysis::{
            aggregate::get_suburb_aggregate_analysis, trends::get_suburb_trend_analysis,
            value_signals::get_suburb_value_signals,
        },
        properties::listings::get_listings,
        sales_history::{
            properties::get_property_sales_history, suburbs::get_suburb_sales_history,
        },
    },
};
use property_analysis::services::csv::load_listings;

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
    let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set in .env");

    let sales_history = load_sales_history_csv_files(&sales_history_path)?;
    let property_listings = load_listings(&property_listings_path)?;

    let shared_app_state = AppState {
        data: AppData {
            sales_history: Arc::new(RwLock::new(sales_history)),
            property_listings: Arc::new(RwLock::new(property_listings)),
        },
        encoding_key: jsonwebtoken::EncodingKey::from_secret(jwt_secret.as_bytes()),
        decoding_key: jsonwebtoken::DecodingKey::from_secret(jwt_secret.as_bytes()),
        jwt_secret,
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
        .route(
            "/analysis/suburbs/{suburb}/value-signals",
            get(get_suburb_value_signals),
        )
        .route("/listings", get(get_listings))
        .layer(middleware::from_fn_with_state(shared_app_state.clone(), validate_token));

    let app_router = Router::new()
        .route("/health", get(get_health))
        .route("/auth/token", post(post_create_access_token))
        .nest("/v1", v1_routes)
        .with_state(shared_app_state);

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await?;
    axum::serve(listener, app_router).await?;

    Ok(())
}
