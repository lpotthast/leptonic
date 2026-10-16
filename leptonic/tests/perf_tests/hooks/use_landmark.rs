// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the hook `use_landmark` (`/perf/hooks/use_landmark`, 2 or 50 regions):
//! what each thing it offers costs, measured (`Page::measure`) and checked against its budget
//! (`tests/budgets/hooks/use_landmark.json`): mounting and unmounting the landmarks, F6 moving
//! the focus to the first landmark, to the next and wrapping around, Shift+F6 back, focus moving
//! on inside a landmark (it leaves the tab order again), at both sizes (navigating orders the
//! landmarks), and a landmark's label changing.
use browser_test::{TestGroup, browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/hooks/use_landmark";

/// Registers the performance tests of `use_landmark`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(MountAndUnmount50))
        .with(fixture_test(Navigate))
        .with(fixture_test(Navigate50))
        .with(fixture_test(Rename))
}

/// The landmark `i`.
fn landmark(i: usize) -> String {
    format!("#perf-landmark-{i}")
}

/// Opens the page with `landmarks` landmarks (2 or 50).
async fn open(page: &Page<'_>, landmarks: usize) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control(&format!("use-{landmarks}")).await?;
    page.wait_for_count("[id^=perf-landmark-]", landmarks)
        .await?;
    Ok(())
}

/// Mounting the landmarks and unmounting them.
async fn mount_and_unmount_with(page: &Page<'_>, landmarks: usize) -> Result<(), Report> {
    open(page, landmarks).await?;
    page.control("unmount").await?;
    page.wait_for_count("[id^=perf-landmark-]", 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.element(landmark(landmarks - 1)).await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count("[id^=perf-landmark-]", 0).await?;
    Ok(())
}

/// F6 to the first landmark, F6 to the second, Shift+F6 back, Shift+F6 wrapping to the last, F6
/// wrapping to the first, and a click on its button (the landmark leaves the tab order).
async fn navigate_with(page: &Page<'_>, landmarks: usize) -> Result<(), Report> {
    open(page, landmarks).await?;
    let first = page.element(landmark(0)).await?;
    let last = page.element(landmark(landmarks - 1)).await?;
    page.measure("first", page.send_keys(Key::F6)).await?;
    page.wait_for_focus(&first).await?;
    page.measure("next", page.send_keys(Key::F6)).await?;
    page.wait_for_focus(&page.element(landmark(1)).await?)
        .await?;
    page.measure("previous", page.send_keys(Key::Shift + Key::F6))
        .await?;
    page.wait_for_focus(&first).await?;
    page.measure("previous, wrapping", page.send_keys(Key::Shift + Key::F6))
        .await?;
    page.wait_for_focus(&last).await?;
    page.measure("next, wrapping", page.send_keys(Key::F6))
        .await?;
    page.wait_for_focus(&first).await?;
    let button = page.element("#perf-button-0").await?;
    page.measure("focus inside", button.click()).await?;
    first.wait_for_attr("tabindex", None).await?;
    Ok(())
}

/// Mounting 2 landmarks and unmounting them.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    mount_and_unmount_with(page, 2).await
}

/// Mounting 50 landmarks and unmounting them.
#[browser_test]
pub async fn mount_and_unmount_50(page: &Page<'_>) -> Result<(), Report> {
    mount_and_unmount_with(page, 50).await
}

/// Navigating 2 landmarks.
#[browser_test]
pub async fn navigate(page: &Page<'_>) -> Result<(), Report> {
    navigate_with(page, 2).await
}

/// Navigating 50 landmarks.
#[browser_test]
pub async fn navigate_50(page: &Page<'_>) -> Result<(), Report> {
    navigate_with(page, 50).await
}

/// Renaming the first landmark.
#[browser_test]
pub async fn rename(page: &Page<'_>) -> Result<(), Report> {
    open(page, 2).await?;
    let first = page.element(landmark(0)).await?;
    page.measure("rename", page.control("rename")).await?;
    first.wait_for_attr("aria-label", Some("Renamed")).await?;
    Ok(())
}
