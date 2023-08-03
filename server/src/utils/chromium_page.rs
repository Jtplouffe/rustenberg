use std::time::Duration;

use chromiumoxide::{
    cdp::browser_protocol::{
        network::EventLoadingFinished,
        page::{EventDomContentEventFired, EventLifecycleEvent, EventLoadEventFired},
    },
    Page,
};
use futures::{try_join, FutureExt, StreamExt};

pub(crate) async fn wait_until_page_fully_loaded_with_bounds(
    page: Page,
    min_wait_duration: Duration,
    max_wait_duration: Duration,
) -> anyhow::Result<()> {
    try_join!(
        // TODO: Find a way to merge the error from `timeout` and from the inner function
        tokio::time::timeout(max_wait_duration, wait_until_page_fully_loaded(page)),
        tokio::time::sleep(min_wait_duration).map(|_| Ok(())),
    )?
    .0?;

    Ok(())
}

pub(crate) async fn wait_until_page_fully_loaded(page: Page) -> anyhow::Result<()> {
    try_join!(
        wait_for_network_idle_event(&page),
        wait_for_dom_content_event(&page),
        wait_for_load_event(&page),
        wait_for_loading_finished_event(&page),
    )?;

    Ok(())
}

async fn wait_for_network_idle_event(page: &Page) -> anyhow::Result<()> {
    let mut listener = page.event_listener::<EventLifecycleEvent>().await?;

    while let Some(event) = listener.next().await {
        if event.name == "networkIdle" {
            break;
        }
    }

    Ok(())
}

async fn wait_for_dom_content_event(page: &Page) -> anyhow::Result<()> {
    let mut listener = page.event_listener::<EventDomContentEventFired>().await?;
    listener.next().await;

    Ok(())
}

async fn wait_for_load_event(page: &Page) -> anyhow::Result<()> {
    let mut listener = page.event_listener::<EventLoadEventFired>().await?;
    listener.next().await;

    Ok(())
}

async fn wait_for_loading_finished_event(page: &Page) -> anyhow::Result<()> {
    let mut listener = page.event_listener::<EventLoadingFinished>().await?;
    listener.next().await;

    Ok(())
}
