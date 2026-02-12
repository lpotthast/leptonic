use leptos::attr::custom::{custom_attribute, CustomAttr};
use leptos::attr::Attr;
use leptos::ev::{On, SharedEventCallback};
use leptos::prelude::*;
use leptos::{attr, ev};
use uuid::Uuid;
use wasm_bindgen::JsCast;
use web_sys::{Event, FocusEvent, KeyboardEvent, MouseEvent};

use super::use_field::ValidationState;
use crate::hooks::focus::use_focus_ring::{use_focus_ring, UseFocusRingInput, UseFocusRingReturn};
use crate::utils::aria::{AriaInvalid, AriaLive, AriaRequired};
use crate::utils::{EventAccessors, EventHandler};

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/@react-aria/numberfield/src/useNumberField.ts

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// No intentional deviations from the react-aria implementation.
//
// =============================================================================

/// Input parameters for the `use_number_field` hook.
#[derive(Clone)]
pub struct UseNumberFieldInput {
    /// The current value (controlled).
    pub value: Signal<Option<f64>>,

    /// Callback when the value changes.
    pub on_change: Option<Callback<Option<f64>>>,

    /// Callback when input receives focus.
    pub on_focus: Option<Callback<()>>,

    /// Callback when input loses focus.
    pub on_blur: Option<Callback<()>>,

    /// Whether the field is disabled.
    pub is_disabled: Signal<bool>,

    /// Whether the field is read-only.
    pub is_read_only: Signal<bool>,

    /// Whether the field is required.
    pub is_required: bool,

    /// The validation state of the field.
    pub validation_state: ValidationState,

    /// Placeholder text.
    pub placeholder: Option<&'static str>,

    /// An accessibility label for the field.
    pub aria_label: Option<&'static str>,

    /// The name attribute for form submission.
    pub name: Option<&'static str>,

    /// The label for the field.
    pub label: Option<String>,

    /// A description for the field.
    pub description: Option<String>,

    /// An error message for the field.
    pub error_message: Option<String>,

    /// The minimum value.
    pub min_value: Option<f64>,

    /// The maximum value.
    pub max_value: Option<f64>,

    /// The step value for increment/decrement.
    pub step: f64,

    /// Number of decimal places.
    pub decimal_places: Option<usize>,

    /// Number of decimal places.
    pub format_options: Option<NumberFormatOptions>,

    /// Whether to auto-focus the input on mount.
    pub auto_focus: bool,
}

/// Options for number formatting.
#[derive(Debug, Clone, Default)]
pub struct NumberFormatOptions {
    /// Minimum fraction digits.
    pub minimum_fraction_digits: Option<usize>,

    /// Maximum fraction digits.
    pub maximum_fraction_digits: Option<usize>,

    /// Whether to use grouping (thousands separators).
    pub use_grouping: bool,
}

impl Default for UseNumberFieldInput {
    fn default() -> Self {
        Self {
            value: Signal::derive(|| None),
            on_change: None,
            on_focus: None,
            on_blur: None,
            is_disabled: Signal::derive(|| false),
            is_read_only: Signal::derive(|| false),
            is_required: false,
            validation_state: ValidationState::Valid,
            placeholder: None,
            aria_label: None,
            name: None,
            label: None,
            description: None,
            error_message: None,
            min_value: None,
            max_value: None,
            step: 1.0,
            decimal_places: None,
            format_options: None,
            auto_focus: false,
        }
    }
}

/// The return value of the `use_number_field` hook.
#[derive(Clone)]
pub struct UseNumberFieldReturn {
    /// Props for the input element.
    pub input_props: UseNumberFieldInputProps,

    /// Props for the increment button element.
    pub increment_button_props: UseNumberFieldButtonProps,

    /// Props for the decrement button element.
    pub decrement_button_props: UseNumberFieldButtonProps,

    /// Props for the label element.
    pub label_props: UseNumberFieldLabelProps,

    /// Props for the description element.
    pub description_props: UseNumberFieldDescriptionProps,

    /// Props for the error message element.
    pub error_props: UseNumberFieldErrorProps,

    /// The formatted display value.
    pub display_value: Signal<String>,

    /// Whether increment is allowed.
    pub can_increment: Signal<bool>,

    /// Whether decrement is allowed.
    pub can_decrement: Signal<bool>,

    /// Whether the focus ring should be visible (keyboard navigation only).
    pub is_focus_visible: Signal<bool>,
}

/// Props from `use_number_field` for the input element.
/// Call `.to_attrs()` or `.into_attrs()` for view spreading.
#[derive(Debug, Clone)]
pub struct UseNumberFieldInputProps {
    pub id: String,
    pub r#type: &'static str,
    pub role: &'static str,
    pub name: Option<&'static str>,
    pub placeholder: Option<&'static str>,
    pub disabled: Signal<bool>,
    pub readonly: Signal<bool>,
    pub aria_label: Option<&'static str>,
    pub aria_labelledby: Option<String>,
    pub aria_describedby: Option<String>,
    pub aria_invalid: Option<AriaInvalid>,
    pub aria_required: Option<AriaRequired>,
    pub aria_valuenow: Signal<Option<f64>>,
    pub aria_valuemin: Option<f64>,
    pub aria_valuemax: Option<f64>,
    pub autofocus: bool,
    pub inputmode: &'static str,
    pub data_focus_visible: Signal<Option<&'static str>>,
    pub on_input: EventHandler<Event>,
    pub on_keydown: EventHandler<KeyboardEvent>,
    pub on_focus: EventHandler<FocusEvent>,
    pub on_blur: EventHandler<FocusEvent>,
    pub on_focusin: EventHandler<FocusEvent>,
    pub on_focusout: EventHandler<FocusEvent>,
}

impl UseNumberFieldInputProps {
    /// Convert to spreadable attributes for Leptos views, cloning internally.
    #[must_use]
    pub fn to_attrs(&self) -> UseNumberFieldInputAttrs {
        self.clone().into_attrs()
    }

    /// Convert to spreadable attributes for Leptos views, consuming self.
    #[must_use]
    pub fn into_attrs(self) -> UseNumberFieldInputAttrs {
        (
            Attr(attr::Id, self.id),
            Attr(attr::Type, self.r#type),
            Attr(attr::Role, self.role),
            Attr(attr::Name, self.name),
            Attr(attr::Placeholder, self.placeholder),
            Attr(attr::Disabled, self.disabled),
            Attr(attr::Readonly, self.readonly),
            Attr(attr::AriaLabel, self.aria_label),
            Attr(attr::AriaLabelledby, self.aria_labelledby),
            Attr(attr::AriaDescribedby, self.aria_describedby),
            Attr(attr::AriaInvalid, self.aria_invalid),
            Attr(attr::AriaRequired, self.aria_required),
            Attr(attr::AriaValuenow, self.aria_valuenow),
            Attr(attr::AriaValuemin, self.aria_valuemin),
            Attr(attr::AriaValuemax, self.aria_valuemax),
            Attr(attr::Autofocus, self.autofocus),
            Attr(attr::Inputmode, self.inputmode),
            custom_attribute("data-focus-visible", self.data_focus_visible),
            self.on_input.into_on(ev::input),
            self.on_keydown.into_on(ev::keydown),
            self.on_focus.into_on(ev::focus),
            self.on_blur.into_on(ev::blur),
            self.on_focusin.into_on(ev::focusin),
            self.on_focusout.into_on(ev::focusout),
        )
    }
}

/// Attributes for the number field input element.
/// Spread onto the input element using `<input {..input_props.into_attrs()}>`.
pub type UseNumberFieldInputAttrs = (
    Attr<attr::Id, String>,
    Attr<attr::Type, &'static str>,
    Attr<attr::Role, &'static str>,
    Attr<attr::Name, Option<&'static str>>,
    Attr<attr::Placeholder, Option<&'static str>>,
    Attr<attr::Disabled, Signal<bool>>,
    Attr<attr::Readonly, Signal<bool>>,
    Attr<attr::AriaLabel, Option<&'static str>>,
    Attr<attr::AriaLabelledby, Option<String>>,
    Attr<attr::AriaDescribedby, Option<String>>,
    Attr<attr::AriaInvalid, Option<AriaInvalid>>,
    Attr<attr::AriaRequired, Option<AriaRequired>>,
    Attr<attr::AriaValuenow, Signal<Option<f64>>>,
    Attr<attr::AriaValuemin, Option<f64>>,
    Attr<attr::AriaValuemax, Option<f64>>,
    Attr<attr::Autofocus, bool>,
    Attr<attr::Inputmode, &'static str>,
    CustomAttr<&'static str, Signal<Option<&'static str>>>,
    On<ev::input, SharedEventCallback<Event>>,
    On<ev::keydown, SharedEventCallback<KeyboardEvent>>,
    On<ev::focus, SharedEventCallback<FocusEvent>>,
    On<ev::blur, SharedEventCallback<FocusEvent>>,
    On<ev::focusin, SharedEventCallback<FocusEvent>>,
    On<ev::focusout, SharedEventCallback<FocusEvent>>,
);

/// Props from `use_number_field` for the increment/decrement button elements.
/// Call `.to_attrs()` or `.into_attrs()` for view spreading.
#[derive(Debug, Clone)]
pub struct UseNumberFieldButtonProps {
    pub r#type: &'static str,
    pub aria_label: &'static str,
    pub tabindex: &'static str,
    pub disabled: Signal<bool>,
    pub on_click: EventHandler<MouseEvent>,
}

impl UseNumberFieldButtonProps {
    /// Convert to spreadable attributes for Leptos views, cloning internally.
    #[must_use]
    pub fn to_attrs(&self) -> UseNumberFieldButtonAttrs {
        self.clone().into_attrs()
    }

    /// Convert to spreadable attributes for Leptos views, consuming self.
    #[must_use]
    pub fn into_attrs(self) -> UseNumberFieldButtonAttrs {
        (
            Attr(attr::Type, self.r#type),
            Attr(attr::AriaLabel, self.aria_label),
            Attr(attr::Tabindex, self.tabindex),
            Attr(attr::Disabled, self.disabled),
            self.on_click.into_on(ev::click),
        )
    }
}

/// Attributes for increment/decrement button elements.
/// Spread onto the button element using `<button {..button_props.into_attrs()}>`.
pub type UseNumberFieldButtonAttrs = (
    Attr<attr::Type, &'static str>,
    Attr<attr::AriaLabel, &'static str>,
    Attr<attr::Tabindex, &'static str>,
    Attr<attr::Disabled, Signal<bool>>,
    On<ev::click, SharedEventCallback<MouseEvent>>,
);

/// Props for the label element.
#[derive(Debug, Clone)]
pub struct UseNumberFieldLabelProps {
    /// The id of the label element.
    pub id: String,

    /// The "for" attribute linking to the input.
    pub html_for: String,
}

impl UseNumberFieldLabelProps {
    /// Convert to spreadable attributes for Leptos views, cloning internally.
    #[must_use]
    pub fn to_attrs(&self) -> UseNumberFieldLabelAttrs {
        self.clone().into_attrs()
    }

    /// Convert to spreadable attributes for Leptos views, consuming self.
    #[must_use]
    pub fn into_attrs(self) -> UseNumberFieldLabelAttrs {
        (Attr(attr::Id, self.id), Attr(attr::For, self.html_for))
    }
}

/// Attributes for the label element (id, for).
/// Spread onto the label element using `<label {..label_props.into_attrs()}>`.
pub type UseNumberFieldLabelAttrs = (Attr<attr::Id, String>, Attr<attr::For, String>);

/// Props for the description element.
#[derive(Debug, Clone)]
pub struct UseNumberFieldDescriptionProps {
    /// The id of the description element.
    pub id: String,
}

impl UseNumberFieldDescriptionProps {
    /// Convert to spreadable attributes for Leptos views, cloning internally.
    #[must_use]
    pub fn to_attrs(&self) -> UseNumberFieldDescriptionAttrs {
        self.clone().into_attrs()
    }

    /// Convert to spreadable attributes for Leptos views, consuming self.
    #[must_use]
    pub fn into_attrs(self) -> UseNumberFieldDescriptionAttrs {
        (Attr(attr::Id, self.id),)
    }
}

/// Attributes for the description element (id).
/// Spread onto the description element using `<p {..description_props.into_attrs()}>`.
pub type UseNumberFieldDescriptionAttrs = (Attr<attr::Id, String>,);

/// Props for the error message element.
#[derive(Debug, Clone)]
pub struct UseNumberFieldErrorProps {
    /// The id of the error message element.
    pub id: String,

    /// The role attribute.
    pub role: &'static str,

    /// The aria-live attribute.
    pub aria_live: AriaLive,
}

impl UseNumberFieldErrorProps {
    /// Convert to spreadable attributes for Leptos views, cloning internally.
    #[must_use]
    pub fn to_attrs(&self) -> UseNumberFieldErrorAttrs {
        self.clone().into_attrs()
    }

    /// Convert to spreadable attributes for Leptos views, consuming self.
    #[must_use]
    pub fn into_attrs(self) -> UseNumberFieldErrorAttrs {
        (
            Attr(attr::Id, self.id),
            Attr(attr::Role, self.role),
            Attr(attr::AriaLive, self.aria_live),
        )
    }
}

/// Attributes for the error message element (id, role, aria-live).
/// Spread onto the error message element using `<p {..error_props.into_attrs()}>`.
pub type UseNumberFieldErrorAttrs = (
    Attr<attr::Id, String>,
    Attr<attr::Role, &'static str>,
    Attr<attr::AriaLive, AriaLive>,
);

/// Provides the behavior and accessibility implementation for a number field.
///
/// Number fields allow users to enter numeric values with increment/decrement buttons.
///
/// # Example
///
/// ```ignore
/// let (value, set_value) = signal(Some(0.0));
///
/// let number_field = use_number_field(UseNumberFieldInput {
///     value: value.into(),
///     on_change: Some(Callback::new(move |v| {
///         set_value.set(v);
///     })),
///     min_value: Some(0.0),
///     max_value: Some(100.0),
///     step: 1.0,
///     label: Some("Quantity".to_string()),
///     ..Default::default()
/// });
///
/// view! {
///     <div>
///         <label {..number_field.label_props.into_attrs()}>"Quantity"</label>
///         <button {..number_field.decrement_button_props.into_attrs()}>"-"</button>
///         <input value=number_field.display_value {..number_field.input_props.into_attrs()} />
///         <button {..number_field.increment_button_props.into_attrs()}>"+"</button>
///     </div>
/// }
/// ```
#[allow(clippy::too_many_lines, clippy::needless_pass_by_value)]
pub fn use_number_field(input: UseNumberFieldInput) -> UseNumberFieldReturn {
    let UseNumberFieldInput {
        value,
        on_change,
        on_focus,
        on_blur,
        is_disabled,
        is_read_only,
        is_required,
        validation_state,
        placeholder,
        aria_label,
        name,
        label,
        description,
        error_message,
        min_value,
        max_value,
        step,
        decimal_places,
        format_options: _,
        auto_focus,
    } = input;

    let base_id = Uuid::new_v4();
    let input_id = format!("numberfield-{base_id}");
    let label_id = format!("numberfield-label-{base_id}");
    let description_id = format!("numberfield-description-{base_id}");
    let error_id = format!("numberfield-error-{base_id}");

    // Clamp value to min/max
    let clamp_value = move |v: f64| -> f64 {
        let mut result = v;
        if let Some(min) = min_value {
            result = result.max(min);
        }
        if let Some(max) = max_value {
            result = result.min(max);
        }
        result
    };

    // Format the display value
    let display_value = Signal::derive(move || {
        value.get().map_or(String::new(), |v| {
            if let Some(places) = decimal_places {
                format!("{v:.places$}")
            } else {
                v.to_string()
            }
        })
    });

    // Whether we can increment
    let can_increment = Signal::derive(move || {
        if is_disabled.get() || is_read_only.get() {
            return false;
        }
        match (value.get(), max_value) {
            (Some(v), Some(max)) => v < max,
            (None, _) | (Some(_), None) => true,
        }
    });

    // Whether we can decrement
    let can_decrement = Signal::derive(move || {
        if is_disabled.get() || is_read_only.get() {
            return false;
        }
        match (value.get(), min_value) {
            (Some(v), Some(min)) => v > min,
            (None, _) | (Some(_), None) => true,
        }
    });

    // Increment the value
    let increment = move || {
        if !can_increment.get_untracked() {
            return;
        }

        let current = value.get_untracked().unwrap_or(min_value.unwrap_or(0.0));
        let new_value = clamp_value(current + step);

        if let Some(on_change) = on_change {
            on_change.run(Some(new_value));
        }
    };

    // Decrement the value
    let decrement = move || {
        if !can_decrement.get_untracked() {
            return;
        }

        let current = value.get_untracked().unwrap_or(max_value.unwrap_or(0.0));
        let new_value = clamp_value(current - step);

        if let Some(on_change) = on_change {
            on_change.run(Some(new_value));
        }
    };

    // Handle input event
    let handle_input = move |e: Event| {
        if is_disabled.get_untracked() || is_read_only.get_untracked() {
            return;
        }

        if let Some(input_el) = e
            .expect_target()
            .dyn_into::<web_sys::HtmlInputElement>()
            .ok()
        {
            let text_value = input_el.value();

            // Parse the value
            let new_value = if text_value.is_empty() {
                None
            } else {
                text_value.parse::<f64>().ok().map(clamp_value)
            };

            if let Some(on_change) = on_change {
                on_change.run(new_value);
            }
        }
    };

    // Handle keydown event
    let increment_key = increment;
    let decrement_key = decrement;
    let handle_keydown = move |e: KeyboardEvent| {
        if is_disabled.get_untracked() || is_read_only.get_untracked() {
            return;
        }

        match e.key().as_str() {
            "ArrowUp" => {
                e.prevent_default();
                increment_key();
            }
            "ArrowDown" => {
                e.prevent_default();
                decrement_key();
            }
            "Home" => {
                if let Some(min) = min_value {
                    e.prevent_default();
                    if let Some(on_change) = on_change {
                        on_change.run(Some(min));
                    }
                }
            }
            "End" => {
                if let Some(max) = max_value {
                    e.prevent_default();
                    if let Some(on_change) = on_change {
                        on_change.run(Some(max));
                    }
                }
            }
            _ => {}
        }
    };

    // Use focus ring to track focus visibility with user callbacks
    let UseFocusRingReturn {
        props: focus_ring_props,
        is_focus_visible,
        is_focused: _,
    } = use_focus_ring(UseFocusRingInput {
        disabled: is_disabled,
        within: false,
        auto_focus,
        on_focus: on_focus.map(|cb| Callback::new(move |_| cb.run(()))),
        on_blur: on_blur.map(|cb| Callback::new(move |_| cb.run(()))),
        on_focus_change: None,
    });

    // Handle increment button click
    let increment_click = increment;
    let handle_increment = move |_e: MouseEvent| {
        increment_click();
    };

    // Handle decrement button click
    let decrement_click = decrement;
    let handle_decrement = move |_e: MouseEvent| {
        decrement_click();
    };

    // Build aria-describedby
    let mut describedby_parts = Vec::new();
    if description.is_some() {
        describedby_parts.push(description_id.clone());
    }
    if validation_state == ValidationState::Invalid && error_message.is_some() {
        describedby_parts.push(error_id.clone());
    }

    let aria_describedby = if describedby_parts.is_empty() {
        None
    } else {
        Some(describedby_parts.join(" "))
    };

    // Build aria-labelledby
    let aria_labelledby = if label.is_some() {
        Some(label_id.clone())
    } else {
        None
    };

    // Compute aria-invalid
    let aria_invalid = (validation_state == ValidationState::Invalid).then_some(AriaInvalid::True);

    // Compute aria-required
    let aria_required = is_required.then_some(AriaRequired::True);

    // Disabled signals for buttons
    let increment_disabled = Signal::derive(move || !can_increment.get());
    let decrement_disabled = Signal::derive(move || !can_decrement.get());

    UseNumberFieldReturn {
        input_props: UseNumberFieldInputProps {
            id: input_id.clone(),
            r#type: "text",
            role: "spinbutton",
            name,
            placeholder,
            disabled: is_disabled,
            readonly: is_read_only,
            aria_label,
            aria_labelledby,
            aria_describedby,
            aria_invalid,
            aria_required,
            aria_valuenow: value,
            aria_valuemin: min_value,
            aria_valuemax: max_value,
            autofocus: auto_focus,
            inputmode: "decimal",
            data_focus_visible: focus_ring_props.data_focus_visible,
            on_input: EventHandler::new(handle_input),
            on_keydown: EventHandler::new(handle_keydown),
            on_focus: focus_ring_props.on_focus,
            on_blur: focus_ring_props.on_blur,
            on_focusin: focus_ring_props.on_focusin,
            on_focusout: focus_ring_props.on_focusout,
        },
        increment_button_props: UseNumberFieldButtonProps {
            r#type: "button",
            aria_label: "Increase value",
            tabindex: "-1",
            disabled: increment_disabled,
            on_click: EventHandler::new(handle_increment),
        },
        decrement_button_props: UseNumberFieldButtonProps {
            r#type: "button",
            aria_label: "Decrease value",
            tabindex: "-1",
            disabled: decrement_disabled,
            on_click: EventHandler::new(handle_decrement),
        },
        label_props: UseNumberFieldLabelProps {
            id: label_id,
            html_for: input_id,
        },
        description_props: UseNumberFieldDescriptionProps { id: description_id },
        error_props: UseNumberFieldErrorProps {
            id: error_id,
            role: "alert",
            aria_live: AriaLive::Polite,
        },
        display_value,
        can_increment,
        can_decrement,
        is_focus_visible,
    }
}
