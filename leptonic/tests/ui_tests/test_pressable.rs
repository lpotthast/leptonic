// Upstream: react-aria/test/interactions/Pressable.test.js @ 99e6102368
// Upstream: react-aria/test/interactions/PressResponder.test.js @ 99e6102368
use std::borrow::Cow;

use assertr::prelude::*;
use browser_test::{BrowserTest, async_trait, thirtyfour::prelude::*};
use rootcause::Report;

use crate::{
    pages::{ElementActions, Page, PageActions},
    polling::{expect, wait_for},
};

/// The `Pressable` atom: its press handling goes onto the child itself (no wrapper), merged with
/// the child's own handlers; the child becomes focusable unless disabled; a surrounding
/// `PressResponder` applies to it.
pub struct PressableTests {}

#[async_trait]
impl BrowserTest<str> for PressableTests {
    fn name(&self) -> Cow<'_, str> {
        "pressable_tests".into()
    }

    async fn run(&self, driver: &WebDriver, base_url: &str) -> Result<(), Report> {
        let page = Page { driver, base_url };
        page.goto_path("/atoms/pressable").await?;

        cases!(
            merges_with_the_childs_handlers(&page),
            makes_the_child_focusable(&page),
            disabled(&page),
            press_responder(&page),
            press_responder_warns_without_pressable(&page),
        );
        Ok(())
    }
}

/// The log of presses and clicks.
async fn log(page: &Page<'_>) -> Result<WebElement, Report> {
    page.element("#test-pressable-log").await
}

/// "should should merge with existing props, not overwrite".
async fn merges_with_the_childs_handlers(page: &Page<'_>) -> Result<(), Report> {
    page.element("#test-pressable-button")
        .await?
        .click()
        .await?;
    log(page)
        .await?
        .wait_for_inner_text("button click, button press")
        .await?;
    Ok(())
}

/// "should automatically make child focusable"; the element itself is pressable (also by
/// keyboard), without a wrapper.
async fn makes_the_child_focusable(page: &Page<'_>) -> Result<(), Report> {
    let span = page.element("#test-pressable-span").await?;
    span.wait_for_attr("tabindex", Some("0")).await?;
    assert_that!(span.parent().await?.attr("data-pressable").await?).is_none();
    span.focus().await?;
    page.send_keys(Key::Enter).await?;
    log(page)
        .await?
        .wait_for_inner_text("button click, button press, span press")
        .await?;
    Ok(())
}

/// "supports isDisabled": not focusable, no press.
async fn disabled(page: &Page<'_>) -> Result<(), Report> {
    let disabled = page.element("#test-pressable-disabled").await?;
    assert_that!(disabled.attr("tabindex").await?).is_none();
    disabled.click().await?;
    log(page)
        .await?
        .inner_text_stays("button click, button press, span press")
        .await?;
    Ok(())
}

/// Inside a `PressResponder`: the responder's handler runs first.
async fn press_responder(page: &Page<'_>) -> Result<(), Report> {
    page.element("#test-pressable-responder")
        .await?
        .click()
        .await?;
    log(page)
        .await?
        .wait_for_inner_text("button click, button press, span press, responder press, inner press")
        .await?;
    Ok(())
}

/// A `PressResponder` warns once without a pressable child, not with one.
async fn press_responder_warns_without_pressable(page: &Page<'_>) -> Result<(), Report> {
    page.clear_diagnostics().await?;
    page.element("#test-pressable-mount").await?.click().await?;
    page.element("#test-responder-pressable").await?;
    wait_for("the PressResponder warnings")
        .observing(|| responder_warnings(page))
        .to_be_equal_to(1)
        .await?;
    expect("the PressResponder warnings")
        .observing(|| responder_warnings(page))
        .to_stay_equal_to(1)
        .await?;
    page.clear_diagnostics().await?;
    Ok(())
}

/// How many `PressResponder` warnings the page logged since the diagnostics were cleared.
async fn responder_warnings(page: &Page<'_>) -> Result<usize, Report> {
    let warnings = page.diagnostics().await?.console_warnings;
    Ok(warnings
        .iter()
        .filter(|warning| warning.contains("PressResponder"))
        .count())
}
