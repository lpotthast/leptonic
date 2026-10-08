//! `utils::clipboard`: writing text, and writing text that is still loading (issued in the press,
//! completed when it arrives), or nothing when none arrives. Fixture: `/hooks/clipboard-write`.
use assertr::prelude::*;
use rootcause::Report;

use crate::pages::{ElementActions, Page, PageActions};

/// Opens the fixture, allowed to read the clipboard back (writing during a press needs no
/// permission).
async fn open(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/hooks/clipboard-write").await?;
    // For the browser context of this tab: browser-test runs every test in a context of its own.
    let tab = page
        .driver
        .cdp()
        .send_raw("Target.getTargetInfo", serde_json::json!({}))
        .await?;
    page.driver
        .cdp()
        .send_raw(
            "Browser.grantPermissions",
            serde_json::json!({
                "origin": page.base_url.trim_end_matches('/'),
                "permissions": ["clipboardReadWrite", "clipboardSanitizedWrite"],
                "browserContextId": tab["targetInfo"]["browserContextId"],
            }),
        )
        .await?;
    Ok(())
}

/// The clipboard's text (`navigator.clipboard.readText()`; WebDriver awaits the promise).
async fn clipboard_text(page: &Page<'_>) -> Result<String, Report> {
    page.eval("return navigator.clipboard.readText();", vec![])
        .await
}

/// Text available at the press is written right away.
pub async fn writes_text(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    page.element("#test-clipboard-write").await?.click().await?;
    page.element("#test-clipboard-write-status")
        .await?
        .wait_for_inner_text("written")
        .await?;
    assert_that!(clipboard_text(page).await?).is_equal_to("Written now");
    Ok(())
}

/// Text still loading at the press is written once it arrives.
pub async fn writes_text_loaded_later(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
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
pub async fn writes_nothing_without_text(page: &Page<'_>) -> Result<(), Report> {
    // Known text in the clipboard first (opens the page).
    writes_text(page).await?;
    page.element("#test-clipboard-write-nothing")
        .await?
        .click()
        .await?;
    page.element("#test-clipboard-write-status")
        .await?
        .wait_for_inner_text("no text to write to the clipboard")
        .await?;
    assert_that!(clipboard_text(page).await?).is_equal_to("Written now");
    Ok(())
}
