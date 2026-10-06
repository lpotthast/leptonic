use std::borrow::Cow;

use assertr::prelude::*;
use browser_test::{
    BrowserTest, async_trait,
    thirtyfour::{Key, WebDriver, WebElement},
};
use rootcause::Report;

use crate::pages::{BaseActions, Page};

/// The styled `TiptapEditor`: a group named by `aria_label`, a formatting toolbar (arrow keys
/// move between its buttons), format buttons reporting the selection's formats through
/// `aria-pressed`, and edits reported through `on_change`.
pub struct TiptapEditorComponentsTests {}

#[async_trait]
impl BrowserTest<str> for TiptapEditorComponentsTests {
    fn name(&self) -> Cow<'_, str> {
        "tiptap_editor_components_tests".into()
    }

    async fn run(&self, driver: &WebDriver, base_url: &str) -> Result<(), Report> {
        let page = Page { driver, base_url };
        page.goto_path("/components/tiptap-editor").await?;

        let group = page.css("#test-ctip [role=group]").await?;
        assert_that!(group.attr("aria-label").await?).is_equal_to(Some("Notes".to_owned()));
        let toolbar = page.css("#test-ctip [role=toolbar]").await?;
        assert_that!(toolbar.attr("aria-label").await?).is_equal_to(Some("Formatting".to_owned()));

        // The editor starts with the initial content (tiptap mounts it from its script).
        page.wait_for_selector("#test-ctip .ProseMirror").await?;
        let text = page
            .css("#test-ctip > div > .leptonic-tiptap-instance .ProseMirror")
            .await?;
        assert_that!(text.text().await?).is_equal_to("Hello world".to_owned());

        // Select everything and make it bold: the "Bold" button is pressed, the edit reported.
        text.click().await?;
        page.send_keys_to_active(Key::Control + "a").await?;
        let bold = page.by_role_and_text("button", "Bold").await?;
        assert_that!(bold.attr("aria-pressed").await?).is_equal_to(Some("false".to_owned()));
        bold.click().await?;
        page.wait_for_attr(&bold, "aria-pressed", Some("true"))
            .await?;
        // (tiptap may report one edit in several steps)
        assert_that!(page.read_text_of("test-ctip-changes").await?).is_not_equal_to("0".to_owned());
        assert_that!(page.read_text_of("test-ctip-html").await?).contains("<strong>");

        // Bound to app state: edits write it, content set from outside replaces the editor's.
        let bound = page.css("#test-ctip-bound .ProseMirror").await?;
        assert_that!(bound.text().await?).is_equal_to("Bound".to_owned());
        page.click_element_with_id("test-ctip-replace").await?;
        wait_for_element_text(&bound, "Replaced").await?;
        bound.click().await?;
        page.send_keys_to_active(Key::End).await?;
        page.send_keys_to_active("!").await?;
        page.wait_for_text("test-ctip-bound-html", "<p>Replaced!</p>")
            .await?;

        // Arrow keys move along the toolbar.
        bold.focus().await?;
        page.send_keys_to_active(Key::Right).await?;
        page.wait_for_active_text("Italic").await?;

        page.expect_no_page_errors().await
    }
}

/// Waits up to 10s for `element` to show `expected`.
async fn wait_for_element_text(element: &WebElement, expected: &str) -> Result<(), Report> {
    let mut last = String::new();
    for _ in 0..100 {
        last = element.text().await?;
        if last == expected {
            return Ok(());
        }
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    }
    leptos_browser_test::bail!(
        "the editor did not show {expected:?} within 10s; last seen {last:?}"
    );
}
