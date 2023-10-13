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

use crate::utils::chromium_pages::wait_until_page_fully_loaded_with_bounds;

const DEFAULT_MIN_PAGE_LOAD_WAIT_MS: u64 = 0;
const DEFAULT_MAX_PAGE_LOAD_WAIT_MS: u64 = 5000;

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
    pub page_range: Option<String>,
    pub header_template: Option<String>,
    pub footer_template: Option<String>,
    pub prefer_css_page_size: Option<bool>,

    pub min_page_load_wait_ms: Option<u64>,
    pub max_page_load_wait_ms: Option<u64>,
}

impl From<&GeneratePdfOptions> for PrintToPdfParams {
    fn from(value: &GeneratePdfOptions) -> Self {
        Self {
            landscape: value.landscape,
            display_header_footer: value.display_header_footer,
            print_background: value.print_background,
            scale: value.scale,
            paper_width: value.paper_width,
            paper_height: value.paper_height,
            margin_top: value.margin_top,
            margin_bottom: value.margin_bottom,
            margin_left: value.margin_left,
            margin_right: value.margin_right,
            page_ranges: value.page_range.clone(),
            header_template: value.header_template.clone(),
            footer_template: value.footer_template.clone(),
            prefer_css_page_size: value.prefer_css_page_size,
            transfer_mode: None,
        }
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
        options: &GeneratePdfOptions,
    ) -> anyhow::Result<Vec<u8>> {
        let browser_context_id = self
            .browser
            .create_browser_context(CreateBrowserContextParams::default())
            .await?;

        let new_page_params = CreateTargetParams::builder()
            .url("about:blank")
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

        let page_wait_handle = tokio::spawn(wait_until_page_fully_loaded_with_bounds(
            page.clone(),
            Duration::from_millis(
                options
                    .min_page_load_wait_ms
                    .unwrap_or(DEFAULT_MIN_PAGE_LOAD_WAIT_MS),
            ),
            Duration::from_millis(
                options
                    .max_page_load_wait_ms
                    .unwrap_or(DEFAULT_MAX_PAGE_LOAD_WAIT_MS),
            ),
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
