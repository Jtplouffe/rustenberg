mod conversion;

use axum::Router;

pub fn router() -> Router {
    Router::new().nest("/conversion", conversion::router())
}
