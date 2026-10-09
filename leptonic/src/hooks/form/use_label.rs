// Upstream: react-aria/src/label/useLabel.ts @ 99e6102368
// Upstream: react-aria/test/label/useLabel.test.js @ 99e6102368
use leptos::{
    attr::{self, Attr},
    prelude::*,
};

use crate::{IdRefs, IntoAttrs, labels, utils::id::use_id};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - `has_label` says whether a visible label is rendered (react-aria: the `label` content, which
//   the hook only checks for presence). Reason: the label is rendered by the caller's view, the
//   hook never sees its content.
// - `label_id` sets the label element's id (react-aria always generates it). Reason: lets a
//   caller label further elements with an id it already knows.
// - `aria_labelledby` is read once (a list of ids, plain `Option<String>` like all ids, C2);
//   `aria_label` and `has_label` are reactive, and so is whether `aria-labelledby` lists the
//   field itself.
//
// =============================================================================

/// The element a label is rendered as.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LabelElementType {
    /// A `<label>`, associated with the field through `for` (the default).
    #[default]
    Label,
    /// Another element (e.g. a `<span>`), for fields a `<label>` can't label natively (groups,
    /// `contenteditable` elements). The field references it only with `aria-labelledby`.
    Span,
}

/// Input of [`use_label`].
#[derive(Debug, Clone, Default)]
pub struct UseLabelInput {
    /// The field element's id. Generated when `None`.
    pub id: Option<String>,
    /// The label element's id. Generated when `None`.
    pub label_id: Option<String>,
    /// Whether a visible label is rendered (with `label_props`). A signal, so atoms can follow
    /// whether their `Label` part is actually rendered.
    pub has_label: Signal<bool>,
    pub label_element_type: LabelElementType,
    /// Labels the field when there is no visible label. Next to a visible label, it is added to the
    /// field's name.
    pub aria_label: MaybeProp<String>,
    /// Further elements labelling the field.
    pub aria_labelledby: Option<String>,
}

/// Return value of [`use_label`].
#[derive(Debug)]
pub struct UseLabelReturn {
    /// For the label element.
    pub label_props: UseLabelProps,
    /// For the field element.
    pub field_props: UseLabelFieldProps,
}

/// Props for a label element.
#[derive(Debug, Clone)]
pub struct UseLabelProps {
    pub id: String,
    /// The field's id, for `<label>` elements.
    pub html_for: Option<String>,
}

pub type UseLabelAttrs = (Attr<attr::Id, String>, Attr<attr::For, Option<String>>);

impl IntoAttrs for UseLabelProps {
    type Attrs = UseLabelAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (Attr(attr::Id, self.id), Attr(attr::For, self.html_for))
    }
}

/// Props for the labelled field element.
#[derive(Debug, Clone)]
pub struct UseLabelFieldProps {
    pub id: String,
    pub aria_label: MaybeProp<String>,
    pub aria_labelledby: Signal<Option<String>>,
}

pub type UseLabelFieldAttrs = (
    Attr<attr::Id, String>,
    Attr<attr::AriaLabel, MaybeProp<String>>,
    Attr<attr::AriaLabelledby, Signal<Option<String>>>,
);

impl IntoAttrs for UseLabelFieldProps {
    type Attrs = UseLabelFieldAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            Attr(attr::Id, self.id),
            Attr(attr::AriaLabel, self.aria_label),
            Attr(attr::AriaLabelledby, self.aria_labelledby),
        )
    }
}

/// Connects a field with its visible label: ids, the label's `for` and the field's
/// `aria-labelledby`. [`use_field`](super::use_field::use_field) adds a description and an error
/// message.
///
/// ```ignore
/// let label = use_label(UseLabelInput { has_label: true.into(), ..UseLabelInput::default() });
/// view! {
///     <label {..label.label_props.into_attrs()}>"Email"</label>
///     <input type="email" {..label.field_props.into_attrs()} />
/// }
/// ```
pub fn use_label(input: UseLabelInput) -> UseLabelReturn {
    let UseLabelInput {
        id,
        label_id,
        has_label,
        label_element_type,
        aria_label,
        aria_labelledby,
    } = input;

    let id = id.unwrap_or_else(|| use_id("field"));
    let label_id = label_id.unwrap_or_else(|| use_id("label"));

    // Checked once mounted: an atom knows only then whether its `Label` rendered.
    #[cfg(debug_assertions)]
    {
        let has_aria_labelledby = aria_labelledby.is_some();
        Effect::new(move || {
            if !has_label.get() && !has_aria_labelledby && aria_label.with(Option::is_none) {
                crate::utils::dev_warn!(
                    "If you do not provide a visible label, you must specify an aria-label or \
                     aria-labelledby attribute for accessibility"
                );
            }
        });
    }
    let labelled_by = {
        let (id, label_id) = (id.clone(), label_id.clone());
        Signal::derive(move || {
            let mut labelled_by = IdRefs::default();
            if has_label.get() {
                labelled_by.push(&label_id);
            }
            labelled_by.extend(aria_labelledby.as_deref());
            // With an `aria-label` next to other labels, the field labels itself too (first), so
            // that both make up its name.
            labels(&id, aria_label.get(), labelled_by.into_value().as_deref()).aria_labelledby
        })
    };

    UseLabelReturn {
        label_props: UseLabelProps {
            id: label_id,
            // Only a rendered `<label>` uses it.
            html_for: (label_element_type == LabelElementType::Label).then(|| id.clone()),
        },
        field_props: UseLabelFieldProps {
            id,
            aria_labelledby: labelled_by,
            aria_label,
        },
    }
}

#[cfg(test)]
mod tests {
    use assertr::prelude::*;

    use super::*;
    use crate::testing::with_owner;

    #[test]
    fn a_visible_label_labels_the_field() {
        with_owner(|| {
            let label = use_label(UseLabelInput {
                id: Some("f".to_owned()),
                has_label: Signal::stored(true),
                ..UseLabelInput::default()
            });
            let label_id = label.label_props.id.clone();
            assert_that!(label.field_props.aria_labelledby.get_untracked())
                .is_equal_to(Some(label_id));
            assert_that!(label.label_props.html_for).is_equal_to(Some("f".to_owned()));
        });
    }

    #[test]
    fn labelled_by_lists_the_visible_label_first() {
        with_owner(|| {
            let label = use_label(UseLabelInput {
                label_id: Some("l".to_owned()),
                has_label: Signal::stored(true),
                aria_labelledby: Some("other".to_owned()),
                ..UseLabelInput::default()
            });
            assert_that!(label.field_props.aria_labelledby.get_untracked())
                .is_equal_to(Some("l other".to_owned()));
        });
    }

    #[test]
    fn aria_label_next_to_other_labels_adds_the_field_itself() {
        with_owner(|| {
            let label = use_label(UseLabelInput {
                id: Some("f".to_owned()),
                aria_label: "Name".into(),
                aria_labelledby: Some("other".to_owned()),
                ..UseLabelInput::default()
            });
            assert_that!(label.field_props.aria_labelledby.get_untracked())
                .is_equal_to(Some("f other".to_owned()));
        });
    }

    /// Upstream: "should combine aria-labelledby if visible label and aria-label is also
    /// provided".
    #[test]
    fn aria_label_visible_label_and_labelled_by_combine_field_first() {
        with_owner(|| {
            let label = use_label(UseLabelInput {
                id: Some("f".to_owned()),
                label_id: Some("l".to_owned()),
                has_label: Signal::stored(true),
                aria_label: "aria".into(),
                aria_labelledby: Some("foo f".to_owned()),
                ..UseLabelInput::default()
            });
            assert_that!(label.field_props.aria_labelledby.get_untracked())
                .is_equal_to(Some("f l foo".to_owned()));
        });
    }

    #[test]
    fn aria_label_alone_needs_no_labelled_by() {
        with_owner(|| {
            let label = use_label(UseLabelInput {
                aria_label: "Name".into(),
                ..UseLabelInput::default()
            });
            assert_that!(label.field_props.aria_labelledby.get_untracked()).is_none();
        });
    }

    #[test]
    fn span_labels_have_no_for_attribute() {
        with_owner(|| {
            let label = use_label(UseLabelInput {
                has_label: Signal::stored(true),
                label_element_type: LabelElementType::Span,
                ..UseLabelInput::default()
            });
            assert_that!(label.label_props.html_for).is_none();
            assert_that!(label.field_props.aria_labelledby.get_untracked())
                .is_equal_to(Some(label.label_props.id.clone()));
        });
    }
}
