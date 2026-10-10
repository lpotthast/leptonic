// Upstream: react-aria/test/interactions/useContextMenu.test.tsx @ 99e6102368
//! `use_context_menu`: a right click requests the menu at its position relative to the element,
//! prevents the browser's menu and stops propagation; without a handler nothing happens. On a Mac
//! (emulated) Ctrl+Enter requests it once, also when the browser fires `contextmenu` too; on an
//! iPhone (emulated) a long press does.
use std::time::Duration;

use assertr::{matchers::eq, prelude::*};
use browser_test::{browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::pages::{
    Dispatched, ElementActions, KeyKind, Modifier, MouseButton, MouseKind, Page, Platform,
    PointerKind, PointerType, SyntheticEvent,
};

const PATH: &str = "/hooks/context-menu";

/// The fixture's log of handled context menus.
async fn log(page: &Page<'_>) -> Result<WebElement, Report> {
    page.element("#test-context-menu-log").await
}

/// A `contextmenu` event about 20px right of and below the corner of `element`, and the position
/// relative to the element the handler gets (the event's coordinates are whole pixels, the
/// element's need not be).
async fn right_click(element: &WebElement) -> Result<(Dispatched, f64, f64), Report> {
    let rect = element.client_rect().await?;
    let (x, y) = ((rect.left + 20.0).trunc(), (rect.top + 20.0).trunc());
    let dispatched = element
        .dispatch(
            SyntheticEvent::mouse(MouseKind::ContextMenu)
                .at(x, y)
                .button(MouseButton::Secondary),
        )
        .await?;
    Ok((dispatched, x - rect.left, y - rect.top))
}

/// A right click requests the menu once at its position in the element, prevents the browser's
/// menu and doesn't reach the wrapper ("calls onContextMenu on right click", "prevents default and
/// stops propagation on contextmenu event").
#[browser_test]
pub async fn right_click_requests_the_menu(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let element = page.element("#test-context-menu-handler").await?;
    let (dispatched, x, y) = right_click(&element).await?;
    assert_that!(dispatched.default_prevented).is_true();
    let log = log(page).await?;
    let expected = format!("menu:{x}:{y}:test-context-menu-handler");
    log.wait_for_inner_text(&expected).await?;
    log.inner_text_stays(&expected, std::time::Duration::from_millis(100))
        .await?;
    Ok(())
}

/// Without a handler, a right click reaches the wrapper and the browser's menu isn't prevented
/// ("does not call onContextMenu when prop is not provided").
#[browser_test]
pub async fn without_a_handler_nothing_happens(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let element = page.element("#test-context-menu-none").await?;
    let (dispatched, ..) = right_click(&element).await?;
    assert_that!(dispatched.default_prevented).is_false();
    let log = log(page).await?;
    log.wait_for_inner_text("none-wrapper").await?;
    log.inner_text_stays("none-wrapper", std::time::Duration::from_millis(100))
        .await?;
    Ok(())
}

/// The log is `expected` and stays so past the 10 ms delay of the Ctrl+Enter fallback.
async fn log_stays_past_the_ctrl_enter_delay(
    page: &Page<'_>,
    expected: &str,
) -> Result<(), Report> {
    let log = log(page).await?;
    page.settle().await?;
    assert_that!(|| log.inner_text())
        .consistently_ok()
        .for_at_least(std::time::Duration::from_millis(100))
        // Past the delay after which macOS' Ctrl+Enter requests the menu (10 ms).
        .for_at_least(Duration::from_millis(200))
        .matches(eq(expected))
        .await;
    Ok(())
}

/// Elsewhere than on a Mac, Ctrl+Enter requests no context menu ("does not trigger on Ctrl+Enter
/// on non-macOS").
#[browser_test]
pub async fn ctrl_enter_is_mac_only(page: &Page<'_>) -> Result<(), Report> {
    page.emulate_platform(Platform::Linux).await?;
    page.goto_path(PATH).await?;
    let element = page.element("#test-context-menu-handler").await?;
    element.focus().await?;
    page.wait_for_focus(&element).await?;
    page.send_keys(Key::Control + Key::Enter).await?;
    log_stays_past_the_ctrl_enter_delay(page, "").await?;
    Ok(())
}

/// On a Mac, Ctrl+Enter without a `contextmenu` event requests the menu once, at the element's
/// center ("triggers onContextMenu via Ctrl+Enter on macOS when no contextmenu event fires").
#[browser_test]
pub async fn ctrl_enter_on_mac(page: &Page<'_>) -> Result<(), Report> {
    page.emulate_platform(Platform::Mac).await?;
    page.goto_path(PATH).await?;
    let element = page.element("#test-context-menu-handler").await?;
    element.focus().await?;
    page.wait_for_focus(&element).await?;
    page.send_keys(Key::Control + Key::Enter).await?;
    let rect = element.client_rect().await?;
    let expected = format!(
        "menu:{}:{}:test-context-menu-handler",
        rect.width / 2.0,
        rect.height / 2.0
    );
    log(page).await?.wait_for_inner_text(&expected).await?;
    log_stays_past_the_ctrl_enter_delay(page, &expected).await?;
    Ok(())
}

/// On a Mac, a `contextmenu` event right after Ctrl+Enter requests the menu once, at its position,
/// and the Ctrl+Enter fallback adds nothing ("does not double-fire when Ctrl+Enter also triggers a
/// contextmenu event").
#[browser_test]
pub async fn ctrl_enter_with_a_contextmenu_event_fires_once(page: &Page<'_>) -> Result<(), Report> {
    page.emulate_platform(Platform::Mac).await?;
    page.goto_path(PATH).await?;
    let element = page.element("#test-context-menu-handler").await?;
    let rect = element.client_rect().await?;
    let (x, y) = ((rect.left + 20.0).trunc(), (rect.top + 20.0).trunc());
    element
        .dispatch_both(
            SyntheticEvent::keyboard(KeyKind::Down, "Enter").modifiers(&[Modifier::Control]),
            SyntheticEvent::mouse(MouseKind::ContextMenu)
                .at(x, y)
                .button(MouseButton::Secondary),
        )
        .await?;
    let expected = format!(
        "menu:{}:{}:test-context-menu-handler",
        x - rect.left,
        y - rect.top
    );
    log(page).await?.wait_for_inner_text(&expected).await?;
    log_stays_past_the_ctrl_enter_delay(page, &expected).await?;
    Ok(())
}

/// On a Mac, Enter without Ctrl requests no context menu ("does not trigger on Enter without Ctrl
/// on macOS").
#[browser_test]
pub async fn enter_without_ctrl_on_mac(page: &Page<'_>) -> Result<(), Report> {
    page.emulate_platform(Platform::Mac).await?;
    page.goto_path(PATH).await?;
    let element = page.element("#test-context-menu-handler").await?;
    element.focus().await?;
    page.wait_for_focus(&element).await?;
    page.send_keys(Key::Enter).await?;
    log_stays_past_the_ctrl_enter_delay(page, "").await?;
    Ok(())
}

/// Dispatches a touch pointer event of `kind` 10px right of and 20px below `element`'s corner
/// (whole pixels), and returns that position relative to the element.
async fn touch_at(element: &WebElement, kind: PointerKind) -> Result<(f64, f64), Report> {
    let rect = element.client_rect().await?;
    let (x, y) = ((rect.left + 10.0).trunc(), (rect.top + 20.0).trunc());
    element
        .dispatch(
            SyntheticEvent::pointer(kind)
                .pointer_type(PointerType::Touch)
                .at(x, y),
        )
        .await?;
    Ok((x - rect.left, y - rect.top))
}

/// On an iPhone, a touch held past the long press threshold requests the menu once at the touch
/// position, and the release adds nothing ("triggers onContextMenu via long press on iOS").
#[browser_test]
pub async fn long_press_on_ios(page: &Page<'_>) -> Result<(), Report> {
    page.emulate_platform(Platform::IPhone).await?;
    page.goto_path(PATH).await?;
    let element = page.element("#test-context-menu-long-press").await?;
    let (x, y) = touch_at(&element, PointerKind::Down).await?;
    let expected = format!("menu:{x}:{y}:test-context-menu-long-press");
    let log = log(page).await?;
    log.wait_for_inner_text(&expected).await?;
    touch_at(&element, PointerKind::Up).await?;
    log.inner_text_stays(&expected, std::time::Duration::from_millis(100))
        .await?;
    Ok(())
}

/// On an iPhone, a touch cancelled before the long press threshold requests no context menu ("does
/// not trigger if the press is cancelled before the long press threshold").
#[browser_test]
pub async fn cancelled_long_press_on_ios(page: &Page<'_>) -> Result<(), Report> {
    page.emulate_platform(Platform::IPhone).await?;
    page.goto_path(PATH).await?;
    let element = page.element("#test-context-menu-long-press").await?;
    touch_at(&element, PointerKind::Down).await?;
    element
        .dispatch(SyntheticEvent::pointer(PointerKind::Cancel).pointer_type(PointerType::Touch))
        .await?;
    let log = log(page).await?;
    page.settle().await?;
    assert_that!(|| log.inner_text())
        .consistently_ok()
        // Past the long press threshold (500 ms).
        .for_at_least(Duration::from_millis(700))
        .matches(eq(""))
        .await;
    Ok(())
}

/// On Android, where the browser fires `contextmenu` for a long press itself, a long press requests
/// the menu once: from that event, not from the long press too ("does not double-fire when long
/// press and contextmenu event both occur (Android)").
#[browser_test]
pub async fn long_press_on_android_requests_once(page: &Page<'_>) -> Result<(), Report> {
    page.emulate_platform(Platform::Android).await?;
    page.goto_path(PATH).await?;
    let element = page.element("#test-context-menu-long-press").await?;
    touch_at(&element, PointerKind::Down).await?;
    // A real timer: past the long press threshold (500 ms).
    tokio::time::sleep(Duration::from_millis(600)).await;
    let (_, x, y) = right_click(&element).await?;
    touch_at(&element, PointerKind::Up).await?;
    let expected = format!("menu:{x}:{y}:test-context-menu-long-press");
    let log = log(page).await?;
    log.wait_for_inner_text(&expected).await?;
    log.inner_text_stays(&expected, Duration::from_millis(300))
        .await?;
    Ok(())
}
