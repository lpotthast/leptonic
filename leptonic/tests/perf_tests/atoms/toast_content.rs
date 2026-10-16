// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the atom `ToastContent` (`/perf/atoms/toast_content`, in the toasts of a
//! `ToastRegion`): what it costs, measured (`Page::measure`) and checked against its budget
//! (`tests/budgets/atoms/toast_content.json`): coming and closing with its toast (shown to
//! assistive technology once the toast is visible), and mounting and unmounting it in a toast
//! shown.
use browser_test::{TestGroup, browser_test};
use rootcause::Report;

use crate::{harness::fixture_test, pages::Page};

const PATH: &str = "/perf/atoms/toast_content";

/// Registers the performance tests of `ToastContent`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(AddAndClose))
        .with(fixture_test(MountAndUnmount))
}
const TOAST: &str = "[role=alertdialog]";
const CONTENT: &str = ".leptonic-ToastContent";

/// A toast coming with its content (an alert, not hidden once visible) and closing.
#[browser_test]
pub async fn add_and_close(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.measure("add", page.control("add")).await?;
    page.wait_for_attr(CONTENT, "role", Some("alert")).await?;
    page.wait_for_attr(CONTENT, "aria-hidden", None).await?;
    page.measure("close", page.control("close-newest")).await?;
    page.wait_for_count(TOAST, 0).await?;
    Ok(())
}

/// Unmounting the content of a toast shown, and mounting it again.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("add").await?;
    page.wait_for_count(CONTENT, 1).await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(CONTENT, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.wait_for_attr(CONTENT, "role", Some("alert")).await?;
    page.wait_for_attr(CONTENT, "aria-hidden", None).await?;
    Ok(())
}
