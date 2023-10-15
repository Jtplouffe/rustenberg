mod conversion;
mod manipulation;
mod root;

use axum::Router;

pub fn router() -> Router {
    Router::new()
        .nest("/", root::router())
        .nest("/conversion", conversion::router())
        .nest("/manipulation", manipulation::router())
}
