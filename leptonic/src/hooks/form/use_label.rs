use leptos::attr;
use leptos::attr::Attr;
use uuid::Uuid;

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/@react-aria/label/src/useLabel.ts

/// Input parameters for the `use_label` hook.
#[derive(Debug, Clone, Default)]
pub struct UseLabelInput {
    /// A custom id for the label element.
    pub id: Option<String>,

    /// The HTML element type for the label.
    pub label_element_type: Option<LabelElementType>,
}

/// The type of element to use for the label.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LabelElementType {
    /// Use a <label> element (default).
    #[default]
    Label,
    /// Use a <span> element.
    Span,
}

/// The return value of the `use_label` hook.
pub struct UseLabelReturn {
    /// Props for the label element. Spread onto the label using `<label {..label_props}>`.
    pub label_props: UseLabelProps,

    /// Props for the field element. Spread onto the field using `<input {..field_props}>`.
    pub field_props: UseLabelFieldProps,
}

#[derive(Debug, Clone)]
pub struct UseLabelProps {
    id: String,
    for_elem: Option<String>,
}

impl UseLabelProps {
    pub fn into_attrs(self) -> UseLabelAttrs {
        (Attr(attr::Id, self.id), Attr(attr::For, self.for_elem))
    }
}

pub struct UseLabelFieldProps {
    id: String,
    aria_labelledby: Option<String>,
}

impl UseLabelFieldProps {
    pub fn into_attrs(self) -> UseLabelFieldAttrs {
        (
            Attr(attr::Id, self.id),
            Attr(attr::AriaLabelledby, self.aria_labelledby),
        )
    }
}

/// Attributes for the label element (id, for).
/// Spread onto the label element using `<label {..label_props}>`.
pub type UseLabelAttrs = (Attr<attr::Id, String>, Attr<attr::For, Option<String>>);

/// Attributes for the field element that the label is associated with (id, aria-labelledby).
/// Spread onto the field element using `<input {..field_props}>`.
pub type UseLabelFieldAttrs = (
    Attr<attr::Id, String>,
    Attr<attr::AriaLabelledby, Option<String>>,
);

/// Provides the accessibility implementation for labels associated with form fields.
///
/// This hook generates matching IDs for a label and its associated field,
/// ensuring proper accessibility relationships.
///
/// # Example
///
/// ```ignore
/// let UseLabelReturn { label_props, field_props } = use_label(UseLabelInput::default());
///
/// view! {
///     <label {..label_props}>"Email"</label>
///     <input type="email" {..field_props} />
/// }
/// ```
pub fn use_label(input: UseLabelInput) -> UseLabelReturn {
    let UseLabelInput {
        id,
        label_element_type,
    } = input;

    let label_id = id.unwrap_or_else(|| format!("label-{}", Uuid::new_v4()));
    let field_id = format!("field-{}", Uuid::new_v4());

    let element_type = label_element_type.unwrap_or_default();

    // For <label> elements, use the "for" attribute
    // For other elements, use aria-labelledby on the field
    let (html_for, aria_labelledby) = match element_type {
        LabelElementType::Label => (Some(field_id.clone()), None),
        LabelElementType::Span => (None, Some(label_id.clone())),
    };

    UseLabelReturn {
        label_props: UseLabelProps {
            id: label_id,
            for_elem: html_for,
        },
        field_props: UseLabelFieldProps {
            id: field_id,
            aria_labelledby,
        },
    }
}

/// A more complete label hook that handles both label and description.
#[derive(Debug, Clone, Default)]
pub struct UseLabelWithDescriptionInput {
    /// A custom id for the label element.
    pub label_id: Option<String>,

    /// A custom id for the description element.
    pub description_id: Option<String>,

    /// A custom id for the error message element.
    pub error_message_id: Option<String>,

    /// Whether there is an error.
    pub has_error: bool,
}

/// The return value of the `use_label_with_description` hook.
#[derive(Debug, Clone)]
pub struct UseLabelWithDescriptionReturn {
    /// The id for the label element.
    pub label_id: String,

    /// The id for the field element.
    pub field_id: String,

    /// The id for the description element.
    pub description_id: String,

    /// The id for the error message element.
    pub error_message_id: String,

    /// The aria-describedby value for the field.
    pub aria_describedby: Option<String>,
}

/// Provides IDs for label, description, and error message elements.
///
/// This hook generates matching IDs for all the elements that can describe
/// a form field, and computes the appropriate aria-describedby value.
///
/// # Example
///
/// ```ignore
/// let ids = use_label_with_description(UseLabelWithDescriptionInput {
///     has_error: !is_valid,
///     ..Default::default()
/// });
///
/// view! {
///     <label id=ids.label_id for=ids.field_id>"Email"</label>
///     <input
///         id=ids.field_id
///         aria-describedby=ids.aria_describedby
///     />
///     <p id=ids.description_id>"Enter your email address"</p>
///     <Show when=move || !is_valid>
///         <p id=ids.error_message_id role="alert">"Invalid email"</p>
///     </Show>
/// }
/// ```
pub fn use_label_with_description(
    input: UseLabelWithDescriptionInput,
) -> UseLabelWithDescriptionReturn {
    let UseLabelWithDescriptionInput {
        label_id,
        description_id,
        error_message_id,
        has_error,
    } = input;

    let base_id = Uuid::new_v4();

    let label_id = label_id.unwrap_or_else(|| format!("label-{base_id}"));
    let field_id = format!("field-{base_id}");
    let description_id = description_id.unwrap_or_else(|| format!("description-{base_id}"));
    let error_message_id = error_message_id.unwrap_or_else(|| format!("error-{base_id}"));

    // Compute aria-describedby - include error message if there's an error
    let aria_describedby = if has_error {
        Some(format!("{description_id} {error_message_id}"))
    } else {
        Some(description_id.clone())
    };

    UseLabelWithDescriptionReturn {
        label_id,
        field_id,
        description_id,
        error_message_id,
        aria_describedby,
    }
}
