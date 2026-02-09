use leptos::attr;
use leptos::attr::{Attr, Attribute};
use leptos::ev;
use leptos::ev::{on, On, SharedEventCallback};
use leptos::prelude::*;
use uuid::Uuid;
use wasm_bindgen::JsCast;
use web_sys::{Event, FocusEvent, KeyboardEvent, MouseEvent};

use super::use_field::ValidationState;
use crate::hooks::focus::use_focus_ring::{use_focus_ring, UseFocusRingInput, UseFocusRingReturn};
use crate::utils::aria::{AriaInvalid, AriaLive, AriaRequired};

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/@react-aria/numberfield/src/useNumberField.ts

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

    /// Whether to format with thousands separators.
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
    pub input_props: UseNumberFieldInputAttrs,

    /// Props for the increment button element.
    pub increment_button_props: UseNumberFieldButtonAttrs,

    /// Props for the decrement button element.
    pub decrement_button_props: UseNumberFieldButtonAttrs,

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

/// Attributes for the number field input element.
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
    attr::custom::CustomAttr<&'static str, Signal<Option<&'static str>>>,
    On<ev::input, SharedEventCallback<Event>>,
    On<ev::keydown, SharedEventCallback<KeyboardEvent>>,
    On<ev::focus, SharedEventCallback<FocusEvent>>,
    On<ev::blur, SharedEventCallback<FocusEvent>>,
    On<ev::focusin, SharedEventCallback<FocusEvent>>,
    On<ev::focusout, SharedEventCallback<FocusEvent>>,
);

/// Attributes for increment/decrement button elements.
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

/// Props for the description element.
#[derive(Debug, Clone)]
pub struct UseNumberFieldDescriptionProps {
    /// The id of the description element.
    pub id: String,
}

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
///         <label {..number_field.label_props}>"Quantity"</label>
///         <button {..number_field.decrement_button_props}>"-"</button>
///         <input value=number_field.display_value {..number_field.input_props} />
///         <button {..number_field.increment_button_props}>"+"</button>
///     </div>
/// }
/// ```
#[allow(clippy::too_many_lines, clippy::needless_pass_by_value)]
pub fn use_number_field(input: UseNumberFieldInput) -> UseNumberFieldReturn {
    let base_id = Uuid::new_v4();
    let input_id = format!("numberfield-{base_id}");
    let label_id = format!("numberfield-label-{base_id}");
    let description_id = format!("numberfield-description-{base_id}");
    let error_id = format!("numberfield-error-{base_id}");

    let value = input.value;
    let on_change = input.on_change;
    let on_focus = input.on_focus;
    let on_blur = input.on_blur;
    let is_disabled = input.is_disabled;
    let is_read_only = input.is_read_only;
    let min_value = input.min_value;
    let max_value = input.max_value;
    let step = input.step;
    let decimal_places = input.decimal_places;

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
            .target()
            .and_then(|t| t.dyn_into::<web_sys::HtmlInputElement>().ok())
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
        auto_focus: input.auto_focus,
        on_focus: on_focus.map(|cb| Callback::new(move |_| cb.run(()))),
        on_blur: on_blur.map(|cb| Callback::new(move |_| cb.run(()))),
        on_focus_change: None,
    });
    let (on_focus, on_blur, on_focusin, on_focusout, data_focus_visible) =
        focus_ring_props.into_attrs();

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
    if input.description.is_some() {
        describedby_parts.push(description_id.clone());
    }
    if input.validation_state == ValidationState::Invalid && input.error_message.is_some() {
        describedby_parts.push(error_id.clone());
    }

    let aria_describedby = if describedby_parts.is_empty() {
        None
    } else {
        Some(describedby_parts.join(" "))
    };

    // Build aria-labelledby
    let aria_labelledby = if input.label.is_some() {
        Some(label_id.clone())
    } else {
        None
    };

    // Compute aria-invalid
    let aria_invalid = (input.validation_state == ValidationState::Invalid).then_some(AriaInvalid::True);

    // Compute aria-required
    let aria_required = input.is_required.then_some(AriaRequired::True);

    // Disabled signals for buttons
    let increment_disabled = Signal::derive(move || !can_increment.get());
    let decrement_disabled = Signal::derive(move || !can_decrement.get());

    UseNumberFieldReturn {
        input_props: (
            Attr(attr::Id, input_id.clone()),
            Attr(attr::Type, "text"),
            Attr(attr::Role, "spinbutton"),
            Attr(attr::Name, input.name),
            Attr(attr::Placeholder, input.placeholder),
            Attr(attr::Disabled, is_disabled),
            Attr(attr::Readonly, is_read_only),
            Attr(attr::AriaLabel, input.aria_label),
            Attr(attr::AriaLabelledby, aria_labelledby),
            Attr(attr::AriaDescribedby, aria_describedby),
            Attr(attr::AriaInvalid, aria_invalid),
            Attr(attr::AriaRequired, aria_required),
            Attr(attr::AriaValuenow, value),
            Attr(attr::AriaValuemin, min_value),
            Attr(attr::AriaValuemax, max_value),
            Attr(attr::Autofocus, input.auto_focus),
            Attr(attr::Inputmode, "decimal"),
            data_focus_visible,
            on(ev::input, handle_input).into_cloneable(),
            on(ev::keydown, handle_keydown).into_cloneable(),
            on_focus,
            on_blur,
            on_focusin,
            on_focusout,
        ),
        increment_button_props: (
            Attr(attr::Type, "button"),
            Attr(attr::AriaLabel, "Increase value"),
            Attr(attr::Tabindex, "-1"),
            Attr(attr::Disabled, increment_disabled),
            on(ev::click, handle_increment).into_cloneable(),
        ),
        decrement_button_props: (
            Attr(attr::Type, "button"),
            Attr(attr::AriaLabel, "Decrease value"),
            Attr(attr::Tabindex, "-1"),
            Attr(attr::Disabled, decrement_disabled),
            on(ev::click, handle_decrement).into_cloneable(),
        ),
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

/// Creates internal state for a number field component.
pub fn use_number_field_state(default_value: Option<f64>) -> UseNumberFieldStateReturn {
    let (value, set_value) = signal(default_value);

    UseNumberFieldStateReturn {
        value: value.into(),
        set_value: Callback::new(move |v: Option<f64>| {
            set_value.set(v);
        }),
        increment: Callback::new(move |step: f64| {
            set_value.update(|v| {
                *v = Some(v.unwrap_or(0.0) + step);
            });
        }),
        decrement: Callback::new(move |step: f64| {
            set_value.update(|v| {
                *v = Some(v.unwrap_or(0.0) - step);
            });
        }),
        clear: Callback::new(move |_| {
            set_value.set(None);
        }),
    }
}

/// State for managing number field state.
#[derive(Clone, Copy)]
pub struct UseNumberFieldStateReturn {
    /// The current value.
    pub value: Signal<Option<f64>>,

    /// Set the value.
    pub set_value: Callback<Option<f64>>,

    /// Increment by a step.
    pub increment: Callback<f64>,

    /// Decrement by a step.
    pub decrement: Callback<f64>,

    /// Clear the value.
    pub clear: Callback<()>,
}
