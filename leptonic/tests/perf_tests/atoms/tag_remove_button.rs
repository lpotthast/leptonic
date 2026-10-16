// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the atom `TagRemoveButton` (`/perf/atoms/tag_remove_button`, the fixture of
//! `TagGroup`'s performance test): what each thing it offers costs, measured (`Page::measure`) and
//! checked against its budget (`tests/budgets/atoms/tag_remove_button.json`): mounting and
//! unmounting the buttons of 10 tags and of 1,000, a click removing its tag (at both sizes), and
//! its pressed and hovered states.
use browser_test::{TestGroup, browser_test};
use rootcause::Report;

use super::tag_group::{REMOVE_BUTTONS, goto, large, remove_button, remove_with_the_button_in};
use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/atoms/tag_remove_button";

/// Registers the performance tests of `TagRemoveButton`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(MountAndUnmountLarge))
        .with(fixture_test(Remove))
        .with(fixture_test(RemoveLarge))
        .with(fixture_test(HoldAndRelease))
        .with(fixture_test(HoverAndLeave))
}

/// Mounting the remove buttons of `count` tags and unmounting them.
async fn mount_and_unmount_in(page: &Page<'_>, count: usize) -> Result<(), Report> {
    page.control("unmount").await?;
    page.wait_for_count(REMOVE_BUTTONS, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.wait_for_count(REMOVE_BUTTONS, count).await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(REMOVE_BUTTONS, 0).await?;
    Ok(())
}

/// Mounting the remove buttons of 10 tags and unmounting them.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    goto(page, PATH).await?;
    mount_and_unmount_in(page, 10).await
}

/// Mounting the remove buttons of 1,000 tags and unmounting them.
#[browser_test]
pub async fn mount_and_unmount_large(page: &Page<'_>) -> Result<(), Report> {
    goto(page, PATH).await?;
    large(page).await?;
    mount_and_unmount_in(page, 1_000).await
}

/// A click removing one of 10 tags.
#[browser_test]
pub async fn remove(page: &Page<'_>) -> Result<(), Report> {
    goto(page, PATH).await?;
    remove_with_the_button_in(page).await
}

/// A click removing one of 1,000 tags.
#[browser_test]
pub async fn remove_large(page: &Page<'_>) -> Result<(), Report> {
    goto(page, PATH).await?;
    large(page).await?;
    remove_with_the_button_in(page).await
}

/// Holding the pointer down on a remove button (its pressed state) and releasing it (removing
/// its tag).
#[browser_test]
pub async fn hold_and_release(page: &Page<'_>) -> Result<(), Report> {
    goto(page, PATH).await?;
    let button = page.element(remove_button(3)).await?;
    let held = page.measure("hold", button.press_and_hold()).await?;
    button.wait_for_attr("data-pressed", Some("true")).await?;
    page.measure("release", held.release()).await?;
    page.wait_for_count(remove_button(3), 0).await?;
    Ok(())
}

/// The pointer entering and leaving a remove button.
#[browser_test]
pub async fn hover_and_leave(page: &Page<'_>) -> Result<(), Report> {
    goto(page, PATH).await?;
    let button = page.element(remove_button(3)).await?;
    let after = page.element("#perf-after").await?;
    page.measure("hover", button.hover()).await?;
    button.wait_for_attr("data-hovered", Some("true")).await?;
    page.measure("leave", after.hover()).await?;
    button.wait_for_attr("data-hovered", None).await?;
    Ok(())
}
