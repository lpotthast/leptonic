// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the hook `use_breadcrumbs` (`/perf/hooks/use_breadcrumbs`, on a plain
//! `<ol>`): mounting and unmounting it, and changes of its label (checked against its budget,
//! `tests/budgets/hooks/use_breadcrumbs.json`).
use browser_test::{TestGroup, browser_test};
use rootcause::Report;

use crate::{harness::fixture_test, pages::Page};

const PATH: &str = "/perf/hooks/use_breadcrumbs";

/// Registers the performance tests of `use_breadcrumbs`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(ChangeTheLabel))
}
const LIST: &str = "#perf-breadcrumbs";

/// Mounting the list (with its localized default label) and unmounting it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(LIST, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.wait_for_attr(LIST, "aria-label", Some("Breadcrumbs"))
        .await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(LIST, 0).await?;
    Ok(())
}

/// Setting a label and removing it again (the default label).
#[browser_test]
pub async fn change_the_label(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.measure("set", page.control("set-label")).await?;
    page.wait_for_attr(LIST, "aria-label", Some("Trail"))
        .await?;
    page.measure("unset", page.control("unset-label")).await?;
    page.wait_for_attr(LIST, "aria-label", Some("Breadcrumbs"))
        .await?;
    Ok(())
}
