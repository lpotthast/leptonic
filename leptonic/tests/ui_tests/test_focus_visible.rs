// Upstream: react-aria/test/interactions/useFocusVisible.test.js @ 99e6102368
// Upstream: react-aria-components/test/Form.test.js @ 99e6102368
use std::borrow::Cow;

use assertr::prelude::*;
use browser_test::{
    BrowserTest, async_trait,
    thirtyfour::{WebDriver, prelude::*},
};
use rootcause::Report;

use crate::pages::{BaseActions, focus_visible::FocusVisiblePage};

/// `use_focus_visible`: the interaction modality (keyboard, pointer, virtual) and whether focus
/// should be visible, including the text input filter, window refocus and invalid forms.
pub struct FocusVisibleTests {}

#[async_trait]
impl BrowserTest<str> for FocusVisibleTests {
    fn name(&self) -> Cow<'_, str> {
        "focus_visible_tests".into()
    }

    async fn run(&self, driver: &WebDriver, base_url: &str) -> Result<(), Report> {
        let page = FocusVisiblePage { driver, base_url };

        test_click_sets_pointer_modality(&page).await?;
        test_tab_sets_keyboard_modality(&page).await?;
        test_arrow_key_sets_keyboard_modality(&page).await?;
        test_typing_on_non_text_input_sets_keyboard_modality(&page).await?;
        test_typing_in_text_input_does_not_set_keyboard_modality(&page).await?;
        test_typing_in_text_input_silently_updates_stored_modality(&page).await?;
        test_pointer_after_keyboard(&page).await?;
        test_escape_sets_keyboard_modality(&page).await?;
        test_enter_sets_keyboard_modality(&page).await?;
        test_space_sets_keyboard_modality(&page).await?;
        test_focus_without_a_preceding_event_is_virtual(&page).await?;
        test_programmatic_focus_keeps_the_modality(&page).await?;
        test_window_refocus_keeps_the_modality(&page).await?;
        test_safari_window_refocus_keeps_the_modality(&page).await?;
        test_focus_moved_after_invalid_shows_focus(&page).await?;

        Ok(())
    }
}

async fn expect_state(
    page: &FocusVisiblePage<'_>,
    modality: &str,
    visible: bool,
) -> Result<(), Report> {
    page.wait_for_text("test-fv-modality", modality).await?;
    page.wait_for_text("test-fv-visible", if visible { "true" } else { "false" })
        .await
}

/// A negative check: give a wrong update time to happen, then check the state again.
async fn expect_state_stays(
    page: &FocusVisiblePage<'_>,
    modality: &str,
    visible: bool,
) -> Result<(), Report> {
    stays!(
        "the modality",
        modality.to_owned(),
        page.read_modality().await?
    );
    assert_that!(page.read_visible().await?).is_equal_to(visible);
    Ok(())
}

/// Focuses `id` the way the browser restores focus: a trusted focus event that doesn't go through
/// the patched `HTMLElement.prototype.focus` (the unpatched one of a fresh iframe), as
/// react-aria's tests use the saved native `focus`.
const NATIVE_FOCUS: &str = r"
    window.__nativeFocus = function (element) {
        const frame = document.createElement('iframe');
        document.body.appendChild(frame);
        frame.contentWindow.HTMLElement.prototype.focus.call(element);
        frame.remove();
    };
";

/// Click target: modality=Pointer, visible=false.
async fn test_click_sets_pointer_modality(page: &FocusVisiblePage<'_>) -> Result<(), Report> {
    page.goto().await?;

    page.click_target().await?;
    expect_state(page, "Pointer", false).await?;
    page.wait_for_text("test-fv-pointer-type", "mouse").await
}

/// Tab to target: modality=Keyboard, visible=true.
async fn test_tab_sets_keyboard_modality(page: &FocusVisiblePage<'_>) -> Result<(), Report> {
    page.goto().await?;

    page.tab_from_before_to_target().await?;
    expect_state(page, "Keyboard", true).await?;
    page.wait_for_text("test-fv-pointer-type", "keyboard").await
}

/// Click target (pointer), then press ArrowDown: modality switches to Keyboard, visible=true.
async fn test_arrow_key_sets_keyboard_modality(page: &FocusVisiblePage<'_>) -> Result<(), Report> {
    page.goto().await?;

    page.click_target().await?;
    expect_state(page, "Pointer", false).await?;

    page.send_keys_to_active(Key::Down).await?;
    expect_state(page, "Keyboard", true).await
}

/// Click target (not a text input), then type "a": modality switches to Keyboard (type-ahead).
async fn test_typing_on_non_text_input_sets_keyboard_modality(
    page: &FocusVisiblePage<'_>,
) -> Result<(), Report> {
    page.goto().await?;

    page.click_target().await?;
    expect_state(page, "Pointer", false).await?;

    page.send_keys_to_active("a").await?;
    expect_state(page, "Keyboard", true).await
}

/// Click text input (pointer), then type "a": modality stays Pointer (text input filter).
async fn test_typing_in_text_input_does_not_set_keyboard_modality(
    page: &FocusVisiblePage<'_>,
) -> Result<(), Report> {
    page.goto().await?;

    page.click_text_input().await?;
    expect_state(page, "Pointer", false).await?;

    page.send_keys_to_active("a").await?;
    expect_state_stays(page, "Pointer", false).await
}

/// Click text input, type "a": the subscriber isn't notified (stays Pointer), but the stored
/// (global) modality is Keyboard, as react-aria's `currentModality`.
async fn test_typing_in_text_input_silently_updates_stored_modality(
    page: &FocusVisiblePage<'_>,
) -> Result<(), Report> {
    page.goto().await?;

    page.click_text_input().await?;
    expect_state(page, "Pointer", false).await?;
    page.wait_for_text("test-fv-stored-modality", "Pointer")
        .await?;

    page.send_keys_to_active("a").await?;
    page.wait_for_text("test-fv-stored-modality", "Keyboard")
        .await?;
    expect_state_stays(page, "Pointer", false).await
}

/// Click target (pointer), then press Escape: modality switches to Keyboard, visible=true.
async fn test_escape_sets_keyboard_modality(page: &FocusVisiblePage<'_>) -> Result<(), Report> {
    page.goto().await?;

    page.click_target().await?;
    expect_state(page, "Pointer", false).await?;

    page.send_keys_to_active(Key::Escape).await?;
    expect_state(page, "Keyboard", true).await
}

/// Click target (pointer), then press Enter: modality switches to Keyboard, visible=true.
async fn test_enter_sets_keyboard_modality(page: &FocusVisiblePage<'_>) -> Result<(), Report> {
    page.goto().await?;

    page.click_target().await?;
    expect_state(page, "Pointer", false).await?;

    page.send_keys_to_active(Key::Enter).await?;
    expect_state(page, "Keyboard", true).await
}

/// Click target (pointer), then press Space: modality switches to Keyboard, visible=true.
async fn test_space_sets_keyboard_modality(page: &FocusVisiblePage<'_>) -> Result<(), Report> {
    page.goto().await?;

    page.click_target().await?;
    expect_state(page, "Pointer", false).await?;

    page.send_keys_to_active(" ").await?;
    expect_state(page, "Keyboard", true).await
}

/// Tab to target (keyboard), then click target: modality switches to Pointer, visible=false.
async fn test_pointer_after_keyboard(page: &FocusVisiblePage<'_>) -> Result<(), Report> {
    page.goto().await?;

    page.tab_from_before_to_target().await?;
    expect_state(page, "Keyboard", true).await?;

    page.click_target().await?;
    expect_state(page, "Pointer", false).await
}

/// A focus event without a preceding keyboard or pointer event (e.g. a screen reader moving
/// focus, the iOS form navigation) switches to virtual modality, which shows focus.
async fn test_focus_without_a_preceding_event_is_virtual(
    page: &FocusVisiblePage<'_>,
) -> Result<(), Report> {
    page.goto().await?;

    page.click_before().await?;
    expect_state(page, "Pointer", false).await?;
    page.driver
        .execute(
            &format!("{NATIVE_FOCUS} __nativeFocus(document.getElementById('test-fv-target'));"),
            vec![],
        )
        .await?;
    page.wait_for_active_id("test-fv-target").await?;
    expect_state(page, "Virtual", true).await
}

/// Programmatic `focus()` calls don't change the modality (react-aria's `focus` override).
async fn test_programmatic_focus_keeps_the_modality(
    page: &FocusVisiblePage<'_>,
) -> Result<(), Report> {
    page.goto().await?;

    page.click_before().await?;
    expect_state(page, "Pointer", false).await?;
    page.driver
        .execute("document.getElementById('test-fv-target').focus();", vec![])
        .await?;
    page.wait_for_active_id("test-fv-target").await?;
    expect_state_stays(page, "Pointer", false).await
}

/// Leaving and returning to the window (Chrome's order: window blur, window focus, then the
/// restored element focus) keeps the modality: useFocusVisible.test.js, "returns
/// positive/negative isFocusVisible result after toggling browser window".
async fn test_window_refocus_keeps_the_modality(page: &FocusVisiblePage<'_>) -> Result<(), Report> {
    const TOGGLE_WINDOW: &str = "
        const element = document.activeElement;
        element.blur();
        window.dispatchEvent(new FocusEvent('blur'));
        window.dispatchEvent(new FocusEvent('focus'));
        __nativeFocus(element);
    ";
    page.goto().await?;

    page.tab_from_before_to_target().await?;
    expect_state(page, "Keyboard", true).await?;
    page.driver
        .execute(&format!("{NATIVE_FOCUS} {TOGGLE_WINDOW}"), vec![])
        .await?;
    page.wait_for_active_id("test-fv-target").await?;
    expect_state_stays(page, "Keyboard", true).await?;

    page.click_target().await?;
    expect_state(page, "Pointer", false).await?;
    page.driver
        .execute(&format!("{NATIVE_FOCUS} {TOGGLE_WINDOW}"), vec![])
        .await?;
    page.wait_for_active_id("test-fv-target").await?;
    expect_state_stays(page, "Pointer", false).await
}

/// Safari fires the window/element focus pair twice when returning to a tab: the second element
/// focus mustn't switch to virtual modality. useFocusVisible.test.js, "... after toggling browser
/// tabs in Safari ...".
async fn test_safari_window_refocus_keeps_the_modality(
    page: &FocusVisiblePage<'_>,
) -> Result<(), Report> {
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
    page.goto().await?;

    page.tab_from_before_to_target().await?;
    expect_state(page, "Keyboard", true).await?;
    page.driver
        .execute(&format!("{NATIVE_FOCUS} {TOGGLE_TABS_SAFARI}"), vec![])
        .await?;
    page.wait_for_active_id("test-fv-target").await?;
    expect_state_stays(page, "Keyboard", true).await?;

    page.click_target().await?;
    expect_state(page, "Pointer", false).await?;
    page.driver
        .execute(&format!("{NATIVE_FOCUS} {TOGGLE_TABS_SAFARI}"), vec![])
        .await?;
    page.wait_for_active_id("test-fv-target").await?;
    expect_state_stays(page, "Pointer", false).await
}

/// A forms library moving focus to the first invalid field right after the form became invalid
/// shows focus there: Form.test.js, "shows focus-visible when a form library moves focus to the
/// first invalid field on submit".
async fn test_focus_moved_after_invalid_shows_focus(
    page: &FocusVisiblePage<'_>,
) -> Result<(), Report> {
    page.goto().await?;

    page.click_target().await?;
    expect_state(page, "Pointer", false).await?;
    page.driver
        .execute(
            "const input = document.getElementById('test-fv-required');
             input.checkValidity();
             input.focus();",
            vec![],
        )
        .await?;
    page.wait_for_active_id("test-fv-required").await?;
    expect_state(page, "Keyboard", true).await
}
