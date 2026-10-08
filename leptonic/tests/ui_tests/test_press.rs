// Upstream: react-aria/test/interactions/usePress.test.js @ 99e6102368
use std::borrow::Cow;

use assertr::prelude::*;
use browser_test::{
    BrowserTest, async_trait,
    thirtyfour::{Key, WebDriver},
};
use rootcause::Report;

use crate::pages::{BaseActions, Page};

pub struct PressTests {}

#[async_trait]
impl BrowserTest<str> for PressTests {
    fn name(&self) -> Cow<'_, str> {
        "press_tests".into()
    }

    async fn run(&self, driver: &WebDriver, base_url: &str) -> Result<(), Report> {
        let page = Page { driver, base_url };
        page.goto_path("/hooks/press").await?;

        mouse_click_fires_all_events_in_order(&page).await?;
        keyboard_enter_and_space_press(&page).await?;
        releasing_outside_does_not_press(&page).await?;
        disabled_element_ignores_presses(&page).await?;
        becoming_disabled_cancels_active_press(&page).await?;
        enter_on_checkbox_submits_form(&page).await?;
        prevent_focus_on_press_keeps_the_focus(&page).await?;
        nested_press_stops_by_default(&page).await?;
        nested_press_propagates_when_continued(&page).await?;
        keyboard_press_ends_when_key_up_is_stopped(&page).await?;
        a_drag_inside_cancels_the_press(&page).await?;
        a_child_stopping_the_click_cancels_the_press(&page).await?;
        focus_moving_before_key_up_ends_without_press(&page).await?;
        repeating_key_downs_are_ignored(&page).await?;
        dragging_out_and_back_in(&page).await?;
        cancel_on_pointer_exit(&page).await?;
        pointer_cancel_cancels_the_press(&page).await?;
        space_on_a_link_with_button_role(&page).await?;
        double_press(&page).await?;
        press_propagation_continue(&page).await?;
        virtual_click(&page).await?;
        removed_while_pressed(&page).await?;

        page.expect_no_page_errors().await
    }
}

/// `use_press` on a Mac (emulated: Chrome on Linux fires every key up): macOS fires no key up for
/// keys released while Meta is held, so releasing Meta ends their presses ("should fire press
/// events when Meta key is held to work around macOS bug").
pub struct PressMacTests {}

#[async_trait]
impl BrowserTest<str> for PressMacTests {
    fn name(&self) -> Cow<'_, str> {
        "press_mac_tests".into()
    }

    async fn run(&self, driver: &WebDriver, base_url: &str) -> Result<(), Report> {
        // Before the page loads: the platform is detected once.
        driver
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
        let page = Page { driver, base_url };
        page.goto_path("/hooks/press").await?;
        let is_mac = driver.execute("return navigator.platform;", vec![]).await?;
        assert_that!(is_mac.json().as_str()).is_equal_to(Some("MacIntel"));

        clear_log(&page).await?;
        page.element("test-press-target").await?.focus().await?;
        page.wait_for_active_id("test-press-target").await?;
        // Meta held, Enter pressed; Meta released while Enter is still down.
        page.driver
            .action_chain()
            .key_down(Key::Meta)
            .key_down(Key::Enter)
            .key_up(Key::Meta)
            .perform()
            .await?;
        page.wait_for_text(
            LOG,
            "start:keyboard,up:keyboard,end:keyboard,press:keyboard",
        )
        .await?;
        page.wait_for_text("test-press-is-pressed", "false").await?;
        page.driver
            .action_chain()
            .key_up(Key::Enter)
            .perform()
            .await?;
        expect_stays(
            &page,
            LOG,
            "start:keyboard,up:keyboard,end:keyboard,press:keyboard",
        )
        .await?;
        page.expect_no_page_errors().await
    }
}

/// A negative check: give a wrong update time to happen, then check the text again.
async fn expect_stays(page: &Page<'_>, id: &str, expected: &str) -> Result<(), Report> {
    stays!(
        format!("the text of #{id}"),
        expected.to_owned(),
        page.read_text_of(id).await?
    );
    Ok(())
}

const LOG: &str = "test-press-log";

async fn clear_log(page: &Page<'_>) -> Result<(), Report> {
    page.click_element_with_id("test-press-clear").await?;
    page.wait_for_text(LOG, "").await
}

async fn mouse_click_fires_all_events_in_order(page: &Page<'_>) -> Result<(), Report> {
    clear_log(page).await?;
    page.click_element_with_id("test-press-target").await?;
    page.wait_for_text(LOG, "start:mouse,up:mouse,end:mouse,press:mouse")
        .await?;
    assert_that!(page.read_bool("test-press-is-pressed").await?).is_false();
    // Text selection is disabled on the pressed element (not on the text inside it) and restored
    // (react-aria's `state.target`).
    let label = page.element("test-press-label").await?;
    assert_that!(label.attr("style").await?).is_none();
    let target = page.element("test-press-target").await?;
    assert_that!(target.attr("data-leptonic-saved-user-select").await?).is_none();
    Ok(())
}

async fn keyboard_enter_and_space_press(page: &Page<'_>) -> Result<(), Report> {
    // Focus the target first, then start from a clean log.
    page.click_element_with_id("test-press-target").await?;
    clear_log(page).await?;
    page.element("test-press-target")
        .await?
        .send_keys(Key::Enter)
        .await?;
    page.wait_for_text(
        LOG,
        "start:keyboard,up:keyboard,end:keyboard,press:keyboard",
    )
    .await?;

    clear_log(page).await?;
    page.element("test-press-target")
        .await?
        .send_keys(" ")
        .await?;
    page.wait_for_text(
        LOG,
        "start:keyboard,up:keyboard,end:keyboard,press:keyboard",
    )
    .await?;
    Ok(())
}

async fn releasing_outside_does_not_press(page: &Page<'_>) -> Result<(), Report> {
    clear_log(page).await?;
    let target = page.element("test-press-target").await?;
    let elsewhere = page.element("test-press-elsewhere").await?;

    page.driver
        .action_chain()
        .move_to_element_center(&target)
        .click_and_hold()
        .perform()
        .await?;
    page.wait_for_text(LOG, "start:mouse").await?;
    assert_that!(page.read_bool("test-press-is-pressed").await?).is_true();

    page.driver
        .action_chain()
        .move_to_element_center(&elsewhere)
        .release()
        .perform()
        .await?;
    page.wait_for_text(LOG, "start:mouse,end:mouse").await?;
    assert_that!(page.read_bool("test-press-is-pressed").await?).is_false();
    // No press after all (the click fallback mustn't fire one either).
    expect_stays(page, LOG, "start:mouse,end:mouse").await
}

async fn disabled_element_ignores_presses(page: &Page<'_>) -> Result<(), Report> {
    page.click_element_with_id("test-press-toggle-disabled")
        .await?;
    clear_log(page).await?;
    page.click_element_with_id("test-press-target").await?;
    // Give a (wrongly) handled press the chance to show up before asserting its absence.
    stays!("the log", String::new(), page.read_text_of(LOG).await?);
    page.click_element_with_id("test-press-toggle-disabled")
        .await?;
    Ok(())
}

/// Upstream fix (react-spectrum #9813): an element that becomes disabled while pressed must end
/// the press instead of staying pressed (a spin button would otherwise keep spinning).
async fn becoming_disabled_cancels_active_press(page: &Page<'_>) -> Result<(), Report> {
    let target = page.element("test-press-self-disabling").await?;
    page.driver
        .action_chain()
        .move_to_element_center(&target)
        .click_and_hold()
        .perform()
        .await?;

    // The press ends while the pointer is still held down.
    page.wait_for_text("test-press-self-disabling-log", "start:mouse,end:mouse")
        .await?;
    assert_that!(
        page.read_bool("test-press-self-disabling-is-pressed")
            .await?
    )
    .is_false();

    page.driver.action_chain().release().perform().await?;
    stays!(
        "the text of #test-press-self-disabling-log",
        "start:mouse,end:mouse".to_owned(),
        page.read_text_of("test-press-self-disabling-log").await?
    );
    Ok(())
}

/// Upstream fix (react-spectrum #9972): Enter on a checkbox must trigger the form's implicit
/// submission instead of being swallowed.
async fn enter_on_checkbox_submits_form(page: &Page<'_>) -> Result<(), Report> {
    page.click_element_with_id("test-press-checkbox").await?;
    page.element("test-press-checkbox")
        .await?
        .send_keys(Key::Enter)
        .await?;
    page.wait_for_text("test-press-submits", "1").await?;
    Ok(())
}

/// "event bubbling: should stop propagation by default": the outer pressable sees nothing of a
/// press on the inner one.
async fn nested_press_stops_by_default(page: &Page<'_>) -> Result<(), Report> {
    let id = "test-press-nested-stop";
    page.click_element_with_id(&format!("{id}-inner")).await?;
    page.wait_for_text(&format!("{id}-inner-log"), "start,up,end,press")
        .await?;
    page.send_keys_to_active(Key::Enter).await?;
    page.wait_for_text(
        &format!("{id}-inner-log"),
        "start,up,end,press,start,up,end,press",
    )
    .await?;
    assert_that!(page.read_text_of(&format!("{id}-outer-log")).await?).is_equal_to(String::new());
    Ok(())
}

/// "event bubbling: should allow propagation if continuePropagation is called": the outer
/// pressable is pressed along with the inner one.
async fn nested_press_propagates_when_continued(page: &Page<'_>) -> Result<(), Report> {
    let id = "test-press-nested-continue";
    page.click_element_with_id(&format!("{id}-inner")).await?;
    page.wait_for_text(&format!("{id}-inner-log"), "start,up,end,press")
        .await?;
    page.wait_for_text(
        &format!("{id}-outer-log"),
        "start:mouse,up:mouse,end:mouse,press:mouse",
    )
    .await?;
    Ok(())
}

/// "should not focus the element on click if preventFocusOnPress is true": the focus stays on
/// the previously focused input, which sees no blur, and the press still happens.
async fn prevent_focus_on_press_keeps_the_focus(page: &Page<'_>) -> Result<(), Report> {
    page.element("test-press-keep-input").await?.focus().await?;
    page.click_element_with_id("test-press-keep").await?;
    page.wait_for_text("test-press-keep-presses", "1").await?;
    page.wait_for_active_id("test-press-keep-input").await?;
    assert_that!(page.read_text_of("test-press-keep-blurs").await?).is_equal_to("0".to_owned());
    Ok(())
}

/// The document listens to the keyup in the capture phase (as react-aria): a keyup handler on the
/// pressed element that stops the event (`use_keyboard` with `on_key_up`) doesn't leave the press
/// stuck.
async fn keyboard_press_ends_when_key_up_is_stopped(page: &Page<'_>) -> Result<(), Report> {
    page.driver
        .execute(
            "document.getElementById('test-press-keyup').focus();",
            vec![],
        )
        .await?;
    page.wait_for_active_id("test-press-keyup").await?;
    page.send_keys_to_active(Key::Enter).await?;
    page.wait_for_text("test-press-keyup-presses", "1").await?;
    page.wait_for_text("test-press-keyup-pressed", "false")
        .await?;
    page.wait_for_text("test-press-keyup-key-ups", "1").await?;
    page.send_keys_to_active(" ").await?;
    page.wait_for_text("test-press-keyup-presses", "2").await?;
    page.wait_for_text("test-press-keyup-pressed", "false")
        .await
}

/// A drag starting inside the pressable cancels its press (react-aria's `onDragStart`), with the
/// drag event: press end runs (no panic on a constructed event without a current target), no press.
async fn a_drag_inside_cancels_the_press(page: &Page<'_>) -> Result<(), Report> {
    let image = page.element("test-press-drag-image").await?;
    page.driver
        .action_chain()
        .move_to_element_center(&image)
        .click_and_hold()
        .perform()
        .await?;
    page.wait_for_text("test-press-drag-log", "start").await?;
    page.driver
        .execute(
            "document.getElementById('test-press-drag-image').dispatchEvent(new DragEvent('dragstart', { bubbles: true }));",
            vec![],
        )
        .await?;
    page.wait_for_text("test-press-drag-log", "start,end")
        .await?;
    page.driver.action_chain().release().perform().await?;
    assert_that!(page.read_text_of("test-press-drag-log").await?)
        .is_equal_to("start,end".to_owned());
    page.expect_no_page_errors().await
}

/// "should cancel press if onClick propagation is stopped": a child stops the click, so the click
/// fallback (80 ms after pointer up) cancels the press instead of completing it.
async fn a_child_stopping_the_click_cancels_the_press(page: &Page<'_>) -> Result<(), Report> {
    let log = "test-press-click-stop-log";
    page.click_element_with_id("test-press-click-stop-child")
        .await?;
    page.wait_for_text(log, "start:mouse,end:mouse").await?;
    expect_stays(page, log, "start:mouse,end:mouse").await
}

/// "should handle when focus moves between keydown and keyup": the key up happens on another
/// element (focus moved there on press start): press end without press up or press.
async fn focus_moving_before_key_up_ends_without_press(page: &Page<'_>) -> Result<(), Report> {
    let log = "test-press-focus-move-log";
    page.element("test-press-focus-move").await?.focus().await?;
    page.wait_for_active_id("test-press-focus-move").await?;
    page.send_keys_to_active(" ").await?;
    page.wait_for_active_id("test-press-focus-move-other")
        .await?;
    page.wait_for_text(log, "start:keyboard,end:keyboard")
        .await?;
    expect_stays(page, log, "start:keyboard,end:keyboard").await
}

/// "should ignore repeating keyboard events".
async fn repeating_key_downs_are_ignored(page: &Page<'_>) -> Result<(), Report> {
    clear_log(page).await?;
    page.driver
        .execute(
            "const el = document.getElementById('test-press-target');
             el.dispatchEvent(new KeyboardEvent('keydown', { key: ' ', repeat: true, bubbles: true }));
             document.body.dispatchEvent(new KeyboardEvent('keyup', { key: ' ', bubbles: true }));",
            vec![],
        )
        .await?;
    expect_stays(page, LOG, "").await
}

/// "should fire press change events when moving pointer outside target", on a `display: contents`
/// element (no box: the pointer's position can't be compared with its bounds).
async fn dragging_out_and_back_in(page: &Page<'_>) -> Result<(), Report> {
    let log = "test-press-contents-log";
    let child = page.element("test-press-contents-child").await?;
    let elsewhere = page.element("test-press-elsewhere").await?;
    page.driver
        .action_chain()
        .move_to_element_center(&child)
        .click_and_hold()
        .perform()
        .await?;
    page.wait_for_text(log, "start:mouse").await?;
    page.driver
        .action_chain()
        .move_to_element_center(&elsewhere)
        .perform()
        .await?;
    page.wait_for_text(log, "start:mouse,end:mouse").await?;
    page.driver
        .action_chain()
        .move_to_element_center(&child)
        .perform()
        .await?;
    page.wait_for_text(log, "start:mouse,end:mouse,start:mouse")
        .await?;
    page.driver.action_chain().release().perform().await?;
    page.wait_for_text(
        log,
        "start:mouse,end:mouse,start:mouse,up:mouse,end:mouse,press:mouse",
    )
    .await
}

/// "should cancel press when moving outside and the shouldCancelOnPointerExit option is set".
async fn cancel_on_pointer_exit(page: &Page<'_>) -> Result<(), Report> {
    let log = "test-press-cancel-exit-log";
    let target = page.element("test-press-cancel-exit").await?;
    let elsewhere = page.element("test-press-elsewhere").await?;
    page.driver
        .action_chain()
        .move_to_element_center(&target)
        .click_and_hold()
        .perform()
        .await?;
    page.wait_for_text(log, "start:mouse").await?;
    page.driver
        .action_chain()
        .move_to_element_center(&elsewhere)
        .perform()
        .await?;
    page.wait_for_text(log, "start:mouse,end:mouse").await?;
    page.driver
        .action_chain()
        .move_to_element_center(&target)
        .perform()
        .await?;
    // Back over the element: the press doesn't start again.
    expect_stays(page, log, "start:mouse,end:mouse").await?;
    // Released there: a pointer up without a press (react-aria's element `onPointerUp`), no press.
    page.driver.action_chain().release().perform().await?;
    page.wait_for_text(log, "start:mouse,end:mouse,up:mouse")
        .await?;
    expect_stays(page, log, "start:mouse,end:mouse,up:mouse").await
}

/// "should handle pointer cancel events".
async fn pointer_cancel_cancels_the_press(page: &Page<'_>) -> Result<(), Report> {
    clear_log(page).await?;
    let target = page.element("test-press-target").await?;
    page.driver
        .action_chain()
        .move_to_element_center(&target)
        .click_and_hold()
        .perform()
        .await?;
    page.wait_for_text(LOG, "start:mouse").await?;
    page.driver
        .execute(
            "document.getElementById('test-press-target').dispatchEvent(\
             new PointerEvent('pointercancel', { pointerId: 1, pointerType: 'mouse', bubbles: true }));",
            vec![],
        )
        .await?;
    page.wait_for_text(LOG, "start:mouse,end:mouse").await?;
    // Released over the element: a pointer up without a press (react-aria's element `onPointerUp`
    // fires press up when no press is active), no press.
    page.driver.action_chain().release().perform().await?;
    page.wait_for_text(LOG, "start:mouse,end:mouse,up:mouse")
        .await?;
    expect_stays(page, LOG, "start:mouse,end:mouse,up:mouse").await
}

/// "should explicitly call click method when Space key is triggered on a link with href and
/// role=button": one press (the link's click opened by the press isn't a second, virtual press)
/// and the link is followed.
async fn space_on_a_link_with_button_role(page: &Page<'_>) -> Result<(), Report> {
    let log = "test-press-link-log";
    page.element("test-press-link").await?.focus().await?;
    page.wait_for_active_id("test-press-link").await?;
    page.send_keys_to_active(" ").await?;
    page.wait_for_text(
        log,
        "start:keyboard,up:keyboard,end:keyboard,press:keyboard",
    )
    .await?;
    expect_stays(
        page,
        log,
        "start:keyboard,up:keyboard,end:keyboard,press:keyboard",
    )
    .await?;
    let hash = page.driver.execute("return location.hash;", vec![]).await?;
    assert_that!(hash.json().as_str()).is_equal_to(Some("#test-press-link-target"));
    Ok(())
}

/// `on_double_press` fires on a double click, after both presses; also through press props merged
/// with hover props.
async fn double_press(page: &Page<'_>) -> Result<(), Report> {
    let target = page.element("test-press-double").await?;
    page.driver
        .action_chain()
        .double_click_element(&target)
        .perform()
        .await?;
    page.wait_for_text("test-press-double-log", "press,press,double:mouse")
        .await?;
    // With press and hover props merged too.
    let merged = page.element("test-press-double-hover").await?;
    page.driver
        .action_chain()
        .double_click_element(&merged)
        .perform()
        .await?;
    page.wait_for_text("test-press-double-hover-log", "double")
        .await
}

/// `PressPropagation::Continue`: every press event of the inner pressable propagates, so the
/// outer one is pressed too.
async fn press_propagation_continue(page: &Page<'_>) -> Result<(), Report> {
    page.click_element_with_id("test-press-continue-inner")
        .await?;
    page.wait_for_text("test-press-continue-inner-log", "press")
        .await?;
    page.wait_for_text("test-press-continue-outer-log", "press")
        .await
}

/// "should fire press events events for virtual click events from screen readers" (a click
/// without a pointer: `element.click()`).
async fn virtual_click(page: &Page<'_>) -> Result<(), Report> {
    clear_log(page).await?;
    page.driver
        .execute(
            "document.getElementById('test-press-target').click();",
            vec![],
        )
        .await?;
    page.wait_for_text(LOG, "start:virtual,up:virtual,end:virtual,press:virtual")
        .await
}

/// A pressable removed while pressed: no press, no panic when the pointer is released.
async fn removed_while_pressed(page: &Page<'_>) -> Result<(), Report> {
    let target = page.element("test-press-removed").await?;
    page.driver
        .action_chain()
        .move_to_element_center(&target)
        .click_and_hold()
        .perform()
        .await?;
    page.wait_for_no_selector("#test-press-removed").await?;
    page.driver.action_chain().release().perform().await?;
    expect_stays(page, "test-press-removed-log", "start:mouse").await
}
