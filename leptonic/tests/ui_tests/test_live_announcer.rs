// No upstream: react-aria has no tests of its own for the live announcer (only components
// asserting announcements); this checks leptonic's regions and messages.
//! The live announcer: polite and assertive announcements in one shared announcer, cleared on
//! request and removed after their timeout. Event handlers run without a reactive owner;
//! announcing must still work.
use assertr::{matchers::eq, prelude::*};
use browser_test::browser_test;
use rootcause::Report;

use crate::pages::Page;

const PATH: &str = "/hooks/live-announcer";

/// Polite and assertive announcements go into their regions of one shared announcer.
#[browser_test]
pub async fn announcements(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element("#test-la-polite").await?.click().await?;
    page.element("[data-live-announcer] [aria-live=polite] div")
        .await?;
    assert_that!(
        page.element("[data-live-announcer] [aria-live=polite]")
            .await?
    )
    .text_content()
    .await
    .map_owned(Option::unwrap_or_default)
    .is_equal_to("Polite hello");

    page.element("#test-la-assertive").await?.click().await?;
    page.element("[data-live-announcer] [aria-live=assertive] div")
        .await?;
    assert_that!(
        page.element("[data-live-announcer] [aria-live=assertive]")
            .await?
    )
    .text_content()
    .await
    .map_owned(Option::unwrap_or_default)
    .is_equal_to("Urgent hello");

    assert_that!(page.count("[data-live-announcer]").await?).is_equal_to(1);
    Ok(())
}

/// Clearing empties both regions.
#[browser_test]
pub async fn clear(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element("#test-la-polite").await?.click().await?;
    assert_that!(|| log_text(page, "polite"))
        .eventually_ok()
        .matches(eq("Polite hello"))
        .await;
    page.element("#test-la-assertive").await?.click().await?;
    assert_that!(|| log_text(page, "assertive"))
        .eventually_ok()
        .matches(eq("Urgent hello"))
        .await;

    page.element("#test-la-clear").await?.click().await?;
    assert_that!(|| log_text(page, "polite"))
        .eventually_ok()
        .matches(eq(""))
        .await;
    assert_that!(|| log_text(page, "assertive"))
        .eventually_ok()
        .matches(eq(""))
        .await;
    Ok(())
}

/// Announcements are removed after their timeout.
#[browser_test]
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
