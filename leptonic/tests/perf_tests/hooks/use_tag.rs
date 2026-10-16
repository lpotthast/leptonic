// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the hook `use_tag` (`/perf/hooks/use_tag`, plain tags of a `use_tag_group`
//! group): what each thing it offers costs, measured (`Page::measure`) and checked against its
//! budget (`tests/budgets/hooks/use_tag.json`): mounting and unmounting 10 and 1,000 tags,
//! selecting one with the mouse (at both sizes) and the keyboard, its pressed state, removing it
//! with Delete and Backspace and with its remove button, disabling it, and the state's selection
//! changing (all tags, at both sizes).
use assertr::prelude::*;
use browser_test::{TestGroup, browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use super::use_tag_group::{
    TAGS, goto, large, mount_and_unmount_in, remove_button, remove_with_the_keyboard_in, tab_in,
    tag,
};
use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/hooks/use_tag";

/// Registers the performance tests of `use_tag`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(MountAndUnmountLarge))
        .with(fixture_test(SelectWithTheMouse))
        .with(fixture_test(SelectWithTheMouseLarge))
        .with(fixture_test(HoldAndRelease))
        .with(fixture_test(SelectWithTheKeyboard))
        .with(fixture_test(RemoveWithTheKeyboard))
        .with(fixture_test(RemoveWithTheButton))
        .with(fixture_test(Disabled))
        .with(fixture_test(StateSelection))
        .with(fixture_test(StateSelectionLarge))
}

/// Mounting 10 tags and unmounting them.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    goto(page, PATH).await?;
    mount_and_unmount_in(page, TAGS, 10).await
}

/// Mounting 1,000 tags and unmounting them.
#[browser_test]
pub async fn mount_and_unmount_large(page: &Page<'_>) -> Result<(), Report> {
    goto(page, PATH).await?;
    large(page).await?;
    mount_and_unmount_in(page, TAGS, 1_000).await
}

/// A click selecting a tag and one deselecting it.
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

/// Holding the pointer down on a tag (its pressed state; it selects) and releasing it.
#[browser_test]
pub async fn hold_and_release(page: &Page<'_>) -> Result<(), Report> {
    goto(page, PATH).await?;
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

/// Space selecting the focused tag and Enter deselecting it.
#[browser_test]
pub async fn select_with_the_keyboard(page: &Page<'_>) -> Result<(), Report> {
    goto(page, PATH).await?;
    let first = tab_in(page).await?;
    page.measure("space", page.send_keys(" ")).await?;
    first.wait_for_attr("aria-selected", Some("true")).await?;
    page.measure("enter", page.send_keys(Key::Enter)).await?;
    first.wait_for_attr("aria-selected", Some("false")).await?;
    Ok(())
}

/// Delete and Backspace removing the focused tag.
#[browser_test]
pub async fn remove_with_the_keyboard(page: &Page<'_>) -> Result<(), Report> {
    goto(page, PATH).await?;
    remove_with_the_keyboard_in(page).await
}

/// A click on the fourth tag's remove button removing it.
#[browser_test]
pub async fn remove_with_the_button(page: &Page<'_>) -> Result<(), Report> {
    goto(page, PATH).await?;
    let button = page.element(remove_button(3)).await?;
    page.measure("remove", button.click()).await?;
    page.wait_for_count(tag(3), 0).await?;
    Ok(())
}

/// The first tag disabled, a click it ignores, and enabled again.
#[browser_test]
pub async fn disabled(page: &Page<'_>) -> Result<(), Report> {
    goto(page, PATH).await?;
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

/// The state selecting all `last + 1` tags and clearing the selection.
async fn state_selection_in(page: &Page<'_>, last: u32) -> Result<(), Report> {
    let last = page.element(tag(last)).await?;
    page.measure("select all", page.control("select-all"))
        .await?;
    last.wait_for_attr("data-selected", Some("true")).await?;
    page.measure("clear", page.control("clear-selection"))
        .await?;
    last.wait_for_attr("data-selected", None).await?;
    Ok(())
}

/// The selection of 10 tags changing.
#[browser_test]
pub async fn state_selection(page: &Page<'_>) -> Result<(), Report> {
    goto(page, PATH).await?;
    state_selection_in(page, 9).await
}

/// The selection of 1,000 tags changing.
#[browser_test]
pub async fn state_selection_large(page: &Page<'_>) -> Result<(), Report> {
    goto(page, PATH).await?;
    large(page).await?;
    state_selection_in(page, 999).await
}
