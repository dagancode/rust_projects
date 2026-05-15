use axum::http::StatusCode;

pub async fn get_health() -> (StatusCode, String) {
    (StatusCode::OK, String::from("Healthy!"))
}
