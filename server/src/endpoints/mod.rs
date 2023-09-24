mod conversion;

use axum::Router;

pub(crate) fn router() -> Router {
    Router::new().nest("/conversion", conversion::router())
}
