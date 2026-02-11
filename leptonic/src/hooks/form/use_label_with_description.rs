use uuid::Uuid;

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/@react-aria/label/src/useLabel.ts

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// No intentional deviations from the react-aria implementation.
//
// =============================================================================

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
