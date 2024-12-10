use axum::http::StatusCode;
use axum::response::IntoResponse;

pub async fn not_found() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "Not Found").into_response()
}

pub async fn health_check() -> impl IntoResponse {
    (StatusCode::OK, "Healthy").into_response()
}