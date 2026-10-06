// Upstream: react-aria-components/test/Menu.test.tsx @ 99e6102368
// Upstream: @adobe/react-spectrum/test/menu/MenuTrigger.test.js @ 99e6102368
use std::borrow::Cow;

use assertr::prelude::*;
use browser_test::{
    BrowserTest, async_trait,
    thirtyfour::{Key, WebDriver},
};
use rootcause::Report;

use crate::pages::{BaseActions, Page};

const ACTIONS_TRIGGER: &str = "test-menu-atoms-actions-trigger";
const VIEW_TRIGGER: &str = "test-menu-atoms-view-trigger";
const MENU: &str = "[role=menu]";

/// The menu atoms (`MenuTrigger`, `Popover`, `Menu`, `MenuItem`, `MenuSection`, item slots): the
/// trigger opens and controls the menu, which is labelled by it; keyboard opening focuses the first
/// or last item; actions close the menu and return focus; a selection menu has checkbox items and
/// stays open; sections and item slots are wired up.
pub struct MenuAtomTests {}

#[async_trait]
impl BrowserTest<str> for MenuAtomTests {
    fn name(&self) -> Cow<'_, str> {
        "menu_atom_tests".into()
    }

    async fn run(&self, driver: &WebDriver, base_url: &str) -> Result<(), Report> {
        let page = Page { driver, base_url };
        page.goto_path("/atoms/menu").await?;

        menu_trigger(&page).await?;
        keyboard_opening(&page).await?;
        selection_menu(&page).await?;
        long_press_trigger(&page).await?;
        page.expect_no_page_errors().await
    }
}

/// "should support menu trigger": the trigger opens the menu, which it controls and which it labels;
/// pressing an item performs its action, closes the menu and returns focus to the trigger.
async fn menu_trigger(page: &Page<'_>) -> Result<(), Report> {
    let trigger = page.element(ACTIONS_TRIGGER).await?;
    assert_that!(trigger.attr("aria-haspopup").await?).is_equal_to(Some("true".to_owned()));
    assert_that!(trigger.attr("aria-expanded").await?).is_equal_to(Some("false".to_owned()));
    assert_that!(page.count_matching(MENU).await?).is_equal_to(0);

    trigger.click().await?;
    page.wait_for_selector(MENU).await?;
    let menu = page.css(MENU).await?;
    assert_that!(trigger.attr("aria-expanded").await?).is_equal_to(Some("true".to_owned()));
    let menu_id = menu.attr("id").await?;
    assert_that!(trigger.attr("aria-controls").await?).is_equal_to(menu_id);
    page.wait_for_attr(&menu, "aria-labelledby", Some(ACTIONS_TRIGGER))
        .await?;
    // Opened with the mouse: the menu itself has focus.
    page.wait_for_focus("menu", None).await?;
    assert_that!(page.count_matching("[role=menuitem]").await?).is_equal_to(4);
    let paste = page.by_role_and_text("menuitem", "Paste").await?;
    assert_that!(paste.attr("aria-disabled").await?).is_equal_to(Some("true".to_owned()));

    page.by_role_and_text("menuitem", "Cut")
        .await?
        .click()
        .await?;
    page.wait_for_text("test-menu-atoms-actions", "Cut").await?;
    page.wait_for_no_selector(MENU).await?;
    page.wait_for_active_id(ACTIONS_TRIGGER).await?;
    assert_that!(trigger.attr("aria-expanded").await?).is_equal_to(Some("false".to_owned()));
    Ok(())
}

/// Keyboard opening: ArrowDown focuses the first item, ArrowUp the last; Enter performs the focused
/// item's action; Escape closes the menu; focus returns to the trigger.
async fn keyboard_opening(page: &Page<'_>) -> Result<(), Report> {
    page.click_element_with_id("test-menu-atoms-before").await?;
    page.press_tab().await?;
    page.wait_for_active_id(ACTIONS_TRIGGER).await?;

    page.send_keys_to_active(Key::Down).await?;
    page.wait_for_selector(MENU).await?;
    page.wait_for_focus("menuitem", Some("Copy")).await?;
    page.send_keys_to_active(Key::Escape).await?;
    page.wait_for_no_selector(MENU).await?;
    page.wait_for_active_id(ACTIONS_TRIGGER).await?;

    page.send_keys_to_active(Key::Up).await?;
    page.wait_for_selector(MENU).await?;
    page.wait_for_focus("menuitem", Some("Delete")).await?;
    // Arrow keys skip the disabled "Paste".
    page.send_keys_to_active(Key::Up).await?;
    page.wait_for_focus("menuitem", Some("Cut")).await?;
    page.send_keys_to_active(Key::Enter).await?;
    page.wait_for_text("test-menu-atoms-actions", "Cut,Cut")
        .await?;
    page.wait_for_no_selector(MENU).await?;
    page.wait_for_active_id(ACTIONS_TRIGGER).await?;
    Ok(())
}

/// "should support selection state" and "should support sections": a menu with multiple selection
/// has `menuitemcheckbox` items in labelled groups, toggles them on press and stays open; an item's
/// label, description and shortcut are wired up.
async fn selection_menu(page: &Page<'_>) -> Result<(), Report> {
    page.click_element_with_id(VIEW_TRIGGER).await?;
    page.wait_for_selector(MENU).await?;
    assert_that!(page.count_matching("[role=menuitemcheckbox]").await?).is_equal_to(3);
    assert_that!(page.count_matching("[role=group]").await?).is_equal_to(2);

    // The first item: "Sidebar".
    let sidebar = page.css("[role=menuitemcheckbox]").await?;
    let labelledby = sidebar.attr("aria-labelledby").await?.unwrap_or_default();
    let describedby = sidebar.attr("aria-describedby").await?.unwrap_or_default();
    let label = page.element(&labelledby).await?;
    assert_that!(label.text().await?).is_equal_to("Sidebar".to_owned());
    let described: Vec<&str> = describedby.split(' ').collect();
    assert_that!(described.len()).is_equal_to(2);
    let mut texts = Vec::new();
    for id in described {
        texts.push(page.element(id).await?.text().await?);
    }
    texts.sort();
    assert_that!(texts).is_equal_to(vec!["Ctrl+B".to_owned(), "Show the file tree".to_owned()]);

    assert_that!(sidebar.attr("aria-checked").await?).is_equal_to(Some("false".to_owned()));
    sidebar.click().await?;
    page.wait_for_text("test-menu-atoms-view-selection", "Sidebar")
        .await?;
    assert_that!(sidebar.attr("aria-checked").await?).is_equal_to(Some("true".to_owned()));
    // Multiple selection keeps the menu open.
    assert_that!(page.count_matching(MENU).await?).is_equal_to(1);

    page.send_keys_to_active(Key::Escape).await?;
    page.wait_for_no_selector(MENU).await?;
    page.wait_for_active_id(VIEW_TRIGGER).await?;
    Ok(())
}

/// `trigger="longPress"` (React Spectrum `MenuTrigger.test.js`): the button describes the long press;
/// a press performs the button's own action and doesn't open the menu; a long press opens it, as does
/// Alt+ArrowDown (with the first item focused).
async fn long_press_trigger(page: &Page<'_>) -> Result<(), Report> {
    const LONG_TRIGGER: &str = "test-menu-atoms-long-trigger";
    let trigger = page.element(LONG_TRIGGER).await?;
    let describedby = trigger.attr("aria-describedby").await?.unwrap_or_default();
    let description = page.element(&describedby).await?;
    // Hidden from view, so read its text content.
    assert_that!(description.prop("textContent").await?)
        .is_equal_to(Some("Long press or press Alt + ArrowDown to open menu".to_owned()));

    // A press is the button's own action.
    trigger.click().await?;
    page.wait_for_text("test-menu-atoms-actions", "Cut,Cut,More pressed")
        .await?;
    assert_that!(page.count_matching(MENU).await?).is_equal_to(0);

    // A long press opens the menu.
    page.driver
        .action_chain()
        .click_and_hold_element(&trigger)
        .perform()
        .await?;
    tokio::time::sleep(std::time::Duration::from_millis(800)).await;
    page.driver.action_chain().release().perform().await?;
    page.wait_for_selector(MENU).await?;
    page.wait_for_attr(
        &page.css(MENU).await?,
        "aria-labelledby",
        Some(LONG_TRIGGER),
    )
    .await?;
    page.send_keys_to_active(Key::Escape).await?;
    page.wait_for_no_selector(MENU).await?;
    page.wait_for_active_id(LONG_TRIGGER).await?;

    // Alt+ArrowDown opens it with the first item focused.
    page.send_keys_to_active(Key::Alt + Key::Down).await?;
    page.wait_for_selector(MENU).await?;
    page.wait_for_focus("menuitem", Some("Copy")).await?;
    page.send_keys_to_active(Key::Escape).await?;
    page.wait_for_no_selector(MENU).await?;
    page.wait_for_active_id(LONG_TRIGGER).await?;
    Ok(())
}
