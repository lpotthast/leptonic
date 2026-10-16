// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the hook `use_focus_manager_context`
//! (`/perf/hooks/use_focus_manager_context`, in a group of 10 buttons inside a `FocusManagerProvider`, whose keys move the focus with the
//! manager it finds): what each thing it offers costs, measured (`Page::measure`) and checked
//! against its budget (`tests/budgets/hooks/use_focus_manager_context.json`): mounting and
//! unmounting the group, and the moves of the manager it finds (next, previous, wrapping around,
//! first, last). Those with 1,000 buttons are the `FocusManagerProvider`'s.
use browser_test::{TestGroup, browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::{harness::fixture_test, pages::Page};

const PATH: &str = "/perf/hooks/use_focus_manager_context";

/// Registers the performance tests of `use_focus_manager_context`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(Moves))
}
const GROUP: &str = "#perf-group";
const BUTTONS: usize = 10;

/// The button `i`.
fn button(i: usize) -> String {
    format!("#perf-button-{i}")
}

/// Mounting the group and unmounting it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(GROUP, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.element(button(BUTTONS - 1)).await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(GROUP, 0).await?;
    Ok(())
}

/// End (the last button), ArrowRight (wrapping to the first), ArrowLeft (wrapping to the last),
/// ArrowLeft (the one before) and Home (the first).
#[browser_test]
pub async fn moves(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let first = page.element(button(0)).await?;
    let last = page.element(button(BUTTONS - 1)).await?;
    first.click().await?;
    page.wait_for_focus(&first).await?;
    page.measure("last", page.send_keys(Key::End)).await?;
    page.wait_for_focus(&last).await?;
    page.measure("next, wrapping", page.send_keys(Key::Right))
        .await?;
    page.wait_for_focus(&first).await?;
    page.measure("previous, wrapping", page.send_keys(Key::Left))
        .await?;
    page.wait_for_focus(&last).await?;
    page.measure("previous", page.send_keys(Key::Left)).await?;
    page.wait_for_focus(&page.element(button(BUTTONS - 2)).await?)
        .await?;
    page.measure("first", page.send_keys(Key::Home)).await?;
    page.wait_for_focus(&first).await?;
    Ok(())
}
