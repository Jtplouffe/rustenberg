use axum::{extract::DefaultBodyLimit, Server};

mod endpoints;
mod services;
mod utils;

const MAX_BODY_SIZE: usize = 20 * 1024 * 1024;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let mut router = endpoints::router()
        .layer(tower_http::trace::TraceLayer::new_for_http())
        .layer(DefaultBodyLimit::max(MAX_BODY_SIZE));
    router = services::register_into_router(router).await?;

    Server::bind(&"0.0.0.0:8000".parse()?)
        .serve(router.into_make_service())
        .await?;

    Ok(())
}
