use axum::{Router, http::StatusCode, response::IntoResponse, routing::get};

pub struct HealthRoute;

impl HealthRoute {
    pub fn router() -> Router {
        Router::new().route("/health", get(Self::health_check))
    }
    async fn health_check() -> impl IntoResponse {
        (StatusCode::OK, "Ok")
    }
}
