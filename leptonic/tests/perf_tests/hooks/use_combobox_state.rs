// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the hook `use_combobox_state` (`/perf/hooks/use_combobox_state`, the state of
//! a combo box on plain elements): what each thing it offers costs, measured (`Page::measure`) and
//! checked against its budget (`tests/budgets/hooks/use_combobox_state.json`): creating the state
//! with the combo box (mounting) and dropping it, and its methods: opening and closing (the options
//! render while open), setting the input text (filtering the options) and setting the value (the
//! input shows its text), with 10 options and with 1,000.
use assertr::prelude::*;
use browser_test::{TestGroup, browser_test};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/hooks/use_combobox_state";

/// Registers the performance tests of `use_combobox_state`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(MountAndUnmountLarge))
        .with(fixture_test(OpenAndClose))
        .with(fixture_test(OpenAndCloseLarge))
        .with(fixture_test(SetInputText))
        .with(fixture_test(SetInputTextLarge))
        .with(fixture_test(SetValue))
        .with(fixture_test(SetValueLarge))
}
const INPUT: &str = "[data-perf-input]";

const POPOVER: &str = "[data-perf-popover]";

const OPTIONS: &str = "[data-perf-option]";

/// Loads the fixture at `path` and creates the live announcer's regions (its first announcement
/// waits for them, so an announcing step measured first would count them, or not).
async fn goto(page: &Page<'_>, path: &str) -> Result<(), Report> {
    page.goto_path(path).await?;
    page.control("prepare-announcer").await?;
    page.wait_for_count("[data-live-announcer] [aria-live=polite] > div", 1)
        .await?;
    Ok(())
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

/// Mounting everything (creating the state) and unmounting it.
async fn mount_and_unmount_in(page: &Page<'_>) -> Result<(), Report> {
    page.control("unmount").await?;
    page.wait_for_count(INPUT, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.element(INPUT).await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(INPUT, 0).await?;
    Ok(())
}

/// Creating the state of 10 options with the combo box and dropping it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    goto(page, PATH).await?;
    mount_and_unmount_in(page).await
}

/// Creating the state of 1,000 options with the combo box and dropping it.
#[browser_test]
pub async fn mount_and_unmount_large(page: &Page<'_>) -> Result<(), Report> {
    goto(page, PATH).await?;
    large(page).await?;
    mount_and_unmount_in(page).await
}

/// Opening (`count` options render) and closing.
async fn open_and_close_in(page: &Page<'_>, count: usize) -> Result<(), Report> {
    page.measure("open", page.control("open")).await?;
    page.wait_for_count(OPTIONS, count).await?;
    page.measure("close", page.control("close")).await?;
    page.wait_for_count(POPOVER, 0).await?;
    Ok(())
}

/// Opening and closing the combo box of 10 options.
#[browser_test]
pub async fn open_and_close(page: &Page<'_>) -> Result<(), Report> {
    goto(page, PATH).await?;
    open_and_close_in(page, 10).await
}

/// Opening and closing the combo box of 1,000 options.
#[browser_test]
pub async fn open_and_close_large(page: &Page<'_>) -> Result<(), Report> {
    goto(page, PATH).await?;
    large(page).await?;
    open_and_close_in(page, 1_000).await
}

/// Setting the input text ("Option 5") and clearing it.
async fn set_input_text_in(page: &Page<'_>) -> Result<(), Report> {
    let input = page.element(INPUT).await?;
    page.measure("set", page.control("set-text")).await?;
    input.wait_for_prop("value", "Option 5").await?;
    page.measure("clear", page.control("clear-text")).await?;
    input.wait_for_prop("value", "").await?;
    Ok(())
}

/// Setting the input text of a combo box of 10 options.
#[browser_test]
pub async fn set_input_text(page: &Page<'_>) -> Result<(), Report> {
    goto(page, PATH).await?;
    set_input_text_in(page).await
}

/// Setting the input text of a combo box of 1,000 options.
#[browser_test]
pub async fn set_input_text_large(page: &Page<'_>) -> Result<(), Report> {
    goto(page, PATH).await?;
    large(page).await?;
    set_input_text_in(page).await
}

/// Setting the value (the first option; the input shows its text) and clearing it.
async fn set_value_in(page: &Page<'_>) -> Result<(), Report> {
    let input = page.element(INPUT).await?;
    page.measure("select", page.control("select-first")).await?;
    input.wait_for_prop("value", "Option 0").await?;
    page.measure("clear", page.control("clear-value")).await?;
    assert_that!(value(page).await?).is_equal_to(-1.0);
    Ok(())
}

/// Setting the value of a combo box of 10 options.
#[browser_test]
pub async fn set_value(page: &Page<'_>) -> Result<(), Report> {
    goto(page, PATH).await?;
    set_value_in(page).await
}

/// Setting the value of a combo box of 1,000 options.
#[browser_test]
pub async fn set_value_large(page: &Page<'_>) -> Result<(), Report> {
    goto(page, PATH).await?;
    large(page).await?;
    set_value_in(page).await
}
