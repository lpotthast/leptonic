// Upstream: react-aria/test/interactions/useKeyboard.test.js @ 99e6102368
//! `use_keyboard`: handlers, disabled, propagation (stopped by default, continued on request or
//! by unhandled shortcuts), shortcuts ignoring repeats/composing/keyup unless allowed, and two
//! hooks on one element stopping propagation if any of them does.
use browser_test::{browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::pages::{ElementActions, KeyKind, Page, SyntheticEvent, interface::Keyboard};

const PATH: &str = "/hooks/keyboard";

/// The log of key events and shortcut actions.
async fn log(page: &Page<'_>) -> Result<WebElement, Report> {
    page.element("#test-keyboard-log").await
}

/// The element `#test-keyboard-<name>`.
async fn target(page: &Page<'_>, name: &str) -> Result<WebElement, Report> {
    page.element(format!("#test-keyboard-{name}")).await
}

/// Focuses `#test-keyboard-<name>`, presses `keys` there and checks that the log becomes
/// `expected` (the element's handlers and how far the events propagated) and stays so.
async fn press_logs(
    page: &Page<'_>,
    name: &str,
    keys: impl Into<TypingData> + Send,
    expected: &str,
) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let element = target(page, name).await?;
    element.focus().await?;
    page.wait_for_focus(&element).await?;
    page.send_keys(keys).await?;
    let log = log(page).await?;
    log.wait_for_inner_text(expected).await?;
    log.inner_text_stays(expected, std::time::Duration::from_millis(100))
        .await?;
    Ok(())
}

/// A key on an element runs its keydown and keyup handlers, and the events don't bubble to the
/// wrapper ("should handle keyboard events", "events do not bubble by default").
#[browser_test]
pub async fn handles_keyboard_events(page: &Page<'_>) -> Result<(), Report> {
    press_logs(page, "basic", "a", "basic:keydown:a,basic:keyup:a").await
}

/// A key on a disabled element runs none of its handlers and bubbles to the wrapper ("should not
/// handle events when disabled").
#[browser_test]
pub async fn disabled(page: &Page<'_>) -> Result<(), Report> {
    press_logs(
        page,
        "disabled",
        "a",
        "disabled-wrapper:keydown:a,disabled-wrapper:keyup:a",
    )
    .await
}

/// A handler that continues propagation lets each event reach the wrapper too ("events bubble
/// when continuePropagation is called").
#[browser_test]
pub async fn continue_propagation(page: &Page<'_>) -> Result<(), Report> {
    press_logs(
        page,
        "continue",
        "a",
        "continue:keydown:a,continue-wrapper:keydown:a,continue:keyup:a,\
         continue-wrapper:keyup:a",
    )
    .await
}

/// The element's own keydown and keyup handlers run around its shortcut's action ("chains a
/// user-provided onKeyDown and onKeyUp with shortcuts").
#[browser_test]
pub async fn shortcut_chained_with_handlers(page: &Page<'_>) -> Result<(), Report> {
    press_logs(
        page,
        "shortcut",
        "a",
        "shortcut:keydown:a,shortcut:action,shortcut:keyup:a",
    )
    .await
}

/// A shortcut whose action doesn't handle the key gets the event and lets it bubble to the wrapper
/// ("passes event to handler", "continues propagation if the function did not handle the event").
#[browser_test]
pub async fn unhandled_shortcut_continues(page: &Page<'_>) -> Result<(), Report> {
    press_logs(
        page,
        "ignored",
        Key::Escape,
        "ignored:action,ignored-wrapper:keydown:Escape,ignored-wrapper:keyup:Escape",
    )
    .await
}

/// A shortcut can let its key bubble to the wrapper without preventing its default ("prevent
/// default and stop propagation can both be finely controlled").
#[browser_test]
pub async fn prevent_default_and_propagation_controlled(page: &Page<'_>) -> Result<(), Report> {
    press_logs(
        page,
        "custom",
        Key::Escape,
        "custom:action,custom-wrapper:keydown:Escape,custom-wrapper:keyup:Escape",
    )
    .await
}

/// The keydown doesn't reach the wrapper when one of the shortcuts handling the key stops it
/// ("should stop propagation if any shortcut handling that key stops propagation").
#[browser_test]
pub async fn stop_if_a_shortcut_of_the_key_stops(page: &Page<'_>) -> Result<(), Report> {
    press_logs(
        page,
        "stop-other-key",
        Key::Left,
        "stop-other-key:action,stop-other-key-wrapper:keyup:ArrowLeft",
    )
    .await
}

/// The keydown bubbles to the wrapper, default prevented, when every shortcut handling the key
/// continues it ("should continue propagation if all shortcuts that handle that key agree to
/// continue propagation").
#[browser_test]
pub async fn continue_if_all_shortcuts_of_the_key_continue(page: &Page<'_>) -> Result<(), Report> {
    press_logs(
        page,
        "continue-other-key",
        Key::Left,
        "continue-other-key:action,continue-other-key-wrapper:keydown:ArrowLeft:prevented,\
         continue-other-key-wrapper:keyup:ArrowLeft",
    )
    .await
}

/// With two hooks on one element, the keydown doesn't reach the wrapper when one of their
/// shortcuts stops it ("should stop propagation if any shortcut stops propagation").
#[browser_test]
pub async fn stop_if_any_shortcut_stops(page: &Page<'_>) -> Result<(), Report> {
    press_logs(
        page,
        "stop-any",
        Key::Left,
        "stop-any:action,stop-any:action,stop-any-wrapper:keyup:ArrowLeft",
    )
    .await
}

/// With two hooks on one element, the keydown bubbles to the wrapper when all their shortcuts
/// continue it ("should continue propagation if all shortcuts agree to continue propagation").
#[browser_test]
pub async fn continue_if_all_shortcuts_continue(page: &Page<'_>) -> Result<(), Report> {
    press_logs(
        page,
        "continue-all",
        Key::Left,
        "continue-all:action,continue-all:action,\
         continue-all-wrapper:keydown:ArrowLeft:prevented,continue-all-wrapper:keyup:ArrowLeft",
    )
    .await
}

/// A key no shortcut handles bubbles to the wrapper.
#[browser_test]
pub async fn unhandled_key_bubbles(page: &Page<'_>) -> Result<(), Report> {
    press_logs(
        page,
        "repeats",
        "b",
        "repeats-wrapper:keydown:b,repeats-wrapper:keyup:b",
    )
    .await
}

/// A `keydown` of "a".
fn keydown_a() -> SyntheticEvent<Keyboard> {
    SyntheticEvent::keyboard(KeyKind::Down, "a")
}

/// Dispatches the keyboard `event` on `#test-keyboard-<name>` and
/// checks that the log becomes `expected` and stays so.
async fn keydown_logs(
    page: &Page<'_>,
    name: &str,
    event: SyntheticEvent<Keyboard>,
    expected: &str,
) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    target(page, name).await?.dispatch(event).await?;
    let log = log(page).await?;
    log.wait_for_inner_text(expected).await?;
    log.inner_text_stays(expected, std::time::Duration::from_millis(100))
        .await?;
    Ok(())
}

/// A repeated keydown runs the keydown handler, not the shortcut ("ignores repeated keydown events
/// by default (allowRepeats: false)").
#[browser_test]
pub async fn ignores_repeats_by_default(page: &Page<'_>) -> Result<(), Report> {
    keydown_logs(
        page,
        "shortcut",
        keydown_a().repeat(true),
        "shortcut:keydown:a",
    )
    .await
}

/// A shortcut allowing repeats runs on a repeated keydown ("handles repeated keydown events when
/// allowRepeats is true").
#[browser_test]
pub async fn handles_repeats_when_allowed(page: &Page<'_>) -> Result<(), Report> {
    keydown_logs(page, "repeats", keydown_a().repeat(true), "repeats:action").await
}

/// A keydown while composing runs the keydown handler, not the shortcut ("ignores composing
/// keydown events by default (allowComposing: false)").
#[browser_test]
pub async fn ignores_composing_by_default(page: &Page<'_>) -> Result<(), Report> {
    keydown_logs(
        page,
        "shortcut",
        keydown_a().composing(true),
        "shortcut:keydown:a",
    )
    .await
}

/// A shortcut allowing composing runs on a keydown while composing ("handles composing keydown
/// events when allowComposing is true").
#[browser_test]
pub async fn handles_composing_when_allowed(page: &Page<'_>) -> Result<(), Report> {
    keydown_logs(
        page,
        "composing",
        keydown_a().composing(true),
        "composing:action",
    )
    .await
}

/// Keyups, also repeated and composing ones, run no shortcut and bubble to the wrapper ("does not
/// run shortcuts on keyup, including repeated and composing keyups").
#[browser_test]
pub async fn no_shortcuts_on_keyup(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let repeats = target(page, "repeats").await?;
    let keyup_a = || SyntheticEvent::keyboard(KeyKind::Up, "a");
    for event in [keyup_a(), keyup_a().repeat(true), keyup_a().composing(true)] {
        repeats.dispatch(event).await?;
    }
    // Only the wrapper's keyup handler runs, for each of them: no shortcut action.
    log(page)
        .await?
        .wait_for_inner_text(
            "repeats-wrapper:keyup:a,repeats-wrapper:keyup:a,repeats-wrapper:keyup:a",
        )
        .await?;
    log(page)
        .await?
        .inner_text_stays(
            "repeats-wrapper:keyup:a,repeats-wrapper:keyup:a,repeats-wrapper:keyup:a",
            std::time::Duration::from_millis(100),
        )
        .await?;
    Ok(())
}
