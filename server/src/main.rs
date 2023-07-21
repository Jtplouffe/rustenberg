use axum::Server;

mod endpoints;
mod services;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let mut router = endpoints::router().layer(tower_http::trace::TraceLayer::new_for_http());
    router = services::register_into_router(router).await?;

    Server::bind(&"0.0.0.0:8080".parse()?)
        .serve(router.into_make_service())
        .await?;

    Ok(())
}
