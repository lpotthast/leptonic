// Upstream: react-aria-components/test/Menu.test.tsx @ 99e6102368
// Upstream: react-aria/test/interactions/useContextMenu.test.tsx @ 99e6102368
// (Row-level context menus are a leptonic addition; the menu behavior mirrors MenuTrigger
// trigger="contextMenu" in Menu.test.tsx.)
use std::borrow::Cow;

use assertr::prelude::*;
use browser_test::{BrowserTest, async_trait, thirtyfour::prelude::*};
use rootcause::Report;

use crate::pages::{ElementActions, Page, PageActions, role};

/// Context menus on collection rows (`ContextMenuTrigger` around a `GridList`): a right click on a
/// row opens the menu at the pointer, labelled by the row; an action knows the row; the focus
/// returns to the row when the menu closes. The keyboard opens it with Shift+F10.
pub struct ContextMenuAtomsTests {}

#[async_trait]
impl BrowserTest<str> for ContextMenuAtomsTests {
    fn name(&self) -> Cow<'_, str> {
        "context_menu_atoms_tests".into()
    }

    async fn run(&self, driver: &WebDriver, base_url: &str) -> Result<(), Report> {
        let page = Page { driver, base_url };
        page.goto_path("/atoms/context-menu").await?;
        cases!(
            right_click_opens_at_the_pointer(&page),
            escape_returns_focus_to_the_row(&page),
            shift_f10_opens_it(&page),
        );
        Ok(())
    }
}

const MENU: &str = "[role=menu]";

/// The row with the text `text`.
async fn row(page: &Page<'_>, text: &str) -> Result<WebElement, Report> {
    page.element(role("row").text(text)).await
}

/// A right click on `element` with the pointer.
async fn context_click(page: &Page<'_>, element: &WebElement) -> Result<(), Report> {
    page.driver
        .action_chain()
        .context_click_element(element)
        .perform()
        .await?;
    Ok(())
}

/// A right click on "Pictures" opens the menu at the pointer (the row's center, not its start),
/// labelled by the row; an action knows its row; the focus returns to the row.
async fn right_click_opens_at_the_pointer(page: &Page<'_>) -> Result<(), Report> {
    assert_that!(page.count(MENU).await?).is_equal_to(0);
    let pictures = row(page, "Pictures").await?;
    context_click(page, &pictures).await?;
    let menu = page.element(MENU).await?;
    let row_id = pictures.id().await?;
    assert_that!(menu.attr("aria-labelledby").await?).is_equal_to(row_id);
    let row_rect = pictures.client_rect().await?;
    let menu_rect = menu.client_rect().await?;
    assert_that!(menu_rect.left).is_greater_than(row_rect.left + 10.0);

    page.element(role("menuitem").text("Rename"))
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

/// Escape closes it; the focus returns to the row it opened on.
async fn escape_returns_focus_to_the_row(page: &Page<'_>) -> Result<(), Report> {
    let music = row(page, "Music").await?;
    context_click(page, &music).await?;
    page.element(MENU).await?;
    page.send_keys(Key::Escape).await?;
    page.wait_for_count(MENU, 0).await?;
    page.wait_for_focus(&music).await?;
    Ok(())
}

/// From the keyboard: Shift+F10 on the focused row.
async fn shift_f10_opens_it(page: &Page<'_>) -> Result<(), Report> {
    let music = row(page, "Music").await?;
    page.wait_for_focus(&music).await?;
    page.send_keys(Key::Shift + Key::F10).await?;
    let menu = page.element(MENU).await?;
    let row_id = music.id().await?;
    assert_that!(menu.attr("aria-labelledby").await?).is_equal_to(row_id);
    page.send_keys(Key::Escape).await?;
    page.wait_for_count(MENU, 0).await?;
    page.wait_for_focus(&music).await?;
    Ok(())
}
