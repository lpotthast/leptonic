use leptos::prelude::*;
use web_sys::{KeyboardEvent, PointerEvent};

use crate::utils::{
    EventHandler,
    live_announcer::{Assertiveness, LiveAnnouncerContext},
};

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/@react-aria/spinbutton/src/useSpinButton.ts

//
// 1. PointerEvent-only: We assume PointerEvent is available (per CLAUDE.md).
//    No separate mouse/touch event handling. Touch vs mouse is detected via
//    `e.pointer_type()`.
//
// 2. Hook-owned state: The spin button does not own value state — it receives
//    callbacks for increment/decrement and signals for value display.
//
// 3. No separate `onIncrementPage`/`onDecrementPage` with custom step sizes.
//    PageUp/PageDown fall through to regular increment/decrement.
//

/// Auto-repeat timing constants (milliseconds), matching react-aria.
const INITIAL_DELAY_MOUSE_MS: i32 = 400;
const INITIAL_DELAY_TOUCH_MS: i32 = 600;
const REPEAT_INTERVAL_MS: i32 = 60;

/// Input parameters for the `use_spin_button` hook.
#[derive(Copy, Clone)]
pub struct UseSpinButtonInput {
    /// The current text value for screen reader announcements.
    pub text_value: Signal<String>,

    /// Whether the spin button is disabled.
    pub is_disabled: Signal<bool>,

    /// Whether the spin button is read-only.
    pub is_read_only: Signal<bool>,

    /// Called on increment (`ArrowUp`, `PageUp`, or increment button press).
    pub on_increment: Callback<()>,

    /// Called on decrement (`ArrowDown`, `PageDown`, or decrement button press).
    pub on_decrement: Callback<()>,

    /// Called to jump to max (End key).
    pub on_increment_to_max: Callback<()>,

    /// Called to jump to min (Home key).
    pub on_decrement_to_min: Callback<()>,
}

/// Return value of the `use_spin_button` hook.
pub struct UseSpinButtonReturn {
    /// Keyboard event handler to attach to the input element.
    pub on_keydown: EventHandler<KeyboardEvent>,

    /// Props for the increment button.
    pub increment_button_props: SpinButtonButtonProps,

    /// Props for the decrement button.
    pub decrement_button_props: SpinButtonButtonProps,
}

/// Pointer event handlers for a spin button increment/decrement button.
pub struct SpinButtonButtonProps {
    pub on_pointerdown: EventHandler<PointerEvent>,
    pub on_pointerup: EventHandler<PointerEvent>,
    pub on_pointerleave: EventHandler<PointerEvent>,
}

/// Announce a value change to screen readers.
fn announce_value(announcer: Option<&LiveAnnouncerContext>, text_value: Signal<String>) {
    if let Some(announcer) = announcer {
        let text = text_value.get_untracked();
        let announcement = if text.is_empty() {
            "Empty".to_string()
        } else {
            // Replace ASCII minus with Unicode minus for better screen reader pronunciation.
            text.replace('-', "\u{2212}")
        };
        announcer.announce(announcement, Assertiveness::Assertive);
    }
}

/// Creates a spin button hook providing keyboard navigation and auto-repeat button behavior.
///
/// This hook handles:
/// - Keyboard: ArrowUp/Down, PageUp/Down, Home/End with modifier key guards
/// - Auto-repeat: Press-and-hold on increment/decrement buttons
/// - Live announcer: Announces new values to screen readers
#[allow(clippy::too_many_lines)]
pub fn use_spin_button(input: UseSpinButtonInput) -> UseSpinButtonReturn {
    let UseSpinButtonInput {
        text_value,
        is_disabled,
        is_read_only,
        on_increment,
        on_decrement,
        on_increment_to_max,
        on_decrement_to_min,
    } = input;

    let live_announcer = use_context::<LiveAnnouncerContext>();

    // -- Keyboard handler --
    let announcer_for_keys = live_announcer.clone();
    let handle_keydown = move |e: KeyboardEvent| {
        if is_disabled.get_untracked() || is_read_only.get_untracked() {
            return;
        }

        // Skip if modifier keys are held (matching react-aria).
        if e.ctrl_key() || e.meta_key() || e.shift_key() || e.alt_key() {
            return;
        }

        // Skip during IME composition.
        if e.is_composing() {
            return;
        }

        match e.key().as_str() {
            "ArrowUp" | "PageUp" => {
                e.prevent_default();
                on_increment.run(());
                announce_value(announcer_for_keys.as_ref(), text_value);
            }
            "ArrowDown" | "PageDown" => {
                e.prevent_default();
                on_decrement.run(());
                announce_value(announcer_for_keys.as_ref(), text_value);
            }
            "Home" => {
                e.prevent_default();
                on_decrement_to_min.run(());
                announce_value(announcer_for_keys.as_ref(), text_value);
            }
            "End" => {
                e.prevent_default();
                on_increment_to_max.run(());
                announce_value(announcer_for_keys.as_ref(), text_value);
            }
            _ => {}
        }
    };

    // -- Auto-repeat for increment button --
    let inc_timeout: StoredValue<Option<i32>> = StoredValue::new(None);
    let inc_interval: StoredValue<Option<i32>> = StoredValue::new(None);

    let clear_inc_timers = move || {
        if let Some(id) = inc_timeout.get_value() {
            clear_timeout(id);
            inc_timeout.set_value(None);
        }
        if let Some(id) = inc_interval.get_value() {
            clear_interval(id);
            inc_interval.set_value(None);
        }
    };

    let announcer_for_inc = live_announcer.clone();
    let inc_pointerdown = move |e: PointerEvent| {
        if is_disabled.get_untracked() || is_read_only.get_untracked() {
            return;
        }

        // Clear any leftover timers.
        clear_inc_timers();

        let is_touch = e.pointer_type() == "touch";

        // For mouse: fire immediately. For touch: wait (user might be scrolling).
        if !is_touch {
            on_increment.run(());
            announce_value(announcer_for_inc.as_ref(), text_value);
        }

        let initial_delay = if is_touch {
            INITIAL_DELAY_TOUCH_MS
        } else {
            INITIAL_DELAY_MOUSE_MS
        };

        // Clone announcer for use inside timer callbacks.
        let announcer_for_timeout = announcer_for_inc.clone();
        let timeout_id = set_timeout_once(
            move || {
                if is_touch {
                    on_increment.run(());
                    announce_value(announcer_for_timeout.as_ref(), text_value);
                }

                let announcer_for_interval = announcer_for_timeout.clone();
                let interval_id = set_interval_repeating(
                    move || {
                        on_increment.run(());
                        announce_value(announcer_for_interval.as_ref(), text_value);
                    },
                    REPEAT_INTERVAL_MS,
                );
                inc_interval.set_value(Some(interval_id));
            },
            initial_delay,
        );
        inc_timeout.set_value(Some(timeout_id));
    };

    // -- Auto-repeat for decrement button --
    let dec_timeout: StoredValue<Option<i32>> = StoredValue::new(None);
    let dec_interval: StoredValue<Option<i32>> = StoredValue::new(None);

    let clear_dec_timers = move || {
        if let Some(id) = dec_timeout.get_value() {
            clear_timeout(id);
            dec_timeout.set_value(None);
        }
        if let Some(id) = dec_interval.get_value() {
            clear_interval(id);
            dec_interval.set_value(None);
        }
    };

    let announcer_for_dec = live_announcer;
    let dec_pointerdown = move |e: PointerEvent| {
        if is_disabled.get_untracked() || is_read_only.get_untracked() {
            return;
        }

        clear_dec_timers();

        let is_touch = e.pointer_type() == "touch";

        if !is_touch {
            on_decrement.run(());
            announce_value(announcer_for_dec.as_ref(), text_value);
        }

        let initial_delay = if is_touch {
            INITIAL_DELAY_TOUCH_MS
        } else {
            INITIAL_DELAY_MOUSE_MS
        };

        let announcer_for_timeout = announcer_for_dec.clone();
        let timeout_id = set_timeout_once(
            move || {
                if is_touch {
                    on_decrement.run(());
                    announce_value(announcer_for_timeout.as_ref(), text_value);
                }

                let announcer_for_interval = announcer_for_timeout.clone();
                let interval_id = set_interval_repeating(
                    move || {
                        on_decrement.run(());
                        announce_value(announcer_for_interval.as_ref(), text_value);
                    },
                    REPEAT_INTERVAL_MS,
                );
                dec_interval.set_value(Some(interval_id));
            },
            initial_delay,
        );
        dec_timeout.set_value(Some(timeout_id));
    };

    // Clean up timers on disposal.
    on_cleanup(move || {
        clear_inc_timers();
        clear_dec_timers();
    });

    UseSpinButtonReturn {
        on_keydown: EventHandler::new(handle_keydown),
        increment_button_props: SpinButtonButtonProps {
            on_pointerdown: EventHandler::new(inc_pointerdown),
            on_pointerup: EventHandler::new(move |_e: PointerEvent| clear_inc_timers()),
            on_pointerleave: EventHandler::new(move |_e: PointerEvent| clear_inc_timers()),
        },
        decrement_button_props: SpinButtonButtonProps {
            on_pointerdown: EventHandler::new(dec_pointerdown),
            on_pointerup: EventHandler::new(move |_e: PointerEvent| clear_dec_timers()),
            on_pointerleave: EventHandler::new(move |_e: PointerEvent| clear_dec_timers()),
        },
    }
}

/// Set a one-shot timeout. Returns the timeout ID for cancellation.
fn set_timeout_once(callback: impl FnOnce() + 'static, delay_ms: i32) -> i32 {
    use wasm_bindgen::JsCast;
    use wasm_bindgen::closure::Closure;

    let window = leptos_use::use_window();
    let Some(window) = window.as_ref() else {
        return 0;
    };

    let closure = Closure::once_into_js(callback);
    window
        .set_timeout_with_callback_and_timeout_and_arguments_0(
            closure.as_ref().unchecked_ref(),
            delay_ms,
        )
        .unwrap_or(0)
}

/// Set a repeating interval. Returns the interval ID for cancellation.
fn set_interval_repeating(callback: impl Fn() + 'static, interval_ms: i32) -> i32 {
    use wasm_bindgen::JsCast;
    use wasm_bindgen::closure::Closure;

    let window = leptos_use::use_window();
    let Some(window) = window.as_ref() else {
        return 0;
    };

    let closure = Closure::wrap(Box::new(callback) as Box<dyn Fn()>);
    let id = window
        .set_interval_with_callback_and_timeout_and_arguments_0(
            closure.as_ref().unchecked_ref(),
            interval_ms,
        )
        .unwrap_or(0);
    closure.forget(); // Leak the closure so it persists for the interval's lifetime.
    id
}

/// Clear a timeout.
fn clear_timeout(id: i32) {
    let window = leptos_use::use_window();
    if let Some(window) = window.as_ref() {
        window.clear_timeout_with_handle(id);
    }
}

/// Clear an interval.
fn clear_interval(id: i32) {
    let window = leptos_use::use_window();
    if let Some(window) = window.as_ref() {
        window.clear_interval_with_handle(id);
    }
}
