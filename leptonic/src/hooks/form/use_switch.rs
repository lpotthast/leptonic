use leptos::attr;
use leptos::attr::{Attr, Attribute};
use leptos::ev;
use leptos::ev::{on, On, SharedEventCallback};
use leptos::prelude::*;
use web_sys::{FocusEvent, KeyboardEvent};

use super::use_field::ValidationState;
use crate::hooks::focus::use_focus_ring::{use_focus_ring, UseFocusRingInput, UseFocusRingReturn};

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/@react-aria/switch/src/useSwitch.ts

/// Input parameters for the `use_switch` hook.
#[derive(Debug, Clone, Copy)]
pub struct UseSwitchInput {
    /// Whether the switch is selected (controlled).
    pub is_selected: Signal<bool>,

    /// Callback when the selection changes.
    pub on_change: Option<Callback<bool>>,

    /// Whether the switch is disabled.
    pub is_disabled: Signal<bool>,

    /// Whether the switch is read-only.
    pub is_read_only: Signal<bool>,

    /// The validation state of the switch.
    pub validation_state: ValidationState,

    /// An accessibility label for the switch.
    pub aria_label: Option<&'static str>,

    /// The name attribute for form submission.
    pub name: Option<&'static str>,

    /// The value attribute for form submission.
    pub value: Option<&'static str>,
}

impl Default for UseSwitchInput {
    fn default() -> Self {
        Self {
            is_selected: Signal::derive(|| false),
            on_change: None,
            is_disabled: Signal::derive(|| false),
            is_read_only: Signal::derive(|| false),
            validation_state: ValidationState::Valid,
            aria_label: None,
            name: None,
            value: None,
        }
    }
}

/// The return value of the `use_switch` hook.
#[derive(Debug, Clone)]
pub struct UseSwitchReturn {
    /// Props for the switch element (use with a button or div).
    pub switch_props: UseSwitchAttrs,

    /// Props for a hidden input for form submission.
    pub input_props: UseSwitchInputAttrs,

    /// Whether the switch is currently selected.
    pub is_selected: Signal<bool>,

    /// Whether the switch is pressed (during interaction).
    pub is_pressed: Signal<bool>,

    /// Whether the focus ring should be visible (keyboard navigation only).
    pub is_focus_visible: Signal<bool>,
}

/// Attributes for the switch element.
pub type UseSwitchAttrs = (
    Attr<attr::Role, &'static str>,
    Attr<attr::AriaChecked, Signal<&'static str>>,
    Attr<attr::AriaLabel, Option<&'static str>>,
    Attr<attr::AriaInvalid, Option<&'static str>>,
    Attr<attr::AriaDisabled, Option<&'static str>>,
    Attr<attr::Tabindex, &'static str>,
    On<ev::click, SharedEventCallback<web_sys::MouseEvent>>,
    On<ev::keydown, SharedEventCallback<KeyboardEvent>>,
    On<ev::focus, SharedEventCallback<FocusEvent>>,
    On<ev::blur, SharedEventCallback<FocusEvent>>,
    leptos::attr::custom::CustomAttr<&'static str, Signal<Option<&'static str>>>,
);

/// Attributes for the hidden input element (for form submission).
pub type UseSwitchInputAttrs = (
    Attr<attr::Type, &'static str>,
    Attr<attr::Name, Option<&'static str>>,
    Attr<attr::Value, Option<&'static str>>,
    Attr<attr::Checked, Signal<bool>>,
    Attr<attr::Disabled, Signal<bool>>,
    Attr<attr::AriaHidden, &'static str>,
);

/// Provides the behavior and accessibility implementation for a switch.
///
/// Switches allow users to toggle between two states (on/off).
/// Unlike checkboxes, switches should have an immediate effect.
///
/// # Example
///
/// ```ignore
/// let (is_selected, set_is_selected) = signal(false);
///
/// let switch = use_switch(UseSwitchInput {
///     is_selected: is_selected.into(),
///     on_change: Some(Callback::new(move |selected| {
///         set_is_selected.set(selected);
///     })),
///     aria_label: Some("Enable notifications"),
///     ..Default::default()
/// });
///
/// view! {
///     <div {..switch.switch_props}>
///         <span class="switch-track">
///             <span class="switch-thumb" />
///         </span>
///     </div>
///     <input {..switch.input_props} />
/// }
/// ```
pub fn use_switch(input: UseSwitchInput) -> UseSwitchReturn {
    let is_selected = input.is_selected;
    let on_change = input.on_change;
    let is_disabled = input.is_disabled;
    let is_read_only = input.is_read_only;

    let (is_pressed, _set_is_pressed) = signal(false);

    // Toggle the state
    let toggle = move || {
        if is_disabled.get_untracked() || is_read_only.get_untracked() {
            return;
        }

        let new_value = !is_selected.get_untracked();
        if let Some(on_change) = on_change {
            on_change.run(new_value);
        }
    };

    // Handle click
    let toggle_click = toggle;
    let handle_click = move |_e: web_sys::MouseEvent| {
        toggle_click();
    };

    // Handle keydown
    let toggle_key = toggle;
    let handle_keydown = move |e: KeyboardEvent| {
        if is_disabled.get_untracked() || is_read_only.get_untracked() {
            return;
        }

        match e.key().as_str() {
            " " | "Enter" => {
                e.prevent_default();
                toggle_key();
            }
            _ => {}
        }
    };

    // Compute aria-checked
    let aria_checked = Signal::derive(move || if is_selected.get() { "true" } else { "false" });

    // Compute aria-invalid
    let aria_invalid = if input.validation_state == ValidationState::Invalid {
        Some("true")
    } else {
        None
    };

    // Compute aria-disabled
    let aria_disabled = if is_disabled.get_untracked() {
        Some("true")
    } else {
        None
    };

    let UseFocusRingReturn {
        props: focus_ring_props,
        is_focus_visible,
        is_focused: _,
    } = use_focus_ring(UseFocusRingInput {
        disabled: is_disabled,
        within: false,
        auto_focus: false,
        on_focus: None,
        on_blur: None,
        on_focus_change: None,
    });
    let (on_focus, on_blur, data_focus_visible) = focus_ring_props.into_attrs();

    UseSwitchReturn {
        switch_props: (
            Attr(attr::Role, "switch"),
            Attr(attr::AriaChecked, aria_checked),
            Attr(attr::AriaLabel, input.aria_label),
            Attr(attr::AriaInvalid, aria_invalid),
            Attr(attr::AriaDisabled, aria_disabled),
            Attr(attr::Tabindex, "0"),
            on(ev::click, handle_click).into_cloneable(),
            on(ev::keydown, handle_keydown).into_cloneable(),
            on_focus,
            on_blur,
            data_focus_visible,
        ),
        input_props: (
            Attr(attr::Type, "checkbox"),
            Attr(attr::Name, input.name),
            Attr(attr::Value, input.value),
            Attr(attr::Checked, is_selected),
            Attr(attr::Disabled, is_disabled),
            Attr(attr::AriaHidden, "true"),
        ),
        is_selected,
        is_pressed: is_pressed.into(),
        is_focus_visible,
    }
}

/// Creates internal state for a switch component.
pub fn use_switch_state(default_selected: bool) -> UseSwitchStateReturn {
    let (is_selected, set_is_selected) = signal(default_selected);

    UseSwitchStateReturn {
        is_selected: is_selected.into(),
        toggle: Callback::new(move |_| {
            set_is_selected.update(|v| *v = !*v);
        }),
        set_selected: Callback::new(move |selected| {
            set_is_selected.set(selected);
        }),
    }
}

/// State for managing switch state.
#[derive(Clone, Copy)]
pub struct UseSwitchStateReturn {
    /// Whether the switch is selected.
    pub is_selected: Signal<bool>,

    /// Toggle the selection state.
    pub toggle: Callback<()>,

    /// Set the selection state.
    pub set_selected: Callback<bool>,
}
