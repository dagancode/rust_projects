use axum::middleware;
use axum::{
    routing::{get, post},
    Router,
};
use clap_builder::Parser;
use dotenvy::dotenv;

use property_analysis::cli::Cli;
use property_analysis::db;
use property_analysis::routes::auth::jwt::post_create_access_token;
use property_analysis::routes::auth::jwt::validate_token;
use property_analysis::services::csv::load_sales_history_directory;
use sqlx::PgPool;
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

    let cli = Cli::parse();
    if cli.force && !cli.seed {
        eprintln!("--force requires --seed");
        std::process::exit(1);
    }

    info!("Welcome to the Property API");
    debug!("Listening on port {port}");

    let sales_history_path = std::env::var("SALES_HISTORY_PATH")
        .expect("Failed to load path. SALES_HISTORY_PATH must be set in .env ");
    let property_listings_path = std::env::var("PROPERTY_LISTINGS_PATH")
        .expect("Failed to load path. PROPERTY_LISTINGS_PATH must be set in .env ");
    let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set in .env");
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env");

    let db = PgPool::connect(&database_url).await?;
    info!("Connection established to DB");
    sqlx::migrate!().run(&db).await?;
    info!("DB Migrations ran successfully");

    if cli.seed && cli.force {
        info!("Truncating tables with existing data...");
        db::seeder::truncate_listings(&db).await?;
        db::seeder::truncate_sales_history(&db).await?;
        info!("Seeding database from CSV files...");
        db::seeder::seed_listings(&db, &load_listings(&property_listings_path)?).await?;
        db::seeder::seed_sales_history(&db, &load_sales_history_directory(&sales_history_path)?).await?;
        info!("Seeding complete");
    } else if cli.seed {
        info!("Seeding database from CSV files...");
        db::seeder::seed_listings(&db, &load_listings(&property_listings_path)?).await?;
        db::seeder::seed_sales_history(&db, &load_sales_history_directory(&sales_history_path)?).await?;
        info!("Seeding complete");
    } else if cli.upsert {
        info!("Upserting data from CSV files...");
        db::seeder::upsert_listings(&db, &load_listings(&property_listings_path)?).await?;
        db::seeder::upsert_sales_history(&db, &load_sales_history_directory(&sales_history_path)?).await?;
        info!("Upsert complete");
    }

    let shared_app_state = AppState {
        db,
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
        .layer(middleware::from_fn_with_state(
            shared_app_state.clone(),
            validate_token,
        ));

    let app_router = Router::new()
        .route("/health", get(get_health))
        .route("/auth/token", post(post_create_access_token))
        .nest("/v1", v1_routes)
        .with_state(shared_app_state);

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await?;
    axum::serve(listener, app_router).await?;

    Ok(())
}
