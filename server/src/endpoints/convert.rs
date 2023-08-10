use std::sync::Arc;

use crate::services::chromium::{ChromiumService, GeneratePdfOptions};
use axum::{
    http::header,
    response::{Html, IntoResponse},
    routing::{get, post},
    Extension, Json, Router,
};
use axum_typed_multipart::{TryFromMultipart, TypedMultipart};
use serde_json::json;
use validator::{Validate, ValidationError, ValidationErrors};

pub(crate) fn router() -> Router {
    Router::new()
        .route("/url", post(convert_url))
        .route("/html", get(convert_html))
}

#[derive(TryFromMultipart, Validate)]
struct ConvertUrlDto {
    #[validate(url)]
    url: String,
    landscape: Option<bool>,
    display_header_footer: Option<bool>,
    print_background: Option<bool>,
    #[validate(range(min = 0))]
    scale: Option<f64>,
    #[validate(range(min = 0))]
    paper_width: Option<f64>,
    #[validate(range(min = 0))]
    paper_height: Option<f64>,
    #[validate(range(min = 0))]
    margin_top: Option<f64>,
    #[validate(range(min = 0))]
    margin_bottom: Option<f64>,
    #[validate(range(min = 0))]
    margin_left: Option<f64>,
    #[validate(range(min = 0))]
    margin_right: Option<f64>,

    #[validate(range(max = 10000))]
    min_page_load_wait_ms: Option<u64>,
    #[validate(range(max = 10000))]
    max_page_load_wait_ms: Option<u64>,

    #[validate(length(min = 1, max = 42))]
    output_filename: Option<String>,
}

impl ConvertUrlDto {
    fn validate(&self) -> Result<(), ValidationErrors> {
        let mut errors = match Validate::validate(&self) {
            Err(errors) => errors,
            _ => ValidationErrors::new(),
        };

        match (self.min_page_load_wait_ms, self.max_page_load_wait_ms) {
            (Some(min_page_load_wait_ms), Some(max_page_load_wait_ms))
                if max_page_load_wait_ms < min_page_load_wait_ms =>
            {
                let error = ValidationError::new(
                    "min_page_load_wait_min must be less than max_page_load_wait_ms",
                );

                errors.add("min_page_load_wait_ms", error);
            }
            _ => {}
        };

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    fn to_generate_pdf_options(&self) -> GeneratePdfOptions {
        GeneratePdfOptions {
            landscape: self.landscape,
            display_header_footer: self.display_header_footer,
            print_background: self.print_background,
            scale: self.scale,
            paper_width: self.paper_width,
            paper_height: self.paper_height,
            margin_top: self.margin_top,
            margin_bottom: self.margin_bottom,
            margin_left: self.margin_left,
            margin_right: self.margin_right,
            min_page_load_wait_ms: self.min_page_load_wait_ms,
            max_page_load_wait_ms: self.max_page_load_wait_ms,
        }
    }
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
        .generate_pdf_from_url(&dto.url, dto.to_generate_pdf_options())
        .await
        .unwrap();

    let headers = [(header::CONTENT_TYPE, "application/pdf")];

    (headers, pdf_bytes).into_response()
}

async fn convert_html() -> Html<&'static str> {
    Html("<h1>TODO: Convert html</h1>")
}

/*
 *
 * FormData extractor:
 * - Transform the multipart form into serde json
 * - Pass the json to serde, so that it can deserialize it into the desired struct
 *
 */
