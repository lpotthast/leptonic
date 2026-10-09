// Upstream: react-aria-components/test/Menu.test.tsx @ 99e6102368
//! Submenus ("Submenus" in RAC's `Menu.test.tsx`): opening by hover and the arrow key, the trigger
//! item's ARIA attributes, actions in (nested) submenus closing the whole tree, ArrowLeft and
//! Escape returning to the trigger, focusing another item and interacting outside closing them.
use assertr::{matchers::eq, prelude::*};
use browser_test::thirtyfour::prelude::*;
use rootcause::Report;

use crate::pages::{ElementActions, Page, PageActions, role};

const PATH: &str = "/atoms/submenu";

const MENU: &str = "[role=menu]";

/// The menu item with the text `text`.
async fn item(page: &Page<'_>, text: &str) -> Result<WebElement, Report> {
    page.element(role("menuitem").text(text)).await
}

/// Wait until the menu item `text` has focus.
async fn expect_focus(page: &Page<'_>, text: &str) -> Result<(), Report> {
    page.wait_for_focus(&item(page, text).await?).await
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
    let chain = page.driver.action_chain().move_to_element_center(&outside);
    if context {
        chain.context_click().perform().await?;
    } else {
        chain.click().perform().await?;
    }
    Ok(())
}

/// "should support a submenu trigger": the trigger item announces and controls its submenu,
/// which it names; hovering opens it; an action closes every menu.
pub async fn supports_a_submenu_trigger(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    open_root(page).await?;
    let share = item(page, "Share…").await?;
    assert_that!(share.attr("aria-haspopup").await?)
        .get_some()
        .is_equal_to("menu");
    assert_that!(share.attr("aria-expanded").await?)
        .get_some()
        .is_equal_to("false");
    assert_that!(share.attr("data-has-submenu").await?)
        .get_some()
        .is_equal_to("true");
    assert_that!(share.attr("data-open").await?).is_none();

    share.hover().await?;
    share.wait_for_attr("aria-expanded", Some("true")).await?;
    assert_that!(share.attr("data-open").await?).is_some();
    page.wait_for_count(MENU, 2).await?;
    let menus = page.elements(MENU).await?;
    let submenu = &menus[1];
    let share_id = share.id().await?;
    assert_that!(submenu.attr("aria-labelledby").await?).is_equal_to(share_id);
    let mut triggers = Vec::new();
    for popover in page.elements(".test-popover").await? {
        triggers.push(popover.attr("data-trigger").await?.unwrap_or_default());
    }
    assert_that!(triggers).contains_exactly(["MenuTrigger", "SubmenuTrigger"]);
    let submenu_id = submenu.attr("id").await?;
    assert_that!(share.attr("aria-controls").await?).is_equal_to(submenu_id);

    item(page, "SMS").await?.click().await?;
    actions_log(page).await?.wait_for_inner_text("sms").await?;
    page.wait_for_count(MENU, 0).await?;
    Ok(())
}

/// "should support nested submenu triggers".
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

/// ArrowRight opens the submenu focusing its first item, ArrowLeft and Escape close it returning
/// focus to the trigger ("should restore focus to menu trigger if submenu is closed with Escape",
/// "should restore focus to nested submenu trigger if nested submenu is closed with Escape key").
pub async fn keyboard(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let trigger = page.element("#test-submenu-trigger").await?;
    trigger.focus().await?;
    page.send_keys(Key::Enter).await?;
    expect_focus(page, "Open").await?;
    page.send_keys(Key::Down).await?;
    page.send_keys(Key::Down).await?;
    expect_focus(page, "Share…").await?;

    page.send_keys(Key::Right).await?;
    expect_focus(page, "Email…").await?;
    page.send_keys(Key::Left).await?;
    expect_focus(page, "Share…").await?;
    let share = item(page, "Share…").await?;
    share.wait_for_attr("aria-expanded", Some("false")).await?;

    // Enter on the trigger opens the submenu too; Escape closes only the submenu.
    page.send_keys(Key::Enter).await?;
    expect_focus(page, "Email…").await?;
    page.send_keys(Key::Escape).await?;
    expect_focus(page, "Share…").await?;
    share.wait_for_attr("aria-expanded", Some("false")).await?;
    // The submenu is gone once its exit animation ran.
    page.wait_for_count(MENU, 1).await?;

    page.send_keys(Key::Right).await?;
    expect_focus(page, "Email…").await?;
    page.send_keys(Key::Right).await?;
    expect_focus(page, "Work").await?;
    page.send_keys(Key::Escape).await?;
    expect_focus(page, "Email…").await?;
    page.send_keys(Key::Escape).await?;
    expect_focus(page, "Share…").await?;

    // Escape in the root menu closes it.
    page.send_keys(Key::Escape).await?;
    page.wait_for_count(MENU, 0).await?;
    page.wait_for_focus(&trigger).await?;
    Ok(())
}

/// Focusing (hovering) another item of the menu closes the open submenu.
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

/// "should close all submenus if interacting outside root submenu".
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

/// "should support a context menu trigger" (a right click opens the menu at the pointer, a
/// regular press doesn't) and "should close a context menu when right clicking outside".
pub async fn context_menu(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let trigger = page.element("#test-context-trigger").await?;
    // Not announced as opening a menu.
    assert_that!(trigger.attr("aria-haspopup").await?).is_none();
    assert_that!(trigger.attr("aria-expanded").await?).is_none();
    trigger.click().await?;
    page.count_stays(MENU, 0).await?;

    // A right click 10px right and 15px below the trigger's corner.
    let rect = trigger.client_rect().await?;
    // The action's offset is from the element's center, in whole pixels.
    #[allow(clippy::cast_possible_truncation)]
    let from_center = |size: f64, by: f64| (by - size / 2.0).round() as i64;
    page.driver
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
    assert_that!(trigger.attr("aria-expanded").await?).is_none();

    item(page, "Paste").await?.click().await?;
    actions_log(page)
        .await?
        .wait_for_inner_text("paste")
        .await?;
    page.wait_for_count(MENU, 0).await?;

    // Right clicking inside keeps it open; outside closes it.
    page.driver
        .action_chain()
        .move_to_element_center(&trigger)
        .context_click()
        .perform()
        .await?;
    let menu = page.element(MENU).await?;
    page.driver
        .action_chain()
        .move_to_element_center(&menu)
        .context_click()
        .perform()
        .await?;
    page.count_stays(MENU, 1).await?;
    click_outside(page, true).await?;
    page.wait_for_count(MENU, 0).await?;
    Ok(())
}

/// "should contain focus for subdialogs": a submenu trigger opening a dialog; its popover is the
/// dialog and contains focus; Escape closes it, returning focus to the trigger item.
pub async fn subdialog(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    open_root(page).await?;
    let signup = item(page, "Sign up…").await?;
    assert_that!(signup.attr("aria-haspopup").await?)
        .get_some()
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

/// A subdialog whose popover holds a `Dialog`: opened by keyboard, focus moves into it; Escape
/// closes it and returns focus to the trigger item, the menu stays open.
pub async fn subdialog_with_dialog(page: &Page<'_>) -> Result<(), Report> {
    const DIALOG: &str = "[role=dialog][aria-label=Properties]";
    page.goto_path(PATH).await?;
    page.element("#test-submenu-trigger").await?.focus().await?;
    page.send_keys(Key::Enter).await?;
    expect_focus(page, "Open").await?;
    for _ in 0..4 {
        page.send_keys(Key::Down).await?;
    }
    let properties = item(page, "Properties…").await?;
    page.wait_for_focus(&properties).await?;
    page.send_keys(Key::Right).await?;
    page.wait_for_focus(&page.element(DIALOG).await?).await?;
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

/// Right-to-left ("should open/close submenu with ArrowLeft/ArrowRight in RTL", useSubmenuTrigger):
/// the trigger opens the menu by click and by ArrowDown; ArrowLeft opens a submenu, ArrowRight
/// returns to its trigger.
pub async fn right_to_left(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let trigger = page.element("#test-submenu-rtl-trigger").await?;
    trigger.click().await?;
    page.element(MENU).await?;
    page.send_keys(Key::Escape).await?;
    page.wait_for_count(MENU, 0).await?;
    page.wait_for_focus(&trigger).await?;

    page.send_keys(Key::Down).await?;
    expect_focus(page, "Open (RTL)").await?;
    page.send_keys(Key::Down).await?;
    let share = item(page, "Share (RTL)").await?;
    page.wait_for_focus(&share).await?;
    // ArrowRight is "back" in right-to-left text: it doesn't open the submenu.
    page.send_keys(Key::Right).await?;
    share.attr_stays("aria-expanded", Some("false")).await?;
    page.send_keys(Key::Left).await?;
    expect_focus(page, "Email (RTL)").await?;
    page.send_keys(Key::Right).await?;
    page.wait_for_focus(&share).await?;
    share.wait_for_attr("aria-expanded", Some("false")).await?;
    page.send_keys(Key::Escape).await?;
    page.wait_for_count(MENU, 0).await?;
    page.wait_for_focus(&trigger).await?;
    Ok(())
}

/// The pointer's way to an open submenu (`useSafelyMouseToSubmenu`): moving diagonally towards
/// it across other items, the root menu ignores pointer events, so the submenu stays open; at rest,
/// the menu takes them again.
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
    let mut seen_none = false;
    for _ in 0..8 {
        page.driver
            .action_chain()
            .move_by_offset(step_x.max(1), 4)
            .perform()
            .await?;
        // Real time between the moves: the hook processes one move per 50 ms.
        tokio::time::sleep(std::time::Duration::from_millis(60)).await;
        seen_none |= root.css_value("pointer-events").await? == "none";
    }
    assert_that!(seen_none)
        .with_detail_message("the root menu ignored pointer events on the way")
        .is_true();
    assert_that!(share.attr("aria-expanded").await?)
        .get_some()
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
