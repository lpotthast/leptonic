// Upstream: react-aria/test/interactions/usePress.test.js @ 99e6102368
//! `use_press` on a Mac (emulated: Chrome on Linux fires every key up): macOS fires no key up for
//! keys released while Meta is held, so releasing Meta ends their presses ("should fire press
//! events when Meta key is held to work around macOS bug").

use assertr::prelude::*;
use browser_test::thirtyfour::prelude::*;
use rootcause::Report;

use crate::pages::{ElementActions, Page, PageActions, SyntheticEvent};

/// Opens the fixture in a browser that says it runs on a Mac. The user agent is set before the
/// page loads: the platform is detected once.
async fn open_on_a_mac(page: &Page<'_>) -> Result<(), Report> {
    page.driver
        .cdp()
        .send_raw(
            "Emulation.setUserAgentOverride",
            serde_json::json!({
                "userAgent": "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) \
                              AppleWebKit/537.36 (KHTML, like Gecko) Chrome/140.0.0.0 \
                              Safari/537.36",
                "platform": "MacIntel",
                "userAgentMetadata": {
                    "platform": "macOS",
                    "platformVersion": "15.0.0",
                    "architecture": "arm",
                    "model": "",
                    "mobile": false,
                    "brands": [{ "brand": "Chromium", "version": "140" }],
                },
            }),
        )
        .await?;
    page.goto_path("/hooks/press").await?;
    Ok(())
}

/// Meta held, Enter pressed; Meta released while Enter is still down: the press ends at the Meta
/// key up, the Enter key up adds nothing.
pub async fn meta_release_ends_held_key_presses(page: &Page<'_>) -> Result<(), Report> {
    open_on_a_mac(page).await?;
    let platform: String = page.eval("return navigator.platform;", vec![]).await?;
    assert_that!(platform).is_equal_to("MacIntel");

    clear_log(page).await?;
    page.element("#test-press-target").await?.focus().await?;
    page.wait_for_focus(&page.element("#test-press-target").await?)
        .await?;
    // Meta held, Enter pressed; Meta released while Enter is still down.
    page.driver
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
    page.driver
        .action_chain()
        .key_up(Key::Enter)
        .perform()
        .await?;
    log(page)
        .await?
        .inner_text_stays("start:keyboard,up:keyboard,end:keyboard,press:keyboard")
        .await?;
    Ok(())
}

/// The log of the main target's press events.
async fn log(page: &Page<'_>) -> Result<WebElement, Report> {
    page.element("#test-press-log").await
}

/// Clears the log of the main target.
async fn clear_log(page: &Page<'_>) -> Result<(), Report> {
    page.element("#test-press-clear").await?.click().await?;
    log(page).await?.wait_for_inner_text("").await?;
    Ok(())
}

/// "should fire press events based on pointer events": start, up, end, press, in order; text
/// selection is disabled on the pressed element (not on the text inside it) and restored.
pub async fn mouse_click_fires_all_events_in_order(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/hooks/press").await?;
    clear_log(page).await?;
    page.element("#test-press-target").await?.click().await?;
    log(page)
        .await?
        .wait_for_inner_text("start:mouse,up:mouse,end:mouse,press:mouse")
        .await?;
    page.element("#test-press-is-pressed")
        .await?
        .wait_for_inner_text("false")
        .await?;
    // react-aria's `state.target`: the pressed element, not the text inside it.
    let label = page.element("#test-press-label").await?;
    assert_that!(label.attr("style").await?).is_none();
    let target = page.element("#test-press-target").await?;
    assert_that!(target.attr("data-leptonic-saved-user-select").await?).is_none();
    Ok(())
}

/// "should fire press events for keyboard events": Enter and Space each press once.
pub async fn keyboard_enter_and_space_press(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/hooks/press").await?;
    // Focus the target first, then start from a clean log.
    page.element("#test-press-target").await?.click().await?;
    clear_log(page).await?;
    page.element("#test-press-target")
        .await?
        .send_keys(Key::Enter)
        .await?;
    log(page)
        .await?
        .wait_for_inner_text("start:keyboard,up:keyboard,end:keyboard,press:keyboard")
        .await?;

    clear_log(page).await?;
    page.element("#test-press-target")
        .await?
        .send_keys(" ")
        .await?;
    log(page)
        .await?
        .wait_for_inner_text("start:keyboard,up:keyboard,end:keyboard,press:keyboard")
        .await?;
    Ok(())
}

/// "should fire press change events when moving pointer outside target": released outside, the
/// press ends without a press (also none from the click fallback).
pub async fn releasing_outside_does_not_press(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/hooks/press").await?;
    clear_log(page).await?;
    let target = page.element("#test-press-target").await?;
    let elsewhere = page.element("#test-press-elsewhere").await?;

    page.driver
        .action_chain()
        .move_to_element_center(&target)
        .click_and_hold()
        .perform()
        .await?;
    log(page).await?.wait_for_inner_text("start:mouse").await?;
    page.element("#test-press-is-pressed")
        .await?
        .wait_for_inner_text("true")
        .await?;

    page.driver
        .action_chain()
        .move_to_element_center(&elsewhere)
        .release()
        .perform()
        .await?;
    log(page)
        .await?
        .wait_for_inner_text("start:mouse,end:mouse")
        .await?;
    page.element("#test-press-is-pressed")
        .await?
        .wait_for_inner_text("false")
        .await?;
    // No press after all (the click fallback mustn't fire one either).
    log(page)
        .await?
        .inner_text_stays("start:mouse,end:mouse")
        .await?;
    Ok(())
}

/// "should not fire press events when disabled".
pub async fn disabled_element_ignores_presses(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/hooks/press").await?;
    page.element("#test-press-toggle-disabled")
        .await?
        .click()
        .await?;
    clear_log(page).await?;
    page.element("#test-press-target").await?.click().await?;
    log(page).await?.inner_text_stays("").await?;
    page.element("#test-press-toggle-disabled")
        .await?
        .click()
        .await?;
    Ok(())
}

/// Upstream fix (react-spectrum #9813): an element that becomes disabled while pressed must end
/// the press instead of staying pressed (a spin button would otherwise keep spinning).
pub async fn becoming_disabled_cancels_active_press(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/hooks/press").await?;
    let target = page.element("#test-press-self-disabling").await?;
    page.driver
        .action_chain()
        .move_to_element_center(&target)
        .click_and_hold()
        .perform()
        .await?;

    // The press ends while the pointer is still held down.
    page.element("#test-press-self-disabling-log")
        .await?
        .wait_for_inner_text("start:mouse,end:mouse")
        .await?;
    page.element("#test-press-self-disabling-is-pressed")
        .await?
        .wait_for_inner_text("false")
        .await?;

    page.driver.action_chain().release().perform().await?;
    page.element("#test-press-self-disabling-log")
        .await?
        .inner_text_stays("start:mouse,end:mouse")
        .await?;
    Ok(())
}

/// Upstream fix (react-spectrum #9972): Enter on a checkbox must trigger the form's implicit
/// submission instead of being swallowed.
pub async fn enter_on_checkbox_submits_form(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/hooks/press").await?;
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

/// "event bubbling: should stop propagation by default": the outer pressable sees nothing of a
/// press on the inner one.
pub async fn nested_press_stops_by_default(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/hooks/press").await?;
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
        .inner_text_stays("")
        .await?;
    Ok(())
}

/// "event bubbling: should allow propagation if continuePropagation is called": the outer
/// pressable is pressed along with the inner one.
pub async fn nested_press_propagates_when_continued(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/hooks/press").await?;
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

/// "should not focus the element on click if preventFocusOnPress is true": the focus stays on
/// the previously focused input, which sees no blur, and the press still happens.
pub async fn prevent_focus_on_press_keeps_the_focus(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/hooks/press").await?;
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
    page.element("#test-press-keep-blurs")
        .await?
        .inner_text_stays("0")
        .await?;
    Ok(())
}

/// The document listens to the keyup in the capture phase (as react-aria): a keyup handler on the
/// pressed element that stops the event (`use_keyboard` with `on_key_up`) doesn't leave the press
/// stuck.
pub async fn keyboard_press_ends_when_key_up_is_stopped(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/hooks/press").await?;
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

/// A drag starting inside the pressable cancels its press (react-aria's `onDragStart`), with the
/// drag event: press end runs (no panic on a constructed event without a current target), no press.
pub async fn a_drag_inside_cancels_the_press(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/hooks/press").await?;
    let image = page.element("#test-press-drag-image").await?;
    page.driver
        .action_chain()
        .move_to_element_center(&image)
        .click_and_hold()
        .perform()
        .await?;
    page.element("#test-press-drag-log")
        .await?
        .wait_for_inner_text("start")
        .await?;
    image.dispatch(SyntheticEvent::drag("dragstart")).await?;
    page.element("#test-press-drag-log")
        .await?
        .wait_for_inner_text("start,end")
        .await?;
    page.driver.action_chain().release().perform().await?;
    page.element("#test-press-drag-log")
        .await?
        .inner_text_stays("start,end")
        .await?;
    Ok(())
}

/// "should cancel press if onClick propagation is stopped": a child stops the click, so the click
/// fallback (80 ms after pointer up) cancels the press instead of completing it.
pub async fn a_child_stopping_the_click_cancels_the_press(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/hooks/press").await?;
    let log = page.element("#test-press-click-stop-log").await?;
    page.element("#test-press-click-stop-child")
        .await?
        .click()
        .await?;
    log.wait_for_inner_text("start:mouse,end:mouse").await?;
    log.inner_text_stays("start:mouse,end:mouse").await?;
    Ok(())
}

/// "should handle when focus moves between keydown and keyup": the key up happens on another
/// element (focus moved there on press start): press end without press up or press.
pub async fn focus_moving_before_key_up_ends_without_press(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/hooks/press").await?;
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
    log.inner_text_stays("start:keyboard,end:keyboard").await?;
    Ok(())
}

/// "should ignore repeating keyboard events".
pub async fn repeating_key_downs_are_ignored(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/hooks/press").await?;
    clear_log(page).await?;
    let target = page.element("#test-press-target").await?;
    target
        .dispatch(SyntheticEvent::keyboard("keydown", " ").with("repeat", true))
        .await?;
    page.element("body")
        .await?
        .dispatch(SyntheticEvent::keyboard("keyup", " "))
        .await?;
    log(page).await?.inner_text_stays("").await?;
    Ok(())
}

/// "should fire press change events when moving pointer outside target", on a `display: contents`
/// element (no box: the pointer's position can't be compared with its bounds).
pub async fn dragging_out_and_back_in(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/hooks/press").await?;
    let log = page.element("#test-press-contents-log").await?;
    let child = page.element("#test-press-contents-child").await?;
    let elsewhere = page.element("#test-press-elsewhere").await?;
    page.driver
        .action_chain()
        .move_to_element_center(&child)
        .click_and_hold()
        .perform()
        .await?;
    log.wait_for_inner_text("start:mouse").await?;
    elsewhere.hover().await?;
    log.wait_for_inner_text("start:mouse,end:mouse").await?;
    child.hover().await?;
    log.wait_for_inner_text("start:mouse,end:mouse,start:mouse")
        .await?;
    page.driver.action_chain().release().perform().await?;
    log.wait_for_inner_text("start:mouse,end:mouse,start:mouse,up:mouse,end:mouse,press:mouse")
        .await?;
    Ok(())
}

/// "should cancel press when moving outside and the shouldCancelOnPointerExit option is set".
pub async fn cancel_on_pointer_exit(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/hooks/press").await?;
    let log = page.element("#test-press-cancel-exit-log").await?;
    let target = page.element("#test-press-cancel-exit").await?;
    let elsewhere = page.element("#test-press-elsewhere").await?;
    page.driver
        .action_chain()
        .move_to_element_center(&target)
        .click_and_hold()
        .perform()
        .await?;
    log.wait_for_inner_text("start:mouse").await?;
    elsewhere.hover().await?;
    log.wait_for_inner_text("start:mouse,end:mouse").await?;
    target.hover().await?;
    // Back over the element: the press doesn't start again.
    log.inner_text_stays("start:mouse,end:mouse").await?;
    // Released there: a pointer up without a press (react-aria's element `onPointerUp`), no press.
    page.driver.action_chain().release().perform().await?;
    log.wait_for_inner_text("start:mouse,end:mouse,up:mouse")
        .await?;
    log.inner_text_stays("start:mouse,end:mouse,up:mouse")
        .await?;
    Ok(())
}

/// "should handle pointer cancel events".
pub async fn pointer_cancel_cancels_the_press(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/hooks/press").await?;
    clear_log(page).await?;
    let target = page.element("#test-press-target").await?;
    page.driver
        .action_chain()
        .move_to_element_center(&target)
        .click_and_hold()
        .perform()
        .await?;
    log(page).await?.wait_for_inner_text("start:mouse").await?;
    target
        .dispatch(
            SyntheticEvent::pointer("pointercancel")
                .with("pointerId", 1)
                .with("pointerType", "mouse"),
        )
        .await?;
    log(page)
        .await?
        .wait_for_inner_text("start:mouse,end:mouse")
        .await?;
    // Released over the element: a pointer up without a press (react-aria's element `onPointerUp`
    // fires press up when no press is active), no press.
    page.driver.action_chain().release().perform().await?;
    log(page)
        .await?
        .wait_for_inner_text("start:mouse,end:mouse,up:mouse")
        .await?;
    log(page)
        .await?
        .inner_text_stays("start:mouse,end:mouse,up:mouse")
        .await?;
    Ok(())
}

/// "should explicitly call click method when Space key is triggered on a link with href and
/// role=button": one press (the link's click opened by the press isn't a second, virtual press)
/// and the link is followed.
pub async fn space_on_a_link_with_button_role(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/hooks/press").await?;
    let log = page.element("#test-press-link-log").await?;
    page.element("#test-press-link").await?.focus().await?;
    page.wait_for_focus(&page.element("#test-press-link").await?)
        .await?;
    page.send_keys(" ").await?;
    log.wait_for_inner_text("start:keyboard,up:keyboard,end:keyboard,press:keyboard")
        .await?;
    log.inner_text_stays("start:keyboard,up:keyboard,end:keyboard,press:keyboard")
        .await?;
    let url = page.driver.current_url().await?;
    assert_that!(url.fragment()).is_equal_to(Some("test-press-link-target"));
    Ok(())
}

/// `on_double_press` fires on a double click, after both presses; also through press props merged
/// with hover props.
pub async fn double_press(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/hooks/press").await?;
    let target = page.element("#test-press-double").await?;
    page.driver
        .action_chain()
        .double_click_element(&target)
        .perform()
        .await?;
    page.element("#test-press-double-log")
        .await?
        .wait_for_inner_text("press,press,double:mouse")
        .await?;
    // With press and hover props merged too.
    let merged = page.element("#test-press-double-hover").await?;
    page.driver
        .action_chain()
        .double_click_element(&merged)
        .perform()
        .await?;
    page.element("#test-press-double-hover-log")
        .await?
        .wait_for_inner_text("double")
        .await?;
    Ok(())
}

/// `PressPropagation::Continue`: every press event of the inner pressable propagates, so the
/// outer one is pressed too.
pub async fn press_propagation_continue(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/hooks/press").await?;
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

/// "should fire press events events for virtual click events from screen readers" (a click
/// without a pointer: `element.click()`).
pub async fn virtual_click(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/hooks/press").await?;
    clear_log(page).await?;
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

/// A pressable removed while pressed: no press, no panic when the pointer is released.
pub async fn removed_while_pressed(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/hooks/press").await?;
    let target = page.element("#test-press-removed").await?;
    page.driver
        .action_chain()
        .move_to_element_center(&target)
        .click_and_hold()
        .perform()
        .await?;
    page.wait_for_count("#test-press-removed", 0).await?;
    page.driver.action_chain().release().perform().await?;
    page.element("#test-press-removed-log")
        .await?
        .inner_text_stays("start:mouse")
        .await?;
    Ok(())
}
