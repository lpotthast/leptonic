// Upstream: react-aria/test/interactions/useInteractOutside.test.js @ 99e6102368
//! `use_interact_outside`: a press outside the element (not inside) fires start and the
//! interaction; other buttons and a pointer up without a pointer down don't; nothing while
//! disabled.
use browser_test::thirtyfour::prelude::*;
use rootcause::Report;

use crate::pages::{ElementActions, Page, PageActions, SyntheticEvent};

const PATH: &str = "/hooks/interact-outside";

/// The log of interactions outside.
async fn log(page: &Page<'_>) -> Result<WebElement, Report> {
    page.element("#test-interact-outside-log").await
}

/// "should fire interact outside events based on pointer events".
pub async fn pointer_events(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let log = log(page).await?;
    page.element("#test-interact-outside-target")
        .await?
        .click()
        .await?;
    log.inner_text_stays("").await?;
    page.element("#test-interact-outside-away")
        .await?
        .click()
        .await?;
    log.wait_for_inner_text("start,outside").await?;
    // A virtual click has no pointer events: not an interaction outside.
    page.element("#test-interact-outside-reset")
        .await?
        .virtual_click()
        .await?;
    log.wait_for_inner_text("").await?;
    Ok(())
}

/// "should only listen for the left mouse button", "should not fire interact outside if there is
/// a pointer up event without a pointer down first".
pub async fn left_button_only(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let body = page.element("body").await?;
    for (kind, button) in [("pointerdown", 1), ("pointerup", 1), ("pointerup", 0)] {
        body.dispatch(
            SyntheticEvent::pointer(kind)
                .with("pointerType", "mouse")
                .with("pointerId", 1)
                .with("isPrimary", true)
                .with("button", button),
        )
        .await?;
    }
    log(page).await?.inner_text_stays("").await?;
    Ok(())
}

/// "does not handle pointer events if disabled".
pub async fn disabled(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element("#test-interact-outside-disable")
        .await?
        .virtual_click()
        .await?;
    page.element("#test-interact-outside-away")
        .await?
        .click()
        .await?;
    log(page).await?.inner_text_stays("").await?;
    Ok(())
}
