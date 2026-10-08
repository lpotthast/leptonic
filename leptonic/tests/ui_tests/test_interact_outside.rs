// Upstream: react-aria/test/interactions/useInteractOutside.test.js @ 99e6102368
use std::borrow::Cow;

use browser_test::{BrowserTest, async_trait, thirtyfour::WebDriver};
use rootcause::Report;

use crate::pages::{BaseActions, Page};

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

        // "should fire interact outside events based on pointer events".
        page.element("test-interact-outside-target")
            .await?
            .click()
            .await?;
        expect_log(&page, "").await?;
        page.element("test-interact-outside-away")
            .await?
            .click()
            .await?;
        page.wait_for_text("test-interact-outside-log", "start,outside")
            .await?;
        js_click(driver, "test-interact-outside-reset").await?;
        page.wait_for_text("test-interact-outside-log", "").await?;

        // "should only listen for the left mouse button", "should not fire interact outside if
        // there is a pointer up event without a pointer down first".
        fire_on_body(driver, "pointerdown", 1).await?;
        fire_on_body(driver, "pointerup", 1).await?;
        fire_on_body(driver, "pointerup", 0).await?;
        expect_log(&page, "").await?;

        // "does not handle pointer events if disabled".
        js_click(driver, "test-interact-outside-disable").await?;
        page.element("test-interact-outside-away")
            .await?
            .click()
            .await?;
        expect_log(&page, "").await?;

        page.expect_no_page_errors().await
    }
}

async fn fire_on_body(driver: &WebDriver, kind: &str, button: i32) -> Result<(), Report> {
    driver
        .execute(
            &format!(
                "document.body.dispatchEvent(new PointerEvent('{kind}', {{ bubbles: true, \
                 cancelable: true, pointerType: 'mouse', pointerId: 1, isPrimary: true, \
                 button: {button} }}));"
            ),
            vec![],
        )
        .await?;
    Ok(())
}

/// Clicks a button by script: no pointer events, so not an interaction outside.
async fn js_click(driver: &WebDriver, id: &str) -> Result<(), Report> {
    driver
        .execute(&format!("document.getElementById('{id}').click();"), vec![])
        .await?;
    Ok(())
}

async fn expect_log(page: &Page<'_>, expected: &str) -> Result<(), Report> {
    stays!(
        "the text of #test-interact-outside-log",
        expected.to_owned(),
        page.element("test-interact-outside-log")
            .await?
            .text()
            .await?
    );
    Ok(())
}
