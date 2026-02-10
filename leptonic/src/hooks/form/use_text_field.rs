use super::use_field::ValidationState;
use crate::hooks::focus::use_focus_ring::{use_focus_ring, UseFocusRingInput, UseFocusRingReturn};
use crate::utils::aria::{AriaInvalid, AriaLive, AriaRequired};
use leptos::attr;
use leptos::attr::{Attr, Attribute};
use leptos::ev;
use leptos::ev::{on, On, SharedEventCallback};
use leptos::prelude::*;
use uuid::Uuid;
use wasm_bindgen::JsCast;
use web_sys::{Event, FocusEvent};

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/@react-aria/textfield/src/useTextField.ts

/// Input parameters for the `use_text_field` hook.
#[derive(Clone)]
pub struct UseTextFieldInput {
    /// The current value (controlled).
    pub value: Signal<String>,

    /// Callback when the value changes.
    pub on_change: Option<Callback<String>>,

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

    /// The type of input (text, email, password, etc.).
    pub input_type: &'static str,

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

    /// Minimum length for the input value.
    pub min_length: Option<u32>,

    /// Maximum length for the input value.
    pub max_length: Option<u32>,

    /// Pattern for validation.
    pub pattern: Option<&'static str>,

    /// Whether to enable autocomplete.
    pub auto_complete: Option<&'static str>,

    /// Whether to auto-focus the input on mount.
    pub auto_focus: bool,
}

impl Default for UseTextFieldInput {
    fn default() -> Self {
        Self {
            value: Signal::derive(String::new),
            on_change: None,
            on_focus: None,
            on_blur: None,
            is_disabled: Signal::derive(|| false),
            is_read_only: Signal::derive(|| false),
            is_required: false,
            validation_state: ValidationState::Valid,
            input_type: "text",
            placeholder: None,
            aria_label: None,
            name: None,
            label: None,
            description: None,
            error_message: None,
            min_length: None,
            max_length: None,
            pattern: None,
            auto_complete: None,
            auto_focus: false,
        }
    }
}

/// The return value of the `use_text_field` hook.
#[derive(Clone)]
pub struct UseTextFieldReturn {
    /// Props for the input element.
    pub input_props: UseTextFieldInputAttrs,

    /// Props for the label element.
    pub label_props: UseTextFieldLabelProps,

    /// Props for the description element.
    pub description_props: UseTextFieldDescriptionProps,

    /// Props for the error message element.
    pub error_props: UseTextFieldErrorProps,

    /// Whether the focus ring should be visible (keyboard navigation only).
    pub is_focus_visible: Signal<bool>,
}

/// Attributes for the text field input element.
pub type UseTextFieldInputAttrs = (
    Attr<attr::Id, String>,
    Attr<attr::Type, &'static str>,
    Attr<attr::Name, Option<&'static str>>,
    Attr<attr::Value, Signal<String>>,
    Attr<attr::Placeholder, Option<&'static str>>,
    Attr<attr::Disabled, Signal<bool>>,
    Attr<attr::Readonly, Signal<bool>>,
    Attr<attr::AriaLabel, Option<&'static str>>,
    Attr<attr::AriaLabelledby, Option<String>>,
    Attr<attr::AriaDescribedby, Option<String>>,
    Attr<attr::AriaInvalid, Option<AriaInvalid>>,
    Attr<attr::AriaRequired, Option<AriaRequired>>,
    Attr<attr::Minlength, Option<u32>>,
    Attr<attr::Maxlength, Option<u32>>,
    Attr<attr::Pattern, Option<&'static str>>,
    Attr<attr::Autocomplete, Option<&'static str>>,
    Attr<attr::Autofocus, bool>,
    attr::custom::CustomAttr<&'static str, Signal<Option<&'static str>>>,
    On<ev::input, SharedEventCallback<Event>>,
    On<ev::focus, SharedEventCallback<FocusEvent>>,
    On<ev::blur, SharedEventCallback<FocusEvent>>,
    On<ev::focusin, SharedEventCallback<FocusEvent>>,
    On<ev::focusout, SharedEventCallback<FocusEvent>>,
);

/// Props for the label element.
#[derive(Debug, Clone)]
pub struct UseTextFieldLabelProps {
    /// The id of the label element.
    pub id: String,

    /// The "for" attribute linking to the input.
    pub html_for: String,
}

/// Props for the description element.
#[derive(Debug, Clone)]
pub struct UseTextFieldDescriptionProps {
    /// The id of the description element.
    pub id: String,
}

/// Props for the error message element.
#[derive(Debug, Clone)]
pub struct UseTextFieldErrorProps {
    /// The id of the error message element.
    pub id: String,

    /// The role attribute.
    pub role: &'static str,

    /// The aria-live attribute.
    pub aria_live: AriaLive,
}

/// Provides the behavior and accessibility implementation for a text field.
///
/// Text fields allow users to input text with various configurations.
///
/// # Example
///
/// ```ignore
/// let (value, set_value) = signal(String::new());
///
/// let text_field = use_text_field(UseTextFieldInput {
///     value: value.into(),
///     on_change: Some(Callback::new(move |v| {
///         set_value.set(v);
///     })),
///     label: Some("Email".to_string()),
///     placeholder: Some("Enter your email"),
///     input_type: "email",
///     is_required: true,
///     ..Default::default()
/// });
///
/// view! {
///     <div>
///         <label {..text_field.label_props}>"Email"</label>
///         <input {..text_field.input_props} />
///         <p {..text_field.description_props}>"We'll never share your email"</p>
///     </div>
/// }
/// ```
#[allow(clippy::too_many_lines, clippy::needless_pass_by_value)]
pub fn use_text_field(input: UseTextFieldInput) -> UseTextFieldReturn {
    let base_id = Uuid::new_v4();
    let input_id = format!("textfield-{base_id}");
    let label_id = format!("textfield-label-{base_id}");
    let description_id = format!("textfield-description-{base_id}");
    let error_id = format!("textfield-error-{base_id}");

    let value = input.value;
    let on_change = input.on_change;
    let on_focus = input.on_focus;
    let on_blur = input.on_blur;
    let is_disabled = input.is_disabled;
    let is_read_only = input.is_read_only;

    // Handle input event
    let handle_input = move |e: Event| {
        if is_disabled.get_untracked() || is_read_only.get_untracked() {
            return;
        }

        if let Some(input_el) = e
            .target()
            .and_then(|t| t.dyn_into::<web_sys::HtmlInputElement>().ok())
        {
            let new_value = input_el.value();
            if let Some(on_change) = on_change {
                on_change.run(new_value);
            }
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
    let aria_invalid =
        (input.validation_state == ValidationState::Invalid).then_some(AriaInvalid::True);

    // Compute aria-required
    let aria_required = input.is_required.then_some(AriaRequired::True);

    UseTextFieldReturn {
        input_props: (
            Attr(attr::Id, input_id.clone()),
            Attr(attr::Type, input.input_type),
            Attr(attr::Name, input.name),
            Attr(attr::Value, value),
            Attr(attr::Placeholder, input.placeholder),
            Attr(attr::Disabled, is_disabled),
            Attr(attr::Readonly, is_read_only),
            Attr(attr::AriaLabel, input.aria_label),
            Attr(attr::AriaLabelledby, aria_labelledby),
            Attr(attr::AriaDescribedby, aria_describedby),
            Attr(attr::AriaInvalid, aria_invalid),
            Attr(attr::AriaRequired, aria_required),
            Attr(attr::Minlength, input.min_length),
            Attr(attr::Maxlength, input.max_length),
            Attr(attr::Pattern, input.pattern),
            Attr(attr::Autocomplete, input.auto_complete),
            Attr(attr::Autofocus, input.auto_focus),
            data_focus_visible,
            on(ev::input, handle_input).into_cloneable(),
            on_focus,
            on_blur,
            on_focusin,
            on_focusout,
        ),
        label_props: UseTextFieldLabelProps {
            id: label_id,
            html_for: input_id,
        },
        description_props: UseTextFieldDescriptionProps { id: description_id },
        error_props: UseTextFieldErrorProps {
            id: error_id,
            role: "alert",
            aria_live: AriaLive::Polite,
        },
        is_focus_visible,
    }
}

/// Creates internal state for a text field component.
pub fn use_text_field_state(default_value: &str) -> UseTextFieldStateReturn {
    let (value, set_value) = signal(default_value.to_string());

    UseTextFieldStateReturn {
        value: value.into(),
        set_value: Callback::new(move |v: String| {
            set_value.set(v);
        }),
        clear: Callback::new(move |_| {
            set_value.set(String::new());
        }),
    }
}

/// State for managing text field state.
#[derive(Clone, Copy)]
pub struct UseTextFieldStateReturn {
    /// The current value.
    pub value: Signal<String>,

    /// Set the value.
    pub set_value: Callback<String>,

    /// Clear the value.
    pub clear: Callback<()>,
}
