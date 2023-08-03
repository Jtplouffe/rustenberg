use std::time::Duration;

use anyhow::anyhow;
use chromiumoxide::{
    cdp::browser_protocol::{
        page::PrintToPdfParams,
        target::{CreateBrowserContextParams, CreateTargetParams},
    },
    Browser, BrowserConfig,
};
use futures::StreamExt;
use tokio::task::JoinHandle;

use crate::utils::chromium_page::wait_until_page_fully_loaded_with_bounds;

#[derive(Default)]
pub struct GeneratePdfOptions {
    pub landscape: Option<bool>,
    pub display_header_footer: Option<bool>,
    pub print_background: Option<bool>,
    pub scale: Option<f64>,
    pub paper_width: Option<f64>,
    pub paper_height: Option<f64>,
    pub margin_top: Option<f64>,
    pub margin_bottom: Option<f64>,
    pub margin_left: Option<f64>,
    pub margin_right: Option<f64>,
}

impl Into<PrintToPdfParams> for GeneratePdfOptions {
    fn into(self) -> PrintToPdfParams {
        return PrintToPdfParams {
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
            page_ranges: None,
            header_template: None,
            footer_template: None,
            prefer_css_page_size: None,
            transfer_mode: None,
        };
    }
}

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

    pub async fn generate_pdf_from_url(
        &self,
        url: &str,
        options: GeneratePdfOptions,
    ) -> anyhow::Result<Vec<u8>> {
        let browser_context_id = self
            .browser
            .create_browser_context(CreateBrowserContextParams::default())
            .await?;

        let new_page_params = CreateTargetParams::builder()
            .url(url)
            .browser_context_id(browser_context_id.clone())
            .build()
            .map_err(|err| anyhow!(err))?;

        let page = match self.browser.new_page(new_page_params).await {
            Ok(page) => page,
            Err(err) => {
                self.browser
                    .dispose_browser_context(browser_context_id)
                    .await?;
                return Err(anyhow!(err));
            }
        };

        // TODO: Take bounds from options
        let page_wait_handle = tokio::task::spawn(wait_until_page_fully_loaded_with_bounds(
            page.clone(),
            Duration::from_secs(2),
            Duration::from_secs(10),
        ));

        if let Err(err) = page.goto(url).await {
            page_wait_handle.abort();

            self.browser
                .dispose_browser_context(browser_context_id)
                .await?;

            return Err(anyhow!(err));
        }

        page_wait_handle.await??;

        // Improvements: the bytes can be streamed instead of having to await them all here.
        // By streaming them, maybe we could stream directly to the client.
        let pdf_bytes = page.pdf(options.into()).await?;

        self.browser
            .dispose_browser_context(browser_context_id)
            .await?;

        Ok(pdf_bytes)
    }
}
