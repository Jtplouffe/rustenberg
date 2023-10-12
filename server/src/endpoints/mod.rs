mod conversion;
mod manipulation;

use axum::Router;

pub fn router() -> Router {
    Router::new()
        .nest("/conversion", conversion::router())
        .nest("/manipulation", manipulation::router())
}
