// Upstream: react-aria-components/test/TagGroup.test.js @ 99e6102368
use std::borrow::Cow;

use assertr::prelude::*;
use browser_test::{BrowserTest, async_trait, thirtyfour::prelude::*};
use rootcause::Report;

use crate::pages::{ElementActions, Page, PageActions, xpath};

/// The tag group atoms: default classes, label and description, the focus ring, removing tags
/// with their buttons and the keyboard, tabbing to the remove buttons, selection, the empty state,
/// and focus moving to the grid when the last tag that could take it is removed.
pub struct TagGroupAtomTests {}

#[async_trait]
impl BrowserTest<str> for TagGroupAtomTests {
    fn name(&self) -> Cow<'_, str> {
        "tag_group_atom_tests".into()
    }

    async fn run(&self, driver: &WebDriver, base_url: &str) -> Result<(), Report> {
        let page = Page { driver, base_url };
        page.goto_path("/atoms/tag-group").await?;

        cases!(
            default_classes_and_slots(&page),
            label_context_ends_with_the_group(&page),
            focus_ring(&page),
            tabbing_to_remove_buttons(&page),
            selection_state(&page),
            empty_state(&page),
            focus_moves_to_the_grid_when_no_tag_can_take_it(&page),
        );

        Ok(())
    }
}

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

/// "should render with default classes", "provides slots for description": the tag list is a
/// grid labelled by the group's label and described by its description.
async fn default_classes_and_slots(page: &Page<'_>) -> Result<(), Report> {
    let group = page.element("#test-tg-main .leptonic-TagGroup").await?;
    let grid = group.element(".leptonic-TagList").await?;
    assert_that!(grid.attr("role").await?)
        .get_some()
        .is_equal_to("grid");
    assert_that!(grid.inner_texts(".leptonic-Tag").await?).has_length(3);
    assert_that!(grid.referenced_text("aria-labelledby").await?).is_equal_to("Test");
    let description = page.element("#test-tg-main .leptonic-Description").await?;
    let description_text = description.inner_text().await?;
    assert_that!(grid.referenced_text("aria-describedby").await?).is_equal_to(description_text);
    Ok(())
}

/// The group's label context ends with the group: a label after it is a plain one.
async fn label_context_ends_with_the_group(page: &Page<'_>) -> Result<(), Report> {
    let grid = page.element("#test-tg-main .leptonic-TagList").await?;
    let labelledby = grid.attr("aria-labelledby").await?.unwrap_or_default();
    let outside = page.element("#test-tg-after-group label").await?;
    assert_that!(outside.attr("id").await?).is_none();
    assert_that!(page.count(format!("[id='{labelledby}']")).await?).is_equal_to(1);
    Ok(())
}

/// "should support focus ring": Tab focuses the first tag, focus visible; "should support
/// removing items": the tags allow removing.
async fn focus_ring(page: &Page<'_>) -> Result<(), Report> {
    page.element("#test-tg-before").await?.click().await?;
    page.send_keys(Key::Tab).await?;
    focus_on_tag(page, "Cat").await?;
    let cat = tag(page, "Cat").await?;
    cat.wait_for_attr("data-focus-visible", Some("true"))
        .await?;
    assert_that!(cat.attr("data-allows-removing").await?)
        .get_some()
        .is_equal_to("true");
    Ok(())
}

/// "should support tabbing to remove buttons": Tab reaches the focused tag's remove button
/// ("Remove"), which Space presses; Delete on it removes the tag too.
async fn tabbing_to_remove_buttons(page: &Page<'_>) -> Result<(), Report> {
    let removed = page.element("#test-tg-removed").await?;
    let remove_count = page.element("#test-tg-remove-count").await?;
    let remove = tag(page, "Cat").await?.element("button").await?;
    assert_that!(remove.attr("aria-label").await?)
        .get_some()
        .is_equal_to("Remove");
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&remove).await?;
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

/// "should support selection state": selecting, and removing the selected tags together.
async fn selection_state(page: &Page<'_>) -> Result<(), Report> {
    let dog = tag(page, "Dog").await?;
    page.send_keys(" ").await?;
    dog.wait_for_attr("data-selected", Some("true")).await?;
    assert_that!(dog.attr("data-selection-mode").await?)
        .get_some()
        .is_equal_to("multiple");
    page.send_keys(Key::Right).await?;
    focus_on_tag(page, "Kangaroo").await?;
    page.send_keys(" ").await?;
    page.send_keys(Key::Backspace).await?;
    page.element("#test-tg-remove-count")
        .await?
        .wait_for_inner_text("4")
        .await?;
    page.element("#test-tg-removed")
        .await?
        .wait_for_inner_text("dog,kangaroo")
        .await?;
    Ok(())
}

/// "should support empty state".
async fn empty_state(page: &Page<'_>) -> Result<(), Report> {
    let empty = page.element("#test-tg-empty .leptonic-TagList").await?;
    assert_that!(empty.attr("data-empty").await?)
        .get_some()
        .is_equal_to("true");
    assert_that!(empty.inner_text().await?).is_equal_to("No results");
    Ok(())
}

/// "if we cannot restore focus to next, then restore to previous": Grape and Plum are disabled,
/// so removing Watermelon leaves the focus on the grid.
async fn focus_moves_to_the_grid_when_no_tag_can_take_it(page: &Page<'_>) -> Result<(), Report> {
    let fruits = page.element("#test-tg-fruits .leptonic-TagList").await?;
    assert_that!(fruits.attr("aria-label").await?)
        .get_some()
        .is_equal_to("Fruits");
    let watermelon = fruits
        .element(xpath(".//*[@role='row'][normalize-space(.)='Watermelon']"))
        .await?;
    watermelon.focus().await?;
    page.wait_for_focus(&watermelon).await?;
    page.send_keys(Key::Backspace).await?;
    page.wait_for_focus(&fruits).await?;
    assert_that!(fruits.inner_texts(".leptonic-Tag").await?).contains_exactly(["Grape", "Plum"]);
    Ok(())
}
