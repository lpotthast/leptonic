// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the hook `use_overlay_trigger` (`/perf/hooks/use_overlay_trigger`): what each
//! thing it offers costs, measured (`Page::measure`) and checked against its budget
//! (`tests/budgets/hooks/use_overlay_trigger.json`): mounting and unmounting it, and its overlay
//! opening and closing (the trigger's `aria-expanded` and `aria-controls`).
use assertr::prelude::*;
use browser_test::{TestGroup, browser_test};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/hooks/use_overlay_trigger";
const TRIGGER: &str = "#perf-trigger";

/// Registers the performance tests of `use_overlay_trigger`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(OpenAndClose))
}

/// Mounting the trigger (a menu's, closed) and unmounting it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(TRIGGER, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    let trigger = page.element(TRIGGER).await?;
    trigger
        .wait_for_attr("aria-expanded", Some("false"))
        .await?;
    assert_that!(trigger.attr("aria-haspopup").await?).is_equal_to(Some("true".to_owned()));
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(TRIGGER, 0).await?;
    Ok(())
}

/// The overlay opening (the trigger expanded, controlling it) and closing.
#[browser_test]
pub async fn open_and_close(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let trigger = page.element(TRIGGER).await?;
    page.measure("open", page.control("open")).await?;
    trigger.wait_for_attr("aria-expanded", Some("true")).await?;
    trigger
        .wait_for_attr("aria-controls", Some("perf-overlay"))
        .await?;
    page.measure("close", page.control("close")).await?;
    trigger
        .wait_for_attr("aria-expanded", Some("false"))
        .await?;
    trigger.wait_for_attr("aria-controls", None).await?;
    Ok(())
}
