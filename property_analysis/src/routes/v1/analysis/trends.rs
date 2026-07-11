use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use tracing::debug;

use crate::{
    models::{
        analysis::{Sales, SuburbTrendAnalysis},
        api::{ApiResponse, MetaData},
        app::AppState,
        db::SalesTrendRow,
        error::ApiError,
        filters::RangeQuery,
    }
};

// GET /analysis/suburbs/{suburb}/trends?from=2018&to=2024
#[axum::debug_handler]
pub async fn get_suburb_trend_analysis(
    Path(suburb): Path<String>,
    Query(range): Query<RangeQuery>,
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<SuburbTrendAnalysis>>, ApiError> {
    range.validate_range_query()?;

    let rows: Vec<SalesTrendRow> = sqlx::query_as(
        r#"
        SELECT
            pd.suburb,
            psh.year,
            AVG(psh.price) AS avg_price,
            COUNT(psh.year) AS volume
        FROM property_sales_history psh
        JOIN property_detail pd ON psh.property_id = pd.id
        WHERE ($1 IS NULL OR pd.suburb ILIKE $1)
            AND ($2::int IS NULL OR psh.year >= $2)
            AND ($3::int IS NULL OR psh.year <= $3)
            GROUP BY pd.suburb, psh.year
            ORDER BY psh.year ASC    
    "#,
    )
    .bind(&suburb)
    .bind(range.from_year.map(|y| y as i32))
    .bind(range.to_year.map(|y| y as i32))
    .fetch_all(&state.db)
    .await?;

    let count = rows.len() as u32;

    let sales: Vec<Sales> = rows
        .into_iter()
        .map(|r| Sales {
            year: r.year as u16,
            avg_price: r.avg_price,
            volume: r.volume as u32,
        })
        .collect();

    debug!(
        "GET /analysis/suburbs/{}/trends -> {}",
        &suburb,
        StatusCode::OK
    );
    Ok(Json(ApiResponse {
        data: SuburbTrendAnalysis {
            suburb_name: suburb,
            sales,
        },
        meta: Some(MetaData { count }),
    }))
}
