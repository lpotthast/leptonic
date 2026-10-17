// Upstream: react-aria/src/label/useField.ts @ 99e6102368
// Upstream: react-aria/src/label/useLabel.ts @ 99e6102368
// Upstream: react-aria/test/label/useField.test.js @ 99e6102368
use leptos::prelude::*;

use super::use_label::{
    LabelElementType, UseLabelFieldProps, UseLabelInput, UseLabelProps, UseLabelReturn, use_label,
};
use crate::{
    Id, IdRefs, IntoAttrs, Part, SlotProps, use_slot,
    utils::{
        part::Remount,
        signal_attr::{FixedAttr, SignalAttr, fixed_attr, maybe_attr, signal_attr},
        spread::Spread,
    },
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - As [`use_label`]: `has_label` instead of the `label` content, an optional `label_id`.
// - Descriptions and error messages are detected when rendered (react-aria: `useSlotId`), so
//   there are no `description`/`errorMessage` inputs. Reason: the hook never sees the rendered
//   content; the slot registers itself when its element mounts.
//
// =============================================================================

/// Input of [`use_field`].
#[derive(Debug, Clone, Default)]
pub struct UseFieldInput {
    /// The field element's id. Generated when `None`.
    pub id: Option<Id>,
    /// The label element's id. Generated when `None`.
    pub label_id: Option<Id>,
    /// Whether a visible label is rendered (with `label_props`).
    pub has_label: Signal<bool>,
    pub label_element_type: LabelElementType,
    /// Labels the field when there is no visible label. Next to a visible label, it is added to the
    /// field's name.
    pub aria_label: MaybeProp<String>,
    /// Further elements labelling the field.
    pub aria_labelledby: Signal<Option<Id>>,
    /// Further elements describing the field.
    pub aria_describedby: Signal<Option<Id>>,
}

/// The return value of [`use_field`].
#[derive(Debug)]
pub struct UseFieldReturn {
    /// The label element.
    pub label: Part<UseLabelProps>,
    /// The field element (the input).
    pub field: Part<UseFieldProps>,
    /// For the description element.
    pub description: Part<SlotProps>,
    /// For the error message element. Render it only while the field is invalid.
    pub error_message: Part<SlotProps>,
    /// The description's id while it is rendered (for other elements it describes).
    pub description_id: Signal<Option<Id>>,
    /// The error message's id while it is rendered.
    pub error_message_id: Signal<Option<Id>>,
}

/// Props for the field element.
#[derive(Debug)]
pub struct UseFieldProps {
    pub id: Id,
    pub aria_label: MaybeProp<String>,
    pub aria_labelledby: Signal<Option<Id>>,
    pub aria_describedby: Signal<Option<Id>>,
}

impl Remount for UseFieldProps {
    fn remount(&self) -> Self {
        Self {
            id: self.id.clone(),
            aria_label: self.aria_label,
            aria_labelledby: self.aria_labelledby,
            aria_describedby: self.aria_describedby,
        }
    }
}

pub type UseFieldAttrs = Spread<(
    FixedAttr,
    SignalAttr<Option<String>>,
    SignalAttr<Option<Id>>,
    SignalAttr<Option<Id>>,
)>;

impl IntoAttrs for UseFieldProps {
    type Attrs = UseFieldAttrs;

    fn into_attrs(self) -> Self::Attrs {
        Spread((
            fixed_attr("id", self.id),
            maybe_attr("aria-label", self.aria_label),
            signal_attr("aria-labelledby", self.aria_labelledby),
            signal_attr("aria-describedby", self.aria_describedby),
        ))
    }
}

/// Connects a field with its label, description and error message: ids and the field's
/// `aria-labelledby` / `aria-describedby`.
///
/// ```ignore
/// let field = use_field(UseFieldInput { has_label: true.into(), ..UseFieldInput::default() });
/// view! {
///     <label {..field.label.attrs()}>"Email"</label>
///     <input type="email" {..field.field.attrs()} />
///     <span {..field.description.attrs()}>"We never share it."</span>
/// }
/// ```
pub fn use_field(input: UseFieldInput) -> UseFieldReturn {
    crate::metrics::subject!();
    let UseFieldInput {
        id,
        label_id,
        has_label,
        label_element_type,
        aria_label,
        aria_labelledby,
        aria_describedby,
    } = input;

    let UseLabelReturn { label, field } = use_label(UseLabelInput {
        id,
        label_id,
        has_label,
        label_element_type,
        aria_label,
        aria_labelledby,
    });
    let UseLabelFieldProps {
        id,
        aria_label,
        aria_labelledby,
    } = field.into_props();

    let description = use_slot("description");
    let error_message = use_slot("error-message");
    let (description_id, error_message_id) =
        (description.referenced_id, error_message.referenced_id);

    UseFieldReturn {
        label,
        field: Part::stored(UseFieldProps {
            id,
            aria_label,
            aria_labelledby,
            // The error message is a description too: `aria-errormessage` is unsupported by
            // VoiceOver and NVDA.
            aria_describedby: IdRefs::derive([description_id, error_message_id, aria_describedby]),
        }),
        description: description.part,
        error_message: error_message.part,
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
                id: Some(Id::from("f")),
                has_label: Signal::stored(true),
                ..UseFieldInput::default()
            });
            let field_field_props = field.field.into_props();
            let field_label_props = field.label.into_props();
            let label_id = field_label_props.id.clone();
            assert_that!(field_field_props.aria_labelledby.get_untracked())
                .is_equal_to(Some(label_id));
            assert_that!(field_label_props.html_for).is_equal_to(Some(Id::from("f")));
            // Nothing describes the field until a description is rendered.
            assert_that!(field_field_props.aria_describedby.get_untracked()).is_none();
        });
    }

    #[test]
    fn aria_label_next_to_other_labels_adds_the_field_itself() {
        with_owner(|| {
            let field = use_field(UseFieldInput {
                id: Some(Id::from("f")),
                aria_label: "Name".into(),
                aria_labelledby: Signal::stored(Some(Id::from("other"))),
                ..UseFieldInput::default()
            });
            let field_field_props = field.field.into_props();
            // The field first, as react-aria's `useLabels`.
            assert_that!(field_field_props.aria_labelledby.get_untracked())
                .is_equal_to(Some(Id::from("f other")));
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
            let field_label_props = field.label.into_props();
            assert_that!(field_label_props.html_for).is_none();
        });
    }
}
