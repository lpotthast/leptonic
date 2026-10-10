// Upstream: react-aria-components/test/TagGroup.test.js @ 99e6102368
//! The tag group atoms: default classes, label and description, the focus ring, removing tags
//! with their buttons and the keyboard, tabbing to the remove buttons, selection, the empty state,
//! and focus moving to the grid when the last tag that could take it is removed.
use std::time::Duration;

use assertr::{matchers::eq, prelude::*};
use browser_test::{browser_test, thirtyfour::prelude::*};
use leptonic::AriaRole;
use rootcause::Report;

use crate::pages::{ElementActions, Page, css, role};

const PATH: &str = "/atoms/tag-group";

/// The tag named `name` (its accessible name: its text also holds its remove button's).
async fn tag(page: &Page<'_>, name: &str) -> Result<WebElement, Report> {
    page.element(format!("#test-tg-main [role=row][aria-label='{name}']"))
        .await
}

/// Waits until the tag named `name` has the focus.
async fn focus_on_tag(page: &Page<'_>, name: &str) -> Result<(), Report> {
    let tag = tag(page, name).await?;
    page.wait_for_focus(&tag).await?;
    Ok(())
}

/// Tabs from the button before the main group to its first tag, Cat (as upstream's first
/// `user.tab()`).
async fn tab_to_the_first_tag(page: &Page<'_>) -> Result<(), Report> {
    page.element("#test-tg-before").await?.click().await?;
    page.send_keys(Key::Tab).await?;
    focus_on_tag(page, "Cat").await
}

/// The tag list is a grid with default classes, labelled by the group's label and described by its
/// description ("should render with default classes", "provides slots for description and error
/// message").
#[browser_test]
pub async fn default_classes_and_slots(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let group = page.element("#test-tg-main .leptonic-TagGroup").await?;
    let grid = group.element(".leptonic-TagList").await?;
    assert_that!(grid)
        .has_attribute("role")
        .await
        .is_equal_to("grid");
    assert_that!(grid.inner_texts(".leptonic-Tag").await?).has_length(3);
    assert_that!(grid)
        .accessible_name()
        .await
        .is_equal_to("Test");
    let description = page.element("#test-tg-main .leptonic-Description").await?;
    let description_text = description.inner_text().await?;
    assert_that!(grid)
        .accessible_description()
        .await
        .is_equal_to(description_text);
    Ok(())
}

/// A label rendered after the tag group is a plain label, not taken as the group's label.
#[browser_test]
pub async fn label_context_ends_with_the_group(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let grid = page.element("#test-tg-main .leptonic-TagList").await?;
    let labelledby = grid.attr("aria-labelledby").await?.unwrap_or_default();
    let outside = page.element("#test-tg-after-group label").await?;
    assert_that!(outside).attribute("id").await.is_none();
    assert_that!(page.count(format!("[id='{labelledby}']")).await?).is_equal_to(1);
    Ok(())
}

/// Tabbing into the group focuses the first tag with a focus ring, and the tag is marked as
/// removable ("should support focus ring").
#[browser_test]
pub async fn focus_ring(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    tab_to_the_first_tag(page).await?;
    let cat = tag(page, "Cat").await?;
    cat.wait_for_attr("data-focus-visible", Some("true"))
        .await?;
    assert_that!(cat)
        .has_attribute("data-allows-removing")
        .await
        .is_equal_to("true");
    Ok(())
}

/// Tab moves from a tag to its "Remove" button, taking the focus ring along, where Space or Delete
/// removes the tag, as does Delete on a focused tag ("should support tabbing to remove buttons").
#[browser_test]
pub async fn tabbing_to_remove_buttons(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let removed = page.element("#test-tg-removed").await?;
    let remove_count = page.element("#test-tg-remove-count").await?;
    let remove = tag(page, "Cat").await?.element("button").await?;
    assert_that!(remove)
        .has_attribute("aria-label")
        .await
        .is_equal_to("Remove");
    tab_to_the_first_tag(page).await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&remove).await?;
    remove
        .wait_for_attr("data-focus-visible", Some("true"))
        .await?;
    // The focus ring moved from the tag to its button.
    tag(page, "Cat")
        .await?
        .wait_for_attr("data-focus-visible", None)
        .await?;
    page.send_keys(" ").await?;
    removed.wait_for_inner_text("cat").await?;
    remove_count.wait_for_inner_text("1").await?;
    page.send_keys(Key::Delete).await?;
    remove_count.wait_for_inner_text("2").await?;
    removed.wait_for_inner_text("cat").await?;
    page.send_keys(Key::Shift + Key::Tab).await?;
    focus_on_tag(page, "Cat").await?;
    page.send_keys(Key::Right).await?;
    focus_on_tag(page, "Dog").await?;
    page.send_keys(Key::Delete).await?;
    remove_count.wait_for_inner_text("3").await?;
    removed.wait_for_inner_text("dog").await?;
    Ok(())
}

/// Space selects the focused tag, and Backspace removes all selected tags at once ("should support
/// selection state").
#[browser_test]
pub async fn selection_state(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let dog = tag(page, "Dog").await?;
    tab_to_the_first_tag(page).await?;
    page.send_keys(Key::Right).await?;
    page.wait_for_focus(&dog).await?;
    page.send_keys(" ").await?;
    dog.wait_for_attr("data-selected", Some("true")).await?;
    assert_that!(dog)
        .has_attribute("data-selection-mode")
        .await
        .is_equal_to("multiple");
    page.send_keys(Key::Right).await?;
    focus_on_tag(page, "Kangaroo").await?;
    page.send_keys(" ").await?;
    page.send_keys(Key::Backspace).await?;
    page.element("#test-tg-remove-count")
        .await?
        .wait_for_inner_text("1")
        .await?;
    page.element("#test-tg-removed")
        .await?
        .wait_for_inner_text("dog,kangaroo")
        .await?;
    Ok(())
}

/// A tag list without tags is marked `data-empty` and shows its empty state ("should support empty
/// state").
#[browser_test]
pub async fn empty_state(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let empty = page.element("#test-tg-empty .leptonic-TagList").await?;
    assert_that!(empty)
        .has_attribute("data-empty")
        .await
        .is_equal_to("true");
    assert_that!(empty)
        .inner_text()
        .await
        .is_equal_to("No results");
    Ok(())
}

/// Removing the only enabled tag moves focus to the grid, since the remaining disabled tags can't
/// take it ("if we cannot restore focus to next, then restore to previous but do not try focusing
/// next again").
#[browser_test]
pub async fn focus_moves_to_the_grid_when_no_tag_can_take_it(
    page: &Page<'_>,
) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let fruits = page.element("#test-tg-fruits .leptonic-TagList").await?;
    assert_that!(fruits)
        .has_attribute("aria-label")
        .await
        .is_equal_to("Fruits");
    let watermelon = fruits
        .element(role(AriaRole::Row).text("Watermelon"))
        .await?;
    watermelon.focus().await?;
    page.wait_for_focus(&watermelon).await?;
    page.send_keys(Key::Backspace).await?;
    page.wait_for_focus(&fruits).await?;
    assert_that!(fruits.inner_texts(".leptonic-Tag").await?).contains_exactly(["Grape", "Plum"]);
    Ok(())
}

/// The tag `text` in the group `group` (a selector of its container).
async fn tag_in(page: &Page<'_>, group: &str, text: &str) -> Result<WebElement, Report> {
    page.element(group)
        .await?
        .element(css("[role=row]").text(text))
        .await
}

/// Hovering a selectable tag marks it hovered until the pointer leaves ("should support hover").
#[browser_test]
pub async fn hover(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let cat = tag(page, "Cat").await?;
    assert_that!(cat).attribute("data-hovered").await.is_none();
    cat.hover().await?;
    cat.wait_for_attr("data-hovered", Some("true")).await?;
    page.element("h1").await?.hover().await?;
    cat.wait_for_attr("data-hovered", None).await?;
    Ok(())
}

/// A tag that is neither selectable nor has an action isn't hovered or pressed ("should not show
/// hover state when item is not interactive", "should not show press state when not
/// interactive").
#[browser_test]
pub async fn not_interactive(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let one = tag_in(page, "#test-tg-plain", "One").await?;
    one.hover().await?;
    page.settle().await?;
    one.attr_stays("data-hovered", None, Duration::from_millis(100))
        .await?;
    let held = one.press_and_hold().await?;
    page.settle().await?;
    let pressed = one
        .attr_stays("data-pressed", None, Duration::from_millis(100))
        .await;
    held.release().await?;
    pressed?;
    Ok(())
}

/// A selectable tag is pressed while the pointer is down on it ("should support press state").
#[browser_test]
pub async fn press_state(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let cat = tag(page, "Cat").await?;
    assert_that!(cat).attribute("data-pressed").await.is_none();
    let held = cat.press_and_hold().await?;
    cat.wait_for_attr("data-pressed", Some("true")).await?;
    held.release().await?;
    cat.wait_for_attr("data-pressed", None).await?;
    Ok(())
}

/// A disabled tag's remove button is disabled, and neither it nor Backspace removes the tag
/// ("should disable the remove button if the tag is disabled", "disabled tags should not be
/// deletable").
#[browser_test]
pub async fn disabled_tags_cant_be_removed(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let grape = tag_in(page, "#test-tg-fruits", "Grape").await?;
    let remove = grape.element("button").await?;
    assert_that!(grape)
        .has_attribute("data-allows-removing")
        .await
        .is_equal_to("true");
    assert_that!(remove).enabled().await.is_false();
    assert_that!(remove)
        .has_attribute("data-disabled")
        .await
        .is_equal_to("true");
    remove.click().await?;
    grape.focus().await?;
    page.send_keys(Key::Backspace).await?;
    page.settle().await?;
    page.element("#test-tg-fruits")
        .await?
        .count_stays("[role=row]", 3, Duration::from_millis(100))
        .await?;
    Ok(())
}

/// Activating a tag of a group with actions (by click or Enter) performs the action ("should
/// support onAction").
#[browser_test]
pub async fn on_action(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let log = page.element("#test-tg-actions-log").await?;
    tag_in(page, "#test-tg-actions", "Alpha")
        .await?
        .click()
        .await?;
    log.wait_for_inner_text("Alpha").await?;
    page.send_keys(Key::Enter).await?;
    log.wait_for_inner_text("Alpha,Alpha").await?;
    Ok(())
}

/// With replacing single selection, a click selects a tag, a double click and Enter perform the
/// action ("should support onAction with selectionMode = single, behaviour = replace").
#[browser_test]
pub async fn on_action_with_replace_selection(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let log = page.element("#test-tg-replace-log").await?;
    let red = tag_in(page, "#test-tg-replace", "Red").await?;
    let green = tag_in(page, "#test-tg-replace", "Green").await?;
    red.double_click().await?;
    log.wait_for_inner_text("Red").await?;
    green.click().await?;
    green.wait_for_attr("aria-selected", Some("true")).await?;
    page.settle().await?;
    log.inner_text_stays("Red", Duration::from_millis(100))
        .await?;
    red.double_click().await?;
    log.wait_for_inner_text("Red,Red").await?;
    page.send_keys(Key::Enter).await?;
    log.wait_for_inner_text("Red,Red,Red").await?;
    Ok(())
}

/// Tags inserted at the start show in the collection's order ("should maintain item order when
/// adding new items").
#[browser_test]
pub async fn order_when_adding(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let insert = page.element("#test-tg-insert").await?;
    let group = page.element("#test-tg-ordered").await?;
    for count in 1..=3 {
        insert.click().await?;
        group.wait_for_count("[role=row]", count).await?;
    }
    assert_that!(|| group.inner_texts("[role=row] [data-tag-label]"))
        .eventually_ok()
        .matches(eq(vec![
            "Item 3".to_owned(),
            "Item 2".to_owned(),
            "Item 1".to_owned(),
        ]))
        .await;
    Ok(())
}

/// In right-to-left text, ArrowLeft moves to the next tag and ArrowRight to the previous one,
/// wrapping around ("shifts button focus in the correct direction on key press", rtl).
#[browser_test]
pub async fn right_to_left(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let one = tag_in(page, "#test-tg-rtl", "One").await?;
    one.focus().await?;
    page.wait_for_focus(&one).await?;
    page.send_keys(Key::Left).await?;
    page.wait_for_focus(&tag_in(page, "#test-tg-rtl", "Two").await?)
        .await?;
    page.send_keys(Key::Right).await?;
    page.wait_for_focus(&one).await?;
    page.send_keys(Key::Right).await?;
    page.wait_for_focus(&tag_in(page, "#test-tg-rtl", "Three").await?)
        .await?;
    Ok(())
}

/// Space selects the focused tag while the arrow keys move between tags ("should support keyboard
/// selection while navigating between tags").
#[browser_test]
pub async fn keyboard_selection(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let (cat, dog) = (tag(page, "Cat").await?, tag(page, "Dog").await?);
    tab_to_the_first_tag(page).await?;
    page.send_keys(" ").await?;
    cat.wait_for_attr("aria-selected", Some("true")).await?;
    page.send_keys(Key::Right).await?;
    page.wait_for_focus(&dog).await?;
    page.send_keys(" ").await?;
    dog.wait_for_attr("aria-selected", Some("true")).await?;
    page.send_keys(Key::Right).await?;
    focus_on_tag(page, "Kangaroo").await?;
    assert_that!(cat)
        .has_attribute("aria-selected")
        .await
        .is_equal_to("true");
    Ok(())
}
