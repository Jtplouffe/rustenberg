use chromiumoxide::{
    cdp::browser_protocol::{
        network::EventLoadingFinished,
        page::{EventDomContentEventFired, EventLifecycleEvent, EventLoadEventFired},
    },
    Page,
};
use futures::{future::try_join4, StreamExt};

pub(crate) async fn wait_until_page_fully_loaded(page: Page) -> anyhow::Result<()> {
    try_join4(
        wait_for_network_idle_event(&page),
        wait_for_dom_content_event(&page),
        wait_for_load_event(&page),
        wait_for_loading_finished_event(&page),
    )
    .await?;

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
