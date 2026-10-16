// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the hook `use_highlight_selection_description`
//! (`/perf/hooks/use_highlight_selection_description`, the hook on a grid state with row
//! actions): what each thing it offers costs, measured (`Page::measure`) and checked against its
//! budget (`tests/budgets/hooks/use_highlight_selection_description.json`): mounting and
//! unmounting it, the selection behavior changing (replace and back to toggle) and the
//! interaction modality changing (pointer, keyboard), which it reads. Without touch events (the
//! test browser), it describes nothing.
use browser_test::{TestGroup, browser_test};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/hooks/use_highlight_selection_description";

/// Registers the performance tests of `use_highlight_selection_description`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(SelectionBehavior))
        .with(fixture_test(Modality))
}

const DESCRIBED: &str = "#perf-described";

/// Mounting the hook and unmounting it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(DESCRIBED, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.wait_for_count(DESCRIBED, 1).await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(DESCRIBED, 0).await?;
    Ok(())
}

/// The selection behavior changing to replace (with item actions, the case it describes on touch
/// devices) and back to toggle.
#[browser_test]
pub async fn selection_behavior(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let described = page.element(DESCRIBED).await?;
    page.measure("replace", page.control("replace")).await?;
    described.wait_for_attr("aria-describedby", None).await?;
    page.measure("toggle", page.control("toggle")).await?;
    described.wait_for_attr("aria-describedby", None).await?;
    Ok(())
}

/// The interaction modality changing in replace behavior: a click (pointer), then a key
/// (keyboard).
#[browser_test]
pub async fn modality(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("replace").await?;
    let before = page.element("#perf-before").await?;
    page.measure("pointer", before.click()).await?;
    page.measure("keyboard", before.type_keys("a")).await?;
    page.element(DESCRIBED)
        .await?
        .wait_for_attr("aria-describedby", None)
        .await?;
    Ok(())
}
