use axum::{http::header, response::IntoResponse, routing::post, Json, Router};
use axum_typed_multipart::{FieldData, TryFromMultipart, TypedMultipart};
use serde_json::json;
use tempfile::NamedTempFile;
use validator::{Validate, ValidationError, ValidationErrors};

use crate::{services::manipulation, utils::temp_files::load_temp_file_fields_sorted};

pub fn router() -> Router {
    Router::new().route("/merge", post(merge))
}

#[derive(TryFromMultipart, Validate)]
struct MergeDto {
    documents: Vec<FieldData<NamedTempFile>>,
}

impl MergeDto {
    fn validate(&self) -> Result<(), ValidationErrors> {
        let mut errors = match Validate::validate(&self) {
            Err(errors) => errors,
            _ => ValidationErrors::new(),
        };

        if self.documents.len() <= 1 {
            let error = ValidationError::new("minimum 2 documents required");
            errors.add("documents", error);
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

async fn merge(TypedMultipart(dto): TypedMultipart<MergeDto>) -> impl IntoResponse {
    if let Err(err) = dto.validate() {
        return Json(json!({ "error": err.to_string() })).into_response();
    }

    let raw_documents = match load_temp_file_fields_sorted(dto.documents).await {
        Ok(raw_documents) => raw_documents,
        Err(err) => return Json(json!({ "error": err.to_string() })).into_response(),
    };

    match manipulation::merge(&raw_documents) {
        Ok(merged_document) => {
            let headers = [(header::CONTENT_TYPE, "application/pdf")];
            (headers, merged_document).into_response()
        }
        Err(err) => Json(json!({ "error": err.to_string() })).into_response(),
    }
}
