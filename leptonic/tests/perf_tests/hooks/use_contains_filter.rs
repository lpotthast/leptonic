// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the hook `use_contains_filter` (`/perf/hooks/use_contains_filter`, texts
//! filtered by a query in a plain list): what each thing it offers costs, measured
//! (`Page::measure`) and checked against its budget
//! (`tests/budgets/hooks/use_contains_filter.json`): creating the filter (mounting) and dropping
//! it, and filtering by queries: "1", "12", "opt" (case ignored: every text), "ö" (accents ignored:
//! every text) and none, of 10 texts and of 1,000.
use browser_test::{TestGroup, browser_test};
use rootcause::Report;

use crate::{harness::fixture_test, pages::Page};

const PATH: &str = "/perf/hooks/use_contains_filter";
const LIST: &str = "#perf-list";
const MATCHES: &str = "#perf-list > li";

/// Registers the performance tests of `use_contains_filter`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(MountAndUnmountLarge))
        .with(fixture_test(Filter))
        .with(fixture_test(FilterLarge))
}

/// Switches to 1,000 texts (unmeasured).
async fn large(page: &Page<'_>) -> Result<(), Report> {
    page.control("large").await?;
    page.wait_for_count(MATCHES, 1_000).await?;
    Ok(())
}

/// Creating the filter with `count` texts (mounting) and dropping it.
async fn mount_and_unmount_in(page: &Page<'_>, count: usize) -> Result<(), Report> {
    page.control("unmount").await?;
    page.wait_for_count(LIST, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.wait_for_count(MATCHES, count).await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(LIST, 0).await?;
    Ok(())
}

/// Creating and dropping the filter of 10 texts.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    mount_and_unmount_in(page, 10).await
}

/// Creating and dropping the filter of 1,000 texts.
#[browser_test]
pub async fn mount_and_unmount_large(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    large(page).await?;
    mount_and_unmount_in(page, 1_000).await
}

/// The queries one after the other, each with its number of matches (`[ones, twelves, all]`).
async fn filter_in(page: &Page<'_>, counts: [usize; 3]) -> Result<(), Report> {
    let [ones, twelves, all] = counts;
    page.measure("1", page.control("one")).await?;
    page.wait_for_count(MATCHES, ones).await?;
    page.measure("12", page.control("twelve")).await?;
    page.wait_for_count(MATCHES, twelves).await?;
    page.measure("opt", page.control("opt")).await?;
    page.wait_for_count(MATCHES, all).await?;
    page.measure("ö", page.control("umlaut")).await?;
    page.wait_for_count(MATCHES, all).await?;
    page.measure("none", page.control("clear")).await?;
    page.wait_for_count(MATCHES, all).await?;
    Ok(())
}

/// Filtering 10 texts.
#[browser_test]
pub async fn filter(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    filter_in(page, [1, 0, 10]).await
}

/// Filtering 1,000 texts.
#[browser_test]
pub async fn filter_large(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    large(page).await?;
    filter_in(page, [271, 20, 1_000]).await
}
