// Upstream: react-aria-components/test/Menu.test.tsx @ 99e6102368
// Upstream: @adobe/react-spectrum/test/menu/MenuTrigger.test.js @ 99e6102368
//! The menu atoms (`MenuTrigger`, `Popover`, `Menu`, `MenuItem`, `MenuSection`, item slots): the
//! trigger opens and controls the menu, which is labelled by it; keyboard opening focuses the first
//! or last item; actions close the menu and return focus; a selection menu has checkbox items and
//! stays open; sections and item slots are wired up.
use assertr::prelude::*;
use browser_test::thirtyfour::prelude::*;
use rootcause::Report;

use crate::pages::{ElementActions, Page, PageActions, role};

const PATH: &str = "/atoms/menu";

const ACTIONS_TRIGGER: &str = "#test-menu-atoms-actions-trigger";
const VIEW_TRIGGER: &str = "#test-menu-atoms-view-trigger";
const LONG_TRIGGER: &str = "#test-menu-atoms-long-trigger";
const SANDWICH_TRIGGER: &str = "#test-menu-atoms-sandwich-trigger";
const MENU: &str = "[role=menu]";

/// The fixture's log of performed actions, comma-separated.
async fn actions_log(page: &Page<'_>) -> Result<WebElement, Report> {
    page.element("#test-menu-atoms-actions").await
}

/// Wait until the item with `role` and `text` has focus.
async fn expect_focus(page: &Page<'_>, item_role: &str, text: &str) -> Result<(), Report> {
    page.wait_for_focus(&page.element(role(item_role).text(text)).await?)
        .await
}

/// The menu is gone and the trigger `trigger` (a selector) has focus again.
async fn expect_closed_with_focus_on(page: &Page<'_>, trigger: &str) -> Result<(), Report> {
    page.wait_for_count(MENU, 0).await?;
    page.wait_for_focus(&page.element(trigger).await?).await?;
    Ok(())
}

/// The `aria-checked` of the item with `role` and `text`.
async fn aria_checked(
    page: &Page<'_>,
    item_role: &str,
    text: &str,
) -> Result<Option<String>, Report> {
    Ok(page
        .element(role(item_role).text(text))
        .await?
        .attr("aria-checked")
        .await?)
}

/// "should support menu trigger": the trigger opens the menu, which it controls and which it labels;
/// pressing an item performs its action, closes the menu and returns focus to the trigger.
pub async fn menu_trigger(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let trigger = page.element(ACTIONS_TRIGGER).await?;
    assert_that!(trigger.attr("aria-haspopup").await?)
        .get_some()
        .is_equal_to("true");
    assert_that!(trigger.attr("aria-expanded").await?)
        .get_some()
        .is_equal_to("false");
    assert_that!(page.count(MENU).await?).is_equal_to(0);

    trigger.click().await?;
    let menu = page.element(MENU).await?;
    trigger.wait_for_attr("aria-expanded", Some("true")).await?;
    let menu_id = menu.attr("id").await?;
    assert_that!(trigger.attr("aria-controls").await?).is_equal_to(menu_id);
    let trigger_id = trigger.id().await?;
    menu.wait_for_attr("aria-labelledby", trigger_id.as_deref())
        .await?;
    // Opened with the mouse: the menu itself has focus.
    page.wait_for_focus(&menu).await?;
    assert_that!(menu.inner_texts("[role=menuitem]").await?)
        .contains_exactly(["Copy", "Cut", "Paste", "Delete"]);
    let paste = page.element(role("menuitem").text("Paste")).await?;
    assert_that!(paste.attr("aria-disabled").await?)
        .get_some()
        .is_equal_to("true");

    page.element(role("menuitem").text("Cut"))
        .await?
        .click()
        .await?;
    actions_log(page).await?.wait_for_inner_text("Cut").await?;
    expect_closed_with_focus_on(page, ACTIONS_TRIGGER).await?;
    trigger
        .wait_for_attr("aria-expanded", Some("false"))
        .await?;
    Ok(())
}

/// Keyboard opening: ArrowDown focuses the first item, ArrowUp the last; Enter performs the focused
/// item's action; Escape closes the menu; focus returns to the trigger.
pub async fn keyboard_opening(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element("#test-menu-atoms-before")
        .await?
        .click()
        .await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&page.element(ACTIONS_TRIGGER).await?)
        .await?;

    page.send_keys(Key::Down).await?;
    expect_focus(page, "menuitem", "Copy").await?;
    page.send_keys(Key::Escape).await?;
    expect_closed_with_focus_on(page, ACTIONS_TRIGGER).await?;

    page.send_keys(Key::Up).await?;
    expect_focus(page, "menuitem", "Delete").await?;
    // Arrow keys skip the disabled "Paste".
    page.send_keys(Key::Up).await?;
    expect_focus(page, "menuitem", "Cut").await?;
    page.send_keys(Key::Enter).await?;
    actions_log(page).await?.wait_for_inner_text("Cut").await?;
    expect_closed_with_focus_on(page, ACTIONS_TRIGGER).await?;
    // Enter activates the item through a click, which counts as virtual: the modality is the
    // keyboard's again afterwards (upstream sets it after the click), so the trigger shows its
    // focus ring.
    page.element("#test-menu-atoms-modality")
        .await?
        .wait_for_inner_text("Keyboard")
        .await?;
    Ok(())
}

/// "should support selection state" and "should support sections": a menu with multiple selection
/// has `menuitemcheckbox` items in labelled groups (one `<section role="group">` each, named by
/// its `<header>`), toggles them on press and stays open; an item's label, description and
/// shortcut are wired up; a separator is a `<div role="separator">`.
pub async fn selection_menu(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element(VIEW_TRIGGER).await?.click().await?;
    let menu = page.element(MENU).await?;
    assert_that!(menu.inner_texts("[role=menuitemcheckbox]").await?).has_length(3);
    assert_that!(
        page.count("[role=menu] > section.leptonic-MenuSection[role=group]")
            .await?
    )
    .is_equal_to(2);
    assert_that!(page.count("[role=group]").await?).is_equal_to(2);
    let group = page.element("[role=group]").await?;
    let header_id = group.attr("aria-labelledby").await?.unwrap_or_default();
    let header = page.element(format!("#{header_id}")).await?;
    assert_that!(header.tag_name().await?).is_equal_to("header");
    assert_that!(header.inner_text().await?).is_equal_to("Panels");
    assert_that!(page.count("[role=menu] div[role=separator]").await?).is_equal_to(1);

    // The first item: "Sidebar".
    let sidebar = page.element("[role=menuitemcheckbox]").await?;
    assert_that!(sidebar.referenced_text("aria-labelledby").await?).is_equal_to("Sidebar");
    let describedby = sidebar.attr("aria-describedby").await?.unwrap_or_default();
    let mut descriptions = Vec::new();
    for id in describedby.split_whitespace() {
        descriptions.push(page.element(format!("#{id}")).await?.inner_text().await?);
    }
    assert_that!(descriptions).contains_exactly_in_any_order(["Ctrl+B", "Show the file tree"]);

    assert_that!(sidebar.attr("aria-checked").await?)
        .get_some()
        .is_equal_to("false");
    sidebar.click().await?;
    page.element("#test-menu-atoms-view-selection")
        .await?
        .wait_for_inner_text("Sidebar")
        .await?;
    assert_that!(sidebar.attr("aria-checked").await?)
        .get_some()
        .is_equal_to("true");
    // Multiple selection keeps the menu open.
    assert_that!(page.count(MENU).await?).is_equal_to(1);

    page.send_keys(Key::Escape).await?;
    expect_closed_with_focus_on(page, VIEW_TRIGGER).await?;
    Ok(())
}

/// `trigger="longPress"` (React Spectrum `MenuTrigger.test.js`): the button describes the long press;
/// a press performs the button's own action and doesn't open the menu; a long press opens it, as does
/// Alt+ArrowDown (with the first item focused).
pub async fn long_press_trigger(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let trigger = page.element(LONG_TRIGGER).await?;
    assert_that!(trigger.referenced_text("aria-describedby").await?)
        .is_equal_to("Long press or press Alt + ArrowDown to open menu");

    // A press is the button's own action.
    trigger.click().await?;
    actions_log(page)
        .await?
        .wait_for_inner_text("More pressed")
        .await?;
    page.count_stays(MENU, 0).await?;

    // A long press opens the menu.
    page.driver
        .action_chain()
        .click_and_hold_element(&trigger)
        .perform()
        .await?;
    // A real timer: hold past the long-press delay (500 ms).
    tokio::time::sleep(std::time::Duration::from_millis(800)).await;
    page.driver.action_chain().release().perform().await?;
    let trigger_id = trigger.id().await?;
    page.element(MENU)
        .await?
        .wait_for_attr("aria-labelledby", trigger_id.as_deref())
        .await?;
    page.send_keys(Key::Escape).await?;
    expect_closed_with_focus_on(page, LONG_TRIGGER).await?;

    // Alt+ArrowDown opens it with the first item focused.
    page.send_keys(Key::Alt + Key::Down).await?;
    expect_focus(page, "menuitem", "Copy").await?;
    page.send_keys(Key::Escape).await?;
    expect_closed_with_focus_on(page, LONG_TRIGGER).await?;
    Ok(())
}

/// "should support section-level selection": sections with selections of their own (multiple,
/// single) in one menu; focus moves through both sections as one menu.
pub async fn section_selection(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let veggies = page.element("#test-menu-atoms-veggies").await?;
    let protein = page.element("#test-menu-atoms-protein").await?;
    page.element(SANDWICH_TRIGGER).await?.click().await?;
    page.element(MENU).await?;
    assert_that!(page.inner_texts("[role=menuitemcheckbox]").await?)
        .contains_exactly(["Lettuce", "Tomato", "Onion"]);
    assert_that!(page.inner_texts("[role=menuitemradio]").await?)
        .contains_exactly(["Ham", "Tuna", "Tofu"]);
    assert_that!(aria_checked(page, "menuitemcheckbox", "Lettuce").await?)
        .get_some()
        .is_equal_to("true");
    assert_that!(aria_checked(page, "menuitemcheckbox", "Tomato").await?)
        .get_some()
        .is_equal_to("false");
    assert_that!(aria_checked(page, "menuitemradio", "Ham").await?)
        .get_some()
        .is_equal_to("true");

    // Multiple selection in its section: toggles, the menu stays open.
    page.element(role("menuitemcheckbox").text("Tomato"))
        .await?
        .click()
        .await?;
    veggies.wait_for_inner_text("Lettuce,Tomato").await?;
    assert_that!(page.count(MENU).await?).is_equal_to(1);
    assert_that!(protein.inner_text().await?).is_equal_to("Ham");

    // Single selection in the other: replaces its selection only (and closes the menu).
    page.element(role("menuitemradio").text("Tuna"))
        .await?
        .click()
        .await?;
    protein.wait_for_inner_text("Tuna").await?;
    assert_that!(veggies.inner_text().await?).is_equal_to("Lettuce,Tomato");
    expect_closed_with_focus_on(page, SANDWICH_TRIGGER).await?;

    // The arrow keys move through both sections.
    page.send_keys(Key::Down).await?;
    expect_focus(page, "menuitemcheckbox", "Lettuce").await?;
    for (role, text) in [
        ("menuitemcheckbox", "Tomato"),
        ("menuitemcheckbox", "Onion"),
        ("menuitemradio", "Ham"),
        ("menuitemradio", "Tuna"),
    ] {
        page.send_keys(Key::Down).await?;
        expect_focus(page, role, text).await?;
    }
    assert_that!(aria_checked(page, "menuitemradio", "Tuna").await?)
        .get_some()
        .is_equal_to("true");
    page.send_keys(Key::Escape).await?;
    expect_closed_with_focus_on(page, SANDWICH_TRIGGER).await?;
    Ok(())
}

/// "should not close menu items within a section when shouldCloseOnSelect=false", "should not
/// close the menu when shouldCloseOnSelect is false".
pub async fn close_on_select(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let actions = actions_log(page).await?;
    page.element("#test-menu-atoms-file-trigger")
        .await?
        .click()
        .await?;
    page.element(MENU).await?;
    page.element(role("menuitem").text("Open"))
        .await?
        .click()
        .await?;
    actions.wait_for_inner_text("Open").await?;
    page.count_stays(MENU, 1).await?;
    page.element(role("menuitem").text("Share"))
        .await?
        .click()
        .await?;
    page.wait_for_count(MENU, 0).await?;

    page.element("#test-menu-atoms-edit-trigger")
        .await?
        .click()
        .await?;
    page.element(MENU).await?;
    page.element(role("menuitem").text("Undo"))
        .await?
        .click()
        .await?;
    actions.wait_for_inner_text("Open,Share,Undo").await?;
    page.count_stays(MENU, 1).await?;
    page.send_keys(Key::Escape).await?;
    page.wait_for_count(MENU, 0).await?;
    Ok(())
}
