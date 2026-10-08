// Upstream: react-aria/test/interactions/useKeyboard.test.js @ 99e6102368
//! `use_keyboard`: handlers, disabled, propagation (stopped by default, continued on request or
//! by unhandled shortcuts), shortcuts ignoring repeats/composing/keyup unless allowed, and two
//! hooks on one element stopping propagation if any of them does.
use browser_test::thirtyfour::prelude::*;
use rootcause::{Report, prelude::ResultExt};

use crate::pages::{ElementActions, Page, PageActions, SyntheticEvent};

const PATH: &str = "/hooks/keyboard";

/// The log of key events and shortcut actions.
async fn log(page: &Page<'_>) -> Result<WebElement, Report> {
    page.element("#test-keyboard-log").await
}

/// The element `#test-keyboard-<name>`.
async fn target(page: &Page<'_>, name: &str) -> Result<WebElement, Report> {
    page.element(format!("#test-keyboard-{name}")).await
}

/// Clears the log, by a script click.
async fn reset(page: &Page<'_>) -> Result<(), Report> {
    page.element("#test-keyboard-reset")
        .await?
        .virtual_click()
        .await?;
    log(page).await?.wait_for_inner_text("").await?;
    Ok(())
}

/// A key pressed on each fixture element logs its handlers and how far the events propagated.
pub async fn handlers_and_propagation(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let presses: [(&str, TypingData, &str); 11] = [
        // "should handle keyboard events", "events do not bubble by default".
        ("basic", "a".into(), "basic:keydown:a,basic:keyup:a"),
        // "should not handle events when disabled".
        (
            "disabled",
            "a".into(),
            "disabled-wrapper:keydown:a,disabled-wrapper:keyup:a",
        ),
        // "events bubble when continuePropagation is called".
        (
            "continue",
            "a".into(),
            "continue:keydown:a,continue-wrapper:keydown:a,continue:keyup:a,\
             continue-wrapper:keyup:a",
        ),
        // "chains a user-provided onKeyDown and onKeyUp with shortcuts".
        (
            "shortcut",
            "a".into(),
            "shortcut:keydown:a,shortcut:action,shortcut:keyup:a",
        ),
        // "passes event to handler", "continues propagation if the function did not handle the
        // event".
        (
            "ignored",
            Key::Escape.into(),
            "ignored:action,ignored-wrapper:keydown:Escape,ignored-wrapper:keyup:Escape",
        ),
        // "prevent default and stop propagation can both be finely controlled".
        (
            "custom",
            Key::Escape.into(),
            "custom:action,custom-wrapper:keydown:Escape,custom-wrapper:keyup:Escape",
        ),
        // "should stop propagation if any shortcut handling that key stops propagation".
        (
            "stop-other-key",
            Key::Left.into(),
            "stop-other-key:action,stop-other-key-wrapper:keyup:ArrowLeft",
        ),
        // "should continue propagation if all shortcuts that handle that key agree to continue
        // propagation".
        (
            "continue-other-key",
            Key::Left.into(),
            "continue-other-key:action,continue-other-key-wrapper:keydown:ArrowLeft:prevented,\
             continue-other-key-wrapper:keyup:ArrowLeft",
        ),
        // "should stop propagation if any shortcut stops propagation".
        (
            "stop-any",
            Key::Left.into(),
            "stop-any:action,stop-any:action,stop-any-wrapper:keyup:ArrowLeft",
        ),
        // "should continue propagation if all shortcuts agree to continue propagation".
        (
            "continue-all",
            Key::Left.into(),
            "continue-all:action,continue-all:action,\
             continue-all-wrapper:keydown:ArrowLeft:prevented,continue-all-wrapper:keyup:ArrowLeft",
        ),
        // A key no shortcut handles bubbles.
        (
            "repeats",
            "b".into(),
            "repeats-wrapper:keydown:b,repeats-wrapper:keyup:b",
        ),
    ];
    let log = log(page).await?;
    for (name, keys, expected) in presses {
        target(page, name).await?.focus().await?;
        page.send_keys(keys).await?;
        log.wait_for_inner_text(expected)
            .await
            .context_with(|| format!("a key pressed on #test-keyboard-{name}"))?;
        reset(page).await?;
    }
    Ok(())
}

/// "ignores repeated keydown events by default", "handles repeated keydown events when
/// allowRepeats is true", the same for composing.
pub async fn repeats_and_composing(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let log = log(page).await?;
    for (name, init, value, expected) in [
        ("shortcut", "repeat", true, "shortcut:keydown:a"),
        ("repeats", "repeat", true, "repeats:action"),
        ("shortcut", "isComposing", true, "shortcut:keydown:a"),
        ("composing", "isComposing", true, "composing:action"),
    ] {
        target(page, name)
            .await?
            .dispatch(SyntheticEvent::keyboard("keydown", "a").with(init, value))
            .await?;
        log.wait_for_inner_text(expected)
            .await
            .context_with(|| format!("a keydown with {init} on #test-keyboard-{name}"))?;
        reset(page).await?;
    }
    Ok(())
}

/// "does not run shortcuts on keyup", also not for repeated or composing ones.
pub async fn no_shortcuts_on_keyup(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let repeats = target(page, "repeats").await?;
    for (init, value) in [("repeat", false), ("repeat", true), ("isComposing", true)] {
        repeats
            .dispatch(SyntheticEvent::keyboard("keyup", "a").with(init, value))
            .await?;
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
        .inner_text_stays("repeats-wrapper:keyup:a,repeats-wrapper:keyup:a,repeats-wrapper:keyup:a")
        .await?;
    Ok(())
}
