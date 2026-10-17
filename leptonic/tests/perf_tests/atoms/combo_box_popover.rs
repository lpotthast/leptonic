// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the atom `ComboBoxPopover` (`/perf/atoms/combo_box_popover`, in a
//! `ComboBox`): what each thing it offers costs, measured (`Page::measure`) and checked against its
//! budget (`tests/budgets/atoms/combo_box_popover.json`): mounting and unmounting it while the
//! combo box is open, opening and closing it, and its options changing while open (typing), with 10
//! options and with 1,000; its dismissal with Escape and with a click outside.
use assertr::prelude::*;
use browser_test::{TestGroup, browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/atoms/combo_box_popover";

/// Registers the performance tests of `ComboBoxPopover`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(MountAndUnmountLarge))
        .with(fixture_test(OpenAndSelect))
        .with(fixture_test(OpenAndSelectLarge))
        .with(fixture_test(Filtering))
        .with(fixture_test(FilteringLarge))
        .with(fixture_test(Dismiss))
}
const INPUT: &str = "[role=combobox]";
const BUTTON: &str = ".leptonic-ComboBoxButton";
const POPOVER: &str = ".leptonic-ComboBoxPopover";
const OPTIONS: &str = "[role=option]";

/// Loads the fixture and creates the live announcer's regions (its first announcement waits for
/// them, so an announcing step measured first would count them, or not).
async fn load(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("prepare-announcer").await?;
    page.wait_for_count("[data-live-announcer] [aria-live=polite] > div", 1)
        .await?;
    Ok(())
}

/// The option "Option `i`".
fn option(i: u32) -> String {
    format!("[role=option]:has(> [data-perf-option='{i}'])")
}

/// The combo box's value: the selected option's number, `-1` for none.
async fn value(page: &Page<'_>) -> Result<f64, Report> {
    page.control("value").await
}

/// Switches to 1,000 options (unmeasured).
async fn large(page: &Page<'_>) -> Result<(), Report> {
    page.control("large").await?;
    page.settle().await
}

/// Mounting the popover of the open combo box (`count` options) and unmounting it.
async fn mount_and_unmount_in(page: &Page<'_>, count: usize) -> Result<(), Report> {
    page.element(BUTTON).await?.click().await?;
    page.wait_for_count(OPTIONS, count).await?;
    page.control("unmount").await?;
    page.wait_for_count(POPOVER, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.wait_for_count(OPTIONS, count).await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(POPOVER, 0).await?;
    // Closed again with the popover (the input refers to it while open).
    page.control("mount").await?;
    page.wait_for_count(OPTIONS, count).await?;
    page.click_at(5, 5).await?;
    page.wait_for_count(POPOVER, 0).await?;
    Ok(())
}

/// Mounting the popover of 10 options and unmounting it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    load(page).await?;
    mount_and_unmount_in(page, 10).await
}

/// Mounting the popover of 1,000 options and unmounting it.
#[browser_test]
pub async fn mount_and_unmount_large(page: &Page<'_>) -> Result<(), Report> {
    load(page).await?;
    large(page).await?;
    mount_and_unmount_in(page, 1_000).await
}

/// A click on the button opening the popover with all `count` options, and a click on an option
/// selecting it (closing the popover, the input showing its text).
async fn open_and_select_in(page: &Page<'_>, count: usize) -> Result<(), Report> {
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

/// Opening the popover of 10 options and closing it by selecting one.
#[browser_test]
pub async fn open_and_select(page: &Page<'_>) -> Result<(), Report> {
    load(page).await?;
    open_and_select_in(page, 10).await
}

/// Opening the popover of 1,000 options and closing it by selecting one.
#[browser_test]
pub async fn open_and_select_large(page: &Page<'_>) -> Result<(), Report> {
    load(page).await?;
    large(page).await?;
    open_and_select_in(page, 1_000).await
}

/// Typing into the input: "1" opens the popover with the options containing it (`ones`), "2"
/// narrows them to those containing "12" (`twelves`; none: the popover closes), Backspace widens
/// them again.
async fn filtering_in(page: &Page<'_>, ones: usize, twelves: usize) -> Result<(), Report> {
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

/// The popover's options following the typed text, of 10.
#[browser_test]
pub async fn filtering(page: &Page<'_>) -> Result<(), Report> {
    load(page).await?;
    filtering_in(page, 1, 0).await
}

/// The popover's options following the typed text, of 1,000.
#[browser_test]
pub async fn filtering_large(page: &Page<'_>) -> Result<(), Report> {
    load(page).await?;
    large(page).await?;
    filtering_in(page, 271, 20).await
}

/// Escape and a click outside closing the popover.
#[browser_test]
pub async fn dismiss(page: &Page<'_>) -> Result<(), Report> {
    load(page).await?;
    let button = page.element(BUTTON).await?;
    button.click().await?;
    page.wait_for_count(OPTIONS, 10).await?;
    page.measure("escape", page.send_keys(Key::Escape)).await?;
    page.wait_for_count(POPOVER, 0).await?;
    button.click().await?;
    page.wait_for_count(OPTIONS, 10).await?;
    page.measure("outside click", page.click_at(5, 5)).await?;
    page.wait_for_count(POPOVER, 0).await?;
    page.element(INPUT).await?;
    Ok(())
}
