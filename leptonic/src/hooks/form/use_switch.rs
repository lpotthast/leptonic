use leptos::{
    attr,
    attr::{
        custom::{custom_attribute, CustomAttr},
        Attr,
    },
    ev,
    ev::{On, SharedEventCallback},
    prelude::*,
};
use web_sys::{FocusEvent, KeyboardEvent};

use super::use_field::ValidationState;
use crate::{
    hooks::{
        focus::use_focus_ring::{use_focus_ring, UseFocusRingInput, UseFocusRingReturn},
        IntoAttrs,
    },
    utils::{
        aria::{AriaChecked, AriaDisabled, AriaHidden, AriaInvalid},
        EventHandler,
    },
};

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/@react-aria/switch/src/useSwitch.ts

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// No intentional deviations from the react-aria implementation.
//
// =============================================================================

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
pub struct UseSwitchReturn {
    /// Props for the switch element. Call `.into_attrs()` for view spreading.
    pub switch_props: UseSwitchProps,

    /// Props for a hidden input for form submission. Call `.into_attrs()` for view spreading.
    pub input_props: UseSwitchInputProps,

    /// Whether the switch is currently selected.
    pub is_selected: Signal<bool>,

    /// Whether the switch is pressed (during interaction).
    pub is_pressed: Signal<bool>,

    /// Whether the focus ring should be visible (keyboard navigation only).
    pub is_focus_visible: Signal<bool>,
}

/// Props from `use_switch` for the switch element that can be extracted and merged programmatically.
#[derive(Debug)]
pub struct UseSwitchProps {
    pub role: &'static str,
    pub aria_checked: Signal<AriaChecked>,
    pub aria_label: Option<&'static str>,
    pub aria_invalid: Option<AriaInvalid>,
    pub aria_disabled: Signal<Option<AriaDisabled>>,
    pub tabindex: &'static str,
    pub data_focus_visible: Signal<Option<&'static str>>,
    pub on_click: EventHandler<web_sys::MouseEvent>,
    pub on_keydown: EventHandler<KeyboardEvent>,
    pub on_focus: EventHandler<FocusEvent>,
    pub on_blur: EventHandler<FocusEvent>,
    pub on_focusin: EventHandler<FocusEvent>,
    pub on_focusout: EventHandler<FocusEvent>,
}

impl IntoAttrs for UseSwitchProps {
    type Attrs = UseSwitchAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            Attr(attr::Role, self.role),
            Attr(attr::AriaChecked, self.aria_checked),
            Attr(attr::AriaLabel, self.aria_label),
            Attr(attr::AriaInvalid, self.aria_invalid),
            Attr(attr::AriaDisabled, self.aria_disabled),
            Attr(attr::Tabindex, self.tabindex),
            self.on_click.into_on(ev::click),
            self.on_keydown.into_on(ev::keydown),
            self.on_focus.into_on(ev::focus),
            self.on_blur.into_on(ev::blur),
            self.on_focusin.into_on(ev::focusin),
            self.on_focusout.into_on(ev::focusout),
            custom_attribute("data-focus-visible", self.data_focus_visible),
        )
    }
}

/// These attributes must be spread onto the switch element using the spread syntax `<div {..attrs}/>`.
pub type UseSwitchAttrs = (
    Attr<attr::Role, &'static str>,
    Attr<attr::AriaChecked, Signal<AriaChecked>>,
    Attr<attr::AriaLabel, Option<&'static str>>,
    Attr<attr::AriaInvalid, Option<AriaInvalid>>,
    Attr<attr::AriaDisabled, Signal<Option<AriaDisabled>>>,
    Attr<attr::Tabindex, &'static str>,
    On<ev::click, SharedEventCallback<web_sys::MouseEvent>>,
    On<ev::keydown, SharedEventCallback<KeyboardEvent>>,
    On<ev::focus, SharedEventCallback<FocusEvent>>,
    On<ev::blur, SharedEventCallback<FocusEvent>>,
    On<ev::focusin, SharedEventCallback<FocusEvent>>,
    On<ev::focusout, SharedEventCallback<FocusEvent>>,
    CustomAttr<&'static str, Signal<Option<&'static str>>>,
);

/// Props from `use_switch` for the hidden input element that can be extracted and merged programmatically.
#[derive(Debug)]
pub struct UseSwitchInputProps {
    pub r#type: &'static str,
    pub name: Option<&'static str>,
    pub value: Option<&'static str>,
    pub checked: Signal<bool>,
    pub disabled: Signal<bool>,
    pub aria_hidden: AriaHidden,
}

impl IntoAttrs for UseSwitchInputProps {
    type Attrs = UseSwitchInputAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            Attr(attr::Type, self.r#type),
            Attr(attr::Name, self.name),
            Attr(attr::Value, self.value),
            Attr(attr::Checked, self.checked),
            Attr(attr::Disabled, self.disabled),
            Attr(attr::AriaHidden, self.aria_hidden),
        )
    }
}

/// These attributes must be spread onto the hidden input element using the spread syntax `<input {..attrs}/>`.
pub type UseSwitchInputAttrs = (
    Attr<attr::Type, &'static str>,
    Attr<attr::Name, Option<&'static str>>,
    Attr<attr::Value, Option<&'static str>>,
    Attr<attr::Checked, Signal<bool>>,
    Attr<attr::Disabled, Signal<bool>>,
    Attr<attr::AriaHidden, AriaHidden>,
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
///     <div {..switch.switch_props.into_attrs()}>
///         <span class="switch-track">
///             <span class="switch-thumb" />
///         </span>
///     </div>
///     <input {..switch.input_props.into_attrs()} />
/// }
/// ```
pub fn use_switch(input: UseSwitchInput) -> UseSwitchReturn {
    let UseSwitchInput {
        is_selected,
        on_change,
        is_disabled,
        is_read_only,
        validation_state,
        aria_label,
        name,
        value,
    } = input;

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
    let aria_checked = Signal::derive(move || AriaChecked::from(is_selected.get()));

    // Compute aria-invalid
    let aria_invalid = (validation_state == ValidationState::Invalid).then_some(AriaInvalid::True);

    // Compute aria-disabled
    let aria_disabled = Signal::derive(move || is_disabled.get().then_some(AriaDisabled::True));

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

    UseSwitchReturn {
        switch_props: UseSwitchProps {
            role: "switch",
            aria_checked,
            aria_label,
            aria_invalid,
            aria_disabled,
            tabindex: "0",
            data_focus_visible: focus_ring_props.data_focus_visible,
            on_click: EventHandler::new(handle_click),
            on_keydown: EventHandler::new(handle_keydown),
            on_focus: focus_ring_props.on_focus,
            on_blur: focus_ring_props.on_blur,
            on_focusin: focus_ring_props.on_focusin,
            on_focusout: focus_ring_props.on_focusout,
        },
        input_props: UseSwitchInputProps {
            r#type: "checkbox",
            name,
            value,
            checked: is_selected,
            disabled: is_disabled,
            aria_hidden: AriaHidden::True,
        },
        is_selected,
        is_pressed: is_pressed.into(),
        is_focus_visible,
    }
}
