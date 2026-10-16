// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the atom `DismissButton` (`/perf/atoms/dismiss_button`): what each thing
//! it offers costs, measured (`Page::measure`) and checked against its budget
//! (`tests/budgets/atoms/dismiss_button.json`): mounting and unmounting it, its activation by a
//! screen reader (a virtual click: it is visually hidden and out of the tab order), and its
//! label changing (and back to the default).
use assertr::prelude::*;
use browser_test::{TestGroup, browser_test};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/atoms/dismiss_button";

/// Registers the performance tests of `DismissButton`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(Dismiss))
        .with(fixture_test(Label))
}
const DISMISS: &str = "#perf-dismiss";

/// Mounting the dismiss button and unmounting it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(DISMISS, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.element(DISMISS).await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(DISMISS, 0).await?;
    Ok(())
}

/// A screen reader activating the button: it dismisses.
#[browser_test]
pub async fn dismiss(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let dismiss = page.element(DISMISS).await?;
    page.measure("activate", dismiss.virtual_click()).await?;
    assert_that!(page.control("dismissals").await?).is_equal_to(1.0);
    Ok(())
}

/// Naming the button "Close" and back to its default "Dismiss".
#[browser_test]
pub async fn label(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let dismiss = page.element(DISMISS).await?;
    page.measure("label", page.control("label")).await?;
    dismiss.wait_for_attr("aria-label", Some("Close")).await?;
    page.measure("unlabel", page.control("unlabel")).await?;
    dismiss.wait_for_attr("aria-label", Some("Dismiss")).await?;
    Ok(())
}
