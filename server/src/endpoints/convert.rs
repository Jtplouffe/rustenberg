use std::sync::Arc;

use crate::services::chromium::ChromiumService;
use axum::{
    http::header,
    response::{Html, IntoResponse},
    routing::get,
    Extension, Router,
};

pub(crate) fn router() -> Router {
    Router::new()
        .route("/url", get(convert_url))
        .route("/html", get(convert_html))
}

async fn convert_url(
    Extension(chromium_service): Extension<Arc<ChromiumService>>,
) -> impl IntoResponse {
    let pdf_bytes = chromium_service
        .generate_pdf_from_url("https://recursyve.io".into())
        .await
        .unwrap();

    let headers = [(header::CONTENT_TYPE, "application/pdf")];

    (headers, pdf_bytes)
}

async fn convert_html() -> Html<&'static str> {
    Html("<h1>TODO: Convert html</h1>")
}
