// Upstream: react-aria/test/interactions/useInteractOutside.test.js @ 99e6102368
use std::borrow::Cow;

use browser_test::{BrowserTest, async_trait, thirtyfour::prelude::*};
use rootcause::Report;

use crate::pages::{ElementActions, Page, PageActions, SyntheticEvent};

/// `use_interact_outside`: a press outside the element (not inside) fires start and the
/// interaction; other buttons and a pointer up without a pointer down don't; nothing while
/// disabled.
pub struct InteractOutsideTests {}

#[async_trait]
impl BrowserTest<str> for InteractOutsideTests {
    fn name(&self) -> Cow<'_, str> {
        "interact_outside_tests".into()
    }

    async fn run(&self, driver: &WebDriver, base_url: &str) -> Result<(), Report> {
        let page = Page { driver, base_url };
        page.goto_path("/hooks/interact-outside").await?;
        cases!(
            pointer_events(&page),
            left_button_only(&page),
            disabled(&page)
        );
        Ok(())
    }
}

/// The log of interactions outside.
async fn log(page: &Page<'_>) -> Result<WebElement, Report> {
    page.element("#test-interact-outside-log").await
}

/// "should fire interact outside events based on pointer events".
async fn pointer_events(page: &Page<'_>) -> Result<(), Report> {
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
async fn left_button_only(page: &Page<'_>) -> Result<(), Report> {
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
async fn disabled(page: &Page<'_>) -> Result<(), Report> {
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
