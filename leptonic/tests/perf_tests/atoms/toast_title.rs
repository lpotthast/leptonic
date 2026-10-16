// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the atom `ToastTitle` (`/perf/atoms/toast_title`, in the toasts of a
//! `ToastRegion`): what it costs, measured (`Page::measure`) and checked against its budget
//! (`tests/budgets/atoms/toast_title.json`): coming and closing with its toast (which it labels),
//! and mounting and unmounting it in a toast shown.
use assertr::prelude::*;
use browser_test::{TestGroup, browser_test};
use rootcause::Report;

use crate::{harness::fixture_test, pages::Page};

const PATH: &str = "/perf/atoms/toast_title";

/// Registers the performance tests of `ToastTitle`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(AddAndClose))
        .with(fixture_test(MountAndUnmount))
}
const TOAST: &str = "[role=alertdialog]";
const TITLE: &str = ".leptonic-ToastTitle";

/// The toast is labelled by the title.
async fn assert_labelled_by_title(page: &Page<'_>) -> Result<(), Report> {
    let title_id = page.element(TITLE).await?.attr("id").await?;
    let labelled_by = page.element(TOAST).await?.attr("aria-labelledby").await?;
    assert_that!(labelled_by).is_equal_to(title_id);
    Ok(())
}

/// A toast coming with its title and closing.
#[browser_test]
pub async fn add_and_close(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.measure("add", page.control("add")).await?;
    page.wait_for_count(TITLE, 1).await?;
    assert_labelled_by_title(page).await?;
    page.measure("close", page.control("close-newest")).await?;
    page.wait_for_count(TOAST, 0).await?;
    Ok(())
}

/// Unmounting the title of a toast shown, and mounting it again.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("add").await?;
    page.wait_for_count(TITLE, 1).await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(TITLE, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.wait_for_count(TITLE, 1).await?;
    assert_labelled_by_title(page).await?;
    Ok(())
}
