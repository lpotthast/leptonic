// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the atom `FocusManagerProvider` (`/perf/atoms/focus_manager_provider`,
//! around 10 or 1,000 buttons whose keys move the focus with its focus manager): what each thing
//! it offers costs, measured (`Page::measure`) and checked against its budget
//! (`tests/budgets/atoms/focus_manager_provider.json`): mounting and unmounting it, and the
//! manager's moves (next, previous, wrapping around, first, last), which walk its elements: at
//! both sizes.
use browser_test::{TestGroup, browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::{harness::fixture_test, pages::Page};

const PATH: &str = "/perf/atoms/focus_manager_provider";

/// Registers the performance tests of `FocusManagerProvider`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(MountAndUnmount1000))
        .with(fixture_test(Moves))
        .with(fixture_test(Moves1000))
}
const MANAGER: &str = ".perf-manager";

/// The button `i`.
fn button(i: usize) -> String {
    format!("#perf-manager-{i}")
}

/// Opens the page with `buttons` buttons (10 or 1,000).
async fn open(page: &Page<'_>, buttons: usize) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control(&format!("use-{buttons}")).await?;
    page.wait_for_count(".perf-manager button", buttons).await?;
    Ok(())
}

/// Mounting the provider with its buttons and unmounting it.
async fn mount_and_unmount_with(page: &Page<'_>, buttons: usize) -> Result<(), Report> {
    open(page, buttons).await?;
    page.control("unmount").await?;
    page.wait_for_count(MANAGER, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.element(button(buttons - 1)).await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(MANAGER, 0).await?;
    Ok(())
}

/// End (the last button), ArrowRight (wrapping to the first), ArrowLeft (wrapping to the last),
/// ArrowLeft (the one before) and Home (the first).
async fn moves_with(page: &Page<'_>, buttons: usize) -> Result<(), Report> {
    open(page, buttons).await?;
    let first = page.element(button(0)).await?;
    let last = page.element(button(buttons - 1)).await?;
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
    page.wait_for_focus(&page.element(button(buttons - 2)).await?)
        .await?;
    page.measure("first", page.send_keys(Key::Home)).await?;
    page.wait_for_focus(&first).await?;
    Ok(())
}

/// Mounting the provider with 10 buttons and unmounting it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    mount_and_unmount_with(page, 10).await
}

/// Mounting the provider with 1,000 buttons and unmounting it.
#[browser_test]
pub async fn mount_and_unmount_1000(page: &Page<'_>) -> Result<(), Report> {
    mount_and_unmount_with(page, 1000).await
}

/// The manager's moves among 10 buttons.
#[browser_test]
pub async fn moves(page: &Page<'_>) -> Result<(), Report> {
    moves_with(page, 10).await
}

/// The manager's moves among 1,000 buttons.
#[browser_test]
pub async fn moves_1000(page: &Page<'_>) -> Result<(), Report> {
    moves_with(page, 1000).await
}
