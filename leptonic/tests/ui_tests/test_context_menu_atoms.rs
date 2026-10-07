use std::borrow::Cow;

use assertr::prelude::*;
use browser_test::{
    BrowserTest, async_trait,
    thirtyfour::{Key, WebDriver, WebElement},
};
use rootcause::Report;

use crate::pages::{BaseActions, Page};

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
        assert_that!(page.count_matching("[role=menu]").await?).is_equal_to(0);

        // A right click on "Pictures" opens the menu there, labelled by the row.
        let pictures = row(&page, "Pictures").await?;
        context_click(&page, &pictures).await?;
        page.wait_for_selector("[role=menu]").await?;
        let menu = page.css("[role=menu]").await?;
        let row_id = pictures.attr("id").await?.unwrap_or_default();
        assert_that!(menu.attr("aria-labelledby").await?).is_equal_to(Some(row_id));
        let row_rect = pictures.rect().await?;
        let menu_rect = menu.rect().await?;
        // At the pointer (the row's center), not at the row's start.
        assert_that!(menu_rect.x > row_rect.x + 10.0).is_true();

        // An action knows its row; the focus returns to the row.
        page.by_role_and_text("menuitem", "Rename")
            .await?
            .click()
            .await?;
        page.wait_for_text("test-cm-actions", "Rename Pictures")
            .await?;
        page.wait_for_no_selector("[role=menu]").await?;
        page.wait_for_focus_on(&pictures, "the Pictures row")
            .await?;

        // Escape closes it; the focus returns to the row it opened on.
        let music = row(&page, "Music").await?;
        context_click(&page, &music).await?;
        page.wait_for_selector("[role=menu]").await?;
        page.send_keys_to_active(Key::Escape).await?;
        page.wait_for_no_selector("[role=menu]").await?;
        page.wait_for_focus_on(&music, "the Music row").await?;

        // From the keyboard: Shift+F10 on the focused row.
        page.send_keys_to_active(Key::Shift + Key::F10).await?;
        page.wait_for_selector("[role=menu]").await?;
        page.send_keys_to_active(Key::Escape).await?;
        page.wait_for_no_selector("[role=menu]").await?;
        page.expect_no_page_errors().await
    }
}

async fn row(page: &Page<'_>, text: &str) -> Result<WebElement, Report> {
    page.by_role_and_text("row", text).await
}

async fn context_click(page: &Page<'_>, element: &WebElement) -> Result<(), Report> {
    page.driver
        .action_chain()
        .context_click_element(element)
        .perform()
        .await?;
    Ok(())
}
