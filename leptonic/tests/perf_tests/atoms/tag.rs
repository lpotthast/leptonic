// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the atom `Tag` (`/perf/atoms/tag`, in a `TagGroup`): what each thing it
//! offers costs, measured (`Page::measure`) and checked against its budget
//! (`tests/budgets/atoms/tag.json`): mounting and unmounting 10 and 1,000 tags in their list,
//! selecting one with the mouse (at both sizes) and the keyboard, removing it with Delete and
//! Backspace, its pressed and hovered states, and the changes of its selected and disabled state
//! from the group.
use assertr::prelude::*;
use browser_test::{TestGroup, browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

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
const TAGS: &str = ".leptonic-Tag";

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

/// Focuses the first tag with the keyboard (Tab from the plain button before the group).
async fn tab_in(page: &Page<'_>) -> Result<WebElement, Report> {
    page.element("#perf-before").await?.click().await?;
    page.send_keys(Key::Tab).await?;
    let first = page.element(tag(0)).await?;
    page.wait_for_focus(&first).await?;
    Ok(first)
}

/// A click selecting a tag (it takes focus) and one deselecting it.
async fn select_with_the_mouse_in(page: &Page<'_>) -> Result<(), Report> {
    let third = page.element(tag(3)).await?;
    page.measure("select", third.click()).await?;
    third.wait_for_attr("aria-selected", Some("true")).await?;
    page.measure("deselect", third.click()).await?;
    third.wait_for_attr("aria-selected", Some("false")).await?;
    Ok(())
}

/// Space selecting the focused tag and Enter deselecting it.
async fn select_with_the_keyboard_in(page: &Page<'_>) -> Result<(), Report> {
    let first = tab_in(page).await?;
    page.measure("space", page.send_keys(" ")).await?;
    first.wait_for_attr("aria-selected", Some("true")).await?;
    page.measure("enter", page.send_keys(Key::Enter)).await?;
    first.wait_for_attr("aria-selected", Some("false")).await?;
    Ok(())
}

/// Delete removing the focused fourth tag (focus moves to the next) and Backspace removing that
/// one.
async fn remove_with_the_keyboard_in(page: &Page<'_>) -> Result<(), Report> {
    tab_in(page).await?;
    for i in 1..4 {
        page.send_keys(Key::Right).await?;
        page.wait_for_focus(&page.element(tag(i)).await?).await?;
    }
    page.measure("delete", page.send_keys(Key::Delete)).await?;
    page.wait_for_count(tag(3), 0).await?;
    page.wait_for_focus(&page.element(tag(4)).await?).await?;
    page.measure("backspace", page.send_keys(Key::Backspace))
        .await?;
    page.wait_for_count(tag(4), 0).await?;
    Ok(())
}

/// Holding the pointer down on a tag (its pressed state; it selects) and releasing it.
async fn hold_and_release_in(page: &Page<'_>) -> Result<(), Report> {
    let third = page.element(tag(3)).await?;
    let held = page.measure("hold", third.press_and_hold()).await?;
    third.wait_for_attr("data-pressed", Some("true")).await?;
    page.measure("release", held.release()).await?;
    third.wait_for_attr("data-pressed", None).await?;
    assert_that!(third)
        .has_attribute("aria-selected")
        .await
        .is_equal_to("true");
    Ok(())
}

/// The pointer entering and leaving a tag.
async fn hover_and_leave_in(page: &Page<'_>) -> Result<(), Report> {
    let third = page.element(tag(3)).await?;
    let after = page.element("#perf-after").await?;
    page.measure("hover", third.hover()).await?;
    third.wait_for_attr("data-hovered", Some("true")).await?;
    page.measure("leave", after.hover()).await?;
    third.wait_for_attr("data-hovered", None).await?;
    Ok(())
}

/// The controlled selection changing: the first tag, all tags (`last + 1`), none.
async fn controlled_selection_in(page: &Page<'_>, last: u32) -> Result<(), Report> {
    let first = page.element(tag(0)).await?;
    let last = page.element(tag(last)).await?;
    page.measure("select one", page.control("select-first"))
        .await?;
    first.wait_for_attr("data-selected", Some("true")).await?;
    page.measure("select all", page.control("select-all"))
        .await?;
    last.wait_for_attr("data-selected", Some("true")).await?;
    page.measure("clear", page.control("clear-selection"))
        .await?;
    first.wait_for_attr("data-selected", None).await?;
    last.wait_for_attr("data-selected", None).await?;
    Ok(())
}

/// Disabling the first tag, a click it ignores, and enabling it again.
async fn disabled_tag_in(page: &Page<'_>) -> Result<(), Report> {
    let first = page.element(tag(0)).await?;
    page.measure("disable", page.control("disable-first"))
        .await?;
    first.wait_for_attr("data-disabled", Some("true")).await?;
    page.measure("ignored click", first.click()).await?;
    assert_that!(first)
        .attribute("data-selected")
        .await
        .is_none();
    page.measure("enable", page.control("enable-all")).await?;
    first.wait_for_attr("data-disabled", None).await?;
    Ok(())
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
    load(page).await?;
    mount_and_unmount_in(page, 10).await
}

/// Mounting 1,000 tags and unmounting them.
#[browser_test]
pub async fn mount_and_unmount_large(page: &Page<'_>) -> Result<(), Report> {
    load(page).await?;
    large(page).await?;
    mount_and_unmount_in(page, 1_000).await
}

/// Selecting and deselecting one of 10 tags with clicks.
#[browser_test]
pub async fn select_with_the_mouse(page: &Page<'_>) -> Result<(), Report> {
    load(page).await?;
    select_with_the_mouse_in(page).await
}

/// Selecting and deselecting one of 1,000 tags with clicks.
#[browser_test]
pub async fn select_with_the_mouse_large(page: &Page<'_>) -> Result<(), Report> {
    load(page).await?;
    large(page).await?;
    select_with_the_mouse_in(page).await
}

/// Space selecting the focused tag and Enter deselecting it.
#[browser_test]
pub async fn select_with_the_keyboard(page: &Page<'_>) -> Result<(), Report> {
    load(page).await?;
    select_with_the_keyboard_in(page).await
}

/// Delete and Backspace removing the focused tag.
#[browser_test]
pub async fn remove_with_the_keyboard(page: &Page<'_>) -> Result<(), Report> {
    load(page).await?;
    remove_with_the_keyboard_in(page).await
}

/// Holding a tag (its pressed state) and releasing it.
#[browser_test]
pub async fn hold_and_release(page: &Page<'_>) -> Result<(), Report> {
    load(page).await?;
    hold_and_release_in(page).await
}

/// Hovering a tag and leaving it.
#[browser_test]
pub async fn hover_and_leave(page: &Page<'_>) -> Result<(), Report> {
    load(page).await?;
    hover_and_leave_in(page).await
}

/// The group's controlled selection changing the tags' selected state.
#[browser_test]
pub async fn controlled_selection(page: &Page<'_>) -> Result<(), Report> {
    load(page).await?;
    controlled_selection_in(page, 9).await
}

/// A tag disabled, a click it ignores, and enabled again.
#[browser_test]
pub async fn disabled(page: &Page<'_>) -> Result<(), Report> {
    load(page).await?;
    disabled_tag_in(page).await
}
