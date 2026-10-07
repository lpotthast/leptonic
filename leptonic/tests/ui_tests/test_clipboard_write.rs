use std::borrow::Cow;

use assertr::prelude::*;
use browser_test::{BrowserTest, async_trait, thirtyfour::WebDriver};
use rootcause::Report;

use crate::pages::{BaseActions, Page};

const STATUS: &str = "test-clipboard-write-status";

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
        let origin = base_url.trim_end_matches('/');
        driver
            .cdp()
            .send_raw(
                "Browser.grantPermissions",
                serde_json::json!({
                    "origin": origin,
                    "permissions": ["clipboardReadWrite", "clipboardSanitizedWrite"],
                }),
            )
            .await?;

        page.click_element_with_id("test-clipboard-write").await?;
        page.wait_for_text(STATUS, "written").await?;
        assert_that!(clipboard_text(&page).await?).is_equal_to("Written now".to_owned());

        page.click_element_with_id("test-clipboard-write-later")
            .await?;
        page.wait_for_text(STATUS, "written").await?;
        assert_that!(clipboard_text(&page).await?).is_equal_to("Loaded later".to_owned());

        // Nothing arrives: the clipboard keeps its text.
        page.click_element_with_id("test-clipboard-write-nothing")
            .await?;
        page.wait_for_text(STATUS, "no text to write to the clipboard")
            .await?;
        assert_that!(clipboard_text(&page).await?).is_equal_to("Loaded later".to_owned());
        page.expect_no_page_errors().await
    }
}

/// The clipboard's text (`navigator.clipboard.readText()`; WebDriver awaits the promise).
async fn clipboard_text(page: &Page<'_>) -> Result<String, Report> {
    Ok(page
        .driver
        .execute("return navigator.clipboard.readText();", vec![])
        .await?
        .convert::<String>()?)
}
