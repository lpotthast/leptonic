// Upstream: react-aria/test/interactions/useFocusVisible.test.js @ 99e6102368
// Upstream: react-aria-components/test/Form.test.js @ 99e6102368
//! `use_focus_visible`: the interaction modality (keyboard, pointer, virtual) and whether focus
//! should be visible, including the text input filter, window refocus and invalid forms. Every
//! case starts on a fresh page.
use browser_test::thirtyfour::prelude::*;
use rootcause::Report;

use crate::pages::{ElementActions, Page, PageActions};

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
        .inner_text_stays(modality)
        .await?;
    page.element("#test-fv-visible")
        .await?
        .inner_text_stays(&visible.to_string())
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

/// Click target: modality=Pointer, visible=false.
pub async fn click_sets_pointer_modality(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    click_with_pointer(page, TARGET).await?;
    page.element("#test-fv-pointer-type")
        .await?
        .wait_for_inner_text("mouse")
        .await?;
    Ok(())
}

/// Tab to target: modality=Keyboard, visible=true.
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

/// Click target (pointer), then press ArrowDown: modality switches to Keyboard, visible=true.
pub async fn arrow_key_sets_keyboard_modality(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    click_with_pointer(page, TARGET).await?;
    page.send_keys(Key::Down).await?;
    expect_state(page, "Keyboard", true).await?;
    Ok(())
}

/// Click target (not a text input), then type "a": modality switches to Keyboard (type-ahead).
pub async fn typing_on_non_text_input_sets_keyboard_modality(
    page: &Page<'_>,
) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    click_with_pointer(page, TARGET).await?;
    page.send_keys("a").await?;
    expect_state(page, "Keyboard", true).await?;
    Ok(())
}

/// Click text input (pointer), then type "a": modality stays Pointer (text input filter).
pub async fn typing_in_text_input_does_not_set_keyboard_modality(
    page: &Page<'_>,
) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    click_with_pointer(page, TEXT_INPUT).await?;
    page.send_keys("a").await?;
    expect_state_stays(page, "Pointer", false).await?;
    Ok(())
}

/// Click text input, type "a": the subscriber isn't notified (stays Pointer), but the stored
/// (global) modality is Keyboard, as react-aria's `currentModality`.
pub async fn typing_in_text_input_silently_updates_stored_modality(
    page: &Page<'_>,
) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let stored = page.element("#test-fv-stored-modality").await?;
    click_with_pointer(page, TEXT_INPUT).await?;
    stored.wait_for_inner_text("Pointer").await?;

    page.send_keys("a").await?;
    stored.wait_for_inner_text("Keyboard").await?;
    expect_state_stays(page, "Pointer", false).await?;
    Ok(())
}

/// Click target (pointer), then press Escape: modality switches to Keyboard, visible=true.
pub async fn escape_sets_keyboard_modality(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    click_with_pointer(page, TARGET).await?;
    page.send_keys(Key::Escape).await?;
    expect_state(page, "Keyboard", true).await?;
    Ok(())
}

/// Click target (pointer), then press Enter: modality switches to Keyboard, visible=true.
pub async fn enter_sets_keyboard_modality(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    click_with_pointer(page, TARGET).await?;
    page.send_keys(Key::Enter).await?;
    expect_state(page, "Keyboard", true).await?;
    Ok(())
}

/// Click target (pointer), then press Space: modality switches to Keyboard, visible=true.
pub async fn space_sets_keyboard_modality(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    click_with_pointer(page, TARGET).await?;
    page.send_keys(" ").await?;
    expect_state(page, "Keyboard", true).await?;
    Ok(())
}

/// Tab to target (keyboard), then click target: modality switches to Pointer, visible=false.
pub async fn pointer_after_keyboard(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    tab_to_target(page).await?;
    expect_state(page, "Keyboard", true).await?;
    click_with_pointer(page, TARGET).await?;
    Ok(())
}

/// A focus event without a preceding keyboard or pointer event (e.g. a screen reader moving
/// focus, the iOS form navigation) switches to virtual modality, which shows focus.
pub async fn focus_without_a_preceding_event_is_virtual(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let target = page.element(TARGET).await?;
    click_with_pointer(page, BEFORE).await?;
    page.eval::<()>(NATIVE_FOCUS, vec![]).await?;
    page.eval::<()>("__nativeFocus(arguments[0]);", vec![target.to_json()?])
        .await?;
    page.wait_for_focus(&target).await?;
    expect_state(page, "Virtual", true).await?;
    Ok(())
}

/// Programmatic `focus()` calls don't change the modality (react-aria's `focus` override).
pub async fn programmatic_focus_keeps_the_modality(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let target = page.element(TARGET).await?;
    click_with_pointer(page, BEFORE).await?;
    target.focus().await?;
    page.wait_for_focus(&target).await?;
    expect_state_stays(page, "Pointer", false).await?;
    Ok(())
}

/// Leaving and returning to the window keeps the modality: useFocusVisible.test.js, "returns
/// positive/negative isFocusVisible result after toggling browser window".
pub async fn window_refocus_keeps_the_modality(page: &Page<'_>) -> Result<(), Report> {
    refocus_keeps_the_modality(page, TOGGLE_WINDOW).await
}

/// Safari's second element focus mustn't switch to virtual modality. useFocusVisible.test.js,
/// "... after toggling browser tabs in Safari ...".
pub async fn safari_window_refocus_keeps_the_modality(page: &Page<'_>) -> Result<(), Report> {
    refocus_keeps_the_modality(page, TOGGLE_TABS_SAFARI).await
}

/// Run `toggle` (a window refocus) after keyboard focus and after pointer focus: the modality
/// stays.
async fn refocus_keeps_the_modality(page: &Page<'_>, toggle: &str) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let target = page.element(TARGET).await?;
    page.eval::<()>(NATIVE_FOCUS, vec![]).await?;

    tab_to_target(page).await?;
    expect_state(page, "Keyboard", true).await?;
    page.eval::<()>(toggle, vec![]).await?;
    page.wait_for_focus(&target).await?;
    expect_state_stays(page, "Keyboard", true).await?;

    click_with_pointer(page, TARGET).await?;
    page.eval::<()>(toggle, vec![]).await?;
    page.wait_for_focus(&target).await?;
    expect_state_stays(page, "Pointer", false).await?;
    Ok(())
}

/// A forms library moving focus to the first invalid field right after the form became invalid
/// shows focus there: Form.test.js, "shows focus-visible when a form library moves focus to the
/// first invalid field on submit".
pub async fn focus_moved_after_invalid_shows_focus(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let required = page.element("#test-fv-required").await?;
    click_with_pointer(page, TARGET).await?;
    // In one task, as a forms library does on submit.
    page.eval::<()>(
        "arguments[0].checkValidity(); arguments[0].focus();",
        vec![required.to_json()?],
    )
    .await?;
    page.wait_for_focus(&required).await?;
    expect_state(page, "Keyboard", true).await?;
    Ok(())
}
