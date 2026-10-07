// Upstream: react-aria-components/test/TagGroup.test.js @ 99e6102368
use std::borrow::Cow;

use assertr::prelude::*;
use browser_test::{
    BrowserTest, async_trait,
    thirtyfour::{By, Key, WebDriver, WebElement},
};
use rootcause::Report;

use crate::pages::{BaseActions, Page};

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

        // "should render with default classes", "provides slots for description".
        let group = page.css("#test-tg-main .leptonic-TagGroup").await?;
        let grid = group.find(By::Css(".leptonic-TagList")).await?;
        assert_that!(grid.attr("role").await?).is_equal_to(Some("grid".to_owned()));
        let rows = grid.find_all(By::Css(".leptonic-Tag")).await?;
        assert_that!(rows.len()).is_equal_to(3);
        let labelledby = grid.attr("aria-labelledby").await?.unwrap_or_default();
        let label = page.element(&labelledby).await?;
        assert_that!(label.text().await?).is_equal_to("Test".to_owned());
        let describedby = grid.attr("aria-describedby").await?.unwrap_or_default();
        let description = page.css("#test-tg-main .leptonic-Description").await?;
        let description_id = description.attr("id").await?.unwrap_or_default();
        assert_that!(describedby.split(' ').any(|id| id == description_id)).is_true();

        // The group's label context ends with the group: a label after it is a plain one.
        let outside = page.css("#test-tg-after-group label").await?;
        assert_that!(outside.attr("id").await?).is_none();
        assert_that!(page.count_matching(&format!("[id='{labelledby}']")).await?).is_equal_to(1);

        // "should support focus ring": Tab focuses the first tag, focus visible.
        page.click_element_with_id("test-tg-before").await?;
        page.press_tab().await?;
        focus_on_tag(&page, "Cat").await?;
        let cat = tag(&page, "Cat").await?;
        assert_that!(cat.attr("data-focus-visible").await?).is_equal_to(Some("true".to_owned()));
        // "should support removing items": removable tags, the button named "Remove".
        assert_that!(cat.attr("data-allows-removing").await?).is_equal_to(Some("true".to_owned()));

        // "should support tabbing to remove buttons".
        page.press_tab().await?;
        page.wait_for_active_text("x").await?;
        let active = page.driver.active_element().await?;
        assert_that!(active.attr("aria-label").await?).is_equal_to(Some("Remove".to_owned()));
        page.send_keys_to_active(" ").await?;
        page.wait_for_text("test-tg-removed", "cat").await?;
        page.wait_for_text("test-tg-remove-count", "1").await?;
        page.send_keys_to_active(Key::Delete).await?;
        page.wait_for_text("test-tg-remove-count", "2").await?;
        assert_that!(page.read_text_of("test-tg-removed").await?).is_equal_to("cat".to_owned());
        page.send_keys_to_active(Key::Shift + Key::Tab).await?;
        focus_on_tag(&page, "Cat").await?;
        page.send_keys_to_active(Key::Right).await?;
        focus_on_tag(&page, "Dog").await?;
        page.send_keys_to_active(Key::Delete).await?;
        page.wait_for_text("test-tg-remove-count", "3").await?;
        assert_that!(page.read_text_of("test-tg-removed").await?).is_equal_to("dog".to_owned());

        // "should support selection state": selecting, and removing the selected tags together.
        page.send_keys_to_active(" ").await?;
        let dog = tag(&page, "Dog").await?;
        page.wait_for_selector("#test-tg-main .leptonic-Tag[data-selected]")
            .await?;
        assert_that!(dog.attr("data-selected").await?).is_equal_to(Some("true".to_owned()));
        assert_that!(dog.attr("data-selection-mode").await?)
            .is_equal_to(Some("multiple".to_owned()));
        page.send_keys_to_active(Key::Right).await?;
        focus_on_tag(&page, "Kangaroo").await?;
        page.send_keys_to_active(" ").await?;
        page.send_keys_to_active(Key::Backspace).await?;
        page.wait_for_text("test-tg-remove-count", "4").await?;
        assert_that!(page.read_text_of("test-tg-removed").await?)
            .is_equal_to("dog,kangaroo".to_owned());

        // "should support empty state".
        let empty = page.css("#test-tg-empty .leptonic-TagList").await?;
        assert_that!(empty.attr("data-empty").await?).is_equal_to(Some("true".to_owned()));
        assert_that!(empty.text().await?).is_equal_to("No results".to_owned());

        // "if we cannot restore focus to next, then restore to previous": Grape and Plum are
        // disabled, so removing Watermelon leaves the focus on the grid.
        let fruits = page.css("#test-tg-fruits .leptonic-TagList").await?;
        page.driver
            .execute(
                "Array.from(document.querySelectorAll('#test-tg-fruits [role=row]')).find(r => r.textContent === 'Watermelon').focus();",
                vec![],
            )
            .await?;
        page.wait_for_focus("row", Some("Watermelon")).await?;
        page.send_keys_to_active(Key::Backspace).await?;
        for _ in 0..100 {
            let active = page.driver.active_element().await?;
            if active.attr("role").await?.as_deref() == Some("grid")
                && active.attr("aria-label").await?.as_deref() == Some("Fruits")
            {
                break;
            }
            tokio::time::sleep(std::time::Duration::from_millis(50)).await;
        }
        let active = page.driver.active_element().await?;
        assert_that!(active.attr("aria-label").await?).is_equal_to(Some("Fruits".to_owned()));
        assert_that!(fruits.find_all(By::Css(".leptonic-Tag")).await?.len()).is_equal_to(2);
        page.expect_no_page_errors().await
    }
}

/// The tag named `name` (its accessible name: its text also holds its remove button's).
async fn tag(page: &Page<'_>, name: &str) -> Result<WebElement, Report> {
    page.css(&format!("#test-tg-main [role=row][aria-label='{name}']"))
        .await
}

/// Waits until the tag named `name` has the focus.
async fn focus_on_tag(page: &Page<'_>, name: &str) -> Result<(), Report> {
    let tag = tag(page, name).await?;
    page.wait_for_focus_on(&tag, name).await
}
