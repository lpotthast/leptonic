use leptos::attr;
use leptos::attr::Attr;
use uuid::Uuid;

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/@react-aria/label/src/useLabel.ts

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// No intentional deviations from the react-aria implementation.
//
// =============================================================================

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
#[derive(Debug)]
pub struct UseLabelReturn {
    /// Props for the label element. Spread onto the label using `<label {..label_props}>`.
    pub label_props: UseLabelProps,

    /// Props for the field element. Spread onto the field using `<input {..field_props}>`.
    pub field_props: UseLabelFieldProps,
}

#[derive(Debug)]
pub struct UseLabelProps {
    id: String,
    for_elem: Option<String>,
}

impl UseLabelProps {
    pub fn into_attrs(self) -> UseLabelAttrs {
        (Attr(attr::Id, self.id), Attr(attr::For, self.for_elem))
    }
}

#[derive(Debug)]
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
