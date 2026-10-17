// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the hook `use_select_state` (`/perf/hooks/use_select_state`, the state of a
//! select on plain elements): what each thing it offers costs, measured (`Page::measure`) and
//! checked against its budget (`tests/budgets/hooks/use_select_state.json`): creating the state
//! with the select (mounting) and dropping it, and its methods: opening, closing and toggling (the
//! listbox renders while open), and setting the value (the first option, the last, none), with 10
//! options and with 1,000.
use assertr::prelude::*;
use browser_test::{TestGroup, browser_test};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/hooks/use_select_state";

/// Registers the performance tests of `use_select_state`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(MountAndUnmountLarge))
        .with(fixture_test(OpenAndClose))
        .with(fixture_test(OpenAndCloseLarge))
        .with(fixture_test(SetValue))
        .with(fixture_test(SetValueLarge))
}
const TRIGGER: &str = "[data-perf-trigger]";

const VALUE: &str = "[data-perf-value]";

const LISTBOX: &str = "[role=listbox]";

/// The option "Option `i`".
fn option(i: u32) -> String {
    format!("[data-perf-option='{i}']")
}

/// The select's value: the selected option's number, `-1` for none.
async fn value(page: &Page<'_>) -> Result<f64, Report> {
    page.control("value").await
}

/// Switches to 1,000 options (unmeasured).
async fn large(page: &Page<'_>) -> Result<(), Report> {
    page.control("large").await?;
    page.settle().await
}

/// Mounting everything (creating the state), `container` among it, and unmounting it.
async fn mount_and_unmount_in(page: &Page<'_>, container: &str) -> Result<(), Report> {
    page.control("unmount").await?;
    page.wait_for_count(container, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.element(container).await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(container, 0).await?;
    Ok(())
}

/// Creating the state of 10 options with the select and dropping it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    mount_and_unmount_in(page, TRIGGER).await
}

/// Creating the state of 1,000 options with the select and dropping it.
#[browser_test]
pub async fn mount_and_unmount_large(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    large(page).await?;
    mount_and_unmount_in(page, TRIGGER).await
}

/// Opening (`last + 1` options render) and closing, then toggling open and closed.
async fn open_and_close_in(page: &Page<'_>, last: u32) -> Result<(), Report> {
    page.measure("open", page.control("open")).await?;
    page.element(option(last)).await?;
    page.measure("close", page.control("close")).await?;
    page.wait_for_count(LISTBOX, 0).await?;
    page.measure("toggle open", page.control("toggle")).await?;
    page.element(option(last)).await?;
    page.measure("toggle closed", page.control("toggle"))
        .await?;
    page.wait_for_count(LISTBOX, 0).await?;
    Ok(())
}

/// Opening and closing the select of 10 options.
#[browser_test]
pub async fn open_and_close(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    open_and_close_in(page, 9).await
}

/// Opening and closing the select of 1,000 options.
#[browser_test]
pub async fn open_and_close_large(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    large(page).await?;
    open_and_close_in(page, 999).await
}

/// Setting the value: the first option, the last of `last + 1`, none.
async fn set_value_in(page: &Page<'_>, last: u32) -> Result<(), Report> {
    let shown = page.element(VALUE).await?;
    page.measure("select first", page.control("select-first"))
        .await?;
    shown.wait_for_inner_text("Option 0").await?;
    page.measure("select last", page.control("select-last"))
        .await?;
    shown.wait_for_inner_text(&format!("Option {last}")).await?;
    page.measure("clear", page.control("clear-value")).await?;
    shown.wait_for_inner_text("Pick an option").await?;
    assert_that!(value(page).await?).is_equal_to(-1.0);
    Ok(())
}

/// Setting the value of a select of 10 options.
#[browser_test]
pub async fn set_value(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    set_value_in(page, 9).await
}

/// Setting the value of a select of 1,000 options.
#[browser_test]
pub async fn set_value_large(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    large(page).await?;
    set_value_in(page, 999).await
}
