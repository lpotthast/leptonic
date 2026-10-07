// No upstream: `use_global_shortcuts` is a leptonic addition.
use std::borrow::Cow;

use assertr::prelude::*;
use browser_test::{
    BrowserTest, async_trait,
    thirtyfour::{By, Key, WebDriver},
};
use rootcause::Report;

use crate::pages::{BaseActions, Page};

/// `use_global_shortcuts`: `Mod+K` works anywhere, also in a text field (and the browser's default
/// is prevented); a bare `/` works only outside text fields, where it types instead; `?` matches
/// although typing it takes Shift; a later binding of `Mod+K` wins while it exists. The
/// `ShortcutKeys` atom shows `Mod+K` as "Ctrl" (read "Control"), "+", "K" off Apple platforms.
pub struct GlobalShortcutsTests {}

#[async_trait]
impl BrowserTest<str> for GlobalShortcutsTests {
    fn name(&self) -> Cow<'_, str> {
        "global_shortcuts_tests".into()
    }

    async fn run(&self, driver: &WebDriver, base_url: &str) -> Result<(), Report> {
        let page = Page { driver, base_url };
        page.goto_path("/hooks/global-shortcuts").await?;

        // `/` outside a text field focuses the filter (and isn't typed).
        page.click_element_with_id("test-gs-before").await?;
        page.send_keys_to_active("/").await?;
        let filter = page.css("#test-gs-filter input").await?;
        let filter_focused =
            "return document.activeElement === document.querySelector('#test-gs-filter input');";
        let mut focused = false;
        for _ in 0..100 {
            focused = page
                .driver
                .execute(filter_focused, vec![])
                .await?
                .convert::<bool>()?;
            if focused {
                break;
            }
            tokio::time::sleep(std::time::Duration::from_millis(50)).await;
        }
        assert_that!(focused).is_true();
        assert_that!(filter.attr("aria-keyshortcuts").await?).is_equal_to(Some("/".to_owned()));
        assert_that!(filter.prop("value").await?).is_equal_to(Some(String::new()));

        // In a text field, `/` is typed.
        page.click_element_with_id("test-gs-other").await?;
        page.send_keys_to_active("a/b").await?;
        let other = page.element("test-gs-other").await?;
        assert_that!(other.prop("value").await?).is_equal_to(Some("a/b".to_owned()));
        page.wait_for_active_id("test-gs-other").await?;

        // Mod+K works anywhere, also while typing.
        page.send_keys_to_active(Key::Control + "k").await?;
        page.wait_for_text("test-gs-palette", "1").await?;
        page.click_element_with_id("test-gs-before").await?;
        page.send_keys_to_active(Key::Control + "k").await?;
        page.wait_for_text("test-gs-palette", "2").await?;

        // `?` takes Shift to type: the shortcut without Shift still matches.
        page.send_keys_to_active("?").await?;
        page.wait_for_text("test-gs-help", "1").await?;

        // A later binding wins while it exists.
        page.click_element_with_id("test-gs-nested-toggle").await?;
        page.wait_for_selector("#test-gs-nested-shown").await?;
        page.send_keys_to_active(Key::Control + "k").await?;
        page.wait_for_text("test-gs-nested", "1").await?;
        assert_that!(page.read_text_of("test-gs-palette").await?).is_equal_to("2".to_owned());
        page.click_element_with_id("test-gs-nested-toggle").await?;
        for _ in 0..100 {
            if page
                .driver
                .find_all(By::Css("#test-gs-nested-shown"))
                .await?
                .is_empty()
            {
                break;
            }
            tokio::time::sleep(std::time::Duration::from_millis(50)).await;
        }
        page.send_keys_to_active(Key::Control + "k").await?;
        page.wait_for_text("test-gs-palette", "3").await?;
        assert_that!(page.read_text_of("test-gs-nested").await?).is_equal_to("1".to_owned());

        // The shortcut's keys.
        let keys = page.element("test-gs-keys").await?;
        let mut texts = Vec::new();
        for key in keys.find_all(By::Css("kbd")).await? {
            texts.push(key.text().await?);
        }
        // "Ctrl" shown, "Control" read (visually hidden).
        assert_that!(texts).is_equal_to(vec!["Ctrl\nControl".to_owned(), "K".to_owned()]);
        assert_that!(keys.find_all(By::Css("[data-separator]")).await?.len()).is_equal_to(1);
        // Left to right in right-to-left text too (react-aria-components' `Keyboard`).
        assert_that!(keys.attr("dir").await?).is_equal_to(Some("ltr".to_owned()));

        // Literal keys: as given, on every platform.
        let literal = page.element("test-gs-literal").await?;
        let mut texts = Vec::new();
        for key in literal.find_all(By::Css("kbd")).await? {
            texts.push(key.text().await?);
        }
        assert_that!(texts).is_equal_to(vec!["⌘\nCommand".to_owned(), "X".to_owned()]);
        assert_that!(literal.find_all(By::Css("[data-separator]")).await?.len()).is_equal_to(1);
        page.expect_no_page_errors().await
    }
}
