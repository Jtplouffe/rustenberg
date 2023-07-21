mod convert;

use axum::Router;

pub(crate) fn router() -> Router {
    Router::new().nest("/convert", convert::router())
}
