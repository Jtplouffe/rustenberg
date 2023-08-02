use std::sync::Arc;

use crate::services::chromium::ChromiumService;
use axum::{
    http::header,
    response::{Html, IntoResponse},
    routing::{get, post},
    Extension, Json, Router,
};
use axum_typed_multipart::{TryFromMultipart, TypedMultipart};
use serde_json::json;
use validator::Validate;

pub(crate) fn router() -> Router {
    Router::new()
        .route("/url", post(convert_url))
        .route("/html", get(convert_html))
}

#[derive(TryFromMultipart, Validate)]
struct ConvertUrlDto {
    #[validate(url)]
    url: String,
}

async fn convert_url(
    Extension(chromium_service): Extension<Arc<ChromiumService>>,
    TypedMultipart(dto): TypedMultipart<ConvertUrlDto>,
) -> impl IntoResponse {
    if let Err(err) = dto.validate() {
        return Json(json!({
            "error": err.to_string()
        }))
        .into_response();
    }

    let pdf_bytes = chromium_service
        .generate_pdf_from_url(&dto.url)
        .await
        .unwrap();

    let headers = [(header::CONTENT_TYPE, "application/pdf")];

    (headers, pdf_bytes).into_response()
}

async fn convert_html() -> Html<&'static str> {
    Html("<h1>TODO: Convert html</h1>")
}
