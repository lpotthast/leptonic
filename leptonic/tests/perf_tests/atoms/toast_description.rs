// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the atom `ToastDescription` (`/perf/atoms/toast_description`, in the
//! toasts of a `ToastRegion`): what it costs, measured (`Page::measure`) and checked against its
//! budget (`tests/budgets/atoms/toast_description.json`): coming and closing with its toast
//! (which it describes), and unmounting it in a toast shown (the toast is no longer described).
//! Mounting it again in a toast shown isn't measured: a `ToastDescription` takes its toast's
//! description props once, so a second one in the same toast renders nothing.
use assertr::prelude::*;
use browser_test::{TestGroup, browser_test};
use rootcause::Report;

use crate::{harness::fixture_test, pages::Page};

const PATH: &str = "/perf/atoms/toast_description";

/// Registers the performance tests of `ToastDescription`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(AddAndClose))
        .with(fixture_test(Unmount))
}
const TOAST: &str = "[role=alertdialog]";
const DESCRIPTION: &str = ".leptonic-ToastDescription";

/// The toast is described by the description.
async fn wait_for_described_by_description(page: &Page<'_>) -> Result<(), Report> {
    let description_id = page.element(DESCRIPTION).await?.attr("id").await?;
    assert_that!(description_id.is_some()).is_true();
    page.wait_for_attr(TOAST, "aria-describedby", description_id.as_deref())
        .await
}

/// A toast coming with its description and closing.
#[browser_test]
pub async fn add_and_close(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.measure("add", page.control("add")).await?;
    page.wait_for_count(DESCRIPTION, 1).await?;
    wait_for_described_by_description(page).await?;
    page.measure("close", page.control("close-newest")).await?;
    page.wait_for_count(TOAST, 0).await?;
    Ok(())
}

/// Unmounting the description of a toast shown: the toast is no longer described.
#[browser_test]
pub async fn unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("add").await?;
    page.wait_for_count(DESCRIPTION, 1).await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(DESCRIPTION, 0).await?;
    page.wait_for_attr(TOAST, "aria-describedby", None).await?;
    Ok(())
}
