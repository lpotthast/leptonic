use std::borrow::Cow;

use assertr::prelude::*;
use browser_test::{BrowserTest, async_trait, thirtyfour::prelude::*};
use rootcause::Report;

use crate::pages::{ElementActions, Page, PageActions};

/// `utils::clipboard`: writing text, and writing text that is still loading (issued in the press,
/// completed when it arrives), or nothing when none arrives. Fixture: `/hooks/clipboard-write`.
pub struct ClipboardWriteTests {}

#[async_trait]
impl BrowserTest<str> for ClipboardWriteTests {
    fn name(&self) -> Cow<'_, str> {
        "clipboard_write_tests".into()
    }

    async fn run(&self, driver: &WebDriver, base_url: &str) -> Result<(), Report> {
        let page = Page { driver, base_url };
        page.goto_path("/hooks/clipboard-write").await?;
        // Reading the clipboard back needs the permission (writing during a press doesn't).
        driver
            .cdp()
            .send_raw(
                "Browser.grantPermissions",
                serde_json::json!({
                    "origin": base_url.trim_end_matches('/'),
                    "permissions": ["clipboardReadWrite", "clipboardSanitizedWrite"],
                }),
            )
            .await?;

        cases!(
            writes_text(&page),
            writes_text_loaded_later(&page),
            writes_nothing_without_text(&page),
        );
        Ok(())
    }
}

/// The clipboard's text (`navigator.clipboard.readText()`; WebDriver awaits the promise).
async fn clipboard_text(page: &Page<'_>) -> Result<String, Report> {
    page.eval("return navigator.clipboard.readText();", vec![])
        .await
}

/// Text available at the press is written right away.
async fn writes_text(page: &Page<'_>) -> Result<(), Report> {
    page.element("#test-clipboard-write").await?.click().await?;
    page.element("#test-clipboard-write-status")
        .await?
        .wait_for_inner_text("written")
        .await?;
    assert_that!(clipboard_text(page).await?).is_equal_to("Written now");
    Ok(())
}

/// Text still loading at the press is written once it arrives.
async fn writes_text_loaded_later(page: &Page<'_>) -> Result<(), Report> {
    page.element("#test-clipboard-write-later")
        .await?
        .click()
        .await?;
    page.element("#test-clipboard-write-status")
        .await?
        .wait_for_inner_text("written")
        .await?;
    assert_that!(clipboard_text(page).await?).is_equal_to("Loaded later");
    Ok(())
}

/// Nothing arrives: the clipboard keeps its text.
async fn writes_nothing_without_text(page: &Page<'_>) -> Result<(), Report> {
    page.element("#test-clipboard-write-nothing")
        .await?
        .click()
        .await?;
    page.element("#test-clipboard-write-status")
        .await?
        .wait_for_inner_text("no text to write to the clipboard")
        .await?;
    assert_that!(clipboard_text(page).await?).is_equal_to("Loaded later");
    Ok(())
}
