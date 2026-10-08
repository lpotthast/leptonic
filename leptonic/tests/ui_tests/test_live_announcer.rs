// No upstream: react-aria has no tests of its own for the live announcer (only components
//! The live announcer: polite and assertive announcements in one shared announcer, cleared on
//! request and removed after their timeout. Event handlers run without a reactive owner;
//! announcing must still work.
// asserting announcements); this checks leptonic's regions and messages.
use assertr::prelude::*;
use rootcause::Report;

use crate::{
    pages::{Page, PageActions},
    polling::wait_for,
};

const PATH: &str = "/hooks/live-announcer";

/// Polite and assertive announcements go into their regions of one shared announcer.
pub async fn announcements(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element("#test-la-polite").await?.click().await?;
    page.element("[data-live-announcer] [aria-live=polite] div")
        .await?;
    assert_that!(log_text(page, "polite").await?).is_equal_to("Polite hello");

    page.element("#test-la-assertive").await?.click().await?;
    page.element("[data-live-announcer] [aria-live=assertive] div")
        .await?;
    assert_that!(log_text(page, "assertive").await?).is_equal_to("Urgent hello");

    assert_that!(page.count("[data-live-announcer]").await?).is_equal_to(1);
    Ok(())
}

/// Clearing empties both regions.
pub async fn clear(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element("#test-la-polite").await?.click().await?;
    wait_for("the polite log")
        .observing(|| log_text(page, "polite"))
        .to_be_equal_to("Polite hello")
        .await?;
    page.element("#test-la-assertive").await?.click().await?;
    wait_for("the assertive log")
        .observing(|| log_text(page, "assertive"))
        .to_be_equal_to("Urgent hello")
        .await?;

    page.element("#test-la-clear").await?.click().await?;
    wait_for("the polite log")
        .observing(|| log_text(page, "polite"))
        .to_be_equal_to("")
        .await?;
    wait_for("the assertive log")
        .observing(|| log_text(page, "assertive"))
        .to_be_equal_to("")
        .await?;
    Ok(())
}

/// Announcements are removed after their timeout.
pub async fn timeout(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element("#test-la-short").await?.click().await?;
    page.element("[data-live-announcer] [aria-live=polite] div")
        .await?;
    page.wait_for_count("[data-live-announcer] [aria-live=polite] div", 0)
        .await?;
    Ok(())
}

/// The text content of a log region (it is visually hidden, so WebDriver's `text()` returns "").
async fn log_text(page: &Page<'_>, live: &str) -> Result<String, Report> {
    let log = page
        .element(format!("[data-live-announcer] [aria-live={live}]"))
        .await?;
    Ok(log.prop("textContent").await?.unwrap_or_default())
}
