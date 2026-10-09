// Upstream: react-aria-components/test/Menu.test.tsx @ 99e6102368
// Upstream: @adobe/react-spectrum/test/menu/MenuTrigger.test.js @ 99e6102368
// Upstream: react-aria/test/menu/useMenu.test.tsx @ 99e6102368
//! The menu atoms (`MenuTrigger`, `Popover`, `Menu`, `MenuItem`, `MenuSection`, item slots): the
//! trigger opens and controls the menu, which is labelled by it; keyboard opening focuses the first
//! or last item; actions close the menu and return focus; a selection menu has checkbox items and
//! stays open; sections and item slots are wired up.
use std::time::Duration;

use assertr::{matchers::eq, prelude::*};
use browser_test::{browser_test, thirtyfour::prelude::*};
use leptonic::AriaRole;
use rootcause::Report;

use crate::pages::{ElementActions, Page, css, role};

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

/// The menu is gone and the trigger `trigger` (a selector) has focus again.
async fn expect_closed_with_focus_on(page: &Page<'_>, trigger: &str) -> Result<(), Report> {
    page.wait_for_count(MENU, 0).await?;
    page.wait_for_focus(&page.element(trigger).await?).await?;
    Ok(())
}

/// Clicking the trigger opens a menu it controls and labels, and pressing an item performs its
/// action, closes the menu and returns the focus to the trigger ("should support menu trigger").
#[browser_test]
pub async fn menu_trigger(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let trigger = page.element(ACTIONS_TRIGGER).await?;
    assert_that!(trigger)
        .has_attribute("aria-haspopup")
        .await
        .is_equal_to("true");
    assert_that!(trigger)
        .has_attribute("aria-expanded")
        .await
        .is_equal_to("false");
    assert_that!(page.count(MENU).await?).is_equal_to(0);

    trigger.click().await?;
    let menu = page.element(MENU).await?;
    trigger.wait_for_attr("aria-expanded", Some("true")).await?;
    let menu_id = menu.attr("id").await?;
    assert_that!(trigger)
        .attribute("aria-controls")
        .await
        .is_equal_to(menu_id);
    let trigger_id = trigger.id().await?;
    menu.wait_for_attr("aria-labelledby", trigger_id.as_deref())
        .await?;
    // Opened with the mouse: the menu itself has focus.
    page.wait_for_focus(&menu).await?;
    assert_that!(menu.inner_texts("[role=menuitem]").await?)
        .contains_exactly(["Copy", "Cut", "Paste", "Delete"]);
    let paste = page.element(role(AriaRole::Menuitem).text("Paste")).await?;
    assert_that!(paste)
        .has_attribute("aria-disabled")
        .await
        .is_equal_to("true");

    page.element(role(AriaRole::Menuitem).text("Cut"))
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

/// ArrowDown on the trigger opens the menu on its first item and ArrowUp on its last. Enter
/// performs the focused item's action and Escape closes the menu, both returning the focus to the
/// trigger.
#[browser_test]
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
    page.wait_for_focus(&page.element(role(AriaRole::Menuitem).text("Copy")).await?)
        .await?;
    page.send_keys(Key::Escape).await?;
    expect_closed_with_focus_on(page, ACTIONS_TRIGGER).await?;

    page.send_keys(Key::Up).await?;
    page.wait_for_focus(
        &page
            .element(role(AriaRole::Menuitem).text("Delete"))
            .await?,
    )
    .await?;
    // Arrow keys skip the disabled "Paste".
    page.send_keys(Key::Up).await?;
    page.wait_for_focus(&page.element(role(AriaRole::Menuitem).text("Cut")).await?)
        .await?;
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

/// A multiple-selection menu has `menuitemcheckbox` items, named and described by their label,
/// description and shortcut slots, in sections named by their headers. Pressing an item toggles it
/// and keeps the menu open ("should support selection state", "should support sections", "should
/// support separators", "should support slots").
#[browser_test]
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
    let group = page
        .element(css("[role=group]").has(css("header").text("Panels")))
        .await?;
    let header_id = group.attr("aria-labelledby").await?.unwrap_or_default();
    let header = page.element(format!("#{header_id}")).await?;
    assert_that!(header.tag_name().await?).is_equal_to("header");
    assert_that!(header)
        .inner_text()
        .await
        .is_equal_to("Panels");
    assert_that!(page.count("[role=menu] div[role=separator]").await?).is_equal_to(1);

    // The first item: "Sidebar".
    let sidebar = page.first_element("[role=menuitemcheckbox]").await?;
    assert_that!(sidebar)
        .accessible_name()
        .await
        .is_equal_to("Sidebar");
    let describedby = sidebar.attr("aria-describedby").await?.unwrap_or_default();
    let mut descriptions = Vec::new();
    for id in describedby.split_whitespace() {
        descriptions.push(page.element(format!("#{id}")).await?.inner_text().await?);
    }
    assert_that!(descriptions).contains_exactly_in_any_order(["Ctrl+B", "Show the file tree"]);

    assert_that!(sidebar)
        .has_attribute("aria-checked")
        .await
        .is_equal_to("false");
    sidebar.click().await?;
    page.element("#test-menu-atoms-view-selection")
        .await?
        .wait_for_inner_text("Sidebar")
        .await?;
    assert_that!(sidebar)
        .has_attribute("aria-checked")
        .await
        .is_equal_to("true");
    // Multiple selection keeps the menu open.
    assert_that!(page.count(MENU).await?).is_equal_to(1);

    page.send_keys(Key::Escape).await?;
    expect_closed_with_focus_on(page, VIEW_TRIGGER).await?;
    Ok(())
}

/// A long-press trigger describes how to open its menu, performs its own action on a short press,
/// and opens the menu on a long press or Alt+ArrowDown ("should open the menu on longPress",
/// "should not open menu on short press (default threshold set to ${DEFAULT_LONG_PRESS_TIME}ms)",
/// "should focus the first item on Alt+ArrowDown if no selectedKeys specified").
#[browser_test]
pub async fn long_press_trigger(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let trigger = page.element(LONG_TRIGGER).await?;
    assert_that!(trigger)
        .accessible_description()
        .await
        .is_equal_to("Long press or press Alt + ArrowDown to open menu");

    // A press is the button's own action.
    trigger.click().await?;
    actions_log(page)
        .await?
        .wait_for_inner_text("More pressed")
        .await?;
    // Past the long-press delay (500 ms after the pointer down).
    page.settle().await?;
    assert_that!(|| page.count(MENU))
        .consistently_ok()
        .for_at_least(Duration::from_millis(700))
        .matches(eq(0))
        .await;

    // A long press opens the menu.
    let held = trigger.press_and_hold().await?;
    // A real timer: hold past the long-press delay (500 ms).
    tokio::time::sleep(Duration::from_millis(800)).await;
    held.release().await?;
    let trigger_id = trigger.id().await?;
    page.element(MENU)
        .await?
        .wait_for_attr("aria-labelledby", trigger_id.as_deref())
        .await?;
    page.send_keys(Key::Escape).await?;
    expect_closed_with_focus_on(page, LONG_TRIGGER).await?;

    // Alt+ArrowDown opens it with the first item focused.
    page.send_keys(Key::Alt + Key::Down).await?;
    page.wait_for_focus(&page.element(role(AriaRole::Menuitem).text("Copy")).await?)
        .await?;
    page.send_keys(Key::Escape).await?;
    expect_closed_with_focus_on(page, LONG_TRIGGER).await?;
    Ok(())
}

/// Two sections with their own selection (multiple, single) each change only their own selection,
/// and the arrow keys move through both as one menu ("should support section-level selection").
#[browser_test]
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
    assert_that!(
        page.element(role(AriaRole::Menuitemcheckbox).text("Lettuce"))
            .await?
    )
    .has_attribute("aria-checked")
    .await
    .is_equal_to("true");
    assert_that!(
        page.element(role(AriaRole::Menuitemcheckbox).text("Tomato"))
            .await?
    )
    .has_attribute("aria-checked")
    .await
    .is_equal_to("false");
    assert_that!(
        page.element(role(AriaRole::Menuitemradio).text("Ham"))
            .await?
    )
    .has_attribute("aria-checked")
    .await
    .is_equal_to("true");

    // Multiple selection in its section: toggles, the menu stays open.
    page.element(role(AriaRole::Menuitemcheckbox).text("Tomato"))
        .await?
        .click()
        .await?;
    veggies.wait_for_inner_text("Lettuce,Tomato").await?;
    assert_that!(page.count(MENU).await?).is_equal_to(1);
    assert_that!(protein).inner_text().await.is_equal_to("Ham");

    // Single selection in the other: replaces its selection only (and closes the menu).
    page.element(role(AriaRole::Menuitemradio).text("Tuna"))
        .await?
        .click()
        .await?;
    protein.wait_for_inner_text("Tuna").await?;
    assert_that!(veggies)
        .inner_text()
        .await
        .is_equal_to("Lettuce,Tomato");
    expect_closed_with_focus_on(page, SANDWICH_TRIGGER).await?;

    // The arrow keys move through both sections.
    page.send_keys(Key::Down).await?;
    page.wait_for_focus(
        &page
            .element(role(AriaRole::Menuitemcheckbox).text("Lettuce"))
            .await?,
    )
    .await?;
    for (item_role, text) in [
        (AriaRole::Menuitemcheckbox, "Tomato"),
        (AriaRole::Menuitemcheckbox, "Onion"),
        (AriaRole::Menuitemradio, "Ham"),
        (AriaRole::Menuitemradio, "Tuna"),
    ] {
        page.send_keys(Key::Down).await?;
        page.wait_for_focus(&page.element(role(item_role).text(text)).await?)
            .await?;
    }
    assert_that!(
        page.element(role(AriaRole::Menuitemradio).text("Tuna"))
            .await?
    )
    .has_attribute("aria-checked")
    .await
    .is_equal_to("true");
    page.send_keys(Key::Escape).await?;
    expect_closed_with_focus_on(page, SANDWICH_TRIGGER).await?;
    Ok(())
}

/// Items of a section or menu with `should_close_on_select` off keep the menu open when pressed,
/// other items close it ("should not close menu items within a section when
/// shouldCloseOnSelect=false", "should not close the menu when shouldCloseOnSelect is false").
#[browser_test]
pub async fn close_on_select(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let actions = actions_log(page).await?;
    page.element("#test-menu-atoms-file-trigger")
        .await?
        .click()
        .await?;
    page.element(MENU).await?;
    page.element(role(AriaRole::Menuitem).text("Open"))
        .await?
        .click()
        .await?;
    actions.wait_for_inner_text("Open").await?;
    page.count_stays(MENU, 1, std::time::Duration::from_millis(100))
        .await?;
    page.element(role(AriaRole::Menuitem).text("Share"))
        .await?
        .click()
        .await?;
    page.wait_for_count(MENU, 0).await?;

    page.element("#test-menu-atoms-edit-trigger")
        .await?
        .click()
        .await?;
    page.element(MENU).await?;
    page.element(role(AriaRole::Menuitem).text("Undo"))
        .await?
        .click()
        .await?;
    actions.wait_for_inner_text("Open,Share,Undo").await?;
    page.count_stays(MENU, 1, std::time::Duration::from_millis(100))
        .await?;
    page.send_keys(Key::Escape).await?;
    page.wait_for_count(MENU, 0).await?;
    Ok(())
}

const STATES_PATH: &str = "/atoms/menu-states";

/// The item `text` of the menu labelled `menu` on the menu states page.
async fn state_item(page: &Page<'_>, menu: &str, text: &str) -> Result<WebElement, Report> {
    page.element(format!("[role=menu][aria-label='{menu}']"))
        .await?
        .element(css("[role^=menuitem]").text(text))
        .await
}

/// Hovering an item marks it hovered until the pointer leaves ("should support hover").
#[browser_test]
pub async fn item_hover(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(STATES_PATH).await?;
    let copy = state_item(page, "Inline", "Copy").await?;
    assert_that!(copy).attribute("data-hovered").await.is_none();
    copy.hover().await?;
    copy.wait_for_attr("data-hovered", Some("true")).await?;
    page.element("h1").await?.hover().await?;
    copy.wait_for_attr("data-hovered", None).await?;
    Ok(())
}

/// A disabled item isn't hovered ("should not show hover state when item is not interactive").
#[browser_test]
pub async fn disabled_item_isnt_hovered(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(STATES_PATH).await?;
    let paste = state_item(page, "Inline", "Paste").await?;
    paste.hover().await?;
    page.settle().await?;
    paste
        .attr_stays("data-hovered", None, Duration::from_millis(100))
        .await?;
    Ok(())
}

/// Focusing an item by keyboard shows its focus ring until focus moves on ("should support focus
/// ring").
#[browser_test]
pub async fn item_focus_ring(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(STATES_PATH).await?;
    let copy = state_item(page, "Inline", "Copy").await?;
    assert_that!(copy)
        .attribute("data-focus-visible")
        .await
        .is_none();
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&copy).await?;
    copy.wait_for_attr("data-focus-visible", Some("true"))
        .await?;
    page.send_keys(Key::Down).await?;
    page.wait_for_focus(&state_item(page, "Inline", "Cut").await?)
        .await?;
    copy.wait_for_attr("data-focus-visible", None).await?;
    Ok(())
}

/// An item is pressed while the pointer is down on it ("should support press state").
#[browser_test]
pub async fn item_press_state(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(STATES_PATH).await?;
    let copy = state_item(page, "Inline", "Copy").await?;
    assert_that!(copy).attribute("data-pressed").await.is_none();
    let held = copy.press_and_hold().await?;
    copy.wait_for_attr("data-pressed", Some("true")).await?;
    held.release().await?;
    copy.wait_for_attr("data-pressed", None).await?;
    Ok(())
}

/// A disabled item is `aria-disabled` and `data-disabled` ("should support disabled state").
#[browser_test]
pub async fn item_disabled_state(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(STATES_PATH).await?;
    let paste = state_item(page, "Inline", "Paste").await?;
    assert_that!(paste)
        .has_attribute("aria-disabled")
        .await
        .is_equal_to("true");
    assert_that!(paste)
        .has_attribute("data-disabled")
        .await
        .is_equal_to("true");
    let copy = state_item(page, "Inline", "Copy").await?;
    assert_that!(copy)
        .attribute("data-disabled")
        .await
        .is_none();
    Ok(())
}

/// A menu without items is `data-empty` and shows its empty state in a menu item ("should support
/// empty state").
#[browser_test]
pub async fn empty_state(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(STATES_PATH).await?;
    let menu = page.element("[role=menu][aria-label='Empty']").await?;
    assert_that!(menu)
        .has_attribute("data-empty")
        .await
        .is_equal_to("true");
    assert_that!(menu.inner_texts(role(AriaRole::Menuitem)).await?)
        .contains_exactly(["No actions"]);
    let inline = page.element("[role=menu][aria-label='Inline']").await?;
    assert_that!(inline).attribute("data-empty").await.is_none();
    Ok(())
}

/// Without `should_focus_wrap`, the arrow keys stop at the first and last (enabled) item.
#[browser_test]
pub async fn arrow_keys_stop_at_the_ends(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(STATES_PATH).await?;
    let copy = state_item(page, "Inline", "Copy").await?;
    let cut = state_item(page, "Inline", "Cut").await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&copy).await?;
    page.send_keys(Key::Up).await?;
    page.settle().await?;
    page.focus_stays(&copy, Duration::from_millis(100)).await?;
    page.send_keys(Key::Down).await?;
    page.wait_for_focus(&cut).await?;
    // Paste is disabled: Cut is the last item focus can reach.
    page.send_keys(Key::Down).await?;
    page.settle().await?;
    page.focus_stays(&cut, Duration::from_millis(100)).await?;
    Ok(())
}

/// With `disallow_empty_selection`, pressing the only selected item keeps it selected.
#[browser_test]
pub async fn selection_cant_become_empty(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(STATES_PATH).await?;
    let selection = page.element("#test-menu-atoms-alignment").await?;
    let left = state_item(page, "Alignment", "Left").await?;
    assert_that!(left)
        .has_attribute("aria-checked")
        .await
        .is_equal_to("true");
    left.click().await?;
    page.settle().await?;
    selection
        .inner_text_stays("Left", Duration::from_millis(100))
        .await?;
    state_item(page, "Alignment", "Center")
        .await?
        .click()
        .await?;
    selection.wait_for_inner_text("Center").await?;
    left.wait_for_attr("aria-checked", Some("false")).await?;
    Ok(())
}

/// A section without a heading is a group named by its `aria-label` (useMenu: "labels a section
/// group via aria-label when the section has no heading").
#[browser_test]
pub async fn section_without_heading_is_labelled(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(STATES_PATH).await?;
    let group = page
        .element("[role=menu][aria-label='Tools menu'] [role=group]")
        .await?;
    assert_that!(group)
        .has_attribute("aria-label")
        .await
        .is_equal_to("Tools");
    assert_that!(group)
        .attribute("aria-labelledby")
        .await
        .is_none();
    assert_that!(group.count("header").await?).is_equal_to(0);
    Ok(())
}

/// Pressing the trigger, dragging onto an item and releasing there performs the item's action and
/// closes the menu, as native menus do ("should support press events on menu items when dragging
/// and releasing").
#[browser_test]
pub async fn press_drag_release_activates_an_item(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let trigger = page.element(ACTIONS_TRIGGER).await?;
    let held = trigger.press_and_hold().await?;
    let copy = page.element(role(AriaRole::Menuitem).text("Copy")).await?;
    held.move_to(&copy).await?;
    held.release().await?;
    actions_log(page).await?.wait_for_inner_text("Copy").await?;
    expect_closed_with_focus_on(page, ACTIONS_TRIGGER).await?;
    Ok(())
}

/// Tab doesn't move focus out of an open menu: the menu stays open with the focus where it was
/// ("contains focus within the menu").
#[browser_test]
pub async fn tab_keeps_focus_in_the_open_menu(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let trigger = page.element(ACTIONS_TRIGGER).await?;
    trigger.focus().await?;
    page.send_keys(Key::Down).await?;
    let copy = page.element(role(AriaRole::Menuitem).text("Copy")).await?;
    page.wait_for_focus(&copy).await?;
    page.send_keys(Key::Tab).await?;
    page.settle().await?;
    page.focus_stays(&copy, Duration::from_millis(100)).await?;
    assert_that!(page.count(MENU).await?).is_equal_to(1);
    Ok(())
}
