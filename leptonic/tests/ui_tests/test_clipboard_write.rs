// No upstream: `utils::clipboard` is a leptonic addition (react-aria has no clipboard writer).
//! `utils::clipboard`: writing text, and writing text that is still loading (issued in the press,
//! completed when it arrives), or nothing when none arrives; the errors when the browser denies
//! the write or has no clipboard. Fixture: `/hooks/clipboard-write`.
use assertr::prelude::*;
use browser_test::browser_test;
use rootcause::Report;

use crate::{
    fixtures::clipboard::{ClipboardAccess, ClipboardActions},
    pages::{ElementActions, Page},
};

const PATH: &str = "/hooks/clipboard-write";

/// Opens the fixture, allowed to read the clipboard back (writing during a press needs no
/// permission).
async fn open(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    ClipboardActions::new(page)
        .set_access(ClipboardAccess::ReadWrite)
        .await
}

/// Text available at the press is written right away.
#[browser_test]
pub async fn writes_text(page: &Page<'_>) -> Result<(), Report> {
    write_known_text(page).await
}

async fn write_known_text(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    page.element("#test-clipboard-write").await?.click().await?;
    page.element("#test-clipboard-write-status")
        .await?
        .wait_for_inner_text("written")
        .await?;
    let text = ClipboardActions::new(page).text().await?;
    assert_that!(text).is_equal_to("Written now");
    Ok(())
}

/// Text still loading at the press is written once it arrives.
#[browser_test]
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
    let text = ClipboardActions::new(page).text().await?;
    assert_that!(text).is_equal_to("Loaded later");
    Ok(())
}

/// When no text arrives, the clipboard keeps its previous text and the status says so.
#[browser_test]
pub async fn writes_nothing_without_text(page: &Page<'_>) -> Result<(), Report> {
    // Known text in the clipboard first (opens the page).
    write_known_text(page).await?;
    page.element("#test-clipboard-write-nothing")
        .await?
        .click()
        .await?;
    page.element("#test-clipboard-write-status")
        .await?
        .wait_for_inner_text("no text to write to the clipboard")
        .await?;
    let text = ClipboardActions::new(page).text().await?;
    assert_that!(text).is_equal_to("Written now");
    Ok(())
}

/// When the browser denies writing to the clipboard, the write fails with `Denied`.
#[browser_test]
pub async fn a_denied_write(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    ClipboardActions::new(page)
        .set_access(ClipboardAccess::WriteDenied)
        .await?;
    page.element("#test-clipboard-write").await?.click().await?;
    page.element("#test-clipboard-write-status")
        .await?
        .wait_for_inner_text("the browser denied writing to the clipboard")
        .await?;
    Ok(())
}

/// Without a clipboard (browsers offer none outside secure contexts), writes fail with
/// `Unavailable` instead of throwing.
#[browser_test]
pub async fn no_clipboard(page: &Page<'_>) -> Result<(), Report> {
    // Text available at the press, and text still loading.
    for button in ["#test-clipboard-write", "#test-clipboard-write-later"] {
        page.goto_path(PATH).await?;
        ClipboardActions::new(page).remove_clipboard().await?;
        page.element(button).await?.click().await?;
        page.element("#test-clipboard-write-status")
            .await?
            .wait_for_inner_text("no clipboard available")
            .await?;
    }
    Ok(())
}
