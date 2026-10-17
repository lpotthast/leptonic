// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the atom `TagGroup` (`/perf/atoms/tag_group`): what each thing it offers
//! costs, measured (`Page::measure`) and checked against its budget
//! (`tests/budgets/atoms/tag_group.json`): mounting and unmounting it, selecting tags with the
//! mouse and the keyboard, keyboard navigation, removing tags with Delete, Backspace and their
//! remove buttons, a tag's pressed and hovered states, and changes of its reactive props
//! (selection, selection mode, disabled tags), with 10 tags and with 1,000.
use assertr::prelude::*;
use browser_test::{TestGroup, browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/atoms/tag_group";

/// Registers the performance tests of `TagGroup`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(MountAndUnmountLarge))
        .with(fixture_test(SelectWithTheMouse))
        .with(fixture_test(SelectWithTheMouseLarge))
        .with(fixture_test(KeyboardNavigation))
        .with(fixture_test(KeyboardNavigationLarge))
        .with(fixture_test(SelectWithTheKeyboard))
        .with(fixture_test(RemoveWithTheKeyboard))
        .with(fixture_test(RemoveWithTheKeyboardLarge))
        .with(fixture_test(RemoveWithTheButton))
        .with(fixture_test(RemoveWithTheButtonLarge))
        .with(fixture_test(HoldAndRelease))
        .with(fixture_test(HoverAndLeave))
        .with(fixture_test(ControlledSelection))
        .with(fixture_test(ControlledSelectionLarge))
        .with(fixture_test(DisabledTag))
        .with(fixture_test(SelectionMode))
}
const GROUP: &str = ".leptonic-TagGroup";
/// Loads the fixture at `path` and creates the live announcer's regions (its first announcement
/// waits for them, so an announcing step measured first would count them, or not).
async fn goto(page: &Page<'_>, path: &str) -> Result<(), Report> {
    page.goto_path(path).await?;
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

/// The remove button of the tag "Tag `i`".
fn remove_button(i: u32) -> String {
    format!("{} .leptonic-TagRemoveButton", tag(i))
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

/// Mounting the tag group (10 tags) and unmounting it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    goto(page, PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(GROUP, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.element(tag(9)).await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(GROUP, 0).await?;
    Ok(())
}

/// Mounting the tag group with 1,000 tags and unmounting it.
#[browser_test]
pub async fn mount_and_unmount_large(page: &Page<'_>) -> Result<(), Report> {
    goto(page, PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(GROUP, 0).await?;
    page.control("large").await?;
    page.measure("mount", page.control("mount")).await?;
    page.element(tag(LARGE - 1)).await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(GROUP, 0).await?;
    Ok(())
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

/// Tab focusing the first tag, ArrowRight, ArrowLeft, End and Home moving the focus through the
/// tags (`last + 1`), and Shift+Tab leaving the group.
async fn keyboard_navigation_in(page: &Page<'_>, last: u32) -> Result<(), Report> {
    page.element("#perf-before").await?.click().await?;
    let first = page.element(tag(0)).await?;
    page.measure("focus", page.send_keys(Key::Tab)).await?;
    page.wait_for_focus(&first).await?;
    page.measure("arrow right", page.send_keys(Key::Right))
        .await?;
    page.wait_for_focus(&page.element(tag(1)).await?).await?;
    page.measure("arrow left", page.send_keys(Key::Left))
        .await?;
    page.wait_for_focus(&first).await?;
    page.measure("end", page.send_keys(Key::End)).await?;
    page.wait_for_focus(&page.element(tag(last)).await?).await?;
    page.measure("home", page.send_keys(Key::Home)).await?;
    page.wait_for_focus(&first).await?;
    page.measure("blur", page.send_keys(Key::Shift + Key::Tab))
        .await?;
    page.wait_for_focus(&page.element("#perf-before").await?)
        .await?;
    Ok(())
}

/// Keyboard navigation through 10 tags.
#[browser_test]
pub async fn keyboard_navigation(page: &Page<'_>) -> Result<(), Report> {
    goto(page, PATH).await?;
    keyboard_navigation_in(page, 9).await
}

/// Keyboard navigation through 1,000 tags.
#[browser_test]
pub async fn keyboard_navigation_large(page: &Page<'_>) -> Result<(), Report> {
    goto(page, PATH).await?;
    large(page).await?;
    keyboard_navigation_in(page, LARGE - 1).await
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

/// Selecting a tag with the keyboard.
#[browser_test]
pub async fn select_with_the_keyboard(page: &Page<'_>) -> Result<(), Report> {
    goto(page, PATH).await?;
    select_with_the_keyboard_in(page).await
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

/// Removing two of 10 tags with the keyboard.
#[browser_test]
pub async fn remove_with_the_keyboard(page: &Page<'_>) -> Result<(), Report> {
    goto(page, PATH).await?;
    remove_with_the_keyboard_in(page).await
}

/// Removing two of 1,000 tags with the keyboard.
#[browser_test]
pub async fn remove_with_the_keyboard_large(page: &Page<'_>) -> Result<(), Report> {
    goto(page, PATH).await?;
    large(page).await?;
    remove_with_the_keyboard_in(page).await
}

/// A click on the fourth tag's remove button removing it.
async fn remove_with_the_button_in(page: &Page<'_>) -> Result<(), Report> {
    let button = page.element(remove_button(3)).await?;
    page.measure("remove", button.click()).await?;
    page.wait_for_count(tag(3), 0).await?;
    Ok(())
}

/// Removing one of 10 tags with its remove button.
#[browser_test]
pub async fn remove_with_the_button(page: &Page<'_>) -> Result<(), Report> {
    goto(page, PATH).await?;
    remove_with_the_button_in(page).await
}

/// Removing one of 1,000 tags with its remove button.
#[browser_test]
pub async fn remove_with_the_button_large(page: &Page<'_>) -> Result<(), Report> {
    goto(page, PATH).await?;
    large(page).await?;
    remove_with_the_button_in(page).await
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

/// Holding a tag and releasing it.
#[browser_test]
pub async fn hold_and_release(page: &Page<'_>) -> Result<(), Report> {
    goto(page, PATH).await?;
    hold_and_release_in(page).await
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

/// Hovering a tag and leaving it.
#[browser_test]
pub async fn hover_and_leave(page: &Page<'_>) -> Result<(), Report> {
    goto(page, PATH).await?;
    hover_and_leave_in(page).await
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

/// The controlled selection of 10 tags changing.
#[browser_test]
pub async fn controlled_selection(page: &Page<'_>) -> Result<(), Report> {
    goto(page, PATH).await?;
    controlled_selection_in(page, 9).await
}

/// The controlled selection of 1,000 tags changing.
#[browser_test]
pub async fn controlled_selection_large(page: &Page<'_>) -> Result<(), Report> {
    goto(page, PATH).await?;
    large(page).await?;
    controlled_selection_in(page, LARGE - 1).await
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

/// A disabled tag.
#[browser_test]
pub async fn disabled_tag(page: &Page<'_>) -> Result<(), Report> {
    goto(page, PATH).await?;
    disabled_tag_in(page).await
}

/// Switching to single selection and back to multiple.
#[browser_test]
pub async fn selection_mode(page: &Page<'_>) -> Result<(), Report> {
    goto(page, PATH).await?;
    let first = page.element(tag(0)).await?;
    page.measure("single", page.control("single")).await?;
    first
        .wait_for_attr("data-selection-mode", Some("single"))
        .await?;
    page.measure("multiple", page.control("multiple")).await?;
    first
        .wait_for_attr("data-selection-mode", Some("multiple"))
        .await?;
    Ok(())
}
