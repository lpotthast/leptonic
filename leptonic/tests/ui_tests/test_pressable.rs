// Upstream: react-aria/test/interactions/Pressable.test.js @ 99e6102368
// Upstream: react-aria/test/interactions/PressResponder.test.js @ 99e6102368
//! The `Pressable` atom: its press handling goes onto the child itself (no wrapper), merged with
//! the child's own handlers; the child becomes focusable unless disabled; a surrounding
//! `PressResponder` applies to it.
use assertr::{matchers::eq, prelude::*};
use browser_test::{browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::pages::{ElementActions, Page};

const PATH: &str = "/atoms/pressable";

/// The log of presses and clicks.
async fn log(page: &Page<'_>) -> Result<WebElement, Report> {
    page.element("#test-pressable-log").await
}

/// Clicking the child runs both its own click handler and the `Pressable`'s press handler
/// ("should should merge with existing props, not overwrite").
#[browser_test]
pub async fn merges_with_the_childs_handlers(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
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

/// A non-focusable child gets `tabindex="0"` without a wrapper element and is pressable with Enter
/// ("should automatically make child focusable", "should apply press events to child element").
#[browser_test]
pub async fn makes_the_child_focusable(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let span = page.element("#test-pressable-span").await?;
    span.wait_for_attr("tabindex", Some("0")).await?;
    assert_that!(span.parent().await?)
        .attribute("data-pressable")
        .await
        .is_none();
    span.focus().await?;
    page.send_keys(Key::Enter).await?;
    log(page).await?.wait_for_inner_text("span press").await?;
    Ok(())
}

/// A disabled `Pressable`'s child isn't made focusable and clicking it fires no press ("supports
/// isDisabled").
#[browser_test]
pub async fn disabled(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let disabled = page.element("#test-pressable-disabled").await?;
    assert_that!(disabled).attribute("tabindex").await.is_none();
    disabled.click().await?;
    log(page)
        .await?
        .inner_text_stays("", std::time::Duration::from_millis(100))
        .await?;
    Ok(())
}

/// Pressing a `Pressable` inside a `PressResponder` runs the responder's press handler first, then
/// the child's ("should handle press events on nested pressable children").
#[browser_test]
pub async fn press_responder(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element("#test-pressable-responder")
        .await?
        .click()
        .await?;
    log(page)
        .await?
        .wait_for_inner_text("responder press, inner press")
        .await?;
    Ok(())
}

/// A `PressResponder` logs one warning without a pressable child and none with one ("should warn
/// if there is no pressable child", "should not warn if there is a pressable child").
#[browser_test]
pub async fn press_responder_warns_without_pressable(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    crate::fixtures::check_health(page).await?;
    page.element("#test-pressable-mount").await?.click().await?;
    page.element("#test-responder-pressable").await?;
    assert_that!(|| warnings(page, "PressResponder"))
        .eventually_ok()
        .matches(eq(1))
        .await;
    page.settle().await?;
    assert_that!(|| warnings(page, "PressResponder"))
        .consistently_ok()
        .for_at_least(std::time::Duration::from_millis(100))
        .matches(eq(1))
        .await;
    crate::fixtures::take_warnings(page, "PressResponder", 1).await?;
    Ok(())
}

/// A `Pressable` child without a role, and one with a role that isn't interactive, are warned
/// about once each ("should warn if child does not have a role", "should warn if child does not
/// have an interactive role").
#[browser_test]
pub async fn warns_about_children_without_interactive_roles(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element("#test-pressable-mount-roles")
        .await?
        .click()
        .await?;
    page.element("#test-pressable-no-role").await?;
    page.element("#test-pressable-presentation").await?;
    assert_that!(|| warnings(page, "interactive ARIA role"))
        .eventually_ok()
        .matches(eq(2))
        .await;
    assert_that!(|| warnings(page, "interactive ARIA role"))
        .consistently_ok()
        .for_at_least(std::time::Duration::from_millis(100))
        .matches(eq(2))
        .await;
    crate::fixtures::take_warnings(page, "interactive ARIA role. Got \"presentation\"", 1).await?;
    crate::fixtures::take_warnings(page, "child must have an interactive ARIA role.", 1).await?;
    Ok(())
}

/// How many warnings containing `text` the page logged.
async fn warnings(page: &Page<'_>, text: &str) -> Result<usize, Report> {
    let warnings = crate::pages::health::diagnostics(page.low_level().driver())
        .await?
        .console_warnings;
    Ok(warnings
        .iter()
        .filter(|warning| warning.contains(text))
        .count())
}
