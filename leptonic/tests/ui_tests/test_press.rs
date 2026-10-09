// Upstream: react-aria/test/interactions/usePress.test.js @ 99e6102368
//! `use_press` on a Mac (emulated: Chrome on Linux fires every key up): macOS fires no key up for
//! keys released while Meta is held, so releasing Meta ends their presses ("should fire press
//! events when Meta key is held to work around macOS bug").

use std::time::Duration;

use assertr::{matchers::eq, prelude::*};
use browser_test::{browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::pages::{
    DragKind, ElementActions, KeyKind, Modifier, MouseButton, MouseKind, Page, Platform,
    PointerKind, PointerType, SyntheticEvent,
};

const PATH: &str = "/hooks/press";

/// Opens the fixture in a browser that says it runs on a Mac.
async fn open_on_a_mac(page: &Page<'_>) -> Result<(), Report> {
    page.emulate_platform(Platform::Mac).await?;
    page.goto_path(PATH).await?;
    Ok(())
}

/// On a Mac, releasing Meta while Enter is still held completes the Enter press, and the later
/// Enter key up adds nothing ("should fire press events when Meta key is held to work around macOS
/// bug").
#[browser_test]
pub async fn meta_release_ends_held_key_presses(page: &Page<'_>) -> Result<(), Report> {
    open_on_a_mac(page).await?;
    let platform: String = page
        .low_level()
        .eval("return navigator.platform;", vec![])
        .await?;
    assert_that!(platform).is_equal_to("MacIntel");

    page.element("#test-press-target").await?.focus().await?;
    page.wait_for_focus(&page.element("#test-press-target").await?)
        .await?;
    // Meta held, Enter pressed; Meta released while Enter is still down.
    page.low_level()
        .driver()
        .action_chain()
        .key_down(Key::Meta)
        .key_down(Key::Enter)
        .key_up(Key::Meta)
        .perform()
        .await?;
    log(page)
        .await?
        .wait_for_inner_text("start:keyboard,up:keyboard,end:keyboard,press:keyboard")
        .await?;
    page.element("#test-press-is-pressed")
        .await?
        .wait_for_inner_text("false")
        .await?;
    page.low_level()
        .driver()
        .action_chain()
        .key_up(Key::Enter)
        .perform()
        .await?;
    log(page)
        .await?
        .inner_text_stays(
            "start:keyboard,up:keyboard,end:keyboard,press:keyboard",
            Duration::from_millis(100),
        )
        .await?;
    Ok(())
}

/// The log of the main target's press events.
async fn log(page: &Page<'_>) -> Result<WebElement, Report> {
    page.element("#test-press-log").await
}

/// A mouse click fires press start, up, end and press in order and leaves the element unpressed
/// ("should fire press events based on pointer events with pointerType=mouse").
#[browser_test]
pub async fn mouse_click_fires_all_events_in_order(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element("#test-press-target").await?.click().await?;
    log(page)
        .await?
        .wait_for_inner_text("start:mouse,up:mouse,end:mouse,press:mouse")
        .await?;
    page.element("#test-press-is-pressed")
        .await?
        .wait_for_inner_text("false")
        .await?;
    Ok(())
}

/// While pressed, the pressed element (not the text inside it) has `user-select: none`, restored on
/// release ("should add/remove user-select: none to the element on pointer down/up").
#[browser_test]
pub async fn text_selection_disabled_while_pressed(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let target = page.element("#test-press-target").await?;
    let label = page.element("#test-press-label").await?;
    assert_that!(target.css_value("user-select").await?).is_equal_to("auto");

    let held = target.press_and_hold().await?;
    log(page).await?.wait_for_inner_text("start:mouse").await?;
    assert_that!(|| target.css_value("user-select"))
        .eventually_ok()
        .matches(eq("none"))
        .await;
    assert_that!(label).attribute("style").await.is_none();

    held.release().await?;
    log(page)
        .await?
        .wait_for_inner_text("start:mouse,up:mouse,end:mouse,press:mouse")
        .await?;
    assert_that!(|| target.css_value("user-select"))
        .eventually_ok()
        .matches(eq("auto"))
        .await;
    Ok(())
}

/// Enter fires press start, up, end and press once ("should fire press events when the element is
/// not a link").
#[browser_test]
pub async fn enter_presses(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let target = page.element("#test-press-target").await?;
    target.focus().await?;
    page.wait_for_focus(&target).await?;
    page.send_keys(Key::Enter).await?;
    let log = log(page).await?;
    log.wait_for_inner_text("start:keyboard,up:keyboard,end:keyboard,press:keyboard")
        .await?;
    log.inner_text_stays(
        "start:keyboard,up:keyboard,end:keyboard,press:keyboard",
        Duration::from_millis(100),
    )
    .await?;
    Ok(())
}

/// Space fires press start, up, end and press once ("should fire press events when the element is
/// not a link").
#[browser_test]
pub async fn space_presses(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let target = page.element("#test-press-target").await?;
    target.focus().await?;
    page.wait_for_focus(&target).await?;
    page.send_keys(" ").await?;
    let log = log(page).await?;
    log.wait_for_inner_text("start:keyboard,up:keyboard,end:keyboard,press:keyboard")
        .await?;
    log.inner_text_stays(
        "start:keyboard,up:keyboard,end:keyboard,press:keyboard",
        Duration::from_millis(100),
    )
    .await?;
    Ok(())
}

/// A pointer released outside the element ends the press without firing a press, also not later
/// from the click fallback ("should fire press change events when moving pointer outside target").
#[browser_test]
pub async fn releasing_outside_does_not_press(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let target = page.element("#test-press-target").await?;
    let elsewhere = page.element("#test-press-elsewhere").await?;

    let held = target.press_and_hold().await?;
    log(page).await?.wait_for_inner_text("start:mouse").await?;
    page.element("#test-press-is-pressed")
        .await?
        .wait_for_inner_text("true")
        .await?;

    held.move_to(&elsewhere).await?;
    held.release().await?;
    log(page)
        .await?
        .wait_for_inner_text("start:mouse,end:mouse")
        .await?;
    page.element("#test-press-is-pressed")
        .await?
        .wait_for_inner_text("false")
        .await?;
    // No press after all (the click fallback, 80 ms after the pointer up, mustn't fire one
    // either).
    let log = log(page).await?;
    page.settle().await?;
    assert_that!(|| log.inner_text())
        .consistently_ok()
        .for_at_least(Duration::from_millis(300))
        .matches(eq("start:mouse,end:mouse"))
        .await;
    Ok(())
}

/// Clicking a disabled element fires no press events ("should not fire press/click events for
/// disabled elements").
#[browser_test]
pub async fn disabled_element_ignores_presses(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element("#test-press-toggle-disabled")
        .await?
        .click()
        .await?;
    page.element("#test-press-target").await?.click().await?;
    log(page)
        .await?
        .inner_text_stays("", Duration::from_millis(100))
        .await?;
    Ok(())
}

/// An element that becomes disabled while pressed ends the press at once and fires no press on
/// release (react-spectrum #9813).
#[browser_test]
pub async fn becoming_disabled_cancels_active_press(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let target = page.element("#test-press-self-disabling").await?;
    let held = target.press_and_hold().await?;

    // The press ends while the pointer is still held down.
    page.element("#test-press-self-disabling-log")
        .await?
        .wait_for_inner_text("start:mouse,end:mouse")
        .await?;
    page.element("#test-press-self-disabling-is-pressed")
        .await?
        .wait_for_inner_text("false")
        .await?;

    held.release().await?;
    // Past the click fallback (80 ms after a pointer up without a click; a disabled button gets
    // no click).
    let log = page.element("#test-press-self-disabling-log").await?;
    page.settle().await?;
    assert_that!(|| log.inner_text())
        .consistently_ok()
        .for_at_least(Duration::from_millis(300))
        .matches(eq("start:mouse,end:mouse"))
        .await;
    Ok(())
}

/// Enter on a pressable checkbox submits its form instead of being swallowed ("should fire press
/// events on checkboxes but not prevent default", react-spectrum #9972).
#[browser_test]
pub async fn enter_on_checkbox_submits_form(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element("#test-press-checkbox").await?.click().await?;
    page.element("#test-press-checkbox")
        .await?
        .send_keys(Key::Enter)
        .await?;
    page.element("#test-press-submits")
        .await?
        .wait_for_inner_text("1")
        .await?;
    Ok(())
}

/// Clicking or pressing Enter on an inner pressable fires no press events on the outer one ("should
/// stop propagation by default").
#[browser_test]
pub async fn nested_press_stops_by_default(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let id = "test-press-nested-stop";
    page.element(format!("#{id}-inner")).await?.click().await?;
    page.element(format!("#{id}-inner-log"))
        .await?
        .wait_for_inner_text("start,up,end,press")
        .await?;
    page.send_keys(Key::Enter).await?;
    page.element(format!("#{id}-inner-log"))
        .await?
        .wait_for_inner_text("start,up,end,press,start,up,end,press")
        .await?;
    page.element(format!("#{id}-outer-log"))
        .await?
        .inner_text_stays("", Duration::from_millis(100))
        .await?;
    Ok(())
}

/// When the inner pressable's handlers call `continue_propagation()`, the outer one is pressed
/// along with it ("should allow propagation if continuePropagation is called").
#[browser_test]
pub async fn nested_press_propagates_when_continued(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let id = "test-press-nested-continue";
    page.element(format!("#{id}-inner")).await?.click().await?;
    page.element(format!("#{id}-inner-log"))
        .await?
        .wait_for_inner_text("start,up,end,press")
        .await?;
    page.element(format!("#{id}-outer-log"))
        .await?
        .wait_for_inner_text("start:mouse,up:mouse,end:mouse,press:mouse")
        .await?;
    Ok(())
}

/// With focus prevented on press, a click presses the element while the focus stays on the input
/// focused before, which is never blurred ("should not focus the element on click if
/// preventFocusOnPress is true").
#[browser_test]
pub async fn prevent_focus_on_press_keeps_the_focus(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element("#test-press-keep-input")
        .await?
        .focus()
        .await?;
    page.element("#test-press-keep").await?.click().await?;
    page.element("#test-press-keep-presses")
        .await?
        .wait_for_inner_text("1")
        .await?;
    page.wait_for_focus(&page.element("#test-press-keep-input").await?)
        .await?;
    // Past the click fallback (80 ms after the pointer up), which focuses the pressed element.
    let blurs = page.element("#test-press-keep-blurs").await?;
    page.settle().await?;
    assert_that!(|| blurs.inner_text())
        .consistently_ok()
        .for_at_least(Duration::from_millis(300))
        .matches(eq("0"))
        .await;
    Ok(())
}

/// A key up handler on the pressed element that stops the event doesn't leave the press stuck:
/// Enter and Space still complete their presses.
#[browser_test]
pub async fn keyboard_press_ends_when_key_up_is_stopped(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element("#test-press-keyup").await?.focus().await?;
    page.wait_for_focus(&page.element("#test-press-keyup").await?)
        .await?;
    page.send_keys(Key::Enter).await?;
    page.element("#test-press-keyup-presses")
        .await?
        .wait_for_inner_text("1")
        .await?;
    page.element("#test-press-keyup-pressed")
        .await?
        .wait_for_inner_text("false")
        .await?;
    page.element("#test-press-keyup-key-ups")
        .await?
        .wait_for_inner_text("1")
        .await?;
    page.send_keys(" ").await?;
    page.element("#test-press-keyup-presses")
        .await?
        .wait_for_inner_text("2")
        .await?;
    page.element("#test-press-keyup-pressed")
        .await?
        .wait_for_inner_text("false")
        .await?;
    Ok(())
}

/// A drag starting inside the pressable ends its press without firing a press, also after the
/// release ("should cancel press on dragstart").
#[browser_test]
pub async fn a_drag_inside_cancels_the_press(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let image = page.element("#test-press-drag-image").await?;
    let held = image.press_and_hold().await?;
    page.element("#test-press-drag-log")
        .await?
        .wait_for_inner_text("start")
        .await?;
    image
        .dispatch(SyntheticEvent::drag(DragKind::Start))
        .await?;
    page.element("#test-press-drag-log")
        .await?
        .wait_for_inner_text("start,end")
        .await?;
    held.release().await?;
    // Past the click fallback (80 ms after a pointer up without a click; none follows a drag).
    let log = page.element("#test-press-drag-log").await?;
    page.settle().await?;
    assert_that!(|| log.inner_text())
        .consistently_ok()
        .for_at_least(Duration::from_millis(300))
        .matches(eq("start,end"))
        .await;
    Ok(())
}

/// When a child stops the click's propagation, the press ends without firing a press ("should
/// cancel press if onClick propagation is stopped").
#[browser_test]
pub async fn a_child_stopping_the_click_cancels_the_press(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let log = page.element("#test-press-click-stop-log").await?;
    page.element("#test-press-click-stop-child")
        .await?
        .click()
        .await?;
    log.wait_for_inner_text("start:mouse,end:mouse").await?;
    log.inner_text_stays("start:mouse,end:mouse", Duration::from_millis(100))
        .await?;
    Ok(())
}

/// When the focus moves to another element on press start, the key up there ends the press without
/// press up or press ("should handle when focus moves between keydown and keyup").
#[browser_test]
pub async fn focus_moving_before_key_up_ends_without_press(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let log = page.element("#test-press-focus-move-log").await?;
    page.element("#test-press-focus-move")
        .await?
        .focus()
        .await?;
    page.wait_for_focus(&page.element("#test-press-focus-move").await?)
        .await?;
    page.send_keys(" ").await?;
    page.wait_for_focus(&page.element("#test-press-focus-move-other").await?)
        .await?;
    log.wait_for_inner_text("start:keyboard,end:keyboard")
        .await?;
    log.inner_text_stays("start:keyboard,end:keyboard", Duration::from_millis(100))
        .await?;
    Ok(())
}

/// A repeated Space key down (with `repeat` set) and its key up fire no press events ("should
/// ignore repeating keyboard events").
#[browser_test]
pub async fn repeating_key_downs_are_ignored(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let target = page.element("#test-press-target").await?;
    target
        .dispatch(SyntheticEvent::keyboard(KeyKind::Down, " ").repeat(true))
        .await?;
    page.element("body")
        .await?
        .dispatch(SyntheticEvent::keyboard(KeyKind::Up, " "))
        .await?;
    log(page)
        .await?
        .inner_text_stays("", Duration::from_millis(100))
        .await?;
    Ok(())
}

/// On a `display: contents` pressable, dragging out ends the press, dragging back in restarts it
/// and releasing there presses ("should fire press change events when moving pointer outside
/// target").
#[browser_test]
pub async fn dragging_out_and_back_in(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let log = page.element("#test-press-contents-log").await?;
    let child = page.element("#test-press-contents-child").await?;
    let elsewhere = page.element("#test-press-elsewhere").await?;
    let held = child.press_and_hold().await?;
    log.wait_for_inner_text("start:mouse").await?;
    elsewhere.hover().await?;
    log.wait_for_inner_text("start:mouse,end:mouse").await?;
    child.hover().await?;
    log.wait_for_inner_text("start:mouse,end:mouse,start:mouse")
        .await?;
    held.release().await?;
    log.wait_for_inner_text("start:mouse,end:mouse,start:mouse,up:mouse,end:mouse,press:mouse")
        .await?;
    Ok(())
}

/// With cancel on pointer exit, dragging out ends the press for good: back over the element and
/// released, only press up fires ("should cancel press when moving outside and the
/// shouldCancelOnPointerExit option is set").
#[browser_test]
pub async fn cancel_on_pointer_exit(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let log = page.element("#test-press-cancel-exit-log").await?;
    let target = page.element("#test-press-cancel-exit").await?;
    let elsewhere = page.element("#test-press-elsewhere").await?;
    let held = target.press_and_hold().await?;
    log.wait_for_inner_text("start:mouse").await?;
    elsewhere.hover().await?;
    log.wait_for_inner_text("start:mouse,end:mouse").await?;
    target.hover().await?;
    // Back over the element: the press doesn't start again.
    log.inner_text_stays("start:mouse,end:mouse", Duration::from_millis(100))
        .await?;
    // Released there: a pointer up without a press (react-aria's element `onPointerUp`), no press.
    held.release().await?;
    log.wait_for_inner_text("start:mouse,end:mouse,up:mouse")
        .await?;
    log.inner_text_stays("start:mouse,end:mouse,up:mouse", Duration::from_millis(100))
        .await?;
    Ok(())
}

/// A pointer cancel ends the press, and the later release fires only press up, no press ("should
/// handle pointer cancel events").
#[browser_test]
pub async fn pointer_cancel_cancels_the_press(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let target = page.element("#test-press-target").await?;
    let held = target.press_and_hold().await?;
    log(page).await?.wait_for_inner_text("start:mouse").await?;
    target
        .dispatch(SyntheticEvent::pointer(PointerKind::Cancel))
        .await?;
    log(page)
        .await?
        .wait_for_inner_text("start:mouse,end:mouse")
        .await?;
    // Released over the element: a pointer up without a press (react-aria's element `onPointerUp`
    // fires press up when no press is active), no press.
    held.release().await?;
    log(page)
        .await?
        .wait_for_inner_text("start:mouse,end:mouse,up:mouse")
        .await?;
    log(page)
        .await?
        .inner_text_stays("start:mouse,end:mouse,up:mouse", Duration::from_millis(100))
        .await?;
    Ok(())
}

/// Space on a link with the button role presses it once and follows the link ("should explicitly
/// call click method when Space key is triggered on a link with href and role="button"").
#[browser_test]
pub async fn space_on_a_link_with_button_role(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let log = page.element("#test-press-link-log").await?;
    page.element("#test-press-link").await?.focus().await?;
    page.wait_for_focus(&page.element("#test-press-link").await?)
        .await?;
    page.send_keys(" ").await?;
    log.wait_for_inner_text("start:keyboard,up:keyboard,end:keyboard,press:keyboard")
        .await?;
    log.inner_text_stays(
        "start:keyboard,up:keyboard,end:keyboard,press:keyboard",
        Duration::from_millis(100),
    )
    .await?;
    let url = page.low_level().driver().current_url().await?;
    assert_that!(url.fragment()).is_equal_to(Some("test-press-link-target"));
    Ok(())
}

/// A double click fires `on_double_press` after both presses (a leptonic addition).
#[browser_test]
pub async fn double_press(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element("#test-press-double")
        .await?
        .double_click()
        .await?;
    page.element("#test-press-double-log")
        .await?
        .wait_for_inner_text("press,press,double:mouse")
        .await?;
    Ok(())
}

/// A double click fires `on_double_press` also when the press props are spread together with hover
/// props.
#[browser_test]
pub async fn double_press_with_hover(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element("#test-press-double-hover")
        .await?
        .double_click()
        .await?;
    page.element("#test-press-double-hover-log")
        .await?
        .wait_for_inner_text("double")
        .await?;
    Ok(())
}

/// With `PressPropagation::Continue` on the inner pressable, clicking it presses the outer one too.
#[browser_test]
pub async fn press_propagation_continue(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element("#test-press-continue-inner")
        .await?
        .click()
        .await?;
    page.element("#test-press-continue-inner-log")
        .await?
        .wait_for_inner_text("press")
        .await?;
    page.element("#test-press-continue-outer-log")
        .await?
        .wait_for_inner_text("press")
        .await?;
    Ok(())
}

/// A script `click()` without a pointer fires press start, up, end and press as virtual ("should
/// fire press events events for virtual click events from screen readers").
#[browser_test]
pub async fn virtual_click(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element("#test-press-target")
        .await?
        .virtual_click()
        .await?;
    log(page)
        .await?
        .wait_for_inner_text("start:virtual,up:virtual,end:virtual,press:virtual")
        .await?;
    Ok(())
}

/// A pressable removed while pressed fires no press when the pointer is released afterwards.
#[browser_test]
pub async fn removed_while_pressed(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let target = page.element("#test-press-removed").await?;
    let held = target.press_and_hold().await?;
    page.wait_for_count("#test-press-removed", 0).await?;
    held.release().await?;
    page.element("#test-press-removed-log")
        .await?
        .inner_text_stays("start:mouse", Duration::from_millis(100))
        .await?;
    Ok(())
}

/// The log of `#test-press-detail` (`kind:pointer type:modifiers@x,y`).
async fn detail_log(page: &Page<'_>) -> Result<WebElement, Report> {
    page.element("#test-press-detail-log").await
}

/// The viewport position of the point (`x`, `y`) of `element` (from its top left corner).
async fn point_in(element: &WebElement, x: f64, y: f64) -> Result<(f64, f64), Report> {
    let rect = element.client_rect().await?;
    Ok((rect.left + x, rect.top + y))
}

/// A touch press without a click afterwards (iOS and Android after a long press) is completed by
/// the press itself 80 ms after the pointer up: press start, up, end and press, as touch, and the
/// element gets the focus ("should fire press events based on pointer events with
/// pointerType=touch", "should fire press events on long press even if onClick is not fired by
/// the browser").
#[browser_test]
pub async fn touch_press_without_a_click(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let target = page.element("#test-press-target").await?;
    for kind in [PointerKind::Down, PointerKind::Up] {
        target
            .dispatch(SyntheticEvent::pointer(kind).pointer_type(PointerType::Touch))
            .await?;
    }
    log(page)
        .await?
        .wait_for_inner_text("start:touch,up:touch,end:touch,press:touch")
        .await?;
    page.wait_for_focus(&target).await?;
    Ok(())
}

/// Presses with other buttons than the primary one fire nothing ("should only handle left
/// clicks").
#[browser_test]
pub async fn only_the_primary_button_presses(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let target = page.element("#test-press-target").await?;
    for button in [MouseButton::Auxiliary, MouseButton::Secondary] {
        target
            .dispatch(SyntheticEvent::pointer(PointerKind::Down).button(button))
            .await?;
        target
            .dispatch_both(
                SyntheticEvent::pointer(PointerKind::Up).button(button),
                SyntheticEvent::mouse(MouseKind::Click).button(button),
            )
            .await?;
    }
    // Past the click fallback (80 ms after a pointer up).
    log(page)
        .await?
        .inner_text_stays("", Duration::from_millis(300))
        .await?;
    Ok(())
}

/// Each press event carries the modifier keys held during its DOM event: Shift at the pointer
/// down, Control at the pointer up and click ("should handle modifier keys").
#[browser_test]
pub async fn pointer_modifier_keys(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let target = page.element("#test-press-detail").await?;
    let (x, y) = point_in(&target, 10.0, 10.0).await?;
    target
        .dispatch(
            SyntheticEvent::pointer(PointerKind::Down)
                .at(x, y)
                .modifiers(&[Modifier::Shift]),
        )
        .await?;
    target
        .dispatch_both(
            SyntheticEvent::pointer(PointerKind::Up)
                .at(x, y)
                .modifiers(&[Modifier::Control]),
            SyntheticEvent::mouse(MouseKind::Click)
                .at(x, y)
                .modifiers(&[Modifier::Control]),
        )
        .await?;
    detail_log(page)
        .await?
        .wait_for_inner_text(
            "start:mouse:shift@10,10,up:mouse:ctrl@10,10,end:mouse:ctrl@10,10,\
             press:mouse:ctrl@10,10",
        )
        .await?;
    Ok(())
}

/// Each keyboard press event carries the modifier keys held ("should handle modifier keys",
/// keyboard events).
#[browser_test]
pub async fn keyboard_modifier_keys(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let target = page.element("#test-press-detail").await?;
    target.focus().await?;
    page.wait_for_focus(&target).await?;
    page.send_keys(Key::Shift + Key::Enter).await?;
    detail_log(page)
        .await?
        .wait_for_inner_text(
            "start:keyboard:shift@50,25,up:keyboard:shift@50,25,end:keyboard:shift@50,25,\
             press:keyboard:shift@50,25",
        )
        .await?;
    Ok(())
}

/// Mouse press events have the pointer's position relative to the element: the pointer down's for
/// press start, the click's for the others ("mouse pointer events should have coordinates").
#[browser_test]
pub async fn mouse_coordinates(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let target = page.element("#test-press-detail").await?;
    let (down_x, down_y) = point_in(&target, 25.0, 5.0).await?;
    let (up_x, up_y) = point_in(&target, 75.0, 40.0).await?;
    target
        .dispatch(SyntheticEvent::pointer(PointerKind::Down).at(down_x, down_y))
        .await?;
    target
        .dispatch_both(
            SyntheticEvent::pointer(PointerKind::Up).at(up_x, up_y),
            SyntheticEvent::mouse(MouseKind::Click).at(up_x, up_y),
        )
        .await?;
    detail_log(page)
        .await?
        .wait_for_inner_text(
            "start:mouse:-@25,5,up:mouse:-@75,40,end:mouse:-@75,40,press:mouse:-@75,40",
        )
        .await?;
    Ok(())
}

/// Touch press events have the touch's position relative to the element ("pointer touch events
/// should have coordinates").
#[browser_test]
pub async fn touch_coordinates(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let target = page.element("#test-press-detail").await?;
    let (down_x, down_y) = point_in(&target, 25.0, 5.0).await?;
    let (up_x, up_y) = point_in(&target, 75.0, 40.0).await?;
    target
        .dispatch(
            SyntheticEvent::pointer(PointerKind::Down)
                .pointer_type(PointerType::Touch)
                .at(down_x, down_y),
        )
        .await?;
    target
        .dispatch_both(
            SyntheticEvent::pointer(PointerKind::Up)
                .pointer_type(PointerType::Touch)
                .at(up_x, up_y),
            SyntheticEvent::mouse(MouseKind::Click).at(up_x, up_y),
        )
        .await?;
    detail_log(page)
        .await?
        .wait_for_inner_text(
            "start:touch:-@25,5,up:touch:-@75,40,end:touch:-@75,40,press:touch:-@75,40",
        )
        .await?;
    Ok(())
}

/// Keyboard press events are at the element's center ("should return the center of the element
/// when keyboard pressed").
#[browser_test]
pub async fn keyboard_coordinates(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let target = page.element("#test-press-detail").await?;
    target.focus().await?;
    page.wait_for_focus(&target).await?;
    page.send_keys(" ").await?;
    detail_log(page)
        .await?
        .wait_for_inner_text(
            "start:keyboard:-@50,25,up:keyboard:-@50,25,end:keyboard:-@50,25,\
             press:keyboard:-@50,25",
        )
        .await?;
    Ok(())
}

/// A press cancelled by a pointer cancel (heard on the document) ends at the cancel's position
/// relative to the pressed element.
#[browser_test]
pub async fn cancel_coordinates(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let target = page.element("#test-press-detail").await?;
    let (down_x, down_y) = point_in(&target, 25.0, 5.0).await?;
    let (cancel_x, cancel_y) = point_in(&target, 30.0, 10.0).await?;
    target
        .dispatch(SyntheticEvent::pointer(PointerKind::Down).at(down_x, down_y))
        .await?;
    target
        .dispatch(SyntheticEvent::pointer(PointerKind::Cancel).at(cancel_x, cancel_y))
        .await?;
    detail_log(page)
        .await?
        .wait_for_inner_text("start:mouse:-@25,5,end:mouse:-@30,10")
        .await?;
    Ok(())
}

/// A mouse click focuses the pressed element ("should focus the element on click by default").
#[browser_test]
pub async fn click_focuses_the_element(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let target = page.element("#test-press-target").await?;
    target.click().await?;
    page.wait_for_focus(&target).await?;
    Ok(())
}

/// A virtual pointer down and up (VoiceOver on iOS: a zero-sized pointer) press nothing; the click
/// that follows presses as virtual ("should ignore virtual pointer events").
#[browser_test]
pub async fn virtual_pointer_events_are_ignored(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let target = page.element("#test-press-target").await?;
    for kind in [PointerKind::Down, PointerKind::Up] {
        target
            .dispatch(SyntheticEvent::pointer(kind).size(0.0, 0.0))
            .await?;
    }
    let log = log(page).await?;
    log.inner_text_stays("", Duration::from_millis(100)).await?;
    target
        .dispatch(SyntheticEvent::mouse(MouseKind::Click))
        .await?;
    log.wait_for_inner_text("start:virtual,up:virtual,end:virtual,press:virtual")
        .await?;
    Ok(())
}

/// On Android, a zero-sized pointer is a real touch: it presses as a mouse ("should not ignore
/// virtual pointer events on android").
#[browser_test]
pub async fn zero_sized_pointers_press_on_android(page: &Page<'_>) -> Result<(), Report> {
    page.emulate_platform(Platform::Android).await?;
    page.goto_path(PATH).await?;
    let target = page.element("#test-press-target").await?;
    target
        .dispatch(SyntheticEvent::pointer(PointerKind::Down).size(0.0, 0.0))
        .await?;
    target
        .dispatch_both(
            SyntheticEvent::pointer(PointerKind::Up).size(0.0, 0.0),
            SyntheticEvent::mouse(MouseKind::Click),
        )
        .await?;
    log(page)
        .await?
        .wait_for_inner_text("start:mouse,up:mouse,end:mouse,press:mouse")
        .await?;
    Ok(())
}

/// On Android, TalkBack's double tap (a 1×1 mouse pointer without pressure) is virtual: its pointer
/// events press nothing, its click presses as virtual ("should detect Android TalkBack double
/// tap").
#[browser_test]
pub async fn talkback_double_tap(page: &Page<'_>) -> Result<(), Report> {
    page.emulate_platform(Platform::Android).await?;
    page.goto_path(PATH).await?;
    let target = page.element("#test-press-target").await?;
    for kind in [PointerKind::Down, PointerKind::Up] {
        target.dispatch(SyntheticEvent::pointer(kind)).await?;
    }
    let log = log(page).await?;
    log.inner_text_stays("", Duration::from_millis(100)).await?;
    target
        .dispatch(SyntheticEvent::mouse(MouseKind::Click))
        .await?;
    log.wait_for_inner_text("start:virtual,up:virtual,end:virtual,press:virtual")
        .await?;
    Ok(())
}

/// Elsewhere than on Android, a 1×1 mouse pointer without pressure is a real one: it presses (here
/// completed by the click fallback) ("should fire if pressure is 0 but is not android").
#[browser_test]
pub async fn pressure_zero_presses_elsewhere(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let target = page.element("#test-press-target").await?;
    for kind in [PointerKind::Down, PointerKind::Up] {
        target.dispatch(SyntheticEvent::pointer(kind)).await?;
    }
    log(page)
        .await?
        .wait_for_inner_text("start:mouse,up:mouse,end:mouse,press:mouse")
        .await?;
    Ok(())
}

/// After a virtual pointer down, a real press on the element counts as a pointer again: a later
/// pointer up over the element without a press of its own fires press up.
#[browser_test]
pub async fn real_pointer_after_a_virtual_one(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let target = page.element("#test-press-target").await?;
    let elsewhere = page.element("#test-press-elsewhere").await?;
    target
        .dispatch(SyntheticEvent::pointer(PointerKind::Down).size(0.0, 0.0))
        .await?;
    target.click().await?;
    let log = log(page).await?;
    log.wait_for_inner_text("start:mouse,up:mouse,end:mouse,press:mouse")
        .await?;
    // A drag from elsewhere ending on the element.
    elsewhere
        .dispatch(SyntheticEvent::pointer(PointerKind::Down))
        .await?;
    target
        .dispatch(SyntheticEvent::pointer(PointerKind::Up))
        .await?;
    log.wait_for_inner_text("start:mouse,up:mouse,end:mouse,press:mouse,up:mouse")
        .await?;
    Ok(())
}

/// A script click while a pointer press is dragged out of the element ends that press (press up,
/// no press) and restores text selection, instead of pressing as a virtual click.
#[browser_test]
pub async fn virtual_click_during_a_pointer_press(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let target = page.element("#test-press-target").await?;
    let elsewhere = page.element("#test-press-elsewhere").await?;
    let held = target.press_and_hold().await?;
    let log = log(page).await?;
    log.wait_for_inner_text("start:mouse").await?;
    held.move_to(&elsewhere).await?;
    log.wait_for_inner_text("start:mouse,end:mouse").await?;
    target.virtual_click().await?;
    log.wait_for_inner_text("start:mouse,end:mouse,up:mouse")
        .await?;
    assert_that!(|| target.css_value("user-select"))
        .eventually_ok()
        .matches(eq("auto"))
        .await;
    held.release().await?;
    log.inner_text_stays("start:mouse,end:mouse,up:mouse", Duration::from_millis(300))
        .await?;
    Ok(())
}

/// Space and Enter typed into a pressable text input (no `type` attribute) type and press nothing.
#[browser_test]
pub async fn typing_in_a_text_input_presses_nothing(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = page.element("#test-press-text-input").await?;
    input.focus().await?;
    page.wait_for_focus(&input).await?;
    page.send_keys("a b").await?;
    page.send_keys(Key::Enter).await?;
    input.wait_for_prop("value", "a b").await?;
    page.element("#test-press-text-input-log")
        .await?
        .inner_text_stays("", Duration::from_millis(100))
        .await?;
    Ok(())
}

/// Space on a link with a button role and two press hooks opens the link once (react-aria's
/// `LINK_CLICKED` marker).
#[browser_test]
pub async fn two_press_hooks_open_a_link_once(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let link = page.element("#test-press-two-link").await?;
    link.focus().await?;
    page.wait_for_focus(&link).await?;
    page.send_keys(" ").await?;
    let clicks = page.element("#test-press-two-link-clicks").await?;
    clicks.wait_for_inner_text("1").await?;
    clicks
        .inner_text_stays("1", Duration::from_millis(100))
        .await?;
    Ok(())
}

/// A click a press up callback causes on its own element presses nothing more ("should ignore
/// synthetic events fired during an onPressUp event").
#[browser_test]
pub async fn clicks_during_press_up_are_ignored(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element("#test-press-reentrant").await?.click().await?;
    let log = page.element("#test-press-reentrant-log").await?;
    log.wait_for_inner_text("start:mouse,up:mouse,end:mouse,press:mouse")
        .await?;
    log.inner_text_stays(
        "start:mouse,up:mouse,end:mouse,press:mouse",
        Duration::from_millis(300),
    )
    .await?;
    Ok(())
}

/// A touch press or release on `element`.
async fn touch(element: &WebElement, kind: PointerKind) -> Result<(), Report> {
    element
        .dispatch(SyntheticEvent::pointer(kind).pointer_type(PointerType::Touch))
        .await?;
    Ok(())
}

/// The page's (`<html>`) computed `user-select`.
async fn page_user_select(page: &Page<'_>) -> Result<String, Report> {
    Ok(page.element("html").await?.css_value("user-select").await?)
}

/// On iOS, a press start disables text selection on the whole page ("should add user-select:
/// none to the page on press start (iOS)").
#[browser_test]
pub async fn ios_press_start_disables_page_selection(page: &Page<'_>) -> Result<(), Report> {
    page.emulate_platform(Platform::IPhone).await?;
    page.goto_path(PATH).await?;
    assert_that!(page_user_select(page).await?).is_equal_to("auto");
    touch(
        &page.element("#test-press-target").await?,
        PointerKind::Down,
    )
    .await?;
    assert_that!(|| page_user_select(page))
        .eventually_ok()
        .matches(eq("none"))
        .await;
    Ok(())
}

/// Elsewhere than on iOS, a press start leaves the page's text selection alone (the pressed
/// element's is disabled) ("should not add user-select: none to the page when press start
/// (non-iOS)").
#[browser_test]
pub async fn press_start_leaves_page_selection_elsewhere(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let target = page.element("#test-press-target").await?;
    touch(&target, PointerKind::Down).await?;
    assert_that!(|| target.css_value("user-select"))
        .eventually_ok()
        .matches(eq("none"))
        .await;
    assert_that!(page_user_select(page).await?).is_equal_to("auto");
    Ok(())
}

/// On iOS, the page's text selection comes back shortly (300 ms) after the press ends ("should
/// remove user-select: none from the page when press end (iOS)").
#[browser_test]
pub async fn ios_press_end_restores_page_selection(page: &Page<'_>) -> Result<(), Report> {
    page.emulate_platform(Platform::IPhone).await?;
    page.goto_path(PATH).await?;
    let target = page.element("#test-press-target").await?;
    touch(&target, PointerKind::Down).await?;
    touch(&target, PointerKind::Up).await?;
    log(page)
        .await?
        .wait_for_inner_text("start:touch,up:touch,end:touch,press:touch")
        .await?;
    assert_that!(|| page_user_select(page))
        .eventually_ok()
        .matches(eq("auto"))
        .await;
    Ok(())
}

/// On iOS, pressing a second element within the restore delay of the first keeps the page's text
/// selection disabled until the second press ends ("should not remove user-select: none when
/// pressing two different elements quickly (iOS)").
#[browser_test]
pub async fn ios_quick_second_press_keeps_page_selection_disabled(
    page: &Page<'_>,
) -> Result<(), Report> {
    page.emulate_platform(Platform::IPhone).await?;
    page.goto_path(PATH).await?;
    let first = page.element("#test-press-target").await?;
    let second = page.element("#test-press-detail").await?;
    touch(&first, PointerKind::Down).await?;
    first
        .dispatch_both(
            SyntheticEvent::pointer(PointerKind::Up).pointer_type(PointerType::Touch),
            SyntheticEvent::mouse(MouseKind::Click),
        )
        .await?;
    touch(&second, PointerKind::Down).await?;
    // Past the first press' restore delay (300 ms).
    assert_that!(|| page_user_select(page))
        .consistently_ok()
        .for_at_least(Duration::from_millis(500))
        .matches(eq("none"))
        .await;
    second
        .dispatch_both(
            SyntheticEvent::pointer(PointerKind::Up).pointer_type(PointerType::Touch),
            SyntheticEvent::mouse(MouseKind::Click),
        )
        .await?;
    assert_that!(|| page_user_select(page))
        .eventually_ok()
        .matches(eq("auto"))
        .await;
    Ok(())
}

/// On iOS, a pressable removed while pressed restores the page's text selection ("should remove
/// user-select: none from the page if pressable component unmounts (iOS)").
#[browser_test]
pub async fn ios_unmount_restores_page_selection(page: &Page<'_>) -> Result<(), Report> {
    page.emulate_platform(Platform::IPhone).await?;
    page.goto_path(PATH).await?;
    touch(
        &page.element("#test-press-removed").await?,
        PointerKind::Down,
    )
    .await?;
    page.wait_for_count("#test-press-removed", 0).await?;
    assert_that!(|| page_user_select(page))
        .eventually_ok()
        .matches(eq("auto"))
        .await;
    Ok(())
}

/// Two elements pressed at once each get their text selection back when their own press ends
/// ("should clean up user-select: none when pressing and releasing two different elements
/// (non-iOS)").
#[browser_test]
pub async fn two_presses_restore_their_own_selection(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let first = page.element("#test-press-target").await?;
    let second = page.element("#test-press-detail").await?;
    let pointer = |kind: PointerKind, id: i32| {
        SyntheticEvent::pointer(kind)
            .pointer_type(PointerType::Touch)
            .pointer_id(id)
    };
    first.dispatch(pointer(PointerKind::Down, 1)).await?;
    second.dispatch(pointer(PointerKind::Down, 2)).await?;
    assert_that!(|| first.css_value("user-select"))
        .eventually_ok()
        .matches(eq("none"))
        .await;
    assert_that!(|| second.css_value("user-select"))
        .eventually_ok()
        .matches(eq("none"))
        .await;
    first
        .dispatch_both(
            pointer(PointerKind::Up, 1),
            SyntheticEvent::mouse(MouseKind::Click),
        )
        .await?;
    assert_that!(|| first.css_value("user-select"))
        .eventually_ok()
        .matches(eq("auto"))
        .await;
    assert_that!(second.css_value("user-select").await?).is_equal_to("none");
    second
        .dispatch_both(
            pointer(PointerKind::Up, 2),
            SyntheticEvent::mouse(MouseKind::Click),
        )
        .await?;
    assert_that!(|| second.css_value("user-select"))
        .eventually_ok()
        .matches(eq("auto"))
        .await;
    Ok(())
}

/// Other style changes during a press stay when the press restores the text selection ("non
/// related style changes during press down shouldn't overwrite user-select on press end
/// (non-iOS)").
#[browser_test]
pub async fn style_changes_during_a_press_stay(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let target = page.element("#test-press-style-background").await?;
    target.click().await?;
    assert_that!(|| target.css_value("background-color"))
        .eventually_ok()
        .matches(eq("rgba(255, 0, 0, 1)"))
        .await;
    assert_that!(|| target.css_value("user-select"))
        .eventually_ok()
        .matches(eq("auto"))
        .await;
    assert_that!(target.css_value("background-color").await?).is_equal_to("rgba(255, 0, 0, 1)");
    Ok(())
}

/// A `user-select` set during a press stays when the press ends ("changes to user-select during
/// press down remain on press end (non-iOS)").
#[browser_test]
pub async fn user_select_set_during_a_press_stays(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let target = page.element("#test-press-style-user-select").await?;
    target.click().await?;
    assert_that!(|| target.css_value("background-color"))
        .eventually_ok()
        .matches(eq("rgba(255, 0, 0, 1)"))
        .await;
    assert_that!(|| target.css_value("user-select"))
        .consistently_ok()
        .for_at_least(Duration::from_millis(100))
        .matches(eq("text"))
        .await;
    Ok(())
}

/// Space on a pressable checkbox presses it without preventing its toggle; Enter presses nothing
/// ("should fire press events on checkboxes but not prevent default").
#[browser_test]
pub async fn space_on_a_checkbox(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let checkbox = page.element("#test-press-checkbox").await?;
    checkbox.focus().await?;
    page.wait_for_focus(&checkbox).await?;
    let log = page.element("#test-press-checkbox-log").await?;
    page.send_keys(Key::Enter).await?;
    log.inner_text_stays("", Duration::from_millis(100)).await?;
    page.send_keys(" ").await?;
    log.wait_for_inner_text("start:keyboard,up:keyboard,end:keyboard,press:keyboard")
        .await?;
    checkbox.wait_for_prop("checked", "true").await?;
    Ok(())
}

/// Enter presses a link once and follows it (its click); Space neither presses nor follows it
/// ("should fire press events when the element is a link").
#[browser_test]
pub async fn enter_on_a_link(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let link = page.element("#test-press-plain-link").await?;
    link.focus().await?;
    page.wait_for_focus(&link).await?;
    let log = page.element("#test-press-plain-link-log").await?;
    let clicks = page.element("#test-press-plain-link-clicks").await?;
    page.send_keys(" ").await?;
    log.inner_text_stays("", Duration::from_millis(100)).await?;
    assert_that!(clicks.inner_text().await?).is_equal_to("0");
    page.send_keys(Key::Enter).await?;
    log.wait_for_inner_text("start:keyboard,up:keyboard,end:keyboard,press:keyboard")
        .await?;
    clicks
        .inner_text_stays("1", Duration::from_millis(100))
        .await?;
    Ok(())
}

/// An element with the link role is pressed by Enter, not by Space ("should fire press events on
/// Enter when the element role is link").
#[browser_test]
pub async fn enter_on_a_link_role(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let link = page.element("#test-press-role-link").await?;
    link.focus().await?;
    page.wait_for_focus(&link).await?;
    let log = page.element("#test-press-role-link-log").await?;
    page.send_keys(" ").await?;
    log.inner_text_stays("", Duration::from_millis(100)).await?;
    page.send_keys(Key::Enter).await?;
    log.wait_for_inner_text("start:keyboard,up:keyboard,end:keyboard,press:keyboard")
        .await?;
    Ok(())
}
