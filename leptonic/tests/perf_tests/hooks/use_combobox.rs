// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the hook `use_combobox` (`/perf/hooks/use_combobox`, on plain elements): what
//! each thing it offers costs, measured (`Page::measure`) and checked against its budget
//! (`tests/budgets/hooks/use_combobox.json`): mounting and unmounting the combo box's elements,
//! opening it with its button and selecting an option with the mouse, typing (filtering the
//! options), the keyboard (opening, moving through the options, selecting, leaving), Escape closing
//! it, and its disabled input changing, with 10 options and with 1,000.
//!
//! `use_combobox_state`'s performance test uses the same fixture and the helpers here.
use assertr::prelude::*;
use browser_test::{TestGroup, browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/hooks/use_combobox";

/// Registers the performance tests of `use_combobox`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(MountAndUnmountLarge))
        .with(fixture_test(OpenAndSelectWithTheMouse))
        .with(fixture_test(OpenAndSelectWithTheMouseLarge))
        .with(fixture_test(Typing))
        .with(fixture_test(TypingLarge))
        .with(fixture_test(Keyboard))
        .with(fixture_test(KeyboardLarge))
        .with(fixture_test(Escape))
        .with(fixture_test(DisableAndEnable))
}
pub(crate) const INPUT: &str = "[data-perf-input]";
pub(crate) const BUTTON: &str = "[data-perf-button]";
pub(crate) const POPOVER: &str = "[data-perf-popover]";
pub(crate) const OPTIONS: &str = "[data-perf-option]";

/// Loads the fixture at `path` and creates the live announcer's regions (its first announcement
/// waits for them, so an announcing step measured first would count them, or not).
pub(crate) async fn goto(page: &Page<'_>, path: &str) -> Result<(), Report> {
    page.goto_path(path).await?;
    page.control("prepare-announcer").await?;
    page.wait_for_count("[data-live-announcer] [aria-live=polite] > div", 1)
        .await?;
    Ok(())
}

/// The option "Option `i`".
pub(crate) fn option(i: u32) -> String {
    format!("[data-perf-option='{i}']")
}

/// The combo box's value: the selected option's number, `-1` for none.
pub(crate) async fn value(page: &Page<'_>) -> Result<f64, Report> {
    page.control("value").await
}

/// Switches to 1,000 options (unmeasured).
pub(crate) async fn large(page: &Page<'_>) -> Result<(), Report> {
    page.control("large").await?;
    page.settle().await
}

/// Mounting the combo box's elements (or everything) and unmounting them.
pub(crate) async fn mount_and_unmount_in(page: &Page<'_>) -> Result<(), Report> {
    page.control("unmount").await?;
    page.wait_for_count(INPUT, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.element(INPUT).await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(INPUT, 0).await?;
    Ok(())
}

/// Mounting the combo box's elements (10 options) and unmounting them.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    goto(page, PATH).await?;
    mount_and_unmount_in(page).await
}

/// Mounting the combo box's elements (1,000 options) and unmounting them.
#[browser_test]
pub async fn mount_and_unmount_large(page: &Page<'_>) -> Result<(), Report> {
    goto(page, PATH).await?;
    large(page).await?;
    mount_and_unmount_in(page).await
}

/// A click on the button opening the popover with all `count` options, and a click on an option
/// selecting it (closing the popover, the input showing its text).
async fn open_and_select_with_the_mouse_in(page: &Page<'_>, count: usize) -> Result<(), Report> {
    let button = page.element(BUTTON).await?;
    page.measure("open", button.click()).await?;
    page.wait_for_count(OPTIONS, count).await?;
    let third = page.element(option(3)).await?;
    page.measure("select", third.click()).await?;
    page.wait_for_count(POPOVER, 0).await?;
    assert_that!(value(page).await?).is_equal_to(3.0);
    page.element(INPUT)
        .await?
        .wait_for_prop("value", "Option 3")
        .await?;
    Ok(())
}

/// Opening the combo box of 10 options and selecting one with the mouse.
#[browser_test]
pub async fn open_and_select_with_the_mouse(page: &Page<'_>) -> Result<(), Report> {
    goto(page, PATH).await?;
    open_and_select_with_the_mouse_in(page, 10).await
}

/// Opening the combo box of 1,000 options and selecting one with the mouse.
#[browser_test]
pub async fn open_and_select_with_the_mouse_large(page: &Page<'_>) -> Result<(), Report> {
    goto(page, PATH).await?;
    large(page).await?;
    open_and_select_with_the_mouse_in(page, 1_000).await
}

/// Typing into the input: "1" opens the popover with the options containing it (`ones`), "2"
/// narrows them to those containing "12" (`twelves`; none: the popover closes), Backspace widens
/// them again.
async fn typing_in(page: &Page<'_>, ones: usize, twelves: usize) -> Result<(), Report> {
    page.element(INPUT).await?.click().await?;
    page.measure("type", page.send_keys("1")).await?;
    page.wait_for_count(OPTIONS, ones).await?;
    page.measure("type more", page.send_keys("2")).await?;
    page.wait_for_count(OPTIONS, twelves).await?;
    page.measure("delete", page.send_keys(Key::Backspace))
        .await?;
    page.wait_for_count(OPTIONS, ones).await?;
    Ok(())
}

/// Typing into the combo box of 10 options.
#[browser_test]
pub async fn typing(page: &Page<'_>) -> Result<(), Report> {
    goto(page, PATH).await?;
    typing_in(page, 1, 0).await
}

/// Typing into the combo box of 1,000 options.
#[browser_test]
pub async fn typing_large(page: &Page<'_>) -> Result<(), Report> {
    goto(page, PATH).await?;
    large(page).await?;
    typing_in(page, 271, 20).await
}

/// Tab focusing the input, ArrowDown opening the popover (the first option focused), ArrowDown
/// and ArrowUp moving the focus, Enter selecting the focused option (closing the popover), and
/// Tab leaving.
async fn keyboard_in(page: &Page<'_>, count: usize) -> Result<(), Report> {
    page.element("#perf-before").await?.click().await?;
    let input = page.element(INPUT).await?;
    page.measure("focus", page.send_keys(Key::Tab)).await?;
    page.wait_for_focus(&input).await?;
    page.measure("open", page.send_keys(Key::Down)).await?;
    page.wait_for_count(OPTIONS, count).await?;
    let first = page.element(option(0)).await?;
    first.wait_for_attr("data-focused", Some("true")).await?;
    page.measure("arrow down", page.send_keys(Key::Down))
        .await?;
    page.element(option(1))
        .await?
        .wait_for_attr("data-focused", Some("true"))
        .await?;
    page.measure("arrow up", page.send_keys(Key::Up)).await?;
    first.wait_for_attr("data-focused", Some("true")).await?;
    page.measure("select", page.send_keys(Key::Enter)).await?;
    page.wait_for_count(POPOVER, 0).await?;
    input.wait_for_prop("value", "Option 0").await?;
    page.measure("blur", page.send_keys(Key::Tab)).await?;
    page.wait_for_focus(&page.element("#perf-after").await?)
        .await?;
    Ok(())
}

/// The keyboard on the combo box of 10 options.
#[browser_test]
pub async fn keyboard(page: &Page<'_>) -> Result<(), Report> {
    goto(page, PATH).await?;
    keyboard_in(page, 10).await
}

/// The keyboard on the combo box of 1,000 options.
#[browser_test]
pub async fn keyboard_large(page: &Page<'_>) -> Result<(), Report> {
    goto(page, PATH).await?;
    large(page).await?;
    keyboard_in(page, 1_000).await
}

/// Escape on the input closing the open popover.
#[browser_test]
pub async fn escape(page: &Page<'_>) -> Result<(), Report> {
    goto(page, PATH).await?;
    page.element(INPUT).await?.click().await?;
    page.send_keys(Key::Down).await?;
    page.wait_for_count(OPTIONS, 10).await?;
    page.measure("escape", page.send_keys(Key::Escape)).await?;
    page.wait_for_count(POPOVER, 0).await?;
    Ok(())
}

/// Disabling the combo box, a click on its button it ignores, and enabling it again.
#[browser_test]
pub async fn disable_and_enable(page: &Page<'_>) -> Result<(), Report> {
    goto(page, PATH).await?;
    let input = page.element(INPUT).await?;
    page.measure("disable", page.control("disable")).await?;
    input.wait_for_prop("disabled", "true").await?;
    let button = page.element(BUTTON).await?;
    page.measure("ignored click", button.click()).await?;
    assert_that!(page.count(POPOVER).await?).is_equal_to(0);
    page.measure("enable", page.control("enable")).await?;
    input.wait_for_prop("disabled", "false").await?;
    Ok(())
}
