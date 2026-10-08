// Upstream: react-aria-components/test/Menu.test.tsx @ 99e6102368
use std::borrow::Cow;

use assertr::prelude::*;
use browser_test::{
    BrowserTest, async_trait,
    thirtyfour::{Key, WebDriver, WebElement},
};
use rootcause::Report;

use crate::pages::{BaseActions, Page};

/// Submenus ("Submenus" in RAC's `Menu.test.tsx`): opening by hover and the arrow key, the trigger
/// item's ARIA attributes, actions in (nested) submenus closing the whole tree, ArrowLeft and
/// Escape returning to the trigger, focusing another item and interacting outside closing them.
pub struct SubmenuTests {}

async fn item(page: &Page<'_>, text: &str) -> Result<WebElement, Report> {
    page.by_role_and_text("menuitem", text).await
}

async fn hover(page: &Page<'_>, element: &WebElement) -> Result<(), Report> {
    page.driver
        .action_chain()
        .move_to_element_center(element)
        .perform()
        .await?;
    Ok(())
}

async fn open_root(page: &Page<'_>) -> Result<(), Report> {
    page.element("test-submenu-trigger").await?.click().await?;
    page.wait_for_selector("[role=menu]").await
}

#[async_trait]
impl BrowserTest<str> for SubmenuTests {
    fn name(&self) -> Cow<'_, str> {
        "submenu_tests".into()
    }

    async fn run(&self, driver: &WebDriver, base_url: &str) -> Result<(), Report> {
        let page = Page { driver, base_url };
        page.goto_path("/atoms/submenu").await?;

        supports_a_submenu_trigger(&page).await?;
        supports_nested_submenu_triggers(&page).await?;
        keyboard(&page).await?;
        focusing_another_item_closes_the_submenu(&page).await?;
        interacting_outside_closes_all(&page).await?;
        context_menu(&page).await?;
        subdialog(&page).await?;
        subdialog_with_dialog(&page).await?;
        right_to_left(&page).await?;
        safe_triangle(&page).await?;

        page.expect_no_page_errors().await
    }
}

/// "should support a submenu trigger".
async fn supports_a_submenu_trigger(page: &Page<'_>) -> Result<(), Report> {
    open_root(page).await?;
    let share = item(page, "Share…").await?;
    assert_that!(share.attr("aria-haspopup").await?).is_equal_to(Some("menu".to_owned()));
    assert_that!(share.attr("aria-expanded").await?).is_equal_to(Some("false".to_owned()));
    assert_that!(share.attr("data-has-submenu").await?).is_equal_to(Some("true".to_owned()));
    assert_that!(share.attr("data-open").await?).is_none();

    hover(page, &share).await?;
    page.wait_for_attr(&share, "aria-expanded", Some("true"))
        .await?;
    assert_that!(share.attr("data-open").await?).is_some();
    let menus = page
        .driver
        .find_all(browser_test::thirtyfour::By::Css("[role=menu]"))
        .await?;
    assert_that!(menus.len()).is_equal_to(2);
    // The submenu is named by its trigger and controlled by it.
    let submenu = &menus[1];
    let share_id = share.attr("id").await?.unwrap_or_default();
    assert_that!(submenu.attr("aria-labelledby").await?).is_equal_to(Some(share_id));
    let popovers = page
        .driver
        .find_all(browser_test::thirtyfour::By::Css(".test-popover"))
        .await?;
    assert_that!(popovers[0].attr("data-trigger").await?)
        .is_equal_to(Some("MenuTrigger".to_owned()));
    assert_that!(popovers[1].attr("data-trigger").await?)
        .is_equal_to(Some("SubmenuTrigger".to_owned()));
    let submenu_id = submenu.attr("id").await?;
    assert_that!(share.attr("aria-controls").await?).is_equal_to(submenu_id);

    item(page, "SMS").await?.click().await?;
    page.wait_for_text("test-submenu-actions", "sms").await?;
    page.wait_for_no_selector("[role=menu]").await
}

/// "should support nested submenu triggers".
async fn supports_nested_submenu_triggers(page: &Page<'_>) -> Result<(), Report> {
    open_root(page).await?;
    let share = item(page, "Share…").await?;
    hover(page, &share).await?;
    page.wait_for_attr(&share, "aria-expanded", Some("true"))
        .await?;
    let email = item(page, "Email…").await?;
    hover(page, &email).await?;
    page.wait_for_attr(&email, "aria-expanded", Some("true"))
        .await?;

    item(page, "Work").await?.click().await?;
    page.wait_for_text("test-submenu-actions", "sms, work")
        .await?;
    page.wait_for_no_selector("[role=menu]").await
}

/// ArrowRight opens the submenu focusing its first item, ArrowLeft and Escape close it returning
/// focus to the trigger ("should restore focus to menu trigger if submenu is closed with Escape").
async fn keyboard(page: &Page<'_>) -> Result<(), Report> {
    let trigger = page.element("test-submenu-trigger").await?;
    trigger.focus().await?;
    page.send_keys_to_active(Key::Enter).await?;
    page.wait_for_active_text("Open").await?;
    page.send_keys_to_active(Key::Down).await?;
    page.send_keys_to_active(Key::Down).await?;
    page.wait_for_active_text("Share…").await?;

    page.send_keys_to_active(Key::Right).await?;
    page.wait_for_active_text("Email…").await?;
    page.send_keys_to_active(Key::Left).await?;
    page.wait_for_active_text("Share…").await?;
    let share = item(page, "Share…").await?;
    page.wait_for_attr(&share, "aria-expanded", Some("false"))
        .await?;

    // Enter on the trigger opens the submenu too; Escape closes only the submenu.
    page.send_keys_to_active(Key::Enter).await?;
    page.wait_for_active_text("Email…").await?;
    page.send_keys_to_active(Key::Escape).await?;
    page.wait_for_active_text("Share…").await?;
    page.wait_for_attr(&share, "aria-expanded", Some("false"))
        .await?;
    // The submenu is gone once its exit animation ran.
    page.wait_for_count("[role=menu]", 1).await?;

    // Nested: Escape in the nested submenu returns to its trigger ("should restore focus to
    // nested submenu trigger if nested submenu is closed with Escape key").
    page.send_keys_to_active(Key::Right).await?;
    page.wait_for_active_text("Email…").await?;
    page.send_keys_to_active(Key::Right).await?;
    page.wait_for_active_text("Work").await?;
    page.send_keys_to_active(Key::Escape).await?;
    page.wait_for_active_text("Email…").await?;
    page.send_keys_to_active(Key::Escape).await?;
    page.wait_for_active_text("Share…").await?;

    // Escape in the root menu closes it.
    page.send_keys_to_active(Key::Escape).await?;
    page.wait_for_no_selector("[role=menu]").await?;
    page.wait_for_focus_on(&trigger, "menu trigger").await
}

/// Focusing (hovering) another item of the menu closes the open submenu.
async fn focusing_another_item_closes_the_submenu(page: &Page<'_>) -> Result<(), Report> {
    open_root(page).await?;
    let share = item(page, "Share…").await?;
    hover(page, &share).await?;
    page.wait_for_attr(&share, "aria-expanded", Some("true"))
        .await?;
    let rename = item(page, "Rename…").await?;
    hover(page, &rename).await?;
    page.wait_for_attr(&share, "aria-expanded", Some("false"))
        .await?;
    page.send_keys_to_active(Key::Escape).await?;
    page.wait_for_no_selector("[role=menu]").await
}

/// "should close all submenus if interacting outside root submenu".
async fn interacting_outside_closes_all(page: &Page<'_>) -> Result<(), Report> {
    open_root(page).await?;
    let share = item(page, "Share…").await?;
    hover(page, &share).await?;
    page.wait_for_attr(&share, "aria-expanded", Some("true"))
        .await?;
    let email = item(page, "Email…").await?;
    hover(page, &email).await?;
    page.wait_for_attr(&email, "aria-expanded", Some("true"))
        .await?;

    // A click on the page (the modal root popover's underlay catches it).
    let outside = page.element("test-submenu-before").await?;
    page.driver
        .action_chain()
        .move_to_element_center(&outside)
        .click()
        .perform()
        .await?;
    page.wait_for_no_selector("[role=menu]").await
}

/// "should support a context menu trigger" and "should close a context menu when right clicking
/// outside".
async fn context_menu(page: &Page<'_>) -> Result<(), Report> {
    let trigger = page.element("test-context-trigger").await?;
    // Not announced as opening a menu.
    assert_that!(trigger.attr("aria-haspopup").await?).is_none();
    assert_that!(trigger.attr("aria-expanded").await?).is_none();
    // A regular press doesn't open it.
    trigger.click().await?;
    page.wait_for_no_selector("[role=menu]").await?;

    // A right click opens it at the pointer: 10px right and 15px below the trigger's corner.
    let rect = trigger.rect().await?;
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
    page.wait_for_selector("[role=menu]").await?;
    let popover = page.css(".test-popover").await?;
    // Where it settles once its entry animation (a slide) ran.
    let mut popover_rect = popover.rect().await?;
    for _ in 0..40 {
        if (popover_rect.x - (rect.x + 10.0)).abs() <= 2.0
            && (popover_rect.y - (rect.y + 15.0)).abs() <= 2.0
        {
            break;
        }
        tokio::time::sleep(std::time::Duration::from_millis(50)).await;
        popover_rect = popover.rect().await?;
    }
    assert_that!((popover_rect.x - (rect.x + 10.0)).abs() <= 2.0).is_true();
    assert_that!((popover_rect.y - (rect.y + 15.0)).abs() <= 2.0).is_true();
    assert_that!(trigger.attr("aria-expanded").await?).is_none();

    item(page, "Paste").await?.click().await?;
    page.wait_for_text("test-submenu-actions", "sms, work, paste")
        .await?;
    page.wait_for_no_selector("[role=menu]").await?;

    // Right clicking inside keeps it open; outside closes it.
    page.driver
        .action_chain()
        .move_to_element_center(&trigger)
        .context_click()
        .perform()
        .await?;
    page.wait_for_selector("[role=menu]").await?;
    let menu = page.css("[role=menu]").await?;
    page.driver
        .action_chain()
        .move_to_element_center(&menu)
        .context_click()
        .perform()
        .await?;
    page.wait_for_selector("[role=menu]").await?;
    page.driver
        .action_chain()
        .move_to_element_center(&page.element("test-submenu-before").await?)
        .context_click()
        .perform()
        .await?;
    page.wait_for_no_selector("[role=menu]").await
}

/// "should contain focus for subdialogs": a submenu trigger opening a dialog; its popover is the
/// dialog and contains focus; Escape closes it, returning focus to the trigger item.
async fn subdialog(page: &Page<'_>) -> Result<(), Report> {
    open_root(page).await?;
    let signup = item(page, "Sign up…").await?;
    assert_that!(signup.attr("aria-haspopup").await?).is_equal_to(Some("dialog".to_owned()));
    hover(page, &signup).await?;
    page.wait_for_attr(&signup, "aria-expanded", Some("true"))
        .await?;
    let dialogs = page
        .driver
        .find_all(browser_test::thirtyfour::By::Css(
            ".test-popover[role=dialog]",
        ))
        .await?;
    assert_that!(dialogs.len()).is_equal_to(2);

    page.element("test-signup-first").await?.click().await?;
    page.wait_for_active_id("test-signup-first").await?;
    page.send_keys_to_active(Key::Tab).await?;
    page.wait_for_active_id("test-signup-last").await?;
    page.send_keys_to_active(Key::Tab).await?;
    page.wait_for_active_id("test-signup-first").await?;

    page.send_keys_to_active(Key::Escape).await?;
    page.wait_for_attr(&signup, "aria-expanded", Some("false"))
        .await?;
    page.wait_for_active_text("Sign up…").await?;
    page.send_keys_to_active(Key::Escape).await?;
    page.wait_for_no_selector("[role=menu]").await
}

/// A subdialog whose popover holds a `Dialog`: opened by keyboard, focus moves into it; Escape
/// closes it and returns focus to the trigger item, the menu stays open.
async fn subdialog_with_dialog(page: &Page<'_>) -> Result<(), Report> {
    let trigger = page.element("test-submenu-trigger").await?;
    trigger.focus().await?;
    page.send_keys_to_active(Key::Enter).await?;
    page.wait_for_active_text("Open").await?;
    for _ in 0..4 {
        page.send_keys_to_active(Key::Down).await?;
    }
    page.wait_for_active_text("Properties…").await?;
    page.send_keys_to_active(Key::Right).await?;
    page.wait_for_selector("[role=dialog][aria-label=Properties]")
        .await?;
    page.wait_for_focus("dialog", None).await?;
    page.send_keys_to_active(Key::Escape).await?;
    page.wait_for_no_selector("[role=dialog][aria-label=Properties]")
        .await?;
    page.wait_for_active_text("Properties…").await?;

    // Again, from the input inside.
    page.send_keys_to_active(Key::Right).await?;
    page.element("test-properties-input").await?.click().await?;
    page.wait_for_active_id("test-properties-input").await?;
    page.send_keys_to_active(Key::Escape).await?;
    page.wait_for_no_selector("[role=dialog][aria-label=Properties]")
        .await?;
    page.wait_for_active_text("Properties…").await?;
    page.send_keys_to_active(Key::Escape).await?;
    page.wait_for_no_selector("[role=menu]").await
}

/// Right-to-left ("should open/close submenu with ArrowLeft/ArrowRight in RTL", useSubmenuTrigger):
/// the trigger opens the menu by click and by ArrowDown; ArrowLeft opens a submenu, ArrowRight
/// returns to its trigger.
async fn right_to_left(page: &Page<'_>) -> Result<(), Report> {
    let trigger = page.element("test-submenu-rtl-trigger").await?;
    trigger.click().await?;
    page.wait_for_selector("[role=menu]").await?;
    page.send_keys_to_active(Key::Escape).await?;
    page.wait_for_no_selector("[role=menu]").await?;
    page.wait_for_focus_on(&trigger, "RTL menu trigger").await?;

    page.send_keys_to_active(Key::Down).await?;
    page.wait_for_active_text("Open (RTL)").await?;
    page.send_keys_to_active(Key::Down).await?;
    page.wait_for_active_text("Share (RTL)").await?;
    // ArrowRight is "back" in right-to-left text: it doesn't open the submenu.
    page.send_keys_to_active(Key::Right).await?;
    let share = item(page, "Share (RTL)").await?;
    stays!(
        "aria-expanded of the RTL submenu trigger",
        Some("false".to_owned()),
        share.attr("aria-expanded").await?
    );
    page.send_keys_to_active(Key::Left).await?;
    page.wait_for_active_text("Email (RTL)").await?;
    page.send_keys_to_active(Key::Right).await?;
    page.wait_for_active_text("Share (RTL)").await?;
    page.wait_for_attr(&share, "aria-expanded", Some("false"))
        .await?;
    page.send_keys_to_active(Key::Escape).await?;
    page.wait_for_no_selector("[role=menu]").await?;
    page.wait_for_focus_on(&trigger, "RTL menu trigger").await
}

/// The pointer's way to an open submenu (`useSafelyMouseToSubmenu`): moving diagonally towards
/// it across other items, the root menu ignores pointer events, so the submenu stays open; at rest,
/// the menu takes them again.
async fn safe_triangle(page: &Page<'_>) -> Result<(), Report> {
    // Opened by a click: the hook only works for pointer modality.
    open_root(page).await?;
    let share = item(page, "Share…").await?;
    hover(page, &share).await?;
    page.wait_for_attr(&share, "aria-expanded", Some("true"))
        .await?;
    page.wait_for_count("[role=menu]", 2).await?;
    // From the trigger's center towards the submenu, below it (across "Sign up…"), in moves the
    // hook doesn't throttle (one per 50 ms) and small enough that the first two stay on the
    // trigger (the hook judges the direction from the second processed move on), inside the
    // root menu.
    let [share_x, menu_right, submenu_left]: [f64; 3] = page
        .driver
        .execute(
            "const share = arguments[0].getBoundingClientRect();
             const menus = document.querySelectorAll('[role=menu]');
             return [share.left + share.width / 2, menus[0].getBoundingClientRect().right,
                     menus[1].getBoundingClientRect().left];",
            vec![share.to_json()?],
        )
        .await?
        .convert()?;
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
        tokio::time::sleep(std::time::Duration::from_millis(60)).await;
        seen_none |= root_pointer_events(page).await? == "none";
    }
    assert_that!(seen_none)
        .with_detail_message("the root menu ignored pointer events on the way")
        .is_true();
    assert_that!(share.attr("aria-expanded").await?).is_equal_to(Some("true".to_owned()));
    // At rest, the menu takes pointer events again.
    wait_for!(
        "the root menu's pointer-events at rest",
        String::new(),
        root_pointer_events(page).await?
    );
    page.send_keys_to_active(Key::Escape).await?;
    page.send_keys_to_active(Key::Escape).await?;
    page.wait_for_no_selector("[role=menu]").await
}

/// The root menu's inline `pointer-events`.
async fn root_pointer_events(page: &Page<'_>) -> Result<String, Report> {
    Ok(page
        .driver
        .execute(
            "return document.querySelectorAll('[role=menu]')[0].style.pointerEvents;",
            vec![],
        )
        .await?
        .convert()?)
}
