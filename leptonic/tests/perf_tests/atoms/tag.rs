// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the atom `Tag` (`/perf/atoms/tag`, the fixture of `TagGroup`'s performance
//! test): what each thing it offers costs, measured (`Page::measure`) and checked against its
//! budget (`tests/budgets/atoms/tag.json`): mounting and unmounting 10 and 1,000 tags in their
//! list, selecting one with the mouse (at both sizes) and the keyboard, removing it with Delete and
//! Backspace, its pressed and hovered states, and the changes of its selected and disabled state
//! from the group.
use browser_test::{TestGroup, browser_test};
use rootcause::Report;

use super::tag_group::{
    TAGS, controlled_selection_in, disabled_tag_in, goto, hold_and_release_in, hover_and_leave_in,
    large, remove_with_the_keyboard_in, select_with_the_keyboard_in, select_with_the_mouse_in,
};
use crate::{harness::fixture_test, pages::Page};

const PATH: &str = "/perf/atoms/tag";

/// Registers the performance tests of `Tag`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(MountAndUnmountLarge))
        .with(fixture_test(SelectWithTheMouse))
        .with(fixture_test(SelectWithTheMouseLarge))
        .with(fixture_test(SelectWithTheKeyboard))
        .with(fixture_test(RemoveWithTheKeyboard))
        .with(fixture_test(HoldAndRelease))
        .with(fixture_test(HoverAndLeave))
        .with(fixture_test(ControlledSelection))
        .with(fixture_test(Disabled))
}

/// Mounting `count` tags in their list and unmounting them.
async fn mount_and_unmount_in(page: &Page<'_>, count: usize) -> Result<(), Report> {
    page.control("unmount").await?;
    page.wait_for_count(TAGS, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.wait_for_count(TAGS, count).await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(TAGS, 0).await?;
    Ok(())
}

/// Mounting 10 tags and unmounting them.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    goto(page, PATH).await?;
    mount_and_unmount_in(page, 10).await
}

/// Mounting 1,000 tags and unmounting them.
#[browser_test]
pub async fn mount_and_unmount_large(page: &Page<'_>) -> Result<(), Report> {
    goto(page, PATH).await?;
    large(page).await?;
    mount_and_unmount_in(page, 1_000).await
}

/// Selecting and deselecting one of 10 tags with clicks.
#[browser_test]
pub async fn select_with_the_mouse(page: &Page<'_>) -> Result<(), Report> {
    goto(page, PATH).await?;
    select_with_the_mouse_in(page).await
}

/// Selecting and deselecting one of 1,000 tags with clicks.
#[browser_test]
pub async fn select_with_the_mouse_large(page: &Page<'_>) -> Result<(), Report> {
    goto(page, PATH).await?;
    large(page).await?;
    select_with_the_mouse_in(page).await
}

/// Space selecting the focused tag and Enter deselecting it.
#[browser_test]
pub async fn select_with_the_keyboard(page: &Page<'_>) -> Result<(), Report> {
    goto(page, PATH).await?;
    select_with_the_keyboard_in(page).await
}

/// Delete and Backspace removing the focused tag.
#[browser_test]
pub async fn remove_with_the_keyboard(page: &Page<'_>) -> Result<(), Report> {
    goto(page, PATH).await?;
    remove_with_the_keyboard_in(page).await
}

/// Holding a tag (its pressed state) and releasing it.
#[browser_test]
pub async fn hold_and_release(page: &Page<'_>) -> Result<(), Report> {
    goto(page, PATH).await?;
    hold_and_release_in(page).await
}

/// Hovering a tag and leaving it.
#[browser_test]
pub async fn hover_and_leave(page: &Page<'_>) -> Result<(), Report> {
    goto(page, PATH).await?;
    hover_and_leave_in(page).await
}

/// The group's controlled selection changing the tags' selected state.
#[browser_test]
pub async fn controlled_selection(page: &Page<'_>) -> Result<(), Report> {
    goto(page, PATH).await?;
    controlled_selection_in(page, 9).await
}

/// A tag disabled, a click it ignores, and enabled again.
#[browser_test]
pub async fn disabled(page: &Page<'_>) -> Result<(), Report> {
    goto(page, PATH).await?;
    disabled_tag_in(page).await
}
