// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the hook `use_virtual_drop` (`/perf/hooks/use_virtual_drop`): the
//! description of a drop target during keyboard and screen reader drags. Measured
//! (`Page::measure`) and checked against its budget (`tests/budgets/hooks/use_virtual_drop.json`):
//! mounting and unmounting it, and a keyboard drag starting (the description appears) and ending
//! (it goes).
use assertr::prelude::*;
use browser_test::{TestGroup, browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/hooks/use_virtual_drop";
const DESCRIBED: &str = "#perf-described";

/// Registers the performance tests of `use_virtual_drop`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(KeyboardDrag))
}

/// Mounting the hook and unmounting it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.wait_for_count("[data-live-announcer] [aria-live=polite] > div", 1)
        .await?;
    page.control("unmount").await?;
    page.wait_for_count(DESCRIBED, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.element(DESCRIBED).await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(DESCRIBED, 0).await?;
    Ok(())
}

/// A keyboard drag starting (Enter on the drag source: the element is described how to drop)
/// and ending (Escape: the description goes).
#[browser_test]
pub async fn keyboard_drag(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.wait_for_count("[data-live-announcer] [aria-live=polite] > div", 1)
        .await?;
    let described = page.element(DESCRIBED).await?;
    page.element("#perf-before").await?.click().await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&page.element("#perf-draggable").await?)
        .await?;
    page.measure("start", page.send_keys(Key::Enter)).await?;
    page.wait_for_focus(&page.element("#perf-drop").await?)
        .await?;
    // The rest of the page is hidden during the drag: the description is read by its id.
    let Some(id) = described.attr("aria-describedby").await? else {
        rootcause::bail!("the element isn't described during the drag");
    };
    let description = page.element(format!("#{id}")).await?;
    assert_that!(description.prop("textContent").await?).is_equal_to(Some(
        "Press Enter to drop. Press Escape to cancel drag.".to_owned(),
    ));
    page.measure("end", page.send_keys(Key::Escape)).await?;
    described.wait_for_attr("aria-describedby", None).await?;
    Ok(())
}
