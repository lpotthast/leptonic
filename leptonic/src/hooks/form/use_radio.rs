use leptos::attr;
use leptos::attr::custom::{custom_attribute, CustomAttr};
use leptos::attr::Attr;
use leptos::ev;
use leptos::ev::{On, SharedEventCallback};
use leptos::prelude::*;
use web_sys::{Event, FocusEvent};

use super::use_field::ValidationState;
use super::use_radio_group::UseRadioGroupState;
use crate::hooks::focus::use_focus_ring::{use_focus_ring, UseFocusRingInput, UseFocusRingReturn};
use crate::utils::aria::AriaInvalid;
use crate::utils::EventHandler;

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/@react-aria/radio/src/useRadio.ts

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// No intentional deviations from the react-aria implementation.
//
// =============================================================================

/// Input parameters for the `use_radio` hook.
#[derive(Clone, Copy)]
pub struct UseRadioInput<T>
where
    T: PartialEq + Clone + Send + Sync + 'static,
{
    /// The value of this radio button.
    pub value: T,

    /// The radio group state (from `use_radio_group`).
    pub state: UseRadioGroupState<T>,

    /// Whether this radio is disabled.
    pub is_disabled: Signal<bool>,

    /// The validation state of the radio.
    pub validation_state: ValidationState,

    /// An accessibility label for the radio.
    pub aria_label: Option<&'static str>,

    /// The name attribute for form submission.
    pub name: Option<&'static str>,
}

/// The return value of the `use_radio` hook.
#[derive(Debug)]
pub struct UseRadioReturn {
    /// Props for the radio input element. Call `.into_attrs()` for view spreading.
    pub input_props: UseRadioInputProps,

    /// Whether this radio is currently selected.
    pub is_selected: Signal<bool>,

    /// Whether this radio is currently disabled.
    pub is_disabled: Signal<bool>,

    /// Whether this radio is pressed (during interaction).
    pub is_pressed: Signal<bool>,

    /// Whether the focus ring should be visible (keyboard navigation only).
    pub is_focus_visible: Signal<bool>,
}

/// Props from `use_radio` that can be extracted and merged programmatically.
#[derive(Debug)]
pub struct UseRadioInputProps {
    pub r#type: &'static str,
    pub name: Option<&'static str>,
    pub checked: Signal<bool>,
    pub disabled: Signal<bool>,
    pub aria_label: Option<&'static str>,
    pub aria_invalid: Option<AriaInvalid>,
    pub data_focus_visible: Signal<Option<&'static str>>,
    pub on_change: EventHandler<Event>,
    pub on_focus: EventHandler<FocusEvent>,
    pub on_blur: EventHandler<FocusEvent>,
    pub on_focusin: EventHandler<FocusEvent>,
    pub on_focusout: EventHandler<FocusEvent>,
}

impl UseRadioInputProps {
    /// Convert to spreadable attributes for Leptos views, consuming self.
    #[must_use]
    pub fn into_attrs(self) -> UseRadioInputAttrs {
        (
            Attr(attr::Type, self.r#type),
            Attr(attr::Name, self.name),
            Attr(attr::Checked, self.checked),
            Attr(attr::Disabled, self.disabled),
            Attr(attr::AriaLabel, self.aria_label),
            Attr(attr::AriaInvalid, self.aria_invalid),
            custom_attribute("data-focus-visible", self.data_focus_visible),
            self.on_change.into_on(ev::change),
            self.on_focus.into_on(ev::focus),
            self.on_blur.into_on(ev::blur),
            self.on_focusin.into_on(ev::focusin),
            self.on_focusout.into_on(ev::focusout),
        )
    }
}

/// These attributes must be spread onto the target element using the spread syntax `<input {..attrs}/>`.
pub type UseRadioInputAttrs = (
    Attr<attr::Type, &'static str>,
    Attr<attr::Name, Option<&'static str>>,
    Attr<attr::Checked, Signal<bool>>,
    Attr<attr::Disabled, Signal<bool>>,
    Attr<attr::AriaLabel, Option<&'static str>>,
    Attr<attr::AriaInvalid, Option<AriaInvalid>>,
    CustomAttr<&'static str, Signal<Option<&'static str>>>,
    On<ev::change, SharedEventCallback<Event>>,
    On<ev::focus, SharedEventCallback<FocusEvent>>,
    On<ev::blur, SharedEventCallback<FocusEvent>>,
    On<ev::focusin, SharedEventCallback<FocusEvent>>,
    On<ev::focusout, SharedEventCallback<FocusEvent>>,
);

/// Provides the behavior and accessibility implementation for a radio button.
///
/// Radio buttons allow users to select a single option from a set.
///
/// # Example
///
/// ```ignore
/// let group = use_radio_group(UseRadioGroupInput {
///     value: selected.into(),
///     on_change: Some(Callback::new(move |value| {
///         set_selected.set(value);
///     })),
///     ..Default::default()
/// });
///
/// let radio = use_radio(UseRadioInput {
///     value: "option1",
///     state: group.state,
///     is_disabled: Signal::derive(|| false),
///     validation_state: ValidationState::Valid,
///     aria_label: Some("Option 1"),
///     name: Some("choice"),
/// });
///
/// view! {
///     <label>
///         <input {..radio.input_props.into_attrs()} />
///         "Option 1"
///     </label>
/// }
/// ```
pub fn use_radio<T>(input: UseRadioInput<T>) -> UseRadioReturn
where
    T: PartialEq + Clone + Send + Sync + 'static,
{
    let UseRadioInput {
        value,
        state,
        is_disabled: local_disabled,
        validation_state,
        aria_label,
        name,
    } = input;

    let (is_pressed, _set_is_pressed) = signal(false);

    // Combine local and group disabled state
    let is_disabled = Signal::derive(move || local_disabled.get() || state.is_disabled.get());

    // Check if this radio is selected
    let value_for_selected = value.clone();
    let is_selected =
        Signal::derive(move || state.selected_value.get().as_ref() == Some(&value_for_selected));

    // Handle change event
    let value_for_change = value.clone();
    let handle_change = move |_e: Event| {
        if is_disabled.get_untracked() || state.is_read_only.get_untracked() {
            return;
        }

        state.set_selected_value.run(value_for_change.clone());
    };

    // Compute aria-invalid
    let aria_invalid = (validation_state == ValidationState::Invalid).then_some(AriaInvalid::True);

    // Use focus ring for keyboard focus visibility
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
    UseRadioReturn {
        input_props: UseRadioInputProps {
            r#type: "radio",
            name,
            checked: is_selected,
            disabled: is_disabled,
            aria_label,
            aria_invalid,
            data_focus_visible: focus_ring_props.data_focus_visible,
            on_change: EventHandler::new(handle_change),
            on_focus: focus_ring_props.on_focus,
            on_blur: focus_ring_props.on_blur,
            on_focusin: focus_ring_props.on_focusin,
            on_focusout: focus_ring_props.on_focusout,
        },
        is_selected,
        is_disabled,
        is_pressed: is_pressed.into(),
        is_focus_visible,
    }
}
