use anyhow::anyhow;
use chromiumoxide::{Browser, BrowserConfig};
use futures::StreamExt;
use tokio::task::JoinHandle;

pub struct ChromiumService {
    browser: Browser,
    handle: JoinHandle<()>,
}

impl ChromiumService {
    pub async fn new() -> anyhow::Result<Self> {
        let (mut browser, mut handler) = Browser::launch(
            BrowserConfig::builder()
                .chrome_executable(
                    "/var/lib/flatpak/exports/bin/com.github.Eloston.UngoogledChromium",
                )
                .build()
                .map_err(|err| anyhow!(err))?,
        )
        .await?;

        let handle = tokio::task::spawn(async move {
            while let Some(h) = handler.next().await {
                if h.is_err() {
                    break;
                }
            }
        });

        Ok(Self { browser, handle })
    }

    pub async fn generate_pdf_from_url(&self, url: &str) -> anyhow::Result<Vec<u8>> {
        // TODO: Wait for readyState complete
        let page = self.browser.new_page(url).await?;
        let pdf_bytes = page.pdf(Default::default()).await?;

        Ok(pdf_bytes)
    }
}
