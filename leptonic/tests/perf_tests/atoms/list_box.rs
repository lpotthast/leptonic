// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the atom `ListBox` (`/perf/atoms/list_box`): what each thing it offers costs,
//! measured (`Page::measure`) and checked against its budget (`tests/budgets/atoms/list_box.json`):
//! mounting and unmounting it, selecting with the mouse and the keyboard, keyboard navigation,
//! select all, type-ahead, hover, focus, changes of its options and of its reactive props
//! (selection, selection mode and behavior, disabled options, orientation, label), with 10 options
//! and with 1,000.
use assertr::prelude::*;
use browser_test::{TestGroup, browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/atoms/list_box";

/// Registers the performance tests of `ListBox`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(MountAndUnmountLarge))
        .with(fixture_test(SelectWithTheMouse))
        .with(fixture_test(SelectWithTheMouseLarge))
        .with(fixture_test(HoldAndRelease))
        .with(fixture_test(HoverAndLeave))
        .with(fixture_test(KeyboardFocusAndBlur))
        .with(fixture_test(KeyboardNavigation))
        .with(fixture_test(KeyboardNavigationLarge))
        .with(fixture_test(SelectWithTheKeyboard))
        .with(fixture_test(SelectAllAndClear))
        .with(fixture_test(SelectAllAndClearLarge))
        .with(fixture_test(TypeAhead))
        .with(fixture_test(TypeAheadLarge))
        .with(fixture_test(DisabledOption))
        .with(fixture_test(ControlledSelection))
        .with(fixture_test(ControlledSelectionLarge))
        .with(fixture_test(SelectionMode))
        .with(fixture_test(SelectionBehavior))
        .with(fixture_test(DisallowEmptySelection))
        .with(fixture_test(Orientation))
        .with(fixture_test(AriaLabel))
        .with(fixture_test(OptionsChange))
        .with(fixture_test(OptionsChangeLarge))
}
const LISTBOX: &str = "[role=listbox]";
/// The fixture's options after the control `large`.
const LARGE: u32 = 1_000;

/// The option "Option `i`".
fn option(i: u32) -> String {
    format!("[data-perf-option='{i}']")
}

/// Remounts the listbox with [`LARGE`] options (unmeasured).
async fn mount_large(page: &Page<'_>) -> Result<(), Report> {
    page.control("unmount").await?;
    page.wait_for_count(LISTBOX, 0).await?;
    page.control("large").await?;
    page.control("mount").await?;
    page.element(option(LARGE - 1)).await?;
    Ok(())
}

/// Focuses the first option with the keyboard (Tab from the plain button before the listbox).
async fn tab_in(page: &Page<'_>) -> Result<(), Report> {
    page.element("#perf-before").await?.click().await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&page.element(option(0)).await?).await?;
    Ok(())
}

/// The `data-perf-option` of the focused element.
async fn focused_option(page: &Page<'_>) -> Result<Option<String>, Report> {
    Ok(page
        .focused_element()
        .await?
        .attr("data-perf-option")
        .await?)
}

/// Mounting the listbox (10 options) and unmounting it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(LISTBOX, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.element(option(9)).await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(LISTBOX, 0).await?;
    Ok(())
}

/// Mounting the listbox with 1,000 options and unmounting it.
#[browser_test]
pub async fn mount_and_unmount_large(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(LISTBOX, 0).await?;
    page.control("large").await?;
    page.measure("mount", page.control("mount")).await?;
    page.element(option(LARGE - 1)).await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(LISTBOX, 0).await?;
    Ok(())
}

/// Clicks on an option, selecting and deselecting it (it takes focus), and a click outside
/// (blur).
async fn select_with_the_mouse_in(page: &Page<'_>) -> Result<(), Report> {
    let third = page.element(option(3)).await?;
    page.measure("select", third.click()).await?;
    third.wait_for_attr("aria-selected", Some("true")).await?;
    page.measure("deselect", third.click()).await?;
    third.wait_for_attr("aria-selected", Some("false")).await?;
    let after = page.element("#perf-after").await?;
    page.measure("blur", after.click()).await?;
    page.wait_for_focus(&after).await?;
    Ok(())
}

/// Clicks selecting and deselecting an option of 10, and a click outside.
#[browser_test]
pub async fn select_with_the_mouse(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    select_with_the_mouse_in(page).await
}

/// Clicks selecting and deselecting an option of 1,000, and a click outside.
#[browser_test]
pub async fn select_with_the_mouse_large(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    mount_large(page).await?;
    select_with_the_mouse_in(page).await
}

/// Holding the pointer down on an option (its pressed state; it selects) and releasing it.
#[browser_test]
pub async fn hold_and_release(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let third = page.element(option(3)).await?;
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

/// The pointer entering and leaving an option.
#[browser_test]
pub async fn hover_and_leave(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let third = page.element(option(3)).await?;
    let after = page.element("#perf-after").await?;
    page.measure("hover", third.hover()).await?;
    third.wait_for_attr("data-hovered", Some("true")).await?;
    page.measure("leave", after.hover()).await?;
    third.wait_for_attr("data-hovered", None).await?;
    Ok(())
}

/// Tab moving focus into the listbox (its first option, with a focus ring) and on.
#[browser_test]
pub async fn keyboard_focus_and_blur(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element("#perf-before").await?.click().await?;
    let first = page.element(option(0)).await?;
    page.measure("focus", page.send_keys(Key::Tab)).await?;
    first
        .wait_for_attr("data-focus-visible", Some("true"))
        .await?;
    page.measure("blur", page.send_keys(Key::Tab)).await?;
    first.wait_for_attr("data-focus-visible", None).await?;
    page.wait_for_focus(&page.element("#perf-after").await?)
        .await?;
    Ok(())
}

/// ArrowDown, ArrowUp, End, Home, PageDown and PageUp moving the focus, from the first option
/// of `last + 1`.
async fn keyboard_navigation_in(page: &Page<'_>, last: u32) -> Result<(), Report> {
    tab_in(page).await?;
    page.measure("arrow down", page.send_keys(Key::Down))
        .await?;
    page.wait_for_focus(&page.element(option(1)).await?).await?;
    page.measure("arrow up", page.send_keys(Key::Up)).await?;
    page.wait_for_focus(&page.element(option(0)).await?).await?;
    page.measure("end", page.send_keys(Key::End)).await?;
    page.wait_for_focus(&page.element(option(last)).await?)
        .await?;
    page.measure("home", page.send_keys(Key::Home)).await?;
    page.wait_for_focus(&page.element(option(0)).await?).await?;
    page.measure("page down", page.send_keys(Key::PageDown))
        .await?;
    assert_that!(focused_option(page).await?).is_not_equal_to(Some("0".to_owned()));
    page.measure("page up", page.send_keys(Key::PageUp)).await?;
    page.wait_for_focus(&page.element(option(0)).await?).await?;
    Ok(())
}

/// Keyboard navigation through 10 options.
#[browser_test]
pub async fn keyboard_navigation(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    keyboard_navigation_in(page, 9).await
}

/// Keyboard navigation through 1,000 options.
#[browser_test]
pub async fn keyboard_navigation_large(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    mount_large(page).await?;
    keyboard_navigation_in(page, LARGE - 1).await
}

/// Space selecting the focused option, Enter deselecting it, and Shift+ArrowDown extending the
/// selection to the next option.
#[browser_test]
pub async fn select_with_the_keyboard(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    tab_in(page).await?;
    let first = page.element(option(0)).await?;
    page.measure("space", page.send_keys(" ")).await?;
    first.wait_for_attr("aria-selected", Some("true")).await?;
    page.measure("enter", page.send_keys(Key::Enter)).await?;
    first.wait_for_attr("aria-selected", Some("false")).await?;
    page.measure("shift arrow down", page.send_keys(Key::Shift + Key::Down))
        .await?;
    page.element(option(1))
        .await?
        .wait_for_attr("aria-selected", Some("true"))
        .await?;
    Ok(())
}

/// Ctrl+A selecting every option and Escape clearing the selection, from the first option of
/// `last + 1`.
async fn select_all_and_clear_in(page: &Page<'_>, last: u32) -> Result<(), Report> {
    tab_in(page).await?;
    let last = page.element(option(last)).await?;
    let select_all = page.primary_modifier().await? + "a";
    page.measure("select all", page.send_keys(select_all))
        .await?;
    last.wait_for_attr("aria-selected", Some("true")).await?;
    page.measure("escape", page.send_keys(Key::Escape)).await?;
    last.wait_for_attr("aria-selected", Some("false")).await?;
    Ok(())
}

/// Select all and clearing it with 10 options.
#[browser_test]
pub async fn select_all_and_clear(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    select_all_and_clear_in(page, 9).await
}

/// Select all and clearing it with 1,000 options.
#[browser_test]
pub async fn select_all_and_clear_large(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    mount_large(page).await?;
    select_all_and_clear_in(page, LARGE - 1).await
}

/// Type-ahead from the first option: "x" matches no option (a search through all of them), "o"
/// matches the focused option (the search starts at it). (Measured last: the search resets a second later.)
async fn type_ahead_in(page: &Page<'_>) -> Result<(), Report> {
    tab_in(page).await?;
    page.measure("no match", page.send_keys("x")).await?;
    assert_that!(focused_option(page).await?).is_equal_to(Some("0".to_owned()));
    page.measure("match", page.send_keys("o")).await?;
    assert_that!(focused_option(page).await?).is_equal_to(Some("0".to_owned()));
    Ok(())
}

/// Type-ahead through 10 options.
#[browser_test]
pub async fn type_ahead(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    type_ahead_in(page).await
}

/// Type-ahead through 1,000 options.
#[browser_test]
pub async fn type_ahead_large(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    mount_large(page).await?;
    type_ahead_in(page).await
}

/// Disabling the first option, a click it ignores, and enabling it again.
#[browser_test]
pub async fn disabled_option(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let first = page.element(option(0)).await?;
    page.measure("disable", page.control("disable-first"))
        .await?;
    first.wait_for_attr("aria-disabled", Some("true")).await?;
    page.measure("ignored click", first.click()).await?;
    assert_that!(first)
        .has_attribute("aria-selected")
        .await
        .is_equal_to("false");
    page.measure("enable", page.control("enable-all")).await?;
    first.wait_for_attr("aria-disabled", None).await?;
    Ok(())
}

/// The controlled selection changing: one option, none, all, none, of `last + 1`.
async fn controlled_selection_in(page: &Page<'_>, last: u32) -> Result<(), Report> {
    let first = page.element(option(0)).await?;
    let last = page.element(option(last)).await?;
    page.measure("select one", page.control("select-first"))
        .await?;
    first.wait_for_attr("aria-selected", Some("true")).await?;
    page.measure("clear one", page.control("clear-selection"))
        .await?;
    first.wait_for_attr("aria-selected", Some("false")).await?;
    page.measure("select all", page.control("select-all"))
        .await?;
    last.wait_for_attr("aria-selected", Some("true")).await?;
    page.measure("clear all", page.control("clear-selection"))
        .await?;
    last.wait_for_attr("aria-selected", Some("false")).await?;
    Ok(())
}

/// The controlled selection of 10 options changing.
#[browser_test]
pub async fn controlled_selection(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    controlled_selection_in(page, 9).await
}

/// The controlled selection of 1,000 options changing.
#[browser_test]
pub async fn controlled_selection_large(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    mount_large(page).await?;
    controlled_selection_in(page, LARGE - 1).await
}

/// Switching to single selection and back to multiple.
#[browser_test]
pub async fn selection_mode(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let listbox = page.element(LISTBOX).await?;
    let first = page.element(option(0)).await?;
    page.measure("single", page.control("single")).await?;
    listbox.wait_for_attr("aria-multiselectable", None).await?;
    first
        .wait_for_attr("data-selection-mode", Some("single"))
        .await?;
    page.measure("multiple", page.control("multiple")).await?;
    listbox
        .wait_for_attr("aria-multiselectable", Some("true"))
        .await?;
    first
        .wait_for_attr("data-selection-mode", Some("multiple"))
        .await?;
    Ok(())
}

/// Switching to the replace selection behavior (a click replaces the selection) and back to
/// toggle (a click adds to it).
#[browser_test]
pub async fn selection_behavior(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let first = page.element(option(0)).await?;
    let second = page.element(option(1)).await?;
    let third = page.element(option(2)).await?;
    page.measure("replace", page.control("replace")).await?;
    first.click().await?;
    first.wait_for_attr("aria-selected", Some("true")).await?;
    second.click().await?;
    second.wait_for_attr("aria-selected", Some("true")).await?;
    first.wait_for_attr("aria-selected", Some("false")).await?;
    page.measure("toggle", page.control("toggle")).await?;
    third.click().await?;
    third.wait_for_attr("aria-selected", Some("true")).await?;
    assert_that!(second)
        .has_attribute("aria-selected")
        .await
        .is_equal_to("true");
    Ok(())
}

/// Disallowing an empty selection (a click on the only selected option keeps it) and allowing it
/// again.
#[browser_test]
pub async fn disallow_empty_selection(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let first = page.element(option(0)).await?;
    first.click().await?;
    first.wait_for_attr("aria-selected", Some("true")).await?;
    page.measure("disallow", page.control("disallow-empty"))
        .await?;
    first.click().await?;
    first
        .attr_stays(
            "aria-selected",
            Some("true"),
            std::time::Duration::from_millis(100),
        )
        .await?;
    page.measure("allow", page.control("allow-empty")).await?;
    first.click().await?;
    first.wait_for_attr("aria-selected", Some("false")).await?;
    Ok(())
}

/// Switching to a horizontal orientation and back.
#[browser_test]
pub async fn orientation(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let listbox = page.element(LISTBOX).await?;
    page.measure("horizontal", page.control("horizontal"))
        .await?;
    listbox
        .wait_for_attr("aria-orientation", Some("horizontal"))
        .await?;
    page.measure("vertical", page.control("vertical")).await?;
    listbox
        .wait_for_attr("aria-orientation", Some("vertical"))
        .await?;
    Ok(())
}

/// Changing the listbox's label and back.
#[browser_test]
pub async fn aria_label(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let listbox = page.element(LISTBOX).await?;
    page.measure("rename", page.control("rename")).await?;
    listbox.wait_for_attr("aria-label", Some("Choices")).await?;
    page.measure("restore", page.control("restore-name"))
        .await?;
    listbox.wait_for_attr("aria-label", Some("Options")).await?;
    Ok(())
}

/// Appending an option to `last + 1` and removing it again.
async fn options_change_in(page: &Page<'_>, last: u32) -> Result<(), Report> {
    page.measure("append", page.control("append")).await?;
    page.element(option(last + 1)).await?;
    page.measure("remove", page.control("remove-last")).await?;
    page.wait_for_count(option(last + 1), 0).await?;
    Ok(())
}

/// An option appended to 10 and removed again.
#[browser_test]
pub async fn options_change(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    options_change_in(page, 9).await
}

/// An option appended to 1,000 and removed again.
#[browser_test]
pub async fn options_change_large(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    mount_large(page).await?;
    options_change_in(page, LARGE - 1).await
}
