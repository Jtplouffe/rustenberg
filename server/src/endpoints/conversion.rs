use std::sync::Arc;

use axum::http::StatusCode;
use axum::{http::header, response::IntoResponse, routing::post, Extension, Json, Router};
use axum_typed_multipart::{FieldData, TryFromMultipart, TypedMultipart};
use serde_json::json;
use tempfile::NamedTempFile;
use validator::{Validate, ValidationError, ValidationErrors};

use crate::services::chromium::{ChromiumService, GeneratePdfOptions};
use crate::utils::temp_files::group_temp_file_fields;

pub fn router() -> Router {
    Router::new()
        .route("/url", post(convert_url))
        .route("/html", post(convert_html))
}

#[derive(TryFromMultipart, Validate)]
#[try_from_multipart(rename_all = "camelCase")]
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
    page_range: Option<String>,
    header_template: Option<String>,
    footer_template: Option<String>,
    prefer_css_page_size: Option<bool>,

    #[validate(range(min = 0, max = 10000))]
    min_page_load_time_ms: Option<u64>,
    #[validate(range(min = 0, max = 10000))]
    max_page_load_time_ms: Option<u64>,
}

impl ConvertUrlDto {
    fn validate(&self) -> Result<(), ValidationErrors> {
        let mut errors = match Validate::validate(&self) {
            Err(errors) => errors,
            _ => ValidationErrors::new(),
        };

        match (self.min_page_load_time_ms, self.max_page_load_time_ms) {
            (Some(min_page_load_time_ms), Some(max_page_load_time_ms))
                if max_page_load_time_ms < min_page_load_time_ms =>
            {
                let error = ValidationError::new(
                    "min_page_load_time_min must be less than max_page_load_time_ms",
                );

                errors.add("min_page_load_time_ms", error);
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
            page_range: self.page_range.clone(),
            header_template: self.header_template.clone(),
            footer_template: self.footer_template.clone(),
            prefer_css_page_size: self.prefer_css_page_size,
            min_page_load_time_ms: self.min_page_load_time_ms,
            max_page_load_time_ms: self.max_page_load_time_ms,
        }
    }
}

async fn convert_url(
    Extension(chromium_service): Extension<Arc<ChromiumService>>,
    TypedMultipart(dto): TypedMultipart<ConvertUrlDto>,
) -> impl IntoResponse {
    if let Err(err) = dto.validate() {
        return (
            StatusCode::UNPROCESSABLE_ENTITY,
            Json(json!({ "error": err.to_string() })),
        )
            .into_response();
    }

    let pdf_bytes_result = chromium_service
        .generate_pdf_from_url(&dto.url, &dto.to_generate_pdf_options())
        .await;

    match pdf_bytes_result {
        Ok(pdf_bytes) => {
            let headers = [(header::CONTENT_TYPE, "application/pdf")];
            (headers, pdf_bytes).into_response()
        }
        Err(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": err.to_string() })),
        )
            .into_response(),
    }
}

#[derive(TryFromMultipart, Validate)]
#[try_from_multipart(rename_all = "camelCase")]
struct ConvertHtmlDto {
    files: Vec<FieldData<NamedTempFile>>,

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
    page_range: Option<String>,
    header_template: Option<String>,
    footer_template: Option<String>,
    prefer_css_page_size: Option<bool>,

    #[validate(range(min = 0, max = 10000))]
    min_page_load_time_ms: Option<u64>,
    #[validate(range(min = 0, max = 10000))]
    max_page_load_time_ms: Option<u64>,
}

impl ConvertHtmlDto {
    fn validate(&self) -> Result<(), ValidationErrors> {
        let mut errors = match Validate::validate(&self) {
            Err(errors) => errors,
            _ => ValidationErrors::new(),
        };

        if self.files.is_empty() {
            let error = ValidationError::new("files must not be empty");
            errors.add("files", error);
        }

        match (self.min_page_load_time_ms, self.max_page_load_time_ms) {
            (Some(min_page_load_time_ms), Some(max_page_load_time_ms))
                if max_page_load_time_ms < min_page_load_time_ms =>
            {
                let error = ValidationError::new(
                    "min_page_load_time_min must be less than max_page_load_time_ms",
                );

                errors.add("min_page_load_time_ms", error);
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
            page_range: self.page_range.clone(),
            header_template: self.header_template.clone(),
            footer_template: self.footer_template.clone(),
            prefer_css_page_size: self.prefer_css_page_size,
            min_page_load_time_ms: self.min_page_load_time_ms,
            max_page_load_time_ms: self.max_page_load_time_ms,
        }
    }
}

async fn convert_html(
    Extension(chromium_service): Extension<Arc<ChromiumService>>,
    TypedMultipart(dto): TypedMultipart<ConvertHtmlDto>,
) -> impl IntoResponse {
    if let Err(err) = dto.validate() {
        return (
            StatusCode::UNPROCESSABLE_ENTITY,
            Json(json!({ "error": err.to_string() })),
        )
            .into_response();
    }

    let options = dto.to_generate_pdf_options();

    let dir = match group_temp_file_fields(dto.files).await {
        Ok(dir) => dir,
        Err(err) => return Json(json!({ "error": err.to_string() })).into_response(),
    };

    let dir_path = match dir.path().to_str() {
        Some(path) => format!("file://{path}/index.html"),
        None => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": "could not get directory path" })),
            )
                .into_response()
        }
    };

    let pdf_bytes_result = chromium_service
        .generate_pdf_from_url(&dir_path, &options)
        .await;

    match pdf_bytes_result {
        Ok(pdf_bytes) => {
            let headers = [(header::CONTENT_TYPE, "application/pdf")];
            (headers, pdf_bytes).into_response()
        }
        Err(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": err.to_string() })),
        )
            .into_response(),
    }
}
