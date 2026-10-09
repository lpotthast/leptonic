// Upstream: react-aria-components/test/Menu.test.tsx @ 99e6102368
// Upstream: react-aria/test/interactions/useContextMenu.test.tsx @ 99e6102368
//! Context menus on collection rows (`ContextMenuTrigger` around a `GridList`): a right click on a
//! row opens the menu at the pointer, labelled by the row; an action knows the row; the focus
//! returns to the row when the menu closes. The keyboard opens it with Shift+F10.
// (Row-level context menus are a leptonic addition; the menu behavior mirrors MenuTrigger
// trigger="contextMenu" in Menu.test.tsx.)
use assertr::prelude::*;
use browser_test::{browser_test, thirtyfour::prelude::*};
use leptonic::AriaRole;
use rootcause::Report;

use crate::pages::{ElementActions, Page, role};

const PATH: &str = "/atoms/context-menu";

const MENU: &str = "[role=menu]";

/// The row with the text `text`.
async fn row(page: &Page<'_>, text: &str) -> Result<WebElement, Report> {
    page.element(role(AriaRole::Row).text(text)).await
}

/// A right click on `element` with the pointer.
async fn context_click(page: &Page<'_>, element: &WebElement) -> Result<(), Report> {
    page.low_level()
        .driver()
        .action_chain()
        .context_click_element(element)
        .perform()
        .await?;
    Ok(())
}

/// A right click on a row opens the menu at the pointer, labelled by the row; choosing an action
/// runs it for that row and returns focus to the row ("should support a context menu trigger").
#[browser_test]
pub async fn right_click_opens_at_the_pointer(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    assert_that!(page.count(MENU).await?).is_equal_to(0);
    let pictures = row(page, "Pictures").await?;
    context_click(page, &pictures).await?;
    let menu = page.element(MENU).await?;
    let row_id = pictures.id().await?;
    assert_that!(menu)
        .attribute("aria-labelledby")
        .await
        .is_equal_to(row_id);
    let row_rect = pictures.client_rect().await?;
    let menu_rect = menu.client_rect().await?;
    assert_that!(menu_rect.left).is_greater_than(row_rect.left + 10.0);

    page.element(role(AriaRole::Menuitem).text("Rename"))
        .await?
        .click()
        .await?;
    page.element("#test-cm-actions")
        .await?
        .wait_for_inner_text("Rename Pictures")
        .await?;
    page.wait_for_count(MENU, 0).await?;
    page.wait_for_focus(&pictures).await?;
    Ok(())
}

/// Escape closes a row's context menu and returns focus to the row it opened on.
#[browser_test]
pub async fn escape_returns_focus_to_the_row(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let music = row(page, "Music").await?;
    context_click(page, &music).await?;
    page.element(MENU).await?;
    page.send_keys(Key::Escape).await?;
    page.wait_for_count(MENU, 0).await?;
    page.wait_for_focus(&music).await?;
    Ok(())
}

/// Shift+F10 on the focused row opens its context menu, labelled by the row; Escape closes it and
/// returns focus to the row.
#[browser_test]
pub async fn shift_f10_opens_it(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    // Tab into the list (its first row), then down to "Music".
    page.element("#test-cm-before").await?.click().await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&row(page, "Documents").await?).await?;
    page.send_keys(Key::Down).await?;
    page.wait_for_focus(&row(page, "Pictures").await?).await?;
    page.send_keys(Key::Down).await?;
    let music = row(page, "Music").await?;
    page.wait_for_focus(&music).await?;
    page.send_keys(Key::Shift + Key::F10).await?;
    let menu = page.element(MENU).await?;
    let row_id = music.id().await?;
    assert_that!(menu)
        .attribute("aria-labelledby")
        .await
        .is_equal_to(row_id);
    page.send_keys(Key::Escape).await?;
    page.wait_for_count(MENU, 0).await?;
    page.wait_for_focus(&music).await?;
    Ok(())
}
