// Upstream: react-aria/test/interactions/useFocusVisible.test.js @ 99e6102368
// Upstream: react-aria-components/test/Form.test.js @ 99e6102368
//! `use_focus_visible`: the interaction modality (keyboard, pointer, virtual) and whether focus
//! should be visible, including the text input filter, window refocus and invalid forms. Every
//! case starts on a fresh page.
use browser_test::{browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::pages::{ElementActions, EventKind, GlobalTarget, Page, SyntheticEvent};

const PATH: &str = "/hooks/focus-visible";
const BEFORE: &str = "#test-fv-before";
const TARGET: &str = "#test-fv-target";
const TEXT_INPUT: &str = "#test-fv-text-input";

/// Defines `window.__nativeFocus(element)`, which focuses `element` the way the browser restores
/// focus: a trusted focus event that doesn't go through the patched `HTMLElement.prototype.focus`
/// (the unpatched one of a fresh iframe), as react-aria's tests use the saved native `focus`.
const NATIVE_FOCUS: &str = "
    window.__nativeFocus = function (element) {
        const frame = document.createElement('iframe');
        document.body.appendChild(frame);
        frame.contentWindow.HTMLElement.prototype.focus.call(element);
        frame.remove();
    };
";

/// Chrome's order when leaving and returning to the window: window blur, window focus, then the
/// restored element focus.
const TOGGLE_WINDOW: &str = "
    const element = document.activeElement;
    element.blur();
    window.dispatchEvent(new FocusEvent('blur'));
    window.dispatchEvent(new FocusEvent('focus'));
    __nativeFocus(element);
";

/// Safari fires the window/element focus pair twice when returning to a tab.
const TOGGLE_TABS_SAFARI: &str = "
    const element = document.activeElement;
    element.blur();
    window.dispatchEvent(new FocusEvent('blur'));
    window.dispatchEvent(new FocusEvent('focus'));
    __nativeFocus(element);
    window.dispatchEvent(new FocusEvent('focus'));
    element.blur();
    __nativeFocus(element);
";

/// Wait until the reported modality is `modality` and focus visibility is `visible`.
async fn expect_state(page: &Page<'_>, modality: &str, visible: bool) -> Result<(), Report> {
    page.element("#test-fv-modality")
        .await?
        .wait_for_inner_text(modality)
        .await?;
    page.element("#test-fv-visible")
        .await?
        .wait_for_inner_text(&visible.to_string())
        .await?;
    Ok(())
}

/// Negative check: the modality stays `modality`, and focus visibility `visible`.
async fn expect_state_stays(page: &Page<'_>, modality: &str, visible: bool) -> Result<(), Report> {
    page.element("#test-fv-modality")
        .await?
        .inner_text_stays(modality, std::time::Duration::from_millis(100))
        .await?;
    page.element("#test-fv-visible")
        .await?
        .inner_text_stays(&visible.to_string(), std::time::Duration::from_millis(100))
        .await?;
    Ok(())
}

/// Click `before`, then Tab to the target: keyboard focus.
async fn tab_to_target(page: &Page<'_>) -> Result<(), Report> {
    page.element(BEFORE).await?.click().await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&page.element(TARGET).await?).await
}

/// Click `selector`, then wait for pointer modality.
async fn click_with_pointer(page: &Page<'_>, selector: &str) -> Result<(), Report> {
    page.element(selector).await?.click().await?;
    expect_state(page, "Pointer", false).await
}

/// Clicking the target switches to pointer modality (pointer type mouse) without visible focus.
#[browser_test]
pub async fn click_sets_pointer_modality(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    click_with_pointer(page, TARGET).await?;
    page.element("#test-fv-pointer-type")
        .await?
        .wait_for_inner_text("mouse")
        .await?;
    Ok(())
}

/// Tabbing to the target switches to keyboard modality (pointer type keyboard) with visible focus.
#[browser_test]
pub async fn tab_sets_keyboard_modality(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    tab_to_target(page).await?;
    expect_state(page, "Keyboard", true).await?;
    page.element("#test-fv-pointer-type")
        .await?
        .wait_for_inner_text("keyboard")
        .await?;
    Ok(())
}

/// Pressing ArrowDown on the clicked target switches to keyboard modality with visible focus.
#[browser_test]
pub async fn arrow_key_sets_keyboard_modality(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    click_with_pointer(page, TARGET).await?;
    page.send_keys(Key::Down).await?;
    expect_state(page, "Keyboard", true).await?;
    Ok(())
}

/// Typing a letter on the clicked target (not a text input) switches to keyboard modality with
/// visible focus ("emits on modality change (non-text input)").
#[browser_test]
pub async fn typing_on_non_text_input_sets_keyboard_modality(
    page: &Page<'_>,
) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    click_with_pointer(page, TARGET).await?;
    page.send_keys("a").await?;
    expect_state(page, "Keyboard", true).await?;
    Ok(())
}

/// Typing a letter in a clicked text input doesn't make focus visible, while the interaction
/// modality turns keyboard ("emits on modality change (text input)"; react-aria's
/// `useInteractionModality` follows every change).
#[browser_test]
pub async fn typing_in_text_input_does_not_show_focus(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    click_with_pointer(page, TEXT_INPUT).await?;
    page.send_keys("a").await?;
    page.element("#test-fv-modality")
        .await?
        .wait_for_inner_text("Keyboard")
        .await?;
    page.element("#test-fv-visible")
        .await?
        .inner_text_stays("false", std::time::Duration::from_millis(100))
        .await?;
    Ok(())
}

/// Typing a letter in a clicked text input sets the stored (global) modality to keyboard, as
/// react-aria's `currentModality`, without making focus visible.
#[browser_test]
pub async fn typing_in_text_input_silently_updates_stored_modality(
    page: &Page<'_>,
) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let stored = page.element("#test-fv-stored-modality").await?;
    click_with_pointer(page, TEXT_INPUT).await?;
    stored.wait_for_inner_text("Pointer").await?;

    page.send_keys("a").await?;
    stored.wait_for_inner_text("Keyboard").await?;
    expect_state_stays(page, "Keyboard", false).await?;
    Ok(())
}

/// A `beforeunload` that doesn't unload the page (Chrome fires it for `mailto:` links and
/// downloads) keeps the page's modality tracking: a key press afterwards still switches to
/// keyboard modality.
#[browser_test]
pub async fn beforeunload_keeps_tracking(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    click_with_pointer(page, TARGET).await?;
    page.dispatch_to(
        GlobalTarget::Window,
        SyntheticEvent::plain(EventKind::BeforeUnload),
    )
    .await?;
    page.send_keys(Key::Down).await?;
    expect_state(page, "Keyboard", true).await?;
    Ok(())
}

/// Pressing Escape on the clicked target switches to keyboard modality with visible focus.
#[browser_test]
pub async fn escape_sets_keyboard_modality(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    click_with_pointer(page, TARGET).await?;
    page.send_keys(Key::Escape).await?;
    expect_state(page, "Keyboard", true).await?;
    Ok(())
}

/// Pressing Enter on the clicked target switches to keyboard modality with visible focus.
#[browser_test]
pub async fn enter_sets_keyboard_modality(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    click_with_pointer(page, TARGET).await?;
    page.send_keys(Key::Enter).await?;
    expect_state(page, "Keyboard", true).await?;
    Ok(())
}

/// Pressing Space on the clicked target switches to keyboard modality with visible focus.
#[browser_test]
pub async fn space_sets_keyboard_modality(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    click_with_pointer(page, TARGET).await?;
    page.send_keys(" ").await?;
    expect_state(page, "Keyboard", true).await?;
    Ok(())
}

/// Clicking the target after tabbing to it switches from keyboard to pointer modality and hides
/// focus.
#[browser_test]
pub async fn pointer_after_keyboard(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    tab_to_target(page).await?;
    expect_state(page, "Keyboard", true).await?;
    click_with_pointer(page, TARGET).await?;
    Ok(())
}

/// A focus event without a preceding keyboard or pointer event (e.g. a screen reader moving
/// focus, the iOS form navigation) switches to virtual modality, which shows focus.
#[browser_test]
pub async fn focus_without_a_preceding_event_is_virtual(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let target = page.element(TARGET).await?;
    click_with_pointer(page, BEFORE).await?;
    page.low_level().eval::<()>(NATIVE_FOCUS, vec![]).await?;
    page.low_level()
        .eval::<()>("__nativeFocus(arguments[0]);", vec![target.to_json()?])
        .await?;
    page.wait_for_focus(&target).await?;
    expect_state(page, "Virtual", true).await?;
    Ok(())
}

/// A programmatic `focus()` call keeps the pointer modality and doesn't show focus (react-aria's
/// `focus` override).
#[browser_test]
pub async fn programmatic_focus_keeps_the_modality(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let target = page.element(TARGET).await?;
    click_with_pointer(page, BEFORE).await?;
    target.focus().await?;
    page.wait_for_focus(&target).await?;
    expect_state_stays(page, "Pointer", false).await?;
    Ok(())
}

/// Leaving and returning to the window keeps the keyboard or pointer modality ("returns positive
/// isFocusVisible result after toggling browser window after keyboard navigation", "returns
/// negative isFocusVisible result after toggling browser window without prior keyboard
/// navigation").
#[browser_test]
pub async fn window_refocus_keeps_the_modality(page: &Page<'_>) -> Result<(), Report> {
    refocus_keeps_the_modality(page, TOGGLE_WINDOW).await
}

/// Returning to a tab in Safari (two window and element focus pairs) keeps the keyboard or pointer
/// modality instead of switching to virtual ("returns positive isFocusVisible result after
/// toggling browser tabs in Safari after keyboard navigation", "returns negative isFocusVisible
/// result after toggling browser tabs in Safari without prior keyboard navigation").
#[browser_test]
pub async fn safari_window_refocus_keeps_the_modality(page: &Page<'_>) -> Result<(), Report> {
    refocus_keeps_the_modality(page, TOGGLE_TABS_SAFARI).await
}

/// Run `toggle` (a window refocus) after keyboard focus and after pointer focus: the modality
/// stays.
async fn refocus_keeps_the_modality(page: &Page<'_>, toggle: &str) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let target = page.element(TARGET).await?;
    page.low_level().eval::<()>(NATIVE_FOCUS, vec![]).await?;

    tab_to_target(page).await?;
    expect_state(page, "Keyboard", true).await?;
    page.low_level().eval::<()>(toggle, vec![]).await?;
    page.wait_for_focus(&target).await?;
    expect_state_stays(page, "Keyboard", true).await?;

    click_with_pointer(page, TARGET).await?;
    page.low_level().eval::<()>(toggle, vec![]).await?;
    page.wait_for_focus(&target).await?;
    expect_state_stays(page, "Pointer", false).await?;
    Ok(())
}

/// Focus moved to an invalid field right after its validity check (as a forms library does on
/// submit) is visible ("shows focus-visible when a form library moves focus to the first invalid
/// field on submit").
#[browser_test]
pub async fn focus_moved_after_invalid_shows_focus(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let required = page.element("#test-fv-required").await?;
    click_with_pointer(page, TARGET).await?;
    // In one task, as a forms library does on submit.
    page.low_level()
        .eval::<()>(
            "arguments[0].checkValidity(); arguments[0].focus();",
            vec![required.to_json()?],
        )
        .await?;
    page.wait_for_focus(&required).await?;
    expect_state(page, "Keyboard", true).await?;
    Ok(())
}

/// Waits for the iframe of the fixture, then focuses its button (from a control that keeps focus
/// where it is) and checks that its focus ring shows `visible`.
async fn focus_frame_button(page: &Page<'_>) -> Result<(), Report> {
    page.element("#test-fv-frame-ready")
        .await?
        .wait_for_inner_text("ready")
        .await?;
    page.element("#test-fv-frame-focus").await?.click().await?;
    page.element("#test-fv-frame-visible")
        .await?
        .wait_for_inner_text("false")
        .await?;
    Ok(())
}

/// Keys pressed in an iframe change nothing until its window is tracked
/// (`add_window_focus_tracking`); then Enter shows the focus ring of the iframe's button ("sets
/// up focus listener in a different window").
#[browser_test]
pub async fn other_window_tracking(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    focus_frame_button(page).await?;
    let visible = page.element("#test-fv-frame-visible").await?;
    page.send_keys(Key::Enter).await?;
    visible
        .inner_text_stays("false", std::time::Duration::from_millis(100))
        .await?;

    page.element("#test-fv-frame-track").await?.click().await?;
    page.send_keys(Key::Enter).await?;
    visible.wait_for_inner_text("true").await?;
    Ok(())
}

/// A `beforeunload` in the tracked iframe stops its tracking: keys there change nothing anymore
/// ("removes event listeners on beforeunload").
#[browser_test]
pub async fn other_window_beforeunload(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    focus_frame_button(page).await?;
    let visible = page.element("#test-fv-frame-visible").await?;
    page.element("#test-fv-frame-track").await?.click().await?;
    page.send_keys(Key::Enter).await?;
    visible.wait_for_inner_text("true").await?;

    page.element("#test-fv-frame-unload").await?.click().await?;
    visible.wait_for_inner_text("false").await?;
    page.send_keys(Key::Enter).await?;
    visible
        .inner_text_stays("false", std::time::Duration::from_millis(100))
        .await?;
    Ok(())
}

/// Dropping the tracking guard stops tracking the iframe ("removes event listeners using teardown
/// function").
#[browser_test]
pub async fn other_window_teardown(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    focus_frame_button(page).await?;
    let visible = page.element("#test-fv-frame-visible").await?;
    page.element("#test-fv-frame-track").await?.click().await?;
    page.send_keys(Key::Enter).await?;
    visible.wait_for_inner_text("true").await?;

    page.element("#test-fv-frame-stop").await?.click().await?;
    visible.wait_for_inner_text("false").await?;
    page.send_keys(Key::Enter).await?;
    visible
        .inner_text_stays("false", std::time::Duration::from_millis(100))
        .await?;
    Ok(())
}

/// In a tracked iframe, a click on its button followed by Escape shows its focus ring ("correctly
/// shifts focus to the iframe when the iframe is focused").
#[browser_test]
pub async fn other_window_pointer_then_escape(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    focus_frame_button(page).await?;
    page.element("#test-fv-frame-track").await?.click().await?;
    // The button fills the iframe: a click on the iframe hits it.
    page.element("iframe").await?.click().await?;
    let visible = page.element("#test-fv-frame-visible").await?;
    visible.wait_for_inner_text("false").await?;
    page.send_keys(Key::Escape).await?;
    visible.wait_for_inner_text("true").await?;
    Ok(())
}
