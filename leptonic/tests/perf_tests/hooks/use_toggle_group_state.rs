// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the hook `use_toggle_group_state` (`/perf/hooks/use_toggle_group_state`,
//! multiple selection, its keys shown by three elements): what each thing it offers costs,
//! measured (`Page::measure`) and checked against its budget
//! (`tests/budgets/hooks/use_toggle_group_state.json`): creating and dropping it (mounting and
//! unmounting), and its methods: toggling keys, deselecting one, replacing the selection, and a
//! toggle `disallow_empty_selection` ignores.
use browser_test::{TestGroup, browser_test};
use rootcause::Report;

use crate::{harness::fixture_test, pages::Page};

const PATH: &str = "/perf/hooks/use_toggle_group_state";

/// Registers the performance tests of `use_toggle_group_state`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(ToggleKeys))
        .with(fixture_test(ReplaceSelection))
        .with(fixture_test(DisallowEmptySelection))
}
const KEY_A: &str = "#perf-key-a";
const KEY_B: &str = "#perf-key-b";
const KEY_C: &str = "#perf-key-c";

/// Creating the state (mounting) and dropping it (unmounting).
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(KEY_A, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.element(KEY_C).await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(KEY_A, 0).await?;
    Ok(())
}

/// Toggling `a` and `b` on, and deselecting `a`.
#[browser_test]
pub async fn toggle_keys(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.measure("toggle a", page.control("toggle-a")).await?;
    page.wait_for_attr(KEY_A, "data-selected", Some("true"))
        .await?;
    page.measure("toggle b", page.control("toggle-b")).await?;
    page.wait_for_attr(KEY_B, "data-selected", Some("true"))
        .await?;
    page.measure("deselect a", page.control("deselect-a"))
        .await?;
    page.wait_for_attr(KEY_A, "data-selected", None).await?;
    page.wait_for_attr(KEY_B, "data-selected", Some("true"))
        .await?;
    Ok(())
}

/// Selecting all keys at once and clearing the selection.
#[browser_test]
pub async fn replace_selection(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.measure("select all", page.control("select-all"))
        .await?;
    page.wait_for_attr(KEY_C, "data-selected", Some("true"))
        .await?;
    page.measure("clear", page.control("clear")).await?;
    page.wait_for_attr(KEY_A, "data-selected", None).await?;
    page.wait_for_attr(KEY_C, "data-selected", None).await?;
    Ok(())
}

/// With `disallow_empty_selection`, toggling the only selected key off is ignored.
#[browser_test]
pub async fn disallow_empty_selection(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("toggle-a").await?;
    page.wait_for_attr(KEY_A, "data-selected", Some("true"))
        .await?;
    page.measure("disallow empty", page.control("disallow-empty"))
        .await?;
    page.measure("ignored toggle", page.control("toggle-a"))
        .await?;
    page.wait_for_attr(KEY_A, "data-selected", Some("true"))
        .await?;
    Ok(())
}
