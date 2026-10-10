// Upstream: react-aria-components/test/Menu.test.tsx @ 99e6102368
// Upstream: @adobe/react-spectrum/test/menu/SubMenuTrigger.test.tsx @ 99e6102368
//! Submenus ("Submenus" in RAC's `Menu.test.tsx`): opening by hover and the arrow key, the trigger
//! item's ARIA attributes, actions in (nested) submenus closing the whole tree, ArrowLeft and
//! Escape returning to the trigger, focusing another item and interacting outside closing them.
use std::{sync::Arc, time::Duration};

use assertr::{matchers::eq, prelude::*};
use browser_test::{
    browser_test,
    thirtyfour::{action_chain::ActionChain, prelude::*},
};
use leptonic::AriaRole;
use rootcause::Report;

use crate::pages::{ElementActions, Page, role};

const PATH: &str = "/atoms/submenu";

const MENU: &str = "[role=menu]";

/// The menu item with the text `text`.
async fn item(page: &Page<'_>, text: &str) -> Result<WebElement, Report> {
    page.element(role(AriaRole::Menuitem).text(text)).await
}

/// The fixture's log of performed actions.
async fn actions_log(page: &Page<'_>) -> Result<WebElement, Report> {
    page.element("#test-submenu-actions").await
}

/// Opens the root menu with a click on its trigger.
async fn open_root(page: &Page<'_>) -> Result<(), Report> {
    page.element("#test-submenu-trigger").await?.click().await?;
    page.element(MENU).await?;
    Ok(())
}

/// Clicks the page outside the menus with the pointer (the modal root popover's underlay
/// catches it, so the element itself can't be clicked).
async fn click_outside(page: &Page<'_>, context: bool) -> Result<(), Report> {
    let outside = page.element("#test-submenu-before").await?;
    let chain = page
        .low_level()
        .driver()
        .action_chain()
        .move_to_element_center(&outside);
    if context {
        chain.context_click().perform().await?;
    } else {
        chain.click().perform().await?;
    }
    Ok(())
}

/// Hovering a submenu trigger item (`aria-haspopup="menu"`) opens a submenu it controls and names,
/// and an action in the submenu closes every menu ("should support a submenu trigger").
#[browser_test]
pub async fn supports_a_submenu_trigger(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    open_root(page).await?;
    let share = item(page, "Share…").await?;
    assert_that!(share)
        .has_attribute("aria-haspopup")
        .await
        .is_equal_to("menu");
    assert_that!(share)
        .has_attribute("aria-expanded")
        .await
        .is_equal_to("false");
    assert_that!(share)
        .has_attribute("data-has-submenu")
        .await
        .is_equal_to("true");
    assert_that!(share).attribute("data-open").await.is_none();

    share.hover().await?;
    share.wait_for_attr("aria-expanded", Some("true")).await?;
    assert_that!(share).has_attribute("data-open").await;
    page.wait_for_count(MENU, 2).await?;
    let menus = page.elements(MENU).await?;
    let submenu = &menus[1];
    let share_id = share.id().await?;
    assert_that!(submenu)
        .attribute("aria-labelledby")
        .await
        .is_equal_to(share_id);
    let mut triggers = Vec::new();
    for popover in page.elements(".test-popover").await? {
        triggers.push(popover.attr("data-trigger").await?.unwrap_or_default());
    }
    assert_that!(triggers).contains_exactly(["MenuTrigger", "SubmenuTrigger"]);
    let submenu_id = submenu.attr("id").await?;
    assert_that!(share)
        .attribute("aria-controls")
        .await
        .is_equal_to(submenu_id);

    item(page, "SMS").await?.click().await?;
    actions_log(page).await?.wait_for_inner_text("sms").await?;
    page.wait_for_count(MENU, 0).await?;
    Ok(())
}

/// Hovering a trigger inside a submenu opens a nested submenu, and an action in it closes every
/// menu ("should support nested submenu triggers").
#[browser_test]
pub async fn supports_nested_submenu_triggers(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    open_root(page).await?;
    let share = item(page, "Share…").await?;
    share.hover().await?;
    share.wait_for_attr("aria-expanded", Some("true")).await?;
    let email = item(page, "Email…").await?;
    email.hover().await?;
    email.wait_for_attr("aria-expanded", Some("true")).await?;

    item(page, "Work").await?.click().await?;
    actions_log(page).await?.wait_for_inner_text("work").await?;
    page.wait_for_count(MENU, 0).await?;
    Ok(())
}

/// ArrowRight and Enter open a submenu focusing its first item; ArrowLeft and Escape close only the
/// innermost submenu and return focus to its trigger ("should restore focus to menu trigger if
/// submenu is closed with Escape key", "should restore focus to nested submenu trigger if nested
/// submenu is closed with Escape key").
#[browser_test]
pub async fn keyboard(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let trigger = page.element("#test-submenu-trigger").await?;
    trigger.focus().await?;
    page.send_keys(Key::Enter).await?;
    page.wait_for_focus(&item(page, "Open").await?).await?;
    page.send_keys(Key::Down).await?;
    page.send_keys(Key::Down).await?;
    page.wait_for_focus(&item(page, "Share…").await?).await?;

    page.send_keys(Key::Right).await?;
    page.wait_for_focus(&item(page, "Email…").await?).await?;
    page.send_keys(Key::Left).await?;
    page.wait_for_focus(&item(page, "Share…").await?).await?;
    let share = item(page, "Share…").await?;
    share.wait_for_attr("aria-expanded", Some("false")).await?;

    // Enter on the trigger opens the submenu too; Escape closes only the submenu.
    page.send_keys(Key::Enter).await?;
    page.wait_for_focus(&item(page, "Email…").await?).await?;
    page.send_keys(Key::Escape).await?;
    page.wait_for_focus(&item(page, "Share…").await?).await?;
    share.wait_for_attr("aria-expanded", Some("false")).await?;
    // The submenu is gone once its exit animation ran.
    page.wait_for_count(MENU, 1).await?;

    page.send_keys(Key::Right).await?;
    page.wait_for_focus(&item(page, "Email…").await?).await?;
    page.send_keys(Key::Right).await?;
    page.wait_for_focus(&item(page, "Work").await?).await?;
    page.send_keys(Key::Escape).await?;
    page.wait_for_focus(&item(page, "Email…").await?).await?;
    page.send_keys(Key::Escape).await?;
    page.wait_for_focus(&item(page, "Share…").await?).await?;

    // Escape in the root menu closes it.
    page.send_keys(Key::Escape).await?;
    page.wait_for_count(MENU, 0).await?;
    page.wait_for_focus(&trigger).await?;
    Ok(())
}

/// Focusing (hovering) another item of the menu closes the open submenu.
#[browser_test]
pub async fn focusing_another_item_closes_the_submenu(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    open_root(page).await?;
    let share = item(page, "Share…").await?;
    share.hover().await?;
    share.wait_for_attr("aria-expanded", Some("true")).await?;
    item(page, "Rename…").await?.hover().await?;
    share.wait_for_attr("aria-expanded", Some("false")).await?;
    page.send_keys(Key::Escape).await?;
    page.wait_for_count(MENU, 0).await?;
    Ok(())
}

/// Clicking outside closes the root menu and all its open submenus ("should close all submenus if
/// interacting outside root submenu").
#[browser_test]
pub async fn interacting_outside_closes_all(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    open_root(page).await?;
    let share = item(page, "Share…").await?;
    share.hover().await?;
    share.wait_for_attr("aria-expanded", Some("true")).await?;
    let email = item(page, "Email…").await?;
    email.hover().await?;
    email.wait_for_attr("aria-expanded", Some("true")).await?;
    click_outside(page, false).await?;
    page.wait_for_count(MENU, 0).await?;
    Ok(())
}

/// A right click on a context menu trigger opens the menu at the pointer (a regular press
/// doesn't), and a right click outside closes it ("should support a context menu trigger",
/// "should close a context menu when right clicking outside").
#[browser_test]
pub async fn context_menu(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let trigger = page.element("#test-context-trigger").await?;
    // Not announced as opening a menu.
    assert_that!(trigger)
        .attribute("aria-haspopup")
        .await
        .is_none();
    assert_that!(trigger)
        .attribute("aria-expanded")
        .await
        .is_none();
    trigger.click().await?;
    page.count_stays(MENU, 0, std::time::Duration::from_millis(100))
        .await?;

    // A right click 10px right and 15px below the trigger's corner.
    let rect = trigger.client_rect().await?;
    // The action's offset is from the element's center, in whole pixels.
    #[allow(clippy::cast_possible_truncation)]
    let from_center = |size: f64, by: f64| (by - size / 2.0).round() as i64;
    page.low_level()
        .driver()
        .action_chain()
        .move_to_element_with_offset(
            &trigger,
            from_center(rect.width, 10.0),
            from_center(rect.height, 15.0),
        )
        .context_click()
        .perform()
        .await?;
    page.element(MENU).await?;
    let popover = page.element(".test-popover").await?;
    // Where it settles once its entry animation (a slide) ran.
    assert_that!(|| popover.client_rect())
        .eventually_ok()
        .satisfies(|popover_rect| {
            // At the pointer (±2px).
            popover_rect
                .derive(|popover_rect| &popover_rect.left)
                .is_close_to(rect.left + 10.0, 2.0);
            popover_rect
                .derive(|popover_rect| &popover_rect.top)
                .is_close_to(rect.top + 15.0, 2.0);
        })
        .await;
    assert_that!(trigger)
        .attribute("aria-expanded")
        .await
        .is_none();

    item(page, "Paste").await?.click().await?;
    actions_log(page)
        .await?
        .wait_for_inner_text("paste")
        .await?;
    page.wait_for_count(MENU, 0).await?;

    // Right clicking inside keeps it open; outside closes it.
    page.low_level()
        .driver()
        .action_chain()
        .move_to_element_center(&trigger)
        .context_click()
        .perform()
        .await?;
    let menu = page.element(MENU).await?;
    page.low_level()
        .driver()
        .action_chain()
        .move_to_element_center(&menu)
        .context_click()
        .perform()
        .await?;
    page.count_stays(MENU, 1, std::time::Duration::from_millis(100))
        .await?;
    click_outside(page, true).await?;
    page.wait_for_count(MENU, 0).await?;
    Ok(())
}

/// Hovering a trigger item with `aria-haspopup="dialog"` opens a popover dialog that contains
/// focus, and Escape closes it and returns focus to the item ("should contain focus for
/// subdialogs").
#[browser_test]
pub async fn subdialog(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    open_root(page).await?;
    let signup = item(page, "Sign up…").await?;
    assert_that!(signup)
        .has_attribute("aria-haspopup")
        .await
        .is_equal_to("dialog");
    signup.hover().await?;
    signup.wait_for_attr("aria-expanded", Some("true")).await?;
    page.wait_for_count(".test-popover[role=dialog]", 2).await?;

    let first = page.element("#test-signup-first").await?;
    let last = page.element("#test-signup-last").await?;
    first.click().await?;
    page.wait_for_focus(&first).await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&last).await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&first).await?;

    page.send_keys(Key::Escape).await?;
    signup.wait_for_attr("aria-expanded", Some("false")).await?;
    page.wait_for_focus(&signup).await?;
    page.send_keys(Key::Escape).await?;
    page.wait_for_count(MENU, 0).await?;
    Ok(())
}

/// ArrowRight on a trigger item whose popover holds an untitled `Dialog` focuses the dialog, which
/// the item names, and Escape (also from an input inside) closes it and returns focus to the item
/// while the menu stays open.
#[browser_test]
pub async fn subdialog_with_dialog(page: &Page<'_>) -> Result<(), Report> {
    const DIALOG: &str = ".leptonic-Dialog[role=dialog]";
    page.goto_path(PATH).await?;
    page.element("#test-submenu-trigger").await?.focus().await?;
    page.send_keys(Key::Enter).await?;
    page.wait_for_focus(&item(page, "Open").await?).await?;
    for _ in 0..4 {
        page.send_keys(Key::Down).await?;
    }
    let properties = item(page, "Properties…").await?;
    page.wait_for_focus(&properties).await?;
    page.send_keys(Key::Right).await?;
    let dialog = page.element(DIALOG).await?;
    page.wait_for_focus(&dialog).await?;
    let properties_id = properties.id().await?;
    assert_that!(dialog)
        .attribute("aria-labelledby")
        .await
        .is_equal_to(properties_id);
    page.send_keys(Key::Escape).await?;
    page.wait_for_count(DIALOG, 0).await?;
    page.wait_for_focus(&properties).await?;

    // Again, from the input inside.
    page.send_keys(Key::Right).await?;
    let input = page.element("#test-properties-input").await?;
    input.click().await?;
    page.wait_for_focus(&input).await?;
    page.send_keys(Key::Escape).await?;
    page.wait_for_count(DIALOG, 0).await?;
    page.wait_for_focus(&properties).await?;
    page.send_keys(Key::Escape).await?;
    page.wait_for_count(MENU, 0).await?;
    Ok(())
}

/// In a right-to-left menu, ArrowLeft opens a submenu and ArrowRight closes it, returning focus to
/// its trigger (ArrowRight on the trigger doesn't open it).
#[browser_test]
pub async fn right_to_left(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let trigger = page.element("#test-submenu-rtl-trigger").await?;
    trigger.click().await?;
    page.element(MENU).await?;
    page.send_keys(Key::Escape).await?;
    page.wait_for_count(MENU, 0).await?;
    page.wait_for_focus(&trigger).await?;

    page.send_keys(Key::Down).await?;
    page.wait_for_focus(&item(page, "Open (RTL)").await?)
        .await?;
    page.send_keys(Key::Down).await?;
    let share = item(page, "Share (RTL)").await?;
    page.wait_for_focus(&share).await?;
    // ArrowRight is "back" in right-to-left text: it doesn't open the submenu.
    page.send_keys(Key::Right).await?;
    share
        .attr_stays(
            "aria-expanded",
            Some("false"),
            std::time::Duration::from_millis(100),
        )
        .await?;
    page.send_keys(Key::Left).await?;
    page.wait_for_focus(&item(page, "Email (RTL)").await?)
        .await?;
    page.send_keys(Key::Right).await?;
    page.wait_for_focus(&share).await?;
    share.wait_for_attr("aria-expanded", Some("false")).await?;
    page.send_keys(Key::Escape).await?;
    page.wait_for_count(MENU, 0).await?;
    page.wait_for_focus(&trigger).await?;
    Ok(())
}

/// While the pointer moves diagonally across other items towards an open submenu, the root menu
/// ignores pointer events so the submenu stays open, and at rest it takes them again
/// (`useSafelyMouseToSubmenu`).
#[browser_test]
pub async fn safe_triangle(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    // Opened by a click: the hook only works for pointer modality.
    open_root(page).await?;
    let share = item(page, "Share…").await?;
    share.hover().await?;
    share.wait_for_attr("aria-expanded", Some("true")).await?;
    page.wait_for_count(MENU, 2).await?;
    let menus = page.elements(MENU).await?;
    let root = &menus[0];
    // From the trigger's center towards the submenu, below it (across "Sign up…"), in moves the
    // hook doesn't throttle (one per 50 ms) and small enough that the first two stay on the
    // trigger (the hook judges the direction from the second processed move on), inside the
    // root menu.
    let share_rect = share.client_rect().await?;
    let share_x = share_rect.left + share_rect.width / 2.0;
    let menu_right = root.client_rect().await?.right;
    let submenu_left = menus[1].client_rect().await?.left;
    let reach = (menu_right.min(submenu_left) - share_x - 4.0).max(12.0);
    #[allow(clippy::cast_possible_truncation)]
    let step_x = (reach / 8.0) as i64;
    let styles = root.record_attr("style").await?;
    // One gesture, paced by WebDriver: each move takes 60 ms (the hook processes one move per
    // 50 ms), however long WebDriver commands take.
    let mut gesture = ActionChain::new_with_delay(
        Arc::clone(&**page.low_level().driver()),
        None,
        Some(Duration::from_millis(60)),
    );
    for _ in 0..8 {
        gesture = gesture.move_by_offset(step_x.max(1), 4);
    }
    gesture.perform().await?;
    // The root menu ignored pointer events on the way.
    let ignoring: Vec<String> = styles
        .finish()
        .await?
        .into_iter()
        .flatten()
        .filter(|style| style.contains("pointer-events: none"))
        .collect();
    assert_that!(ignoring).is_not_empty();
    assert_that!(share)
        .has_attribute("aria-expanded")
        .await
        .is_equal_to("true");
    // At rest, the menu takes pointer events again.
    assert_that!(|| root.css_value("pointer-events"))
        .eventually_ok()
        .matches(eq("auto"))
        .await;
    page.send_keys(Key::Escape).await?;
    page.send_keys(Key::Escape).await?;
    page.wait_for_count(MENU, 0).await?;
    Ok(())
}

/// Moving the pointer from the open submenu back onto its trigger keeps the submenu open and the
/// focus in it ("should not close the sub menu if user hovers onto the sub menu trigger from the
/// sub menu", @adobe/react-spectrum `SubMenuTrigger.test.tsx`).
#[browser_test]
pub async fn hovering_back_onto_the_trigger_keeps_the_submenu(
    page: &Page<'_>,
) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    open_root(page).await?;
    let share = item(page, "Share…").await?;
    share.hover().await?;
    share.wait_for_attr("aria-expanded", Some("true")).await?;
    let sms = item(page, "SMS").await?;
    sms.hover().await?;
    page.wait_for_focus(&sms).await?;
    share.hover().await?;
    page.settle().await?;
    page.focus_stays(&sms, Duration::from_millis(300)).await?;
    page.count_stays(MENU, 2, Duration::from_millis(100))
        .await?;
    Ok(())
}

/// Clicking a nested submenu's trigger inside a submenu keeps the menu tree open (it opens the
/// nested submenu), and an action in it closes every menu ("should not close the menu when
/// clicking on a element within the submenu tree").
#[browser_test]
pub async fn clicking_inside_the_submenu_tree_keeps_it_open(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    open_root(page).await?;
    let share = item(page, "Share…").await?;
    share.hover().await?;
    share.wait_for_attr("aria-expanded", Some("true")).await?;
    item(page, "Email…").await?.click().await?;
    page.wait_for_count(MENU, 3).await?;
    page.settle().await?;
    page.count_stays(MENU, 3, Duration::from_millis(100))
        .await?;
    item(page, "Work").await?.click().await?;
    actions_log(page).await?.wait_for_inner_text("work").await?;
    page.wait_for_count(MENU, 0).await?;
    Ok(())
}

/// A submenu's sections are groups labelled by their headings, and an action in them closes every
/// menu ("should support sections").
#[browser_test]
pub async fn submenu_sections(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element("#test-submenu-tools-trigger")
        .await?
        .click()
        .await?;
    let arrange = item(page, "Arrange…").await?;
    arrange.hover().await?;
    arrange.wait_for_attr("aria-expanded", Some("true")).await?;
    page.wait_for_count(MENU, 2).await?;
    let submenu = &page.elements(MENU).await?[1];
    let groups = submenu.elements(role(AriaRole::Group)).await?;
    let mut headings = Vec::new();
    for group in &groups {
        let heading_id = group.attr("aria-labelledby").await?.unwrap_or_default();
        headings.push(
            page.element(format!("#{heading_id}"))
                .await?
                .inner_text()
                .await?,
        );
    }
    assert_that!(headings).contains_exactly(["Align", "Order"]);
    assert_that!(submenu.inner_texts(role(AriaRole::Menuitem)).await?)
        .contains_exactly(["Left", "Right", "Front", "Back"]);
    item(page, "Front").await?.click().await?;
    actions_log(page)
        .await?
        .wait_for_inner_text("front")
        .await?;
    page.wait_for_count(MENU, 0).await?;
    Ok(())
}

/// Opens Tools › Contact… › Nested subdialog by keyboard: three dialogs (the root popover and the
/// two submenu popovers).
async fn open_nested_subdialog(page: &Page<'_>) -> Result<(WebElement, WebElement), Report> {
    let trigger = page.element("#test-submenu-tools-trigger").await?;
    trigger.focus().await?;
    page.send_keys(Key::Down).await?;
    page.wait_for_focus(&item(page, "Arrange…").await?).await?;
    page.send_keys(Key::Down).await?;
    let contact = item(page, "Contact…").await?;
    page.wait_for_focus(&contact).await?;
    page.send_keys(Key::Right).await?;
    let nested = item(page, "Nested subdialog").await?;
    page.wait_for_focus(&nested).await?;
    page.send_keys(Key::Right).await?;
    page.element("#test-contact-email").await?;
    page.wait_for_count("[role=dialog]", 3).await?;
    Ok((contact, nested))
}

/// Escape closes the innermost of nested subdialogs, returning focus to its trigger, then the next
/// one ("should support nested subdialogs").
#[browser_test]
pub async fn nested_subdialogs(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let (contact, nested) = open_nested_subdialog(page).await?;
    page.send_keys(Key::Escape).await?;
    page.wait_for_count("[role=dialog]", 2).await?;
    page.wait_for_focus(&nested).await?;
    page.send_keys(Key::Escape).await?;
    page.wait_for_count("[role=dialog]", 1).await?;
    page.wait_for_focus(&contact).await?;
    Ok(())
}

/// Interacting outside the root menu closes every subdialog and the menu ("should close all
/// subdialogs if interacting outside the root menu").
#[browser_test]
pub async fn interacting_outside_closes_all_subdialogs(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    open_nested_subdialog(page).await?;
    click_outside(page, false).await?;
    page.wait_for_count("[role=dialog]", 0).await?;
    page.wait_for_count(MENU, 0).await?;
    Ok(())
}
