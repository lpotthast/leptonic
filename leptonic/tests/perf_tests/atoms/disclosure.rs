// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the atom `Disclosure` (`/perf/atoms/disclosure`): what each thing it offers
//! costs, measured (`Page::measure`) and checked against its budget
//! (`tests/budgets/atoms/disclosure.json`): mounting and unmounting it (collapsed and expanded),
//! expanding and collapsing it with the mouse, the keyboard, find in page and its controlled state,
//! the focus ring within it, and disabling it.
use assertr::prelude::*;
use browser_test::{TestGroup, browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, EventKind, Page, SyntheticEvent},
};

const PATH: &str = "/perf/atoms/disclosure";

/// Registers the performance tests of `Disclosure`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(MountAndUnmountExpanded))
        .with(fixture_test(ToggleWithTheMouse))
        .with(fixture_test(ToggleWithTheKeyboard))
        .with(fixture_test(FindInPage))
        .with(fixture_test(ControlledState))
        .with(fixture_test(FocusWithin))
        .with(fixture_test(DisableAndEnable))
}
const DISCLOSURE: &str = "#perf-disclosure";
const TRIGGER: &str = "#perf-trigger";
const PANEL: &str = "#perf-disclosure .leptonic-DisclosurePanel";

/// How often the disclosure reported an expanded change.
async fn changes(page: &Page<'_>) -> Result<f64, Report> {
    page.control("changes").await
}

/// Waits until the disclosure is `expanded`: its `data-expanded`, the trigger's `aria-expanded`
/// and the panel's `hidden`.
async fn wait_for_expanded(page: &Page<'_>, expanded: bool) -> Result<(), Report> {
    let disclosure = page.element(DISCLOSURE).await?;
    disclosure
        .wait_for_attr("data-expanded", expanded.then_some("true"))
        .await?;
    let trigger = page.element(TRIGGER).await?;
    assert_that!(trigger.attr("aria-expanded").await?).is_equal_to(Some(expanded.to_string()));
    let panel = page.element(PANEL).await?;
    // The `hidden` property: `until-found` while collapsed (the attribute reads as a boolean).
    panel
        .wait_for_prop("hidden", if expanded { "false" } else { "until-found" })
        .await?;
    Ok(())
}

/// Focuses the trigger with the keyboard (Tab from the plain button before it).
async fn focus_with_tab(page: &Page<'_>) -> Result<WebElement, Report> {
    page.element("#perf-before").await?.click().await?;
    page.send_keys(Key::Tab).await?;
    let trigger = page.element(TRIGGER).await?;
    page.wait_for_focus(&trigger).await?;
    Ok(trigger)
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

/// Mounting the expanded disclosure (its panel shown) and unmounting it.
#[browser_test]
pub async fn mount_and_unmount_expanded(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(DISCLOSURE, 0).await?;
    page.control("expand").await?;
    page.measure("mount", page.control("mount")).await?;
    wait_for_expanded(page, true).await?;
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
    assert_that!(changes(page).await?).is_equal_to(2.0);
    Ok(())
}

/// Enter expanding and Space collapsing, the trigger focused.
#[browser_test]
pub async fn toggle_with_the_keyboard(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    focus_with_tab(page).await?;
    page.measure("enter", page.send_keys(Key::Enter)).await?;
    wait_for_expanded(page, true).await?;
    page.measure("space", page.send_keys(" ")).await?;
    wait_for_expanded(page, false).await?;
    assert_that!(changes(page).await?).is_equal_to(2.0);
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
    assert_that!(changes(page).await?).is_equal_to(1.0);
    Ok(())
}

/// The app expanding and collapsing the disclosure (its controlled state).
#[browser_test]
pub async fn controlled_state(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.measure("expand", page.control("expand")).await?;
    wait_for_expanded(page, true).await?;
    page.measure("collapse", page.control("collapse")).await?;
    wait_for_expanded(page, false).await?;
    Ok(())
}

/// Tab moving focus to the trigger (a focus ring within the disclosure) and out of it.
#[browser_test]
pub async fn focus_within(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element("#perf-before").await?.click().await?;
    let disclosure = page.element(DISCLOSURE).await?;
    page.measure("focus", page.send_keys(Key::Tab)).await?;
    disclosure
        .wait_for_attr("data-focus-visible-within", Some("true"))
        .await?;
    page.measure("blur", page.send_keys(Key::Tab)).await?;
    disclosure
        .wait_for_attr("data-focus-visible-within", None)
        .await?;
    page.wait_for_focus(&page.element("#perf-after").await?)
        .await?;
    Ok(())
}

/// Disabling the disclosure, a click on its trigger it ignores, and enabling it again.
#[browser_test]
pub async fn disable_and_enable(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let disclosure = page.element(DISCLOSURE).await?;
    let trigger = page.element(TRIGGER).await?;
    page.measure("disable", page.control("disable")).await?;
    disclosure
        .wait_for_attr("data-disabled", Some("true"))
        .await?;
    page.measure("ignored click", trigger.click()).await?;
    wait_for_expanded(page, false).await?;
    assert_that!(changes(page).await?).is_equal_to(0.0);
    page.measure("enable", page.control("enable")).await?;
    disclosure.wait_for_attr("data-disabled", None).await?;
    Ok(())
}
