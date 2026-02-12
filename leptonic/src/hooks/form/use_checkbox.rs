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
use web_sys::{Event, FocusEvent};

use super::use_field::ValidationState;
use crate::{
    hooks::{
        focus::use_focus_ring::{use_focus_ring, UseFocusRingInput, UseFocusRingReturn},
        IntoAttrs,
    },
    utils::{
        aria::{AriaInvalid, AriaRequired},
        EventHandler,
    },
};

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/@react-aria/checkbox/src/useCheckbox.ts

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// No intentional deviations from the react-aria implementation.
//
// =============================================================================

/// Input parameters for the `use_checkbox` hook.
#[derive(Clone, Copy)]
pub struct UseCheckboxInput {
    /// Whether the checkbox is selected (controlled).
    pub is_selected: Signal<bool>,

    /// Whether the checkbox is in an indeterminate state.
    pub is_indeterminate: Signal<bool>,

    /// Callback when the selection changes.
    pub on_change: Option<Callback<bool>>,

    /// Whether the checkbox is disabled.
    pub is_disabled: Signal<bool>,

    /// Whether the checkbox is read-only.
    pub is_read_only: Signal<bool>,

    /// Whether the checkbox is required.
    pub is_required: bool,

    /// The validation state of the checkbox.
    pub validation_state: ValidationState,

    /// An accessibility label for the checkbox.
    pub aria_label: Option<&'static str>,

    /// The name attribute for form submission.
    pub name: Option<&'static str>,

    /// The value attribute for form submission.
    pub value: Option<&'static str>,
}

impl Default for UseCheckboxInput {
    fn default() -> Self {
        Self {
            is_selected: Signal::derive(|| false),
            is_indeterminate: Signal::derive(|| false),
            on_change: None,
            is_disabled: Signal::derive(|| false),
            is_read_only: Signal::derive(|| false),
            is_required: false,
            validation_state: ValidationState::Valid,
            aria_label: None,
            name: None,
            value: None,
        }
    }
}

/// The return value of the `use_checkbox` hook.
pub struct UseCheckboxReturn {
    /// Props for the checkbox input element. Call `.into_attrs()` for view spreading.
    pub input_props: UseCheckboxInputProps,

    /// Whether the checkbox is currently selected.
    pub is_selected: Signal<bool>,

    /// Whether the checkbox is currently indeterminate.
    pub is_indeterminate: Signal<bool>,

    /// Whether the checkbox is pressed (during interaction).
    pub is_pressed: Signal<bool>,

    /// Whether the focus ring should be visible (keyboard navigation only).
    pub is_focus_visible: Signal<bool>,
}

/// Props from `use_checkbox` that can be extracted and merged programmatically.
#[derive(Clone)]
pub struct UseCheckboxInputProps {
    pub r#type: &'static str,
    pub name: Option<&'static str>,
    pub value: Option<&'static str>,
    pub checked: Signal<bool>,
    pub disabled: Signal<bool>,
    pub aria_label: Option<&'static str>,
    pub aria_invalid: Option<AriaInvalid>,
    pub aria_required: Option<AriaRequired>,
    pub data_focus_visible: Signal<Option<&'static str>>,
    pub on_change: EventHandler<Event>,
    pub on_focus: EventHandler<FocusEvent>,
    pub on_blur: EventHandler<FocusEvent>,
    pub on_focusin: EventHandler<FocusEvent>,
    pub on_focusout: EventHandler<FocusEvent>,
}

impl IntoAttrs for UseCheckboxInputProps {
    type Attrs = UseCheckboxInputAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            Attr(attr::Type, self.r#type),
            Attr(attr::Name, self.name),
            Attr(attr::Value, self.value),
            Attr(attr::Checked, self.checked),
            Attr(attr::Disabled, self.disabled),
            Attr(attr::AriaLabel, self.aria_label),
            Attr(attr::AriaInvalid, self.aria_invalid),
            Attr(attr::AriaRequired, self.aria_required),
            custom_attribute("data-focus-visible", self.data_focus_visible),
            self.on_change.into_on(ev::change),
            self.on_focus.into_on(ev::focus),
            self.on_blur.into_on(ev::blur),
            self.on_focusin.into_on(ev::focusin),
            self.on_focusout.into_on(ev::focusout),
        )
    }
}

/// Attributes for the checkbox input element.
pub type UseCheckboxInputAttrs = (
    Attr<attr::Type, &'static str>,
    Attr<attr::Name, Option<&'static str>>,
    Attr<attr::Value, Option<&'static str>>,
    Attr<attr::Checked, Signal<bool>>,
    Attr<attr::Disabled, Signal<bool>>,
    Attr<attr::AriaLabel, Option<&'static str>>,
    Attr<attr::AriaInvalid, Option<AriaInvalid>>,
    Attr<attr::AriaRequired, Option<AriaRequired>>,
    CustomAttr<&'static str, Signal<Option<&'static str>>>,
    On<ev::change, SharedEventCallback<Event>>,
    On<ev::focus, SharedEventCallback<FocusEvent>>,
    On<ev::blur, SharedEventCallback<FocusEvent>>,
    On<ev::focusin, SharedEventCallback<FocusEvent>>,
    On<ev::focusout, SharedEventCallback<FocusEvent>>,
);

/// Provides the behavior and accessibility implementation for a checkbox.
///
/// Checkboxes allow users to select one or more items from a set,
/// or to toggle an option on or off.
///
/// # Example
///
/// ```ignore
/// let (is_selected, set_is_selected) = signal(false);
///
/// let checkbox = use_checkbox(UseCheckboxInput {
///     is_selected: is_selected.into(),
///     on_change: Some(Callback::new(move |selected| {
///         set_is_selected.set(selected);
///     })),
///     aria_label: Some("Accept terms"),
///     ..Default::default()
/// });
///
/// view! {
///     <label>
///         <input {..checkbox.input_props.into_attrs()} />
///         "Accept terms and conditions"
///     </label>
/// }
/// ```
pub fn use_checkbox(input: UseCheckboxInput) -> UseCheckboxReturn {
    let UseCheckboxInput {
        is_selected,
        is_indeterminate,
        on_change,
        is_disabled,
        is_read_only,
        is_required,
        validation_state,
        aria_label,
        name,
        value,
    } = input;

    let (is_pressed, _set_is_pressed) = signal(false);

    // Handle change event
    let handle_change = move |e: Event| {
        if is_disabled.get_untracked() || is_read_only.get_untracked() {
            e.prevent_default();
            return;
        }

        let new_value = !is_selected.get_untracked();
        if let Some(on_change) = on_change {
            on_change.run(new_value);
        }
    };

    // Compute aria-invalid
    let aria_invalid = (validation_state == ValidationState::Invalid).then_some(AriaInvalid::True);

    // Compute aria-required
    let aria_required = is_required.then_some(AriaRequired::True);

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

    UseCheckboxReturn {
        input_props: UseCheckboxInputProps {
            r#type: "checkbox",
            name,
            value,
            checked: is_selected,
            disabled: is_disabled,
            aria_label,
            aria_invalid,
            aria_required,
            data_focus_visible: focus_ring_props.data_focus_visible,
            on_change: EventHandler::new(handle_change),
            on_focus: focus_ring_props.on_focus,
            on_blur: focus_ring_props.on_blur,
            on_focusin: focus_ring_props.on_focusin,
            on_focusout: focus_ring_props.on_focusout,
        },
        is_selected,
        is_indeterminate,
        is_pressed: is_pressed.into(),
        is_focus_visible,
    }
}
