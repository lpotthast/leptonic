// Upstream: react-aria/src/spinbutton/useSpinButton.ts @ 99e6102368
use std::time::Duration;

use leptos::{
    attr,
    attr::Attr,
    ev,
    ev::{On, SharedEventCallback},
    prelude::*,
};
use leptos_use::use_window;
use send_wrapper::SendWrapper;
use web_sys::{FocusEvent, KeyboardEvent};

use crate::{
    hooks::{
        IntoAttrs, PressEvent, UseButtonInput,
        interactions::use_keyboard::{UseKeyboardInput, UseKeyboardReturn, use_keyboard},
    },
    utils::{
        EventHandler,
        aria::{AriaDisabled, AriaReadonly, AriaRequired, AriaRole},
        event_listeners::{Listener, listen_to},
        keyboard_shortcut::{KeyboardShortcuts, Shortcut},
        live_announcer::{Assertiveness, announce, clear_announcer},
        pointer_type::PointerType,
    },
};
use crate::utils::intl_strings::{SpinButtonStrings, use_localized_strings};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## DIFFERENT BEHAVIOR
// - Touch: react-aria increments once more when a finger is lifted after the button already
//   spun, because `onPressEnd` resets the "spinning" flag before checking it. Leptonic records
//   whether the button was spinning on press up, so a tap steps once and a hold only spins.
// - Without a value and text, `aria-valuetext` is "Empty" (react-aria: "undefined").
// - The returned stepper buttons are disabled while the spin button is disabled or read-only
//   (react-aria leaves that to the caller, e.g. `useNumberField`).
//
// ## API DIFFERENCES
// - The stepper buttons are returned as `UseButtonInput` (react-aria: `AriaButtonProps`).
//   Pass them to `use_button`, adding labels and other settings with struct update syntax.
// - `text_value: None` means "derive from `value`"; `Some("")` announces "Empty".
//
// =============================================================================

/// Delay before a held stepper button starts spinning, for mouse and pen.
const INITIAL_SPIN_DELAY: Duration = Duration::from_millis(400);
/// Delay before a held stepper button starts spinning, for touch. Longer, because the user might
/// be about to scroll.
const INITIAL_SPIN_DELAY_TOUCH: Duration = Duration::from_millis(600);
/// Delay between steps while spinning.
const SPIN_INTERVAL: Duration = Duration::from_millis(60);

/// Input of [`use_spin_button`].
#[derive(Debug, Clone, Default)]
pub struct UseSpinButtonInput {
    /// The current value. `None` (or NaN) when empty.
    pub value: Signal<Option<f64>>,
    /// A textual representation of the value, announced to screen readers and exposed as
    /// `aria-valuetext`. `None` uses `value`.
    pub text_value: Signal<Option<String>>,
    pub min_value: Signal<Option<f64>>,
    pub max_value: Signal<Option<f64>>,
    pub is_disabled: Signal<bool>,
    pub is_read_only: Signal<bool>,
    pub is_required: Signal<bool>,
    /// Step up (ArrowUp, increment button).
    pub on_increment: Option<Callback<()>>,
    /// Step up by a page (PageUp). Falls back to `on_increment`.
    pub on_increment_page: Option<Callback<()>>,
    /// Step down (ArrowDown, decrement button).
    pub on_decrement: Option<Callback<()>>,
    /// Step down by a page (PageDown). Falls back to `on_decrement`.
    pub on_decrement_page: Option<Callback<()>>,
    /// Jump to the minimum (Home).
    pub on_decrement_to_min: Option<Callback<()>>,
    /// Jump to the maximum (End).
    pub on_increment_to_max: Option<Callback<()>>,
}

/// Return value of [`use_spin_button`].
#[derive(Debug)]
pub struct UseSpinButtonReturn {
    /// Props for the element with `role="spinbutton"`.
    pub props: UseSpinButtonProps,
    /// Configuration for the increment button; pass it to `use_button`.
    pub increment_button: UseButtonInput,
    /// Configuration for the decrement button; pass it to `use_button`.
    pub decrement_button: UseButtonInput,
}

/// Props for the spin button element.
#[derive(Debug)]
pub struct UseSpinButtonProps {
    pub role: AriaRole,
    pub aria_valuenow: Signal<Option<String>>,
    pub aria_valuetext: Signal<String>,
    pub aria_valuemin: Signal<Option<String>>,
    pub aria_valuemax: Signal<Option<String>>,
    pub aria_disabled: Signal<Option<AriaDisabled>>,
    pub aria_readonly: Signal<Option<AriaReadonly>>,
    pub aria_required: Signal<Option<AriaRequired>>,
    pub on_keydown: EventHandler<KeyboardEvent>,
    pub on_keyup: EventHandler<KeyboardEvent>,
    pub on_focus: EventHandler<FocusEvent>,
    pub on_blur: EventHandler<FocusEvent>,
}

pub type UseSpinButtonAttrs = (
    Attr<attr::Role, AriaRole>,
    Attr<attr::AriaValuenow, Signal<Option<String>>>,
    Attr<attr::AriaValuetext, Signal<String>>,
    Attr<attr::AriaValuemin, Signal<Option<String>>>,
    Attr<attr::AriaValuemax, Signal<Option<String>>>,
    Attr<attr::AriaDisabled, Signal<Option<AriaDisabled>>>,
    Attr<attr::AriaReadonly, Signal<Option<AriaReadonly>>>,
    Attr<attr::AriaRequired, Signal<Option<AriaRequired>>>,
    On<ev::keydown, SharedEventCallback<KeyboardEvent>>,
    On<ev::keyup, SharedEventCallback<KeyboardEvent>>,
    On<ev::focus, SharedEventCallback<FocusEvent>>,
    On<ev::blur, SharedEventCallback<FocusEvent>>,
);

impl IntoAttrs for UseSpinButtonProps {
    type Attrs = UseSpinButtonAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            Attr(attr::Role, self.role),
            Attr(attr::AriaValuenow, self.aria_valuenow),
            Attr(attr::AriaValuetext, self.aria_valuetext),
            Attr(attr::AriaValuemin, self.aria_valuemin),
            Attr(attr::AriaValuemax, self.aria_valuemax),
            Attr(attr::AriaDisabled, self.aria_disabled),
            Attr(attr::AriaReadonly, self.aria_readonly),
            Attr(attr::AriaRequired, self.aria_required),
            self.on_keydown.into_on(ev::keydown),
            self.on_keyup.into_on(ev::keyup),
            self.on_focus.into_on(ev::focus),
            self.on_blur.into_on(ev::blur),
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
}

/// Steps the value while a stepper button is held: once after an initial delay, then every
/// [`SPIN_INTERVAL`] until the limit is reached or the button is released.
#[derive(Clone, Copy)]
struct Spinner {
    timeout: StoredValue<Option<TimeoutHandle>>,
    /// Whether a step is scheduled.
    is_spinning: StoredValue<bool>,
    /// Whether the current press already stepped through spinning (reset per press).
    spun: StoredValue<bool>,
    value: Signal<Option<f64>>,
    min_value: Signal<Option<f64>>,
    max_value: Signal<Option<f64>>,
    on_increment: Option<Callback<()>>,
    on_decrement: Option<Callback<()>>,
}

impl Spinner {
    fn clear(self) {
        if let Some(handle) = self.timeout.get_value() {
            handle.clear();
        }
        self.timeout.set_value(None);
        self.is_spinning.set_value(false);
    }

    fn start(self, direction: Direction, delay: Duration) {
        self.clear();
        self.is_spinning.set_value(true);
        let handle = set_timeout_with_handle(move || self.step(direction), delay).ok();
        self.timeout.set_value(handle);
    }

    fn step(self, direction: Direction) {
        // Missing or NaN bounds and values never stop the spinning.
        let value = self.value.get_untracked().filter(|v| !v.is_nan());
        let can_step = match direction {
            Direction::Up => {
                let max = self.max_value.get_untracked().filter(|v| !v.is_nan());
                value.zip(max).is_none_or(|(value, max)| value < max)
            }
            Direction::Down => {
                let min = self.min_value.get_untracked().filter(|v| !v.is_nan());
                value.zip(min).is_none_or(|(value, min)| value > min)
            }
        };
        if can_step {
            self.spun.set_value(true);
            if let Some(step) = self.callback(direction) {
                step.run(());
            }
            self.start(direction, SPIN_INTERVAL);
        }
    }

    fn callback(self, direction: Direction) -> Option<Callback<()>> {
        match direction {
            Direction::Up => self.on_increment,
            Direction::Down => self.on_decrement,
        }
    }
}

/// Implements a spin button: an element whose numeric value is changed with the arrow,
/// page and Home/End keys, plus increment/decrement buttons that keep stepping while held.
///
/// Value changes are announced to screen readers while the spin button has focus.
pub fn use_spin_button(input: UseSpinButtonInput) -> UseSpinButtonReturn {
    let UseSpinButtonInput {
        value,
        text_value,
        min_value,
        max_value,
        is_disabled: disabled,
        is_read_only: read_only,
        is_required: required,
        on_increment,
        on_increment_page,
        on_decrement,
        on_decrement_page,
        on_decrement_to_min,
        on_increment_to_max,
    } = input;

    let spinner = Spinner {
        timeout: StoredValue::new(None),
        is_spinning: StoredValue::new(false),
        spun: StoredValue::new(false),
        value,
        min_value,
        max_value,
        on_increment,
        on_decrement,
    };
    on_cleanup(move || spinner.clear());

    // -- Keyboard --
    // A key only counts as handled when the corresponding callback exists. Otherwise the event
    // is left alone (no `preventDefault`, keeps bubbling).
    let run_first = |callbacks: [Option<Callback<()>>; 2]| {
        move |_: &KeyboardEvent| {
            callbacks.iter().flatten().next().is_some_and(|callback| {
                callback.run(());
                true
            })
        }
    };
    let shortcuts = KeyboardShortcuts::new()
        .on(
            Shortcut::key("PageUp"),
            run_first([on_increment_page, on_increment]),
        )
        .on(Shortcut::key("ArrowUp"), run_first([on_increment, None]))
        .on(
            Shortcut::key("PageDown"),
            run_first([on_decrement_page, on_decrement]),
        )
        .on(Shortcut::key("ArrowDown"), run_first([on_decrement, None]))
        .on(
            Shortcut::key("Home"),
            run_first([on_decrement_to_min, None]),
        )
        .on(Shortcut::key("End"), run_first([on_increment_to_max, None]));
    let UseKeyboardReturn {
        props: keyboard_props,
    } = use_keyboard(UseKeyboardInput {
        is_disabled: Signal::derive(move || disabled.get() || read_only.get()),
        shortcuts: Some(shortcuts),
        allow_repeats: true,
        ..UseKeyboardInput::default()
    });

    // -- Focus tracking and announcements --
    let is_focused = StoredValue::new(false);
    let on_focus = Callback::new(move |_: FocusEvent| is_focused.set_value(true));
    let on_blur = Callback::new(move |_: FocusEvent| is_focused.set_value(false));

    // Use the real minus sign (U+2212), so that macOS VoiceOver reads "minus" even when other
    // characters (like a currency symbol) sit between the sign and the number. An empty field is
    // announced as "Empty" instead of iOS VoiceOver reading a stale value.
    let strings = use_localized_strings::<SpinButtonStrings>();
    let aria_text_value = Memo::new(move |_| match text_value.get() {
        Some(text) if text.is_empty() => strings.read().empty(),
        Some(text) => text.replacen('-', "\u{2212}", 1),
        // Without a value: "Empty" as well (react-aria: the text "undefined").
        None => value.get().map_or_else(
            || strings.read().empty(),
            |v| v.to_string().replacen('-', "\u{2212}", 1),
        ),
    });

    Effect::new(move |previous: Option<()>| {
        let text = aria_text_value.get();
        // Only announce changes, and only while focus is on the spin button or its buttons.
        if previous.is_some() && is_focused.get_value() {
            clear_announcer(Some(Assertiveness::Assertive));
            announce(text, Assertiveness::Assertive);
        }
    });

    // -- Stepper buttons --
    // Window listeners that live for the duration of one touch press.
    // (Removed when dropped; empty on the server, where no `SendWrapper` may be created.)
    let global_listeners: StoredValue<Vec<SendWrapper<Listener>>> = StoredValue::new(Vec::new());
    let remove_global_listeners = move || {
        global_listeners.try_update_value(Vec::clear);
    };
    on_cleanup(remove_global_listeners);

    // While pressing with touch, the press is released ("up") before it ends. A press that ends
    // without an up (the finger slid off, e.g. to scroll) must not step.
    let is_up = StoredValue::new(false);

    let stepper = move |direction: Direction| {
        let step = spinner.callback(direction);
        UseButtonInput {
            on_press_start: Some(Callback::new(move |e: PressEvent| {
                spinner.clear();
                spinner.spun.set_value(false);
                if e.pointer_type == PointerType::Touch {
                    // Don't step on touch start: wait for the press end, or spin when held.
                    is_up.set_value(false);
                    // A cancelled pointer means the browser took over (e.g. scrolling).
                    if let Some(window) = use_window().as_ref() {
                        let listener =
                            listen_to(window, ev::pointercancel, true, move |_| spinner.clear());
                        global_listeners.update_value(|l| l.push(SendWrapper::new(listener)));
                    }
                    spinner.start(direction, INITIAL_SPIN_DELAY_TOUCH);
                } else {
                    if let Some(step) = step {
                        step.run(());
                    }
                    spinner.start(direction, INITIAL_SPIN_DELAY);
                }
                // Holding a button with touch would otherwise open the context menu.
                if let Some(window) = use_window().as_ref() {
                    let listener =
                        listen_to(window, ev::contextmenu, false, |e: web_sys::MouseEvent| {
                            e.prevent_default();
                        });
                    global_listeners.update_value(|l| l.push(SendWrapper::new(listener)));
                }
            })),
            on_press_up: Some(Callback::new(move |e: PressEvent| {
                if e.pointer_type == PointerType::Touch {
                    is_up.set_value(true);
                }
                spinner.clear();
                remove_global_listeners();
            })),
            on_press_end: Some(Callback::new(move |e: PressEvent| {
                spinner.clear();
                remove_global_listeners();
                if e.pointer_type == PointerType::Touch
                    && is_up.get_value()
                    && !spinner.spun.get_value()
                    && let Some(step) = step
                {
                    step.run(());
                }
                is_up.set_value(false);
            })),
            on_focus: Some(on_focus),
            on_blur: Some(on_blur),
            // A disabled or read-only spin button can't be stepped by its buttons either.
            is_disabled: Signal::derive(move || disabled.get() || read_only.get()),
            ..UseButtonInput::default()
        }
    };

    let format_bound = |bound: Signal<Option<f64>>| {
        Signal::derive(move || bound.get().filter(|v| !v.is_nan()).map(|v| v.to_string()))
    };

    UseSpinButtonReturn {
        props: UseSpinButtonProps {
            role: AriaRole::Spinbutton,
            aria_valuenow: format_bound(value),
            aria_valuetext: aria_text_value.into(),
            aria_valuemin: format_bound(min_value),
            aria_valuemax: format_bound(max_value),
            aria_disabled: Signal::derive(move || disabled.get().then_some(AriaDisabled::True)),
            aria_readonly: Signal::derive(move || read_only.get().then_some(AriaReadonly::True)),
            aria_required: Signal::derive(move || required.get().then_some(AriaRequired::True)),
            on_keydown: keyboard_props.on_keydown,
            on_keyup: keyboard_props.on_keyup,
            on_focus: EventHandler::new(move |e: FocusEvent| on_focus.run(e)),
            on_blur: EventHandler::new(move |e: FocusEvent| {
                on_blur.try_run(e);
            }),
        },
        increment_button: stepper(Direction::Up),
        decrement_button: stepper(Direction::Down),
    }
}
