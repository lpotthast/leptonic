// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the atom `Toast` (`/perf/atoms/toast`, in its `ToastRegion`, with
//! `ToastContent`, `ToastTitle`, `ToastDescription` and `ToastCloseButton`): what each thing it
//! offers costs, measured (`Page::measure`) and checked against its budget
//! (`tests/budgets/atoms/toast.json`): a toast coming and closing (without and with a timeout,
//! next to another toast), its timeout running out, keyboard focus, and changes of its label.
use assertr::prelude::*;
use browser_test::{TestGroup, browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/atoms/toast";

/// Registers the performance tests of `Toast`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(AddAndClose))
        .with(fixture_test(AddAndCloseTimed))
        .with(fixture_test(AddAndCloseSecond))
        .with(fixture_test(TimeOut))
        .with(fixture_test(KeyboardFocusAndBlur))
        .with(fixture_test(ChangeTheLabel))
}
const TOAST: &str = "[role=alertdialog]";

/// Adds a toast without a timeout and waits until `count` show.
async fn add(page: &Page<'_>, count: usize) -> Result<(), Report> {
    page.control("add").await?;
    page.wait_for_count(TOAST, count).await
}

/// A toast without a timeout coming (with the region) and closing (programmatically).
#[browser_test]
pub async fn add_and_close(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.measure("add", page.control("add")).await?;
    page.wait_for_count(TOAST, 1).await?;
    page.wait_for_attr("[role=alert]", "aria-hidden", None)
        .await?;
    page.measure("close", page.control("close-newest")).await?;
    page.wait_for_count(TOAST, 0).await?;
    assert_that!(page.control("closes").await?).is_equal_to(1.0);
    Ok(())
}

/// A toast with an hour's timeout coming (its timer starts) and closing (its timer stops).
#[browser_test]
pub async fn add_and_close_timed(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.measure("add", page.control("add-timed")).await?;
    page.wait_for_count(TOAST, 1).await?;
    page.measure("close", page.control("close-newest")).await?;
    page.wait_for_count(TOAST, 0).await?;
    Ok(())
}

/// A second toast coming next to a first (the region stays) and closing again.
#[browser_test]
pub async fn add_and_close_second(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    add(page, 1).await?;
    page.measure("add", page.control("add")).await?;
    page.wait_for_count(TOAST, 2).await?;
    page.measure("close", page.control("close-newest")).await?;
    page.wait_for_count(TOAST, 1).await?;
    Ok(())
}

/// The timeout of a toast running out: it closes (the measurement waits for it).
#[browser_test]
pub async fn time_out(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.measure("add", page.control("add-expiring")).await?;
    page.wait_for_count(TOAST, 1).await?;
    page.measure("time out", page.wait_for_count(TOAST, 0))
        .await?;
    assert_that!(page.control("closes").await?).is_equal_to(1.0);
    Ok(())
}

/// Tab moving focus from the region to the toast (with a focus ring) and on to its close
/// button.
#[browser_test]
pub async fn keyboard_focus_and_blur(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    add(page, 1).await?;
    let toast = page.element(TOAST).await?;
    page.element("#perf-before").await?.click().await?;
    page.send_keys(Key::F6).await?;
    page.wait_for_focus(&page.element(".leptonic-ToastRegion").await?)
        .await?;
    page.measure("focus", page.send_keys(Key::Tab)).await?;
    toast
        .wait_for_attr("data-focus-visible", Some("true"))
        .await?;
    page.measure("blur", page.send_keys(Key::Tab)).await?;
    toast.wait_for_attr("data-focus-visible", None).await?;
    Ok(())
}

/// Labeling the toast and removing the label again.
#[browser_test]
pub async fn change_the_label(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    add(page, 1).await?;
    let toast = page.element(TOAST).await?;
    page.measure("set", page.control("set-label")).await?;
    toast.wait_for_attr("aria-label", Some("Notice")).await?;
    page.measure("unset", page.control("unset-label")).await?;
    toast.wait_for_attr("aria-label", None).await?;
    Ok(())
}
