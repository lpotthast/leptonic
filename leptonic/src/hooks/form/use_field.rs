// Upstream: react-aria/src/label/useField.ts @ 99e6102368
// Upstream: react-aria/src/label/useLabel.ts @ 99e6102368
// Upstream: react-aria/test/label/useField.test.js @ 99e6102368
use leptos::{
    attr::{self, Attr},
    prelude::*,
};

use super::use_label::{
    LabelElementType, UseLabelFieldProps, UseLabelInput, UseLabelProps, UseLabelReturn, use_label,
};
use crate::{IdRefs, IntoAttrs, SlotProps, use_slot};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - As [`use_label`]: `has_label` instead of the `label` content, an optional `label_id`, the
//   presence of `aria_label` read once.
// - Descriptions and error messages are detected when rendered (react-aria: `useSlotId`), so
//   there are no `description`/`errorMessage` inputs. Reason: the hook never sees the rendered
//   content; the slot registers itself when its element mounts.
//
// =============================================================================

/// Input of [`use_field`].
#[derive(Debug, Clone, Default)]
pub struct UseFieldInput {
    /// The field element's id. Generated when `None`.
    pub id: Option<String>,
    /// The label element's id. Generated when `None`.
    pub label_id: Option<String>,
    /// Whether a visible label is rendered (with `label_props`).
    pub has_label: Signal<bool>,
    pub label_element_type: LabelElementType,
    /// Labels the field when there is no visible label. Next to a visible label, it is added to the
    /// field's name.
    pub aria_label: MaybeProp<String>,
    /// Further elements labelling the field.
    pub aria_labelledby: Option<String>,
    /// Further elements describing the field.
    pub aria_describedby: Option<String>,
}

/// Return value of [`use_field`].
#[derive(Debug)]
pub struct UseFieldReturn {
    pub label_props: UseLabelProps,
    pub field_props: UseFieldProps,
    /// For the description element.
    pub description_props: SlotProps,
    /// For the error message element. Render it only while the field is invalid.
    pub error_message_props: SlotProps,
    /// The description's id while it is rendered (for other elements it describes).
    pub description_id: Signal<Option<String>>,
    /// The error message's id while it is rendered.
    pub error_message_id: Signal<Option<String>>,
}

/// Props for the field element.
#[derive(Debug, Clone)]
pub struct UseFieldProps {
    pub id: String,
    pub aria_label: MaybeProp<String>,
    pub aria_labelledby: Signal<Option<String>>,
    pub aria_describedby: Signal<Option<String>>,
}

pub type UseFieldAttrs = (
    Attr<attr::Id, String>,
    Attr<attr::AriaLabel, MaybeProp<String>>,
    Attr<attr::AriaLabelledby, Signal<Option<String>>>,
    Attr<attr::AriaDescribedby, Signal<Option<String>>>,
);

impl IntoAttrs for UseFieldProps {
    type Attrs = UseFieldAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            Attr(attr::Id, self.id),
            Attr(attr::AriaLabel, self.aria_label),
            Attr(attr::AriaLabelledby, self.aria_labelledby),
            Attr(attr::AriaDescribedby, self.aria_describedby),
        )
    }
}

/// Connects a field with its label, description and error message: ids and the field's
/// `aria-labelledby` / `aria-describedby`.
///
/// ```ignore
/// let field = use_field(UseFieldInput { has_label: true.into(), ..UseFieldInput::default() });
/// view! {
///     <label {..field.label_props.into_attrs()}>"Email"</label>
///     <input type="email" {..field.field_props.into_attrs()} />
///     <span {..field.description_props.into_attrs()}>"We never share it."</span>
/// }
/// ```
pub fn use_field(input: UseFieldInput) -> UseFieldReturn {
    let UseFieldInput {
        id,
        label_id,
        has_label,
        label_element_type,
        aria_label,
        aria_labelledby,
        aria_describedby,
    } = input;

    let UseLabelReturn {
        label_props,
        field_props:
            UseLabelFieldProps {
                id,
                aria_label,
                aria_labelledby,
            },
    } = use_label(UseLabelInput {
        id,
        label_id,
        has_label,
        label_element_type,
        aria_label,
        aria_labelledby,
    });

    let description = use_slot("description");
    let error_message = use_slot("error-message");
    let (description_id, error_message_id) =
        (description.referenced_id, error_message.referenced_id);

    UseFieldReturn {
        label_props,
        field_props: UseFieldProps {
            id,
            aria_label,
            aria_labelledby,
            // The error message is a description too: `aria-errormessage` is unsupported by
            // VoiceOver and NVDA.
            aria_describedby: IdRefs::derive([
                description_id,
                error_message_id,
                Signal::stored(aria_describedby),
            ]),
        },
        description_props: description.props,
        error_message_props: error_message.props,
        description_id,
        error_message_id,
    }
}

#[cfg(test)]
mod tests {
    use assertr::prelude::*;

    use super::*;
    use crate::testing::with_owner;

    // useField.test.js: "should return label props", "should not return an id for description
    // and error message if they are not passed in".
    #[test]
    fn a_visible_label_labels_the_field() {
        with_owner(|| {
            let field = use_field(UseFieldInput {
                id: Some("f".to_owned()),
                has_label: Signal::stored(true),
                ..UseFieldInput::default()
            });
            let label_id = field.label_props.id.clone();
            assert_that!(field.field_props.aria_labelledby.get_untracked())
                .is_equal_to(Some(label_id));
            assert_that!(field.label_props.html_for).is_equal_to(Some("f".to_owned()));
            // Nothing describes the field until a description is rendered.
            assert_that!(field.field_props.aria_describedby.get_untracked()).is_none();
        });
    }

    #[test]
    fn aria_label_next_to_other_labels_adds_the_field_itself() {
        with_owner(|| {
            let field = use_field(UseFieldInput {
                id: Some("f".to_owned()),
                aria_label: "Name".into(),
                aria_labelledby: Some("other".to_owned()),
                ..UseFieldInput::default()
            });
            // The field first, as react-aria's `useLabels`.
            assert_that!(field.field_props.aria_labelledby.get_untracked())
                .is_equal_to(Some("f other".to_owned()));
        });
    }

    #[test]
    fn span_labels_have_no_for_attribute() {
        with_owner(|| {
            let field = use_field(UseFieldInput {
                has_label: Signal::stored(true),
                label_element_type: LabelElementType::Span,
                ..UseFieldInput::default()
            });
            assert_that!(field.label_props.html_for).is_none();
        });
    }
}
