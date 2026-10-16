// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the hook `use_disclosure` (`/perf/hooks/use_disclosure`): what each thing it
//! offers costs, measured (`Page::measure`) and checked against its budget
//! (`tests/budgets/hooks/use_disclosure.json`): mounting and unmounting it, its trigger toggling
//! the panel with the mouse and the keyboard, find in page, its state changing (the panel shown and
//! hidden), and disabling it.
use assertr::prelude::*;
use browser_test::{TestGroup, browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, EventKind, Page, SyntheticEvent},
};

const PATH: &str = "/perf/hooks/use_disclosure";
const DISCLOSURE: &str = "#perf-disclosure";
const TRIGGER: &str = "#perf-disclosure button";
const PANEL: &str = "#perf-disclosure [role=group]";

/// Registers the performance tests of `use_disclosure`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(ToggleWithTheMouse))
        .with(fixture_test(ToggleWithTheKeyboard))
        .with(fixture_test(FindInPage))
        .with(fixture_test(StateChanges))
        .with(fixture_test(DisableAndEnable))
}

/// Waits until the panel is `expanded`: the trigger's `aria-expanded`, the panel's `hidden`.
async fn wait_for_expanded(page: &Page<'_>, expanded: bool) -> Result<(), Report> {
    let trigger = page.element(TRIGGER).await?;
    trigger
        .wait_for_attr("aria-expanded", Some(&expanded.to_string()))
        .await?;
    let panel = page.element(PANEL).await?;
    panel
        .wait_for_prop("hidden", if expanded { "false" } else { "until-found" })
        .await?;
    Ok(())
}

/// Mounting the collapsed disclosure and unmounting it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(DISCLOSURE, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    wait_for_expanded(page, false).await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(DISCLOSURE, 0).await?;
    Ok(())
}

/// Clicking the trigger: expanding, then collapsing.
#[browser_test]
pub async fn toggle_with_the_mouse(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let trigger = page.element(TRIGGER).await?;
    page.measure("expand", trigger.click()).await?;
    wait_for_expanded(page, true).await?;
    page.measure("collapse", trigger.click()).await?;
    wait_for_expanded(page, false).await?;
    Ok(())
}

/// Enter expanding and Space collapsing, the trigger focused.
#[browser_test]
pub async fn toggle_with_the_keyboard(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element("#perf-before").await?.click().await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&page.element(TRIGGER).await?).await?;
    page.measure("enter", page.send_keys(Key::Enter)).await?;
    wait_for_expanded(page, true).await?;
    page.measure("space", page.send_keys(" ")).await?;
    wait_for_expanded(page, false).await?;
    Ok(())
}

/// Find in page matching the collapsed panel's content (`beforematch`): it expands.
#[browser_test]
pub async fn find_in_page(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let panel = page.element(PANEL).await?;
    page.measure(
        "beforematch",
        panel.dispatch(SyntheticEvent::plain(EventKind::BeforeMatch).bubbles(false)),
    )
    .await?;
    wait_for_expanded(page, true).await?;
    Ok(())
}

/// The state expanding and collapsing (`expand()`, `collapse()`): the hook shows and hides the
/// panel.
#[browser_test]
pub async fn state_changes(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.measure("expand", page.control("expand")).await?;
    wait_for_expanded(page, true).await?;
    page.measure("collapse", page.control("collapse")).await?;
    wait_for_expanded(page, false).await?;
    Ok(())
}

/// Disabling the trigger, a click it ignores, and enabling it again.
#[browser_test]
pub async fn disable_and_enable(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let trigger = page.element(TRIGGER).await?;
    page.measure("disable", page.control("disable")).await?;
    trigger.wait_for_attr("disabled", Some("true")).await?;
    page.measure("ignored click", trigger.click()).await?;
    wait_for_expanded(page, false).await?;
    page.measure("enable", page.control("enable")).await?;
    trigger.wait_for_attr("disabled", None).await?;
    assert_that!(page.element(PANEL).await?.attr("aria-hidden").await?)
        .is_equal_to(Some("true".to_owned()));
    Ok(())
}
