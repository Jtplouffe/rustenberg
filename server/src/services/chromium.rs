use anyhow::anyhow;
use chromiumoxide::{
    cdp::browser_protocol::target::{CreateBrowserContextParams, CreateTargetParams},
    Browser, BrowserConfig,
};
use futures::StreamExt;
use tokio::task::JoinHandle;

use crate::utils::chromium_page::wait_until_page_fully_loaded;

pub struct ChromiumService {
    browser: Browser,
    _handle: JoinHandle<()>,
}

impl ChromiumService {
    pub async fn new() -> anyhow::Result<Self> {
        let (browser, mut handler) = Browser::launch(
            BrowserConfig::builder()
                .no_sandbox()
                .chrome_executable(
                    "/var/lib/flatpak/exports/bin/com.github.Eloston.UngoogledChromium",
                )
                .build()
                .map_err(|err| anyhow!(err))?,
        )
        .await?;

        let handle = tokio::spawn(async move {
            while let Some(h) = handler.next().await {
                if h.is_err() {
                    break;
                }
            }
        });

        Ok(Self {
            browser,
            _handle: handle,
        })
    }

    pub async fn generate_pdf_from_url(&self, url: &str) -> anyhow::Result<Vec<u8>> {
        let browser_context_id = self
            .browser
            .create_browser_context(CreateBrowserContextParams::default())
            .await?;

        let page = match self
            .browser
            .new_page(
                CreateTargetParams::builder()
                    .url(url)
                    .browser_context_id(browser_context_id.clone())
                    .build()
                    .map_err(|err| anyhow!(err))?,
            )
            .await
        {
            Ok(page) => page,
            Err(err) => {
                self.browser
                    .dispose_browser_context(browser_context_id)
                    .await?;
                return Err(anyhow!(err));
            }
        };

        let page_clone = page.clone();

        // TODO: Implement some kind of timeout mechanism, where if a timeout occurs,
        // the task's futures will stopped being polled.
        let page_wait_handle = tokio::task::spawn(wait_until_page_fully_loaded(page_clone));

        if let Err(err) = page.goto(url).await {
            self.browser
                .dispose_browser_context(browser_context_id)
                .await?;
            // TODO: stop wait_handle
            return Err(anyhow!(err));
        }

        page_wait_handle.await??;

        let pdf_bytes = page.pdf(Default::default()).await?;

        page.close().await?;
        self.browser
            .dispose_browser_context(browser_context_id)
            .await?;

        Ok(pdf_bytes)
    }
}
