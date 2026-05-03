use leptos::{
    attr,
    attr::{
        Attr,
        custom::{CustomAttr, custom_attribute},
    },
    ev,
    ev::{On, SharedEventCallback},
    prelude::*,
};
use web_sys::{FocusEvent, KeyboardEvent};

use super::{
    use_form_reset::{UseFormResetInput, use_form_reset},
    use_form_validation::{UseFormValidationInput, use_form_validation},
    use_form_validation_state::{
        UseFormValidationStateInput, ValidateFn, ValidationBehavior, ValidityStateSnapshot,
        use_form_validation_state,
    },
};
use crate::{
    hooks::{
        IntoAttrs,
        focus::use_focus_ring::{UseFocusRingInput, UseFocusRingReturn, use_focus_ring},
    },
    utils::{
        CapturedElement, ElementCaptureAttr, EventHandler,
        aria::{AriaChecked, AriaDisabled, AriaHidden, AriaInvalid, AriaRole},
    },
};

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/@react-aria/switch/src/useSwitch.ts

// No intentional deviations from the react-aria implementation.

/// Input parameters for the `use_switch` hook.
#[derive(Clone)]
pub struct UseSwitchInput {
    /// Whether the switch is selected (controlled).
    pub is_selected: Signal<bool>,

    /// The default value to restore on form reset.
    /// If `None`, the initial value of `is_selected` at hook creation time is used.
    pub default_value: Option<bool>,

    /// Callback when the selection changes.
    pub on_change: Option<Callback<bool>>,

    /// Whether the switch is disabled.
    pub is_disabled: Signal<bool>,

    /// Whether the switch is read-only.
    pub is_read_only: Signal<bool>,

    /// Whether the field is explicitly marked as invalid (controlled validation).
    ///
    /// - `None` — not controlled; validation comes from `validate`, server errors,
    ///   or native constraint validation.
    /// - `Some(signal)` — controlled; the signal value determines valid/invalid
    ///   and overrides all other validation sources.
    pub is_invalid: Option<Signal<bool>>,

    /// Custom client-side validation function.
    ///
    /// Returns `Ok(())` for valid, `Err(messages)` for invalid.
    pub validate: Option<ValidateFn<bool>>,

    /// Validation behavior mode.
    pub validation_behavior: ValidationBehavior,

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
            default_value: None,
            on_change: None,
            is_disabled: Signal::derive(|| false),
            is_read_only: Signal::derive(|| false),
            is_invalid: None,
            validate: None,
            validation_behavior: ValidationBehavior::default(),
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

    /// Whether the displayed validation is invalid.
    pub is_invalid: Signal<bool>,

    /// The displayed validation error messages.
    pub validation_errors: Signal<Vec<String>>,

    /// Detailed validity state (mirrors native `ValidityState`).
    pub validation_details: Signal<ValidityStateSnapshot>,
}

/// Props from `use_switch` for the switch element that can be extracted and merged programmatically.
#[derive(Debug)]
pub struct UseSwitchProps {
    pub role: AriaRole,
    pub aria_checked: Signal<AriaChecked>,
    pub aria_label: Option<&'static str>,
    pub aria_invalid: Signal<Option<AriaInvalid>>,
    pub aria_disabled: Signal<Option<AriaDisabled>>,
    pub tabindex: &'static str,
    pub data_focus_visible: Signal<Option<&'static str>>,
    pub element_capture: ElementCaptureAttr,
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
            self.element_capture,
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
    Attr<attr::Role, AriaRole>,
    Attr<attr::AriaChecked, Signal<AriaChecked>>,
    Attr<attr::AriaLabel, Option<&'static str>>,
    Attr<attr::AriaInvalid, Signal<Option<AriaInvalid>>>,
    Attr<attr::AriaDisabled, Signal<Option<AriaDisabled>>>,
    Attr<attr::Tabindex, &'static str>,
    ElementCaptureAttr,
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
#[allow(clippy::needless_pass_by_value, clippy::too_many_lines)]
pub fn use_switch(input: UseSwitchInput) -> UseSwitchReturn {
    let UseSwitchInput {
        is_selected,
        default_value,
        on_change,
        is_disabled,
        is_read_only,
        is_invalid,
        validate,
        validation_behavior,
        aria_label,
        name,
        value,
    } = input;

    let (is_pressed, _set_is_pressed) = signal(false);

    // ---- Element capture for DOM access ----
    let element = CapturedElement::new();

    // ---- Form validation state ----
    let validation = use_form_validation_state(UseFormValidationStateInput {
        is_invalid,
        value: is_selected,
        validate,
        validation_behavior,
        name: name.map(ToString::to_string),
    });

    // ---- Form reset (restores value on form reset) ----
    let initial_value = default_value.unwrap_or_else(|| is_selected.get_untracked());
    use_form_reset(UseFormResetInput {
        element,
        initial_value,
        on_reset: Callback::new(move |val: bool| {
            if let Some(on_change) = on_change {
                on_change.run(val);
            }
        }),
    });

    // ---- Form validation DOM connection ----
    use_form_validation(UseFormValidationInput {
        element,
        state: validation,
        validation_behavior,
    });

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
    let aria_invalid =
        Signal::derive(move || validation.is_invalid.get().then_some(AriaInvalid::True));

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
        is_text_input: false,
        on_focus: None,
        on_blur: None,
        on_focus_change: None,
    });

    // ---- Validation details convenience signal ----
    let validation_details =
        Signal::derive(move || validation.display_validation.get().validation_details);

    UseSwitchReturn {
        switch_props: UseSwitchProps {
            role: AriaRole::Switch,
            aria_checked,
            aria_label,
            aria_invalid,
            aria_disabled,
            tabindex: "0",
            data_focus_visible: focus_ring_props.data_focus_visible,
            element_capture: element.attr(),
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
        is_invalid: validation.is_invalid,
        validation_errors: validation.validation_errors,
        validation_details,
    }
}
