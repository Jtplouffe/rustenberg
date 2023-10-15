use axum::{routing::get, Json, Router};
use serde::Serialize;

pub fn router() -> Router {
    Router::new().route("/", get(get_info))
}

#[derive(Serialize)]
struct GetInfoResponse {
    version: &'static str,
}

async fn get_info() -> Json<GetInfoResponse> {
    Json(GetInfoResponse { version: "0.0.1" })
}
