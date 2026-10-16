// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the hook `use_separator` (`/perf/hooks/use_separator`, on a plain
//! `<div>`): what each thing it offers costs, measured (`Page::measure`) and checked against its
//! budget (`tests/budgets/hooks/use_separator.json`): mounting and unmounting it, its orientation
//! changing (`aria-orientation`) and its label.
use browser_test::{TestGroup, browser_test};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/hooks/use_separator";

/// Registers the performance tests of `use_separator`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(Orientation))
        .with(fixture_test(Label))
}
const SEPARATOR: &str = "#perf-separator";

/// Mounting the separator and unmounting it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(SEPARATOR, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.element(SEPARATOR).await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(SEPARATOR, 0).await?;
    Ok(())
}

/// Making the separator vertical and horizontal again.
#[browser_test]
pub async fn orientation(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let separator = page.element(SEPARATOR).await?;
    page.measure("vertical", page.control("vertical")).await?;
    separator
        .wait_for_attr("aria-orientation", Some("vertical"))
        .await?;
    page.measure("horizontal", page.control("horizontal"))
        .await?;
    separator.wait_for_attr("aria-orientation", None).await?;
    Ok(())
}

/// Giving the separator an `aria-label` and taking it away.
#[browser_test]
pub async fn label(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let separator = page.element(SEPARATOR).await?;
    page.measure("label", page.control("label")).await?;
    separator
        .wait_for_attr("aria-label", Some("Section"))
        .await?;
    page.measure("unlabel", page.control("unlabel")).await?;
    separator.wait_for_attr("aria-label", None).await?;
    Ok(())
}
