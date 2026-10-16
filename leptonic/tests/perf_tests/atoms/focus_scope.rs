// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the atom `FocusScope` (`/perf/atoms/focus_scope`, containing focus, around
//! 10 or 1,000 buttons): what each thing it offers costs, measured (`Page::measure`) and checked
//! against its budget (`tests/budgets/atoms/focus_scope.json`): mounting and unmounting it, Tab
//! moving within it and wrapping around at its ends (forwards and backwards), a click outside
//! (the focus is recaptured), switching the containment off and on, and restoring the focus when
//! it unmounts. Mounting and wrapping (which walks the scope) at both sizes.
use browser_test::{TestGroup, browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::{harness::fixture_test, pages::Page};

const PATH: &str = "/perf/atoms/focus_scope";

/// Registers the performance tests of `FocusScope`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(MountAndUnmount1000))
        .with(fixture_test(TabWithin))
        .with(fixture_test(TabWraps))
        .with(fixture_test(TabWraps1000))
        .with(fixture_test(ClickOutside))
        .with(fixture_test(ReleaseAndContain))
        .with(fixture_test(RestoreFocus))
}
const SCOPE: &str = ".perf-scope";

/// The scope's button `i`.
fn button(i: usize) -> String {
    format!("#perf-scope-{i}")
}

/// Opens the page with `buttons` buttons in the scope (10 or 1,000).
async fn open(page: &Page<'_>, buttons: usize) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control(&format!("use-{buttons}")).await?;
    page.wait_for_count(".perf-scope button", buttons).await?;
    Ok(())
}

/// Mounting the scope and unmounting it.
async fn mount_and_unmount_with(page: &Page<'_>, buttons: usize) -> Result<(), Report> {
    open(page, buttons).await?;
    page.control("unmount").await?;
    page.wait_for_count(SCOPE, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.element(button(buttons - 1)).await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(SCOPE, 0).await?;
    Ok(())
}

/// Tab from the last button to the first, and Shift+Tab back.
async fn tab_wraps_with(page: &Page<'_>, buttons: usize) -> Result<(), Report> {
    open(page, buttons).await?;
    let first = page.element(button(0)).await?;
    let last = page.element(button(buttons - 1)).await?;
    last.click().await?;
    page.wait_for_focus(&last).await?;
    page.measure("tab", page.send_keys(Key::Tab)).await?;
    page.wait_for_focus(&first).await?;
    page.measure("shift+tab", page.send_keys(Key::Shift + Key::Tab))
        .await?;
    page.wait_for_focus(&last).await?;
    Ok(())
}

/// Mounting the scope of 10 buttons and unmounting it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    mount_and_unmount_with(page, 10).await
}

/// Mounting the scope of 1,000 buttons and unmounting it.
#[browser_test]
pub async fn mount_and_unmount_1000(page: &Page<'_>) -> Result<(), Report> {
    mount_and_unmount_with(page, 1000).await
}

/// Tab moving from the first button to the second.
#[browser_test]
pub async fn tab_within(page: &Page<'_>) -> Result<(), Report> {
    open(page, 10).await?;
    page.element(button(0)).await?.click().await?;
    page.measure("tab", page.send_keys(Key::Tab)).await?;
    page.wait_for_focus(&page.element(button(1)).await?).await?;
    Ok(())
}

/// Tab wrapping around the ends of the scope of 10 buttons.
#[browser_test]
pub async fn tab_wraps(page: &Page<'_>) -> Result<(), Report> {
    tab_wraps_with(page, 10).await
}

/// Tab wrapping around the ends of the scope of 1,000 buttons.
#[browser_test]
pub async fn tab_wraps_1000(page: &Page<'_>) -> Result<(), Report> {
    tab_wraps_with(page, 1000).await
}

/// A click outside the containing scope: the focus returns to the button it left.
#[browser_test]
pub async fn click_outside(page: &Page<'_>) -> Result<(), Report> {
    open(page, 10).await?;
    let inside = page.element(button(3)).await?;
    inside.click().await?;
    page.wait_for_focus(&inside).await?;
    let after = page.element("#perf-after").await?;
    page.measure("click outside", after.click()).await?;
    page.wait_for_focus(&inside).await?;
    Ok(())
}

/// Switching the containment off (Tab leaves the scope) and on again.
#[browser_test]
pub async fn release_and_contain(page: &Page<'_>) -> Result<(), Report> {
    open(page, 10).await?;
    page.measure("release", page.control("release")).await?;
    page.element(button(9)).await?.click().await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&page.element("#perf-after").await?)
        .await?;
    page.measure("contain", page.control("contain")).await?;
    let last = page.element(button(9)).await?;
    last.click().await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&page.element(button(0)).await?).await?;
    Ok(())
}

/// Unmounting the scope, which restores the focus to the button focused when it mounted.
#[browser_test]
pub async fn restore_focus(page: &Page<'_>) -> Result<(), Report> {
    open(page, 10).await?;
    page.control("unmount").await?;
    page.wait_for_count(SCOPE, 0).await?;
    page.control("restore-focus").await?;
    let before = page.element("#perf-before").await?;
    before.click().await?;
    page.wait_for_focus(&before).await?;
    page.control("mount").await?;
    let inside = page.element(button(3)).await?;
    inside.click().await?;
    page.wait_for_focus(&inside).await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_focus(&before).await?;
    Ok(())
}
