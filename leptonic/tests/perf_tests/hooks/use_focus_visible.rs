// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the hook `use_focus_visible` (`/perf/hooks/use_focus_visible`): what each
//! thing it offers costs, measured (`Page::measure`) and checked against its budget
//! (`tests/budgets/hooks/use_focus_visible.json`): mounting and unmounting it, and the changes
//! of the interaction modality it follows: a click (hidden), a key press (visible), Tab moving the
//! focus (visible), and a click again (hidden).
use browser_test::{TestGroup, browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/hooks/use_focus_visible";

/// Registers the performance tests of `use_focus_visible`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(PointerAndKeyboard))
}
const VISIBLE: &str = "#perf-focus-visible";

/// Mounting the hook and unmounting it (after a click: the modality is the pointer).
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element("#perf-before").await?.click().await?;
    page.control("unmount").await?;
    page.wait_for_count(VISIBLE, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.wait_for_attr(VISIBLE, "data-visible", Some("false"))
        .await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(VISIBLE, 0).await?;
    Ok(())
}

/// A click (focus hidden), a key press (visible), a click again (hidden), and Tab (visible).
#[browser_test]
pub async fn pointer_and_keyboard(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let before = page.element("#perf-before").await?;
    let visible = page.element(VISIBLE).await?;
    page.measure("click", before.click()).await?;
    visible.wait_for_attr("data-visible", Some("false")).await?;
    page.measure("key", page.send_keys("x")).await?;
    visible.wait_for_attr("data-visible", Some("true")).await?;
    page.measure("click again", before.click()).await?;
    visible.wait_for_attr("data-visible", Some("false")).await?;
    page.measure("tab", page.send_keys(Key::Tab)).await?;
    visible.wait_for_attr("data-visible", Some("true")).await?;
    Ok(())
}
