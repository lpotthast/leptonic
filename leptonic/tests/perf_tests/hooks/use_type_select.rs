// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the hook `use_type_select` (`/perf/hooks/use_type_select`, on a plain
//! focusable element): what each thing it offers costs, measured (`Page::measure`) and checked
//! against its budget (`tests/budgets/hooks/use_type_select.json`): mounting and unmounting it, a
//! typed character matching no item (a search through all of them), one matching the first
//! item (focusing it), and a key it ignores (a named key), with 10 items and with 1,000.
use browser_test::{TestGroup, browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/hooks/use_type_select";
const LIST: &str = "#perf-list";
const FOCUSED: &str = "#perf-focused";

/// Registers the performance tests of `use_type_select`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(TypeAhead))
        .with(fixture_test(TypeAheadLarge))
}

/// Mounting the hook's element and unmounting it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(LIST, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.element(LIST).await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(LIST, 0).await?;
    Ok(())
}

/// With the list focused: ArrowDown (ignored), "x" (no match: nothing is focused), "o" (the
/// first item is focused). (The match is measured last: the search resets a second later.)
async fn type_ahead_in(page: &Page<'_>) -> Result<(), Report> {
    let list = page.element(LIST).await?;
    list.click().await?;
    page.wait_for_focus(&list).await?;
    let focused = page.element(FOCUSED).await?;
    page.measure("ignored key", page.send_keys(Key::Down))
        .await?;
    page.measure("no match", page.send_keys("x")).await?;
    focused.wait_for_inner_text("").await?;
    page.measure("match", page.send_keys("o")).await?;
    focused.wait_for_inner_text("0").await?;
    Ok(())
}

/// Type-ahead through 10 items.
#[browser_test]
pub async fn type_ahead(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    type_ahead_in(page).await
}

/// Type-ahead through 1,000 items.
#[browser_test]
pub async fn type_ahead_large(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("large").await?;
    page.settle().await?;
    type_ahead_in(page).await
}
