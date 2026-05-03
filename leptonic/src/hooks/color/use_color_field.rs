use leptos::{
    attr,
    attr::Attr,
    ev,
    ev::{On, SharedEventCallback},
    prelude::*,
};
use web_sys::{Event, FocusEvent, KeyboardEvent, WheelEvent};

use crate::{
    hooks::{
        IntoAttrs,
        interactions::use_scroll_wheel::{ScrollEvent, UseScrollWheelInput, use_scroll_wheel},
    },
    utils::{
        EventHandler,
        aria::{AriaDisabled, AriaRole},
        color::RGB8,
    },
};

use super::use_color_field_state::UseColorFieldStateReturn;

// This is based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/@react-aria/color/src/useColorField.ts

// ## INTENTIONAL DEVIATIONS
//
// - Does not compose with `use_spin_button` directly. Instead, the spinbutton
//   ARIA attributes and keyboard handling are inlined for simplicity.
//
// - No `useFormattedTextField` composition. Uses direct event handlers.

/// Hex page step for PageUp/PageDown: increment/decrement by 16 (0x10).
const HEX_PAGE_STEP: i64 = 0x10;

/// Input parameters for `use_color_field`.
#[derive(Debug, Clone)]
pub struct UseColorFieldInput {
    /// The color field state (from `use_color_field_state`).
    pub state: UseColorFieldStateReturn,

    /// Whether the field is disabled.
    pub disabled: Signal<bool>,

    /// Whether the field is read-only.
    pub read_only: Signal<bool>,

    /// An accessibility label for the field.
    pub aria_label: Option<&'static str>,

    /// Whether scroll-wheel adjustment is disabled.
    pub is_wheel_disabled: bool,
}

/// Return value of `use_color_field`.
pub struct UseColorFieldReturn {
    /// Props for the input element.
    pub input_props: UseColorFieldInputProps,
}

/// Props for the color field input element.
#[derive(Debug)]
pub struct UseColorFieldInputProps {
    role: AriaRole,
    r#type: &'static str,
    autocomplete: &'static str,
    autocorrect: &'static str,
    spellcheck: &'static str,
    aria_label: Option<&'static str>,
    aria_disabled: Signal<Option<AriaDisabled>>,
    aria_valuenow: Signal<f64>,
    aria_valuemin: f64,
    aria_valuemax: f64,
    aria_valuetext: Signal<String>,
    on_input: EventHandler<Event>,
    on_focus: EventHandler<FocusEvent>,
    on_blur: EventHandler<FocusEvent>,
    on_keydown: EventHandler<KeyboardEvent>,
    on_wheel: EventHandler<WheelEvent>,
}

impl IntoAttrs for UseColorFieldInputProps {
    type Attrs = UseColorFieldInputAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            Attr(attr::Role, self.role),
            Attr(attr::Type, self.r#type),
            Attr(attr::Autocomplete, self.autocomplete),
            attr::custom::custom_attribute("autocorrect", self.autocorrect),
            Attr(attr::Spellcheck, self.spellcheck),
            Attr(attr::AriaLabel, self.aria_label),
            Attr(attr::AriaDisabled, self.aria_disabled),
            Attr(attr::AriaValuenow, self.aria_valuenow),
            Attr(attr::AriaValuemin, self.aria_valuemin),
            Attr(attr::AriaValuemax, self.aria_valuemax),
            Attr(attr::AriaValuetext, self.aria_valuetext),
            self.on_input.into_on(ev::input),
            self.on_focus.into_on(ev::focusin),
            self.on_blur.into_on(ev::focusout),
            self.on_keydown.into_on(ev::keydown),
            self.on_wheel.into_on(ev::wheel),
        )
    }
}

/// Attribute tuple produced by [`UseColorFieldInputProps::into_attrs`].
pub type UseColorFieldInputAttrs = (
    Attr<attr::Role, AriaRole>,
    Attr<attr::Type, &'static str>,
    Attr<attr::Autocomplete, &'static str>,
    attr::custom::CustomAttr<&'static str, &'static str>,
    Attr<attr::Spellcheck, &'static str>,
    Attr<attr::AriaLabel, Option<&'static str>>,
    Attr<attr::AriaDisabled, Signal<Option<AriaDisabled>>>,
    Attr<attr::AriaValuenow, Signal<f64>>,
    Attr<attr::AriaValuemin, f64>,
    Attr<attr::AriaValuemax, f64>,
    Attr<attr::AriaValuetext, Signal<String>>,
    On<ev::input, SharedEventCallback<Event>>,
    On<ev::focusin, SharedEventCallback<FocusEvent>>,
    On<ev::focusout, SharedEventCallback<FocusEvent>>,
    On<ev::keydown, SharedEventCallback<KeyboardEvent>>,
    On<ev::wheel, SharedEventCallback<WheelEvent>>,
);

/// Creates behavior and ARIA props for a hex color text input.
///
/// The input validates hex characters during typing and commits
/// the value on blur. Uses `role="spinbutton"` with full ARIA support.
///
/// ## Keyboard support
///
/// - `ArrowUp` / `ArrowDown`: Increment/decrement hex value by 1
/// - `PageUp` / `PageDown`: Increment/decrement by 16 (0x10)
/// - `Home`: Jump to minimum (#000000)
/// - `End`: Jump to maximum (#FFFFFF)
///
/// ## Scroll wheel
///
/// When the input is focused, scroll wheel adjusts the value up/down.
#[allow(clippy::too_many_lines)]
pub fn use_color_field(input: UseColorFieldInput) -> UseColorFieldReturn {
    let UseColorFieldInput {
        state,
        disabled,
        read_only,
        aria_label,
        is_wheel_disabled,
    } = input;

    let aria_disabled = Signal::derive(move || disabled.get().then_some(AriaDisabled::True));

    // Spinbutton ARIA values.
    let aria_valuenow = Signal::derive(move || {
        state
            .color_value
            .get()
            .map_or(0.0, |c| f64::from(c.to_hex_int()))
    });
    let aria_valuetext = Signal::derive(move || {
        state
            .color_value
            .get()
            .map_or_else(String::new, |c| format!("#{c:X}"))
    });

    // Focus tracking for scroll wheel.
    let (is_focused, set_is_focused) = signal(false);

    let handle_focus = EventHandler::new(move |_: FocusEvent| {
        set_is_focused.set(true);
    });

    let commit = state.commit;
    let handle_blur = EventHandler::new(move |_: FocusEvent| {
        set_is_focused.set(false);
        commit.run(());
    });

    // Scroll wheel support.
    let scroll_disabled = Signal::derive(move || {
        is_wheel_disabled || disabled.get() || read_only.get() || !is_focused.get()
    });
    let scroll_increment = state.increment;
    let scroll_decrement = state.decrement;
    let scroll_wheel = use_scroll_wheel(UseScrollWheelInput {
        disabled: scroll_disabled,
        on_scroll: Some(Callback::new(move |e: ScrollEvent| {
            if e.delta_y.abs() <= e.delta_x.abs() {
                return;
            }
            if e.delta_y > 0.0 {
                scroll_decrement.run(());
            } else if e.delta_y < 0.0 {
                scroll_increment.run(());
            }
        })),
    });

    // Input validation handler.
    let set_input_value = state.set_input_value;
    let validate = state.validate;
    let handle_input = EventHandler::new(move |e: Event| {
        let target = event_target::<web_sys::HtmlInputElement>(&e);
        let text = target.value();
        if validate.run(text.clone()) {
            set_input_value.run(text);
        } else {
            // Revert the input to the last valid value.
            target.set_value(&state.input_value.get_untracked());
        }
    });

    // Keyboard handler with full spinbutton support.
    let increment = state.increment;
    let decrement = state.decrement;
    let increment_to_max = state.increment_to_max;
    let decrement_to_min = state.decrement_to_min;
    let handle_keydown = EventHandler::new(move |e: KeyboardEvent| {
        if disabled.get_untracked() || read_only.get_untracked() {
            return;
        }
        let key = e.key();
        match key.as_str() {
            "ArrowUp" => {
                e.prevent_default();
                increment.run(());
            }
            "ArrowDown" => {
                e.prevent_default();
                decrement.run(());
            }
            "PageUp" => {
                e.prevent_default();
                // Increment by page step (16 / 0x10).
                for _ in 0..HEX_PAGE_STEP {
                    increment.run(());
                }
            }
            "PageDown" => {
                e.prevent_default();
                // Decrement by page step (16 / 0x10).
                for _ in 0..HEX_PAGE_STEP {
                    decrement.run(());
                }
            }
            "Home" => {
                e.prevent_default();
                decrement_to_min.run(());
            }
            "End" => {
                e.prevent_default();
                increment_to_max.run(());
            }
            _ => {}
        }
    });

    UseColorFieldReturn {
        input_props: UseColorFieldInputProps {
            role: AriaRole::Spinbutton,
            r#type: "text",
            autocomplete: "off",
            autocorrect: "off",
            spellcheck: "false",
            aria_label,
            aria_disabled,
            aria_valuenow,
            aria_valuemin: f64::from(RGB8::new().to_hex_int()),
            aria_valuemax: f64::from(RGB8::from_hex_int(0xFF_FF_FF).to_hex_int()),
            aria_valuetext,
            on_input: handle_input,
            on_focus: handle_focus,
            on_blur: handle_blur,
            on_keydown: handle_keydown,
            on_wheel: scroll_wheel.props.on_wheel,
        },
    }
}
