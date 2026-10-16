// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the hook `use_disclosure_group_state`
//! (`/perf/hooks/use_disclosure_group_state`): what each thing it offers costs, measured
//! (`Page::measure`) and checked against its budget
//! (`tests/budgets/hooks/use_disclosure_group_state.json`): mounting and unmounting it, toggling
//! keys with single and with multiple expansion, switching back to single expansion (keeping one),
//! and setting the expanded keys (reduced to one with single expansion).
use assertr::prelude::*;
use browser_test::{TestGroup, browser_test};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/hooks/use_disclosure_group_state";
const KEYS: &str = "#perf-expanded-keys";

/// Registers the performance tests of `use_disclosure_group_state`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(ToggleSingle))
        .with(fixture_test(ToggleMultiple))
        .with(fixture_test(SetExpandedKeys))
}

/// How often the state reported a change.
async fn changes(page: &Page<'_>) -> Result<f64, Report> {
    page.control("changes").await
}

/// Mounting the state and unmounting it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(KEYS, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.element(KEYS).await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(KEYS, 0).await?;
    Ok(())
}

/// With single expansion: toggling `1` (expanded), `2` (replacing it) and `2` again (none).
#[browser_test]
pub async fn toggle_single(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let keys = page.element(KEYS).await?;
    page.measure("expand 1", page.control("toggle-1")).await?;
    keys.wait_for_inner_text("1").await?;
    page.measure("expand 2", page.control("toggle-2")).await?;
    keys.wait_for_inner_text("2").await?;
    page.measure("collapse 2", page.control("toggle-2")).await?;
    keys.wait_for_inner_text("").await?;
    assert_that!(changes(page).await?).is_equal_to(3.0);
    Ok(())
}

/// Switching to multiple expansion, toggling `1` and `2` (both expanded), and switching back to
/// single expansion (keeping `1`).
#[browser_test]
pub async fn toggle_multiple(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let keys = page.element(KEYS).await?;
    page.measure("multiple", page.control("multiple")).await?;
    page.measure("expand 1", page.control("toggle-1")).await?;
    keys.wait_for_inner_text("1").await?;
    page.measure("expand 2", page.control("toggle-2")).await?;
    keys.wait_for_inner_text("1,2").await?;
    page.measure("single", page.control("single")).await?;
    keys.wait_for_inner_text("1").await?;
    assert_that!(changes(page).await?).is_equal_to(3.0);
    Ok(())
}

/// Setting the expanded keys to `1` and `2` (reduced to `1`: single expansion) and to none.
#[browser_test]
pub async fn set_expanded_keys(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let keys = page.element(KEYS).await?;
    page.measure("set both", page.control("set-both")).await?;
    keys.wait_for_inner_text("1").await?;
    page.measure("set none", page.control("set-none")).await?;
    keys.wait_for_inner_text("").await?;
    assert_that!(changes(page).await?).is_equal_to(2.0);
    Ok(())
}
