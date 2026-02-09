use leptos::attr;
use leptos::attr::{Attr, Attribute};
use leptos::ev;
use leptos::ev::{on, On, SharedEventCallback};
use leptos::prelude::*;
use web_sys::{Event, FocusEvent};

use super::use_field::ValidationState;
use crate::hooks::focus::use_focus_ring::{use_focus_ring, UseFocusRingInput, UseFocusRingReturn};

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/@react-aria/checkbox/src/useCheckbox.ts

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
#[derive(Clone)]
pub struct UseCheckboxReturn {
    /// Props for the checkbox input element.
    pub input_props: UseCheckboxInputAttrs,

    /// Whether the checkbox is currently selected.
    pub is_selected: Signal<bool>,

    /// Whether the checkbox is currently indeterminate.
    pub is_indeterminate: Signal<bool>,

    /// Whether the checkbox is pressed (during interaction).
    pub is_pressed: Signal<bool>,

    /// Whether the focus ring should be visible (keyboard navigation only).
    pub is_focus_visible: Signal<bool>,
}

/// Attributes for the checkbox input element.
pub type UseCheckboxInputAttrs = (
    Attr<attr::Type, &'static str>,
    Attr<attr::Name, Option<&'static str>>,
    Attr<attr::Value, Option<&'static str>>,
    Attr<attr::Checked, Signal<bool>>,
    Attr<attr::Disabled, Signal<bool>>,
    Attr<attr::AriaLabel, Option<&'static str>>,
    Attr<attr::AriaInvalid, Option<&'static str>>,
    Attr<attr::AriaRequired, Option<&'static str>>,
    attr::custom::CustomAttr<&'static str, Signal<Option<&'static str>>>,
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
///         <input {..checkbox.input_props} />
///         "Accept terms and conditions"
///     </label>
/// }
/// ```
pub fn use_checkbox(input: UseCheckboxInput) -> UseCheckboxReturn {
    let is_selected = input.is_selected;
    let is_indeterminate = input.is_indeterminate;
    let on_change = input.on_change;
    let is_disabled = input.is_disabled;
    let is_read_only = input.is_read_only;

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
    let aria_invalid = if input.validation_state == ValidationState::Invalid {
        Some("true")
    } else {
        None
    };

    // Compute aria-required
    let aria_required = if input.is_required {
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
    let (on_focus, on_blur, on_focusin, on_focusout, data_focus_visible) =
        focus_ring_props.into_attrs();

    UseCheckboxReturn {
        input_props: (
            Attr(attr::Type, "checkbox"),
            Attr(attr::Name, input.name),
            Attr(attr::Value, input.value),
            Attr(attr::Checked, is_selected),
            Attr(attr::Disabled, is_disabled),
            Attr(attr::AriaLabel, input.aria_label),
            Attr(attr::AriaInvalid, aria_invalid),
            Attr(attr::AriaRequired, aria_required),
            data_focus_visible,
            on(ev::change, handle_change).into_cloneable(),
            on_focus,
            on_blur,
            on_focusin,
            on_focusout,
        ),
        is_selected,
        is_indeterminate,
        is_pressed: is_pressed.into(),
        is_focus_visible,
    }
}

/// Creates internal state for a checkbox component.
pub fn use_checkbox_state(default_selected: bool) -> UseCheckboxStateReturn {
    let (is_selected, set_is_selected) = signal(default_selected);

    UseCheckboxStateReturn {
        is_selected: is_selected.into(),
        toggle: Callback::new(move |_| {
            set_is_selected.update(|v| *v = !*v);
        }),
        set_selected: Callback::new(move |selected| {
            set_is_selected.set(selected);
        }),
    }
}

/// State for managing checkbox state.
#[derive(Clone, Copy)]
pub struct UseCheckboxStateReturn {
    /// Whether the checkbox is selected.
    pub is_selected: Signal<bool>,

    /// Toggle the selection state.
    pub toggle: Callback<()>,

    /// Set the selection state.
    pub set_selected: Callback<bool>,
}
