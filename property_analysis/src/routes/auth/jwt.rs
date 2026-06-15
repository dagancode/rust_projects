use axum::{
    body::Body,
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::IntoResponse,
    Json,
};
use jsonwebtoken::{decode, encode, Header, Validation};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::models::{app::AppState, auth::Claims, error::ApiError};

#[derive(Deserialize, Serialize)]
pub struct AccessTokenRequest {
    pub secret: String,
}

// POST /auth/token
#[axum::debug_handler]
pub async fn post_create_access_token(
    State(state): State<AppState>,
    Json(req): Json<AccessTokenRequest>,
) -> Result<(StatusCode, Json<serde_json::Value>), ApiError> {
    if req.secret != state.jwt_secret {
        Err(ApiError::Unauthorized)
    } else {
        let claims = Claims {
            exp: (chrono::Utc::now() + chrono::Duration::minutes(1)).timestamp() as usize,
            sub: "api-client".to_string(),
        };

        let token = match encode(&Header::default(), &claims, &state.encoding_key) {
            Ok(t) => Ok(t),
            Err(_) => Err(ApiError::Unauthorized),
        }?;

        Ok((
            StatusCode::CREATED,
            Json(json!({
                "exp": &claims.exp,
                "access_token": token,
            })),
        ))
    }
}

// Validates the token 'sub' and 'exp' claims if Bearer token is valid.
pub async fn validate_token(
    State(state): State<AppState>,
    request: Request<Body>,
    next: Next,
) -> impl IntoResponse {
    let headers = request.headers();

    let auth_header = headers.get("Authorization").ok_or(ApiError::Unauthorized)?;
    let auth_str = auth_header.to_str().map_err(|_| ApiError::Unauthorized)?;

    if auth_str.starts_with("Bearer ") {
        let client_token = auth_str.trim_start_matches("Bearer ");
        return match decode::<Claims>(client_token, &state.decoding_key, &Validation::default()) {
            Ok(c) => {
                if c.claims.sub == "api-client"
                    && c.claims.exp >= chrono::Utc::now().timestamp() as usize
                {
                    Ok(next.run(request).await)
                } else {
                    Err(ApiError::Unauthorized)
                }
            }
            Err(_) => Err(ApiError::Unauthorized),
        };
    }

    Err(ApiError::Unauthorized)
}
