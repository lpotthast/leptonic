// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the atom `TagRemoveButton` (`/perf/atoms/tag_remove_button`, in the tags of a
//! `TagGroup`): what each thing it offers costs, measured (`Page::measure`) and checked against its
//! budget (`tests/budgets/atoms/tag_remove_button.json`): mounting and unmounting the buttons of 10
//! tags and of 1,000, a click removing its tag (at both sizes), and its pressed and hovered states.
use browser_test::{TestGroup, browser_test};
use rootcause::Report;

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
const REMOVE_BUTTONS: &str = ".leptonic-TagRemoveButton";

/// Loads the fixture and creates the live announcer's regions (its first announcement waits for
/// them, so an announcing step measured first would count them, or not).
async fn load(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("prepare-announcer").await?;
    page.wait_for_count("[data-live-announcer] [aria-live=polite] > div", 1)
        .await?;
    Ok(())
}

/// The fixture's tags after the control `large`.
const LARGE: u32 = 1_000;

/// The tag "Tag `i`".
fn tag(i: u32) -> String {
    format!(".leptonic-Tag:has([data-perf-tag='{i}'])")
}

/// Switches to [`LARGE`] tags (unmeasured).
async fn large(page: &Page<'_>) -> Result<(), Report> {
    page.control("large").await?;
    page.element(tag(LARGE - 1)).await?;
    Ok(())
}

/// The remove button of the tag "Tag `i`".
fn remove_button(i: u32) -> String {
    format!("{} .leptonic-TagRemoveButton", tag(i))
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
    load(page).await?;
    mount_and_unmount_in(page, 10).await
}

/// Mounting the remove buttons of 1,000 tags and unmounting them.
#[browser_test]
pub async fn mount_and_unmount_large(page: &Page<'_>) -> Result<(), Report> {
    load(page).await?;
    large(page).await?;
    mount_and_unmount_in(page, 1_000).await
}

/// A click on the fourth tag's remove button removing it.
async fn remove_in(page: &Page<'_>) -> Result<(), Report> {
    let button = page.element(remove_button(3)).await?;
    page.measure("remove", button.click()).await?;
    page.wait_for_count(tag(3), 0).await?;
    Ok(())
}

/// A click removing one of 10 tags.
#[browser_test]
pub async fn remove(page: &Page<'_>) -> Result<(), Report> {
    load(page).await?;
    remove_in(page).await
}

/// A click removing one of 1,000 tags.
#[browser_test]
pub async fn remove_large(page: &Page<'_>) -> Result<(), Report> {
    load(page).await?;
    large(page).await?;
    remove_in(page).await
}

/// Holding the pointer down on a remove button (its pressed state) and releasing it (removing
/// its tag).
#[browser_test]
pub async fn hold_and_release(page: &Page<'_>) -> Result<(), Report> {
    load(page).await?;
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
    load(page).await?;
    let button = page.element(remove_button(3)).await?;
    let after = page.element("#perf-after").await?;
    page.measure("hover", button.hover()).await?;
    button.wait_for_attr("data-hovered", Some("true")).await?;
    page.measure("leave", after.hover()).await?;
    button.wait_for_attr("data-hovered", None).await?;
    Ok(())
}
