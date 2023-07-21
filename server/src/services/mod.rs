pub mod chromium;

use std::sync::Arc;

use axum::{Extension, Router};
use chromium::ChromiumService;

pub async fn register_into_router(mut router: Router) -> anyhow::Result<Router> {
    let chromium_service = ChromiumService::new().await?;

    router = router.layer(Extension(Arc::new(chromium_service)));

    Ok(router)
}
