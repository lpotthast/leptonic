use leptonic::{
    IntoAttrs, KeyboardKey, KeyboardShortcuts, Propagation, Shortcut, ShortcutOutcome,
    hooks::interactions::{KeyboardEventWrapper, UseKeyboardInput, use_keyboard},
};
use leptos::{prelude::*, web_sys};

/// `use_keyboard` (react-aria's `useKeyboard.test.js`). Every example sits in a wrapper logging
/// the key events reaching it; everything goes to `#test-keyboard-log`.
#[component]
pub fn PageHookKeyboard() -> impl IntoView {
    let log = RwSignal::new(Vec::<String>::new());
    let push = move |entry: String| log.update(|l| l.push(entry));
    let handler = move |name: &'static str, continue_propagation: bool| {
        Some(Callback::new(move |e: KeyboardEventWrapper| {
            push(format!("{name}:{}:{}", e.event().type_(), e.key_value()));
            if continue_propagation {
                e.continue_propagation();
            }
        }))
    };
    let action = move |name: &'static str, outcome: ShortcutOutcome| {
        move |_: &web_sys::KeyboardEvent| {
            push(format!("{name}:action"));
            outcome
        }
    };
    let continue_only = ShortcutOutcome::Custom {
        prevent_default: true,
        continue_propagation: true,
    };

    let examples: Vec<(&'static str, UseKeyboardInput, Option<UseKeyboardInput>)> = vec![
        // "should handle keyboard events", "events do not bubble by default".
        (
            "basic",
            UseKeyboardInput {
                on_key_down: handler("basic", false),
                on_key_up: handler("basic", false),
                ..UseKeyboardInput::default()
            },
            None,
        ),
        // "should not handle events when disabled".
        (
            "disabled",
            UseKeyboardInput {
                is_disabled: true.into(),
                on_key_down: handler("disabled", false),
                on_key_up: handler("disabled", false),
                ..UseKeyboardInput::default()
            },
            None,
        ),
        // "events bubble when continuePropagation is called".
        (
            "continue",
            UseKeyboardInput {
                on_key_down: handler("continue", true),
                on_key_up: handler("continue", true),
                ..UseKeyboardInput::default()
            },
            None,
        ),
        // Repeats, composing, keyup, and chaining with `on_key_down`/`on_key_up`.
        (
            "shortcut",
            UseKeyboardInput {
                on_key_down: handler("shortcut", false),
                on_key_up: handler("shortcut", false),
                shortcuts: Some(KeyboardShortcuts::new().on(
                    Shortcut::new(KeyboardKey::A),
                    action("shortcut", ShortcutOutcome::Handled),
                )),
                ..UseKeyboardInput::default()
            },
            None,
        ),
        (
            "repeats",
            UseKeyboardInput {
                shortcuts: Some(KeyboardShortcuts::new().on(
                    Shortcut::new(KeyboardKey::A),
                    action("repeats", ShortcutOutcome::Handled),
                )),
                allow_repeats: true,
                ..UseKeyboardInput::default()
            },
            None,
        ),
        (
            "composing",
            UseKeyboardInput {
                shortcuts: Some(KeyboardShortcuts::new().on(
                    Shortcut::new(KeyboardKey::A),
                    action("composing", ShortcutOutcome::Handled),
                )),
                allow_composing: true,
                ..UseKeyboardInput::default()
            },
            None,
        ),
        // "continues propagation if the function did not handle the event".
        (
            "ignored",
            UseKeyboardInput {
                shortcuts: Some(KeyboardShortcuts::new().on(
                    Shortcut::new(KeyboardKey::Escape),
                    action("ignored", ShortcutOutcome::Ignored),
                )),
                ..UseKeyboardInput::default()
            },
            None,
        ),
        // "prevent default and stop propagation can both be finely controlled".
        (
            "custom",
            UseKeyboardInput {
                shortcuts: Some(KeyboardShortcuts::new().on(
                    Shortcut::new(KeyboardKey::Escape),
                    action(
                        "custom",
                        ShortcutOutcome::Custom {
                            prevent_default: false,
                            continue_propagation: true,
                        },
                    ),
                )),
                ..UseKeyboardInput::default()
            },
            None,
        ),
        // "Chaining and propagation": two hooks on one element.
        (
            "stop-other-key",
            shortcut_input(
                action("stop-other-key", ShortcutOutcome::Handled),
                KeyboardKey::ArrowLeft,
            ),
            Some(shortcut_input(
                action("stop-other-key", ShortcutOutcome::Handled),
                KeyboardKey::Enter,
            )),
        ),
        (
            "continue-other-key",
            shortcut_input(
                action("continue-other-key", continue_only),
                KeyboardKey::ArrowLeft,
            ),
            Some(shortcut_input(
                action("continue-other-key", ShortcutOutcome::Handled),
                KeyboardKey::Enter,
            )),
        ),
        (
            "stop-any",
            shortcut_input(action("stop-any", continue_only), KeyboardKey::ArrowLeft),
            Some(shortcut_input(
                action("stop-any", ShortcutOutcome::Handled),
                KeyboardKey::ArrowLeft,
            )),
        ),
        (
            "continue-all",
            shortcut_input(
                action("continue-all", continue_only),
                KeyboardKey::ArrowLeft,
            ),
            Some(shortcut_input(
                action("continue-all", continue_only),
                KeyboardKey::ArrowLeft,
            )),
        ),
    ];

    view! {
        <div id="test-page-hook-keyboard">
            <h1>"use_keyboard"</h1>
            {examples
                .into_iter()
                .map(|(name, first, second)| {
                    let first = use_keyboard(first).props;
                    let props = match second {
                        Some(second) => {
                            let second = use_keyboard(second).props;
                            leptonic::hooks::interactions::UseKeyboardProps {
                                on_keydown: first.on_keydown.chain(second.on_keydown),
                                on_keyup: first.on_keyup.chain(second.on_keyup),
                            }
                        }
                        None => first,
                    };
                    let wrapper = move |kind: &'static str| {
                        move |e: web_sys::KeyboardEvent| {
                            let prevented = if e.default_prevented() { ":prevented" } else { "" };
                            push(format!("{name}-wrapper:{kind}:{}{prevented}", e.key()));
                        }
                    };
                    view! {
                        <div on:keydown=wrapper("keydown") on:keyup=wrapper("keyup")>
                            <div id=format!("test-keyboard-{name}") tabindex="-1" {..props.into_attrs()}>
                                {name}
                            </div>
                        </div>
                    }
                })
                .collect_view()}
            <button id="test-keyboard-reset" on:click=move |_| log.set(Vec::new())>
                "Reset log"
            </button>
            <div>"Log: " <span id="test-keyboard-log">{move || log.get().join(",")}</span></div>
        </div>
    }
}

/// An input with one shortcut.
fn shortcut_input(
    action: impl Fn(&web_sys::KeyboardEvent) -> ShortcutOutcome + Send + Sync + 'static,
    key: KeyboardKey,
) -> UseKeyboardInput {
    UseKeyboardInput {
        shortcuts: Some(KeyboardShortcuts::new().on(Shortcut::new(key), action)),
        ..UseKeyboardInput::default()
    }
}
