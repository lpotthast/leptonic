use leptos::attr;
use leptos::attr::Attr;
use uuid::Uuid;

use crate::utils::aria::{AriaDisabled, AriaInvalid, AriaLive, AriaReadonly, AriaRequired};

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/@react-aria/label/src/useField.ts

/// Validation state for a field.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ValidationState {
    /// The field value is valid.
    #[default]
    Valid,
    /// The field value is invalid.
    Invalid,
}

/// Input parameters for the `use_field` hook.
#[derive(Debug, Clone)]
pub struct UseFieldInput {
    /// A custom id for the field element.
    pub id: Option<String>,

    /// The label for the field.
    pub label: Option<String>,

    /// A description for the field.
    pub description: Option<String>,

    /// An error message for the field.
    pub error_message: Option<String>,

    /// The validation state of the field.
    pub validation_state: ValidationState,

    /// Whether the field is required.
    pub is_required: bool,

    /// Whether the field is disabled.
    pub is_disabled: bool,

    /// Whether the field is read-only.
    pub is_read_only: bool,
}

/// The return value of the `use_field` hook.
#[derive(Debug, Clone)]
pub struct UseFieldReturn {
    /// Props for the label element.
    pub label_props: UseFieldLabelProps,

    /// Props for the field element.
    pub field_props: UseFieldProps,

    /// Props for the description element.
    pub description_props: UseFieldDescriptionProps,

    /// Props for the error message element.
    pub error_message_props: UseFieldErrorMessageProps,
}

/// Props for the label element.
#[derive(Debug, Clone)]
pub struct UseFieldLabelProps {
    /// The id of the label element.
    pub id: String,

    /// The "for" attribute linking to the field.
    pub html_for: String,
}

impl UseFieldLabelProps {
    pub fn into_attrs(self) -> UseFieldLabelAttrs {
        (Attr(attr::Id, self.id), Attr(attr::For, self.html_for))
    }
}

/// Attributes for the label element (id, for).
/// Spread onto the label element using `<label {..label_props.into_attrs()}>`.
pub type UseFieldLabelAttrs = (Attr<attr::Id, String>, Attr<attr::For, String>);

/// Props for the field element.
#[derive(Debug, Clone)]
pub struct UseFieldProps {
    /// The id of the field element.
    pub id: String,

    /// The aria-labelledby attribute.
    pub aria_labelledby: Option<String>,

    /// The aria-describedby attribute.
    pub aria_describedby: Option<String>,

    /// The aria-invalid attribute.
    pub aria_invalid: Option<AriaInvalid>,

    /// The aria-required attribute.
    pub aria_required: Option<AriaRequired>,

    /// The aria-disabled attribute.
    pub aria_disabled: Option<AriaDisabled>,

    /// The aria-readonly attribute.
    pub aria_readonly: Option<AriaReadonly>,
}

impl UseFieldProps {
    pub fn into_attrs(self) -> UseFieldAttrs {
        (
            Attr(attr::Id, self.id),
            Attr(attr::AriaLabelledby, self.aria_labelledby),
            Attr(attr::AriaDescribedby, self.aria_describedby),
            Attr(attr::AriaInvalid, self.aria_invalid),
            Attr(attr::AriaRequired, self.aria_required),
            Attr(attr::AriaDisabled, self.aria_disabled),
            Attr(attr::AriaReadonly, self.aria_readonly),
        )
    }
}

/// Attributes for the field element.
/// Spread onto the field element using `<input {..field_props.into_attrs()}>`.
pub type UseFieldAttrs = (
    Attr<attr::Id, String>,
    Attr<attr::AriaLabelledby, Option<String>>,
    Attr<attr::AriaDescribedby, Option<String>>,
    Attr<attr::AriaInvalid, Option<AriaInvalid>>,
    Attr<attr::AriaRequired, Option<AriaRequired>>,
    Attr<attr::AriaDisabled, Option<AriaDisabled>>,
    Attr<attr::AriaReadonly, Option<AriaReadonly>>,
);

/// Props for the description element.
#[derive(Debug, Clone)]
pub struct UseFieldDescriptionProps {
    /// The id of the description element.
    pub id: String,
}

impl UseFieldDescriptionProps {
    pub fn into_attrs(self) -> UseFieldDescriptionAttrs {
        (Attr(attr::Id, self.id),)
    }
}

/// Attributes for the description element (id).
/// Spread onto the description element using `<p {..description_props.into_attrs()}>`.
pub type UseFieldDescriptionAttrs = (Attr<attr::Id, String>,);

/// Props for the error message element.
#[derive(Debug, Clone)]
pub struct UseFieldErrorMessageProps {
    /// The id of the error message element.
    pub id: String,

    /// The role attribute.
    pub role: &'static str,

    /// The aria-live attribute.
    pub aria_live: AriaLive,
}

impl UseFieldErrorMessageProps {
    pub fn into_attrs(self) -> UseFieldErrorMessageAttrs {
        (
            Attr(attr::Id, self.id),
            Attr(attr::Role, self.role),
            Attr(attr::AriaLive, self.aria_live),
        )
    }
}

/// Attributes for the error message element (id, role, aria-live).
/// Spread onto the error message element using `<p {..error_message_props.into_attrs()}>`.
pub type UseFieldErrorMessageAttrs = (
    Attr<attr::Id, String>,
    Attr<attr::Role, &'static str>,
    Attr<attr::AriaLive, AriaLive>,
);

/// Provides the accessibility implementation for a form field with label,
/// description, and error message support.
///
/// This hook generates all the necessary IDs and ARIA attributes to
/// properly associate a field with its label, description, and error message.
///
/// # Example
///
/// ```ignore
/// let field = use_field(UseFieldInput {
///     label: Some("Email".to_string()),
///     description: Some("Enter your email address".to_string()),
///     validation_state: if is_valid { ValidationState::Valid } else { ValidationState::Invalid },
///     error_message: Some("Please enter a valid email".to_string()),
///     is_required: true,
///     ..Default::default()
/// });
///
/// view! {
///     <div>
///         <label {..field.label_props.into_attrs()}>
///             "Email"
///         </label>
///         <input {..field.field_props.into_attrs()} />
///         <p {..field.description_props.into_attrs()}>
///             "Enter your email address"
///         </p>
///         <Show when=move || !is_valid>
///             <p {..field.error_message_props.into_attrs()}>
///                 "Please enter a valid email"
///             </p>
///         </Show>
///     </div>
/// }
/// ```
pub fn use_field(input: UseFieldInput) -> UseFieldReturn {
    let UseFieldInput {
        id,
        label,
        description,
        error_message,
        validation_state,
        is_required,
        is_disabled,
        is_read_only,
    } = input;

    let base_id = Uuid::new_v4();

    let field_id = id.unwrap_or_else(|| format!("field-{base_id}"));
    let label_id = format!("label-{base_id}");
    let description_id = format!("description-{base_id}");
    let error_message_id = format!("error-{base_id}");

    // Build aria-describedby
    let mut describedby_parts = Vec::new();
    if description.is_some() {
        describedby_parts.push(description_id.clone());
    }
    if validation_state == ValidationState::Invalid && error_message.is_some() {
        describedby_parts.push(error_message_id.clone());
    }

    let aria_describedby = if describedby_parts.is_empty() {
        None
    } else {
        Some(describedby_parts.join(" "))
    };

    // Build aria-labelledby (only if label exists)
    let aria_labelledby = if label.is_some() {
        Some(label_id.clone())
    } else {
        None
    };

    UseFieldReturn {
        label_props: UseFieldLabelProps {
            id: label_id,
            html_for: field_id.clone(),
        },
        field_props: UseFieldProps {
            id: field_id,
            aria_labelledby,
            aria_describedby,
            aria_invalid: (validation_state == ValidationState::Invalid)
                .then_some(AriaInvalid::True),
            aria_required: is_required.then_some(AriaRequired::True),
            aria_disabled: is_disabled.then_some(AriaDisabled::True),
            aria_readonly: is_read_only.then_some(AriaReadonly::True),
        },
        description_props: UseFieldDescriptionProps { id: description_id },
        error_message_props: UseFieldErrorMessageProps {
            id: error_message_id,
            role: "alert",
            aria_live: AriaLive::Polite,
        },
    }
}
