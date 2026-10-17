// Upstream: react-aria/src/label/useLabel.ts @ 99e6102368
// Upstream: react-aria/test/label/useLabel.test.js @ 99e6102368
use leptos::prelude::*;

use crate::{
    Id, IntoAttrs,
    utils::{
        id::{IdListBuilder, use_id},
        labels::labelledby_ids,
        part::{Part, Remount},
        probe::probe_sources,
        signal_attr::{FixedAttr, SignalAttr, fixed_attr, maybe_attr, signal_attr},
        spread::Spread,
    },
};

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
    pub id: Option<Id>,
    /// The label element's id. Generated when `None`.
    pub label_id: Option<Id>,
    /// Whether a visible label is rendered (with `label_props`). A signal, so atoms can follow
    /// whether their `Label` part is actually rendered.
    pub has_label: Signal<bool>,
    pub label_element_type: LabelElementType,
    /// Labels the field when there is no visible label. Next to a visible label, it is added to the
    /// field's name.
    pub aria_label: MaybeProp<String>,
    /// Further elements labelling the field.
    pub aria_labelledby: Signal<Option<Id>>,
}

/// The return value of [`use_label`].
#[derive(Debug)]
pub struct UseLabelReturn {
    /// The label element.
    pub label: Part<UseLabelProps>,
    /// The labelled field element.
    pub field: Part<UseLabelFieldProps>,
}

/// Props for a label element.
#[derive(Debug)]
pub struct UseLabelProps {
    pub id: Id,
    /// The field's id, for `<label>` elements.
    pub html_for: Option<Id>,
}

impl Remount for UseLabelProps {
    fn remount(&self) -> Self {
        Self {
            id: self.id.clone(),
            html_for: self.html_for.clone(),
        }
    }
}

pub type UseLabelAttrs = Spread<(FixedAttr, FixedAttr)>;

impl IntoAttrs for UseLabelProps {
    type Attrs = UseLabelAttrs;

    fn into_attrs(self) -> Self::Attrs {
        Spread((fixed_attr("id", self.id), fixed_attr("for", self.html_for)))
    }
}

/// Props for the labelled field element.
#[derive(Debug)]
pub struct UseLabelFieldProps {
    pub id: Id,
    pub aria_label: MaybeProp<String>,
    pub aria_labelledby: Signal<Option<Id>>,
}

impl Remount for UseLabelFieldProps {
    fn remount(&self) -> Self {
        Self {
            id: self.id.clone(),
            aria_label: self.aria_label,
            aria_labelledby: self.aria_labelledby,
        }
    }
}

pub type UseLabelFieldAttrs = Spread<(
    FixedAttr,
    SignalAttr<Option<String>>,
    SignalAttr<Option<Id>>,
)>;

impl IntoAttrs for UseLabelFieldProps {
    type Attrs = UseLabelFieldAttrs;

    fn into_attrs(self) -> Self::Attrs {
        Spread((
            fixed_attr("id", self.id),
            maybe_attr("aria-label", self.aria_label),
            signal_attr("aria-labelledby", self.aria_labelledby),
        ))
    }
}

/// Connects a field with its visible label: ids, the label's `for` and the field's
/// `aria-labelledby`. [`use_field`](super::use_field::use_field) adds a description and an error
/// message.
///
/// ```ignore
/// let label = use_label(UseLabelInput { has_label: true.into(), ..UseLabelInput::default() });
/// view! {
///     <label {..label.label.attrs()}>"Email"</label>
///     <input type="email" {..label.field.attrs()} />
/// }
/// ```
pub fn use_label(input: UseLabelInput) -> UseLabelReturn {
    crate::metrics::subject!();
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
        Effect::new(move || {
            if !has_label.get()
                && aria_labelledby.with(Option::is_none)
                && aria_label.with(Option::is_none)
            {
                crate::utils::dev_warn!(
                    "If you do not provide a visible label, you must specify an aria-label or \
                     aria-labelledby attribute for accessibility"
                );
            }
        });
    }
    // A memo: it allocates, and fields pass it on to further readers.
    let labelled_by = {
        let (id, label_id) = (id.clone(), label_id.clone());
        let compute = move || {
            let has_aria_label =
                aria_label.with(|label| label.as_deref().is_some_and(|label| !label.is_empty()));
            aria_labelledby.with(|labelledby| {
                if has_label.get() {
                    let mut ids = IdListBuilder::default();
                    ids.push(&label_id);
                    if let Some(labelledby) = labelledby {
                        ids.push(labelledby);
                    }
                    labelledby_ids(&id, has_aria_label, ids.finish().as_ref())
                } else {
                    labelledby_ids(&id, has_aria_label, labelledby.as_ref())
                }
            })
        };
        // Fixed inputs (no label that may come and go, fixed labels): computed once.
        match probe_sources(compute.clone()) {
            (value, false) => Signal::stored(value),
            _ => Memo::new(move |_| compute()).into(),
        }
    };

    UseLabelReturn {
        label: Part::stored(UseLabelProps {
            id: label_id,
            // Only a rendered `<label>` uses it.
            html_for: (label_element_type == LabelElementType::Label).then(|| id.clone()),
        }),
        field: Part::stored(UseLabelFieldProps {
            id,
            aria_labelledby: labelled_by,
            aria_label,
        }),
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
                id: Some(Id::from("f")),
                has_label: Signal::stored(true),
                ..UseLabelInput::default()
            });
            let label_field_props = label.field.into_props();
            let label_label_props = label.label.into_props();
            let label_id = label_label_props.id.clone();
            assert_that!(label_field_props.aria_labelledby.get_untracked())
                .is_equal_to(Some(label_id));
            assert_that!(label_label_props.html_for).is_equal_to(Some(Id::from("f")));
        });
    }

    #[test]
    fn labelled_by_lists_the_visible_label_first() {
        with_owner(|| {
            let label = use_label(UseLabelInput {
                label_id: Some(Id::from("l")),
                has_label: Signal::stored(true),
                aria_labelledby: Signal::stored(Some(Id::from("other"))),
                ..UseLabelInput::default()
            });
            let label_field_props = label.field.into_props();
            assert_that!(label_field_props.aria_labelledby.get_untracked())
                .is_equal_to(Some(Id::from("l other")));
        });
    }

    #[test]
    fn aria_label_next_to_other_labels_adds_the_field_itself() {
        with_owner(|| {
            let label = use_label(UseLabelInput {
                id: Some(Id::from("f")),
                aria_label: "Name".into(),
                aria_labelledby: Signal::stored(Some(Id::from("other"))),
                ..UseLabelInput::default()
            });
            let label_field_props = label.field.into_props();
            assert_that!(label_field_props.aria_labelledby.get_untracked())
                .is_equal_to(Some(Id::from("f other")));
        });
    }

    /// Upstream: "should combine aria-labelledby if visible label and aria-label is also
    /// provided".
    #[test]
    fn aria_label_visible_label_and_labelled_by_combine_field_first() {
        with_owner(|| {
            let label = use_label(UseLabelInput {
                id: Some(Id::from("f")),
                label_id: Some(Id::from("l")),
                has_label: Signal::stored(true),
                aria_label: "aria".into(),
                aria_labelledby: Signal::stored(Some(Id::from("foo f"))),
                ..UseLabelInput::default()
            });
            let label_field_props = label.field.into_props();
            assert_that!(label_field_props.aria_labelledby.get_untracked())
                .is_equal_to(Some(Id::from("f l foo")));
        });
    }

    #[test]
    fn aria_label_alone_needs_no_labelled_by() {
        with_owner(|| {
            let label = use_label(UseLabelInput {
                aria_label: "Name".into(),
                ..UseLabelInput::default()
            });
            let label_field_props = label.field.into_props();
            assert_that!(label_field_props.aria_labelledby.get_untracked()).is_none();
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
            let label_field_props = label.field.into_props();
            let label_label_props = label.label.into_props();
            assert_that!(label_label_props.html_for).is_none();
            assert_that!(label_field_props.aria_labelledby.get_untracked())
                .is_equal_to(Some(label_label_props.id.clone()));
        });
    }
}
