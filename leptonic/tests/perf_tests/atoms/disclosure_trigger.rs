// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the atom `DisclosureTrigger` (`/perf/atoms/disclosure_trigger`): what each
//! thing it offers costs, measured (`Page::measure`) and checked against its budget
//! (`tests/budgets/atoms/disclosure_trigger.json`): mounting and unmounting it in its disclosure,
//! presses of its button with the mouse (and its pressed state) and the keyboard toggling the
//! disclosure, and a click it ignores while the disclosure is disabled.
use assertr::prelude::*;
use browser_test::{TestGroup, browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/atoms/disclosure_trigger";

/// Registers the performance tests of `DisclosureTrigger`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(PressWithTheMouse))
        .with(fixture_test(HoldAndRelease))
        .with(fixture_test(PressWithTheKeyboard))
        .with(fixture_test(IgnoredWhileDisabled))
}
const DISCLOSURE: &str = "#perf-disclosure";
const TRIGGER: &str = "#perf-trigger";

/// How often the disclosure reported an expanded change.
async fn changes(page: &Page<'_>) -> Result<f64, Report> {
    page.control("changes").await
}

/// Waits until the disclosure is `expanded` (`data-expanded`, the trigger's `aria-expanded`).
async fn wait_for_expanded(page: &Page<'_>, expanded: bool) -> Result<(), Report> {
    page.element(DISCLOSURE)
        .await?
        .wait_for_attr("data-expanded", expanded.then_some("true"))
        .await?;
    let trigger = page.element(TRIGGER).await?;
    assert_that!(trigger.attr("aria-expanded").await?).is_equal_to(Some(expanded.to_string()));
    Ok(())
}

/// Mounting the trigger (with its button) in the disclosure and unmounting it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(TRIGGER, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    wait_for_expanded(page, false).await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(TRIGGER, 0).await?;
    // Mounted again: the panel's `aria-labelledby` refers to the trigger.
    page.control("mount").await?;
    page.element(TRIGGER).await?;
    Ok(())
}

/// Clicks: expanding the disclosure, then collapsing it.
#[browser_test]
pub async fn press_with_the_mouse(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let trigger = page.element(TRIGGER).await?;
    page.measure("expand", trigger.click()).await?;
    wait_for_expanded(page, true).await?;
    page.measure("collapse", trigger.click()).await?;
    wait_for_expanded(page, false).await?;
    assert_that!(changes(page).await?).is_equal_to(2.0);
    Ok(())
}

/// Holding the pointer down on the trigger (its pressed state) and releasing it (expanding).
#[browser_test]
pub async fn hold_and_release(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let trigger = page.element(TRIGGER).await?;
    let held = page.measure("hold", trigger.press_and_hold()).await?;
    trigger.wait_for_attr("data-pressed", Some("true")).await?;
    page.measure("release", held.release()).await?;
    trigger.wait_for_attr("data-pressed", None).await?;
    wait_for_expanded(page, true).await?;
    Ok(())
}

/// Enter expanding and Space collapsing, the trigger focused.
#[browser_test]
pub async fn press_with_the_keyboard(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element("#perf-before").await?.click().await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&page.element(TRIGGER).await?).await?;
    page.measure("enter", page.send_keys(Key::Enter)).await?;
    wait_for_expanded(page, true).await?;
    page.measure("space", page.send_keys(" ")).await?;
    wait_for_expanded(page, false).await?;
    assert_that!(changes(page).await?).is_equal_to(2.0);
    Ok(())
}

/// A click the trigger ignores while the disclosure is disabled.
#[browser_test]
pub async fn ignored_while_disabled(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let trigger = page.element(TRIGGER).await?;
    page.control("disable").await?;
    trigger.wait_for_attr("data-disabled", Some("true")).await?;
    page.measure("ignored click", trigger.click()).await?;
    wait_for_expanded(page, false).await?;
    assert_that!(changes(page).await?).is_equal_to(0.0);
    Ok(())
}
