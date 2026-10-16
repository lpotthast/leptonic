// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the hook `use_tab` (`/perf/hooks/use_tab`, the second of two tabs on plain
//! `<div>`s, manual activation): what each thing it offers costs, measured (`Page::measure`) and
//! checked against its budget (`tests/budgets/hooks/use_tab.json`): mounting and unmounting it,
//! selecting it with the mouse (and its pressed state) and with the keyboard (arrow key focus,
//! Enter and Space), keyboard focus, and disabling it.
use assertr::prelude::*;
use browser_test::{TestGroup, browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/hooks/use_tab";

/// Registers the performance tests of `use_tab`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(SelectWithTheMouse))
        .with(fixture_test(HoldAndRelease))
        .with(fixture_test(SelectWithTheKeyboard))
        .with(fixture_test(KeyboardFocusAndBlur))
        .with(fixture_test(DisableAndEnable))
}
const FIRST: &str = ".perf-first";
const TAB: &str = ".perf-tab";

/// How often the selection changed.
async fn changes(page: &Page<'_>) -> Result<f64, Report> {
    page.control("changes").await
}

/// Mounting the tab and unmounting it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(TAB, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.element(TAB).await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(TAB, 0).await?;
    Ok(())
}

/// A click on the tab: it is selected.
#[browser_test]
pub async fn select_with_the_mouse(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let tab = page.element(TAB).await?;
    page.measure("click", tab.click()).await?;
    tab.wait_for_attr("aria-selected", Some("true")).await?;
    assert_that!(changes(page).await?).is_equal_to(1.0);
    Ok(())
}

/// Holding the pointer down on the tab (the pressed state; a mouse selects on press start) and
/// releasing it.
#[browser_test]
pub async fn hold_and_release(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let tab = page.element(TAB).await?;
    let held = page.measure("hold", tab.press_and_hold()).await?;
    tab.wait_for_attr("data-pressed", Some("true")).await?;
    page.measure("release", held.release()).await?;
    tab.wait_for_attr("data-pressed", None).await?;
    tab.wait_for_attr("aria-selected", Some("true")).await?;
    assert_that!(changes(page).await?).is_equal_to(1.0);
    Ok(())
}

/// The right arrow key moving focus from the first tab to the tab (not selecting it), Enter
/// selecting it, and Space selecting the first tab again (after the left arrow key).
#[browser_test]
pub async fn select_with_the_keyboard(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element("#perf-before").await?.click().await?;
    page.send_keys(Key::Tab).await?;
    let first = page.element(FIRST).await?;
    page.wait_for_focus(&first).await?;
    let tab = page.element(TAB).await?;
    page.measure("arrow right", page.send_keys(Key::Right))
        .await?;
    page.wait_for_focus(&tab).await?;
    tab.wait_for_attr("aria-selected", Some("false")).await?;
    page.measure("enter", page.send_keys(Key::Enter)).await?;
    tab.wait_for_attr("aria-selected", Some("true")).await?;
    page.send_keys(Key::Left).await?;
    page.wait_for_focus(&first).await?;
    page.measure("space", page.send_keys(" ")).await?;
    first.wait_for_attr("aria-selected", Some("true")).await?;
    assert_that!(changes(page).await?).is_equal_to(2.0);
    Ok(())
}

/// Tab moving focus to the (selected) tab and on.
#[browser_test]
pub async fn keyboard_focus_and_blur(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let tab = page.element(TAB).await?;
    tab.click().await?;
    tab.wait_for_attr("aria-selected", Some("true")).await?;
    page.element("#perf-before").await?.click().await?;
    page.measure("focus", page.send_keys(Key::Tab)).await?;
    tab.wait_for_attr("data-focused", Some("true")).await?;
    page.measure("blur", page.send_keys(Key::Tab)).await?;
    tab.wait_for_attr("data-focused", None).await?;
    Ok(())
}

/// Disabling the tab, a click it ignores, and enabling it again.
#[browser_test]
pub async fn disable_and_enable(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let tab = page.element(TAB).await?;
    page.measure("disable", page.control("disable")).await?;
    tab.wait_for_attr("aria-disabled", Some("true")).await?;
    page.measure("ignored click", tab.click()).await?;
    assert_that!(changes(page).await?).is_equal_to(0.0);
    tab.wait_for_attr("aria-selected", Some("false")).await?;
    page.measure("enable", page.control("enable")).await?;
    tab.wait_for_attr("aria-disabled", None).await?;
    Ok(())
}
