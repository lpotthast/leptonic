// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the hook `use_toast` (`/perf/hooks/use_toast`, toasts on plain elements
//! without a region): what each thing it offers costs, measured (`Page::measure`) and checked
//! against its budget (`tests/budgets/hooks/use_toast.json`): a toast coming and closing
//! (without and with a timeout), its timeout running out, its close button, and changes of its
//! label.
use assertr::prelude::*;
use browser_test::{TestGroup, browser_test};
use rootcause::Report;

use crate::{harness::fixture_test, pages::Page};

const PATH: &str = "/perf/hooks/use_toast";

/// Registers the performance tests of `use_toast`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(AddAndClose))
        .with(fixture_test(AddAndCloseTimed))
        .with(fixture_test(TimeOut))
        .with(fixture_test(CloseWithTheButton))
        .with(fixture_test(ChangeTheLabel))
}
const TOAST: &str = ".perf-toast";

/// A toast without a timeout coming (its content shown once visible) and closing.
#[browser_test]
pub async fn add_and_close(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.measure("add", page.control("add")).await?;
    page.wait_for_attr(TOAST, "role", Some("alertdialog"))
        .await?;
    page.wait_for_attr(".perf-toast-content", "aria-hidden", None)
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

/// A click on the toast's close button (its `close_button`): the toast closes.
#[browser_test]
pub async fn close_with_the_button(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("add").await?;
    let close = page.element(".perf-toast-close").await?;
    page.measure("click", close.click()).await?;
    page.wait_for_count(TOAST, 0).await?;
    assert_that!(page.control("closes").await?).is_equal_to(1.0);
    Ok(())
}

/// Labeling the toast and removing the label again.
#[browser_test]
pub async fn change_the_label(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("add").await?;
    page.wait_for_count(TOAST, 1).await?;
    page.measure("set", page.control("set-label")).await?;
    page.wait_for_attr(TOAST, "aria-label", Some("Notice"))
        .await?;
    page.measure("unset", page.control("unset-label")).await?;
    page.wait_for_attr(TOAST, "aria-label", None).await?;
    Ok(())
}
