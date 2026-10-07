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
        section_selection(&page).await?;
        close_on_select(&page).await?;
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
    // Enter activates the item through a click, which counts as virtual: the modality is the
    // keyboard's again afterwards (upstream sets it after the click), so the trigger shows its
    // focus ring.
    page.wait_for_text("test-menu-atoms-modality", "Keyboard")
        .await?;
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
    // "should support sections": each section is one `<section role="group">` named by its
    // header.
    assert_that!(
        page.count_matching("[role=menu] > section.leptonic-MenuSection[role=group]")
            .await?
    )
    .is_equal_to(2);
    let group = page.css("[role=group]").await?;
    let group_labelledby = group.attr("aria-labelledby").await?.unwrap_or_default();
    let header = page.element(&group_labelledby).await?;
    assert_that!(header.tag_name().await?).is_equal_to("header".to_owned());
    assert_that!(header.text().await?).is_equal_to("Panels".to_owned());
    // A `Separator` in a menu is a `<div role="separator">` ("should support separators").
    assert_that!(
        page.count_matching("[role=menu] div[role=separator]")
            .await?
    )
    .is_equal_to(1);

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
    assert_that!(description.prop("textContent").await?).is_equal_to(Some(
        "Long press or press Alt + ArrowDown to open menu".to_owned(),
    ));

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

/// "should support section-level selection": sections with selections of their own (multiple,
/// single) in one menu; focus moves through both sections as one menu.
async fn section_selection(page: &Page<'_>) -> Result<(), Report> {
    const SANDWICH_TRIGGER: &str = "test-menu-atoms-sandwich-trigger";
    page.click_element_with_id(SANDWICH_TRIGGER).await?;
    page.wait_for_selector(MENU).await?;
    assert_that!(page.count_matching("[role=menuitemcheckbox]").await?).is_equal_to(3);
    assert_that!(page.count_matching("[role=menuitemradio]").await?).is_equal_to(3);
    assert_that!(checked(page, "menuitemcheckbox", "Lettuce").await?)
        .is_equal_to(Some("true".to_owned()));
    assert_that!(checked(page, "menuitemcheckbox", "Tomato").await?)
        .is_equal_to(Some("false".to_owned()));
    assert_that!(checked(page, "menuitemradio", "Ham").await?).is_equal_to(Some("true".to_owned()));

    // Multiple selection in its section: toggles, the menu stays open.
    page.by_role_and_text("menuitemcheckbox", "Tomato")
        .await?
        .click()
        .await?;
    page.wait_for_text("test-menu-atoms-veggies", "Lettuce,Tomato")
        .await?;
    assert_that!(page.count_matching(MENU).await?).is_equal_to(1);
    assert_that!(page.read_text_of("test-menu-atoms-protein").await?).is_equal_to("Ham".to_owned());

    // Single selection in the other: replaces its selection only (and closes the menu).
    page.by_role_and_text("menuitemradio", "Tuna")
        .await?
        .click()
        .await?;
    page.wait_for_text("test-menu-atoms-protein", "Tuna")
        .await?;
    assert_that!(page.read_text_of("test-menu-atoms-veggies").await?)
        .is_equal_to("Lettuce,Tomato".to_owned());
    page.wait_for_no_selector(MENU).await?;
    page.wait_for_active_id(SANDWICH_TRIGGER).await?;

    // The arrow keys move through both sections.
    page.send_keys_to_active(Key::Down).await?;
    page.wait_for_focus("menuitemcheckbox", Some("Lettuce"))
        .await?;
    for (role, text) in [
        ("menuitemcheckbox", "Tomato"),
        ("menuitemcheckbox", "Onion"),
        ("menuitemradio", "Ham"),
        ("menuitemradio", "Tuna"),
    ] {
        page.send_keys_to_active(Key::Down).await?;
        page.wait_for_focus(role, Some(text)).await?;
    }
    assert_that!(checked(page, "menuitemradio", "Tuna").await?)
        .is_equal_to(Some("true".to_owned()));
    page.send_keys_to_active(Key::Escape).await?;
    page.wait_for_no_selector(MENU).await?;
    page.wait_for_active_id(SANDWICH_TRIGGER).await
}

/// "should not close menu items within a section when shouldCloseOnSelect=false", "should not
/// close the menu when shouldCloseOnSelect is false".
async fn close_on_select(page: &Page<'_>) -> Result<(), Report> {
    page.click_element_with_id("test-menu-atoms-file-trigger")
        .await?;
    page.wait_for_selector(MENU).await?;
    page.by_role_and_text("menuitem", "Open")
        .await?
        .click()
        .await?;
    page.wait_for_text("test-menu-atoms-actions", "Cut,Cut,More pressed,Open")
        .await?;
    assert_that!(page.count_matching(MENU).await?).is_equal_to(1);
    page.by_role_and_text("menuitem", "Share")
        .await?
        .click()
        .await?;
    page.wait_for_no_selector(MENU).await?;

    page.click_element_with_id("test-menu-atoms-edit-trigger")
        .await?;
    page.wait_for_selector(MENU).await?;
    page.by_role_and_text("menuitem", "Undo")
        .await?
        .click()
        .await?;
    page.wait_for_text(
        "test-menu-atoms-actions",
        "Cut,Cut,More pressed,Open,Share,Undo",
    )
    .await?;
    assert_that!(page.count_matching(MENU).await?).is_equal_to(1);
    page.send_keys_to_active(Key::Escape).await?;
    page.wait_for_no_selector(MENU).await
}

/// The `aria-checked` of the item with `role` and `text`.
async fn checked(page: &Page<'_>, role: &str, text: &str) -> Result<Option<String>, Report> {
    Ok(page
        .by_role_and_text(role, text)
        .await?
        .attr("aria-checked")
        .await?)
}
