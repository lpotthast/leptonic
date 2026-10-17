// Upstream: react-aria/src/utils/useLabels.ts @ 99e6102368
//! [`labels`]: how an element named by both an `aria-label` and `aria-labelledby` is labelled.
//
// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - A plain function of the element's id (callers have one already, from `use_id`), instead of
//   a hook generating it. Reactive callers call it inside their signal.
//
// ## OMITTED FEATURES
// - `defaultLabel` (no caller needs one).
//
// =============================================================================

use crate::{Id, utils::id::IdListBuilder};

/// The labelling attributes of an element, from [`labels`].
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Labels {
    /// `aria-label`.
    pub aria_label: Option<String>,
    /// `aria-labelledby`.
    pub aria_labelledby: Option<Id>,
}

/// The labelling of the element `id`, named by `aria_label` and/or the elements `aria_labelledby`
/// lists. With both (an empty `aria_label` counts as none), the element's own id joins the
/// labelling ids (first), so that the label is part of its name; the ids are normalized (each
/// once, single spaces). `aria_label` is returned as given. Upstream: `useLabels`.
pub fn labels(id: &Id, aria_label: Option<String>, aria_labelledby: Option<&Id>) -> Labels {
    let has_label = aria_label.as_deref().is_some_and(|label| !label.is_empty());
    Labels {
        aria_labelledby: labelledby_ids(id, has_label, aria_labelledby),
        aria_label,
    }
}

/// [`labels`]' `aria-labelledby` for [`Id`]s: the labelling of the element `id`, named by an
/// `aria-label` (`has_aria_label`: a non-empty one) and/or `aria_labelledby`. Shares the given
/// list when it needs no change (no allocation).
///
/// Migration: takes `labels`' name once every reference is an `Id`.
pub(crate) fn labelledby_ids(
    id: &Id,
    has_aria_label: bool,
    aria_labelledby: Option<&Id>,
) -> Option<Id> {
    let labelledby = aria_labelledby.filter(|ids| !ids.trim().is_empty())?;
    if !has_aria_label && labelledby.is_joined() {
        return Some(labelledby.clone());
    }
    let mut ids = IdListBuilder::default();
    if has_aria_label {
        ids.push(id);
    }
    ids.push(labelledby);
    ids.finish()
}

#[cfg(test)]
mod tests {
    use assertr::prelude::*;

    use super::*;

    // Upstream has no test of `useLabels`; the cases follow its implementation.
    #[test]
    fn label_only() {
        assert_that!(labels(&Id::from("el"), Some("Name".to_owned()), None)).is_equal_to(Labels {
            aria_label: Some("Name".to_owned()),
            aria_labelledby: None,
        });
    }

    #[test]
    fn labelledby_only_is_normalized() {
        assert_that!(labels(&Id::from("el"), None, Some(&Id::from("  a   b a ")))).is_equal_to(
            Labels {
                aria_label: None,
                aria_labelledby: Some(Id::from("a b")),
            },
        );
    }

    #[test]
    fn both_add_the_own_id_first() {
        assert_that!(labels(
            &Id::from("el"),
            Some("Name".to_owned()),
            Some(&Id::from("a el b"))
        ))
        .is_equal_to(Labels {
            aria_label: Some("Name".to_owned()),
            aria_labelledby: Some(Id::from("el a b")),
        });
    }

    #[test]
    fn an_empty_label_counts_as_none() {
        assert_that!(labels(
            &Id::from("el"),
            Some(String::new()),
            Some(&Id::from("a"))
        ))
        .is_equal_to(Labels {
            aria_label: Some(String::new()),
            aria_labelledby: Some(Id::from("a")),
        });
    }

    #[test]
    fn ids_variant_matches() {
        let el = Id::from("el");
        assert_that!(labelledby_ids(&el, true, Some(&Id::from("a el b"))))
            .is_equal_to(Some(Id::from("el a b")));
        assert_that!(labelledby_ids(&el, false, Some(&Id::from("  a   b a "))))
            .is_equal_to(Some(Id::from("a b")));
        assert_that!(labelledby_ids(&el, true, Some(&Id::from("  ")))).is_none();
        assert_that!(labelledby_ids(&el, true, None)).is_none();
    }

    #[test]
    fn neither() {
        assert_that!(labels(&Id::from("el"), None, None)).is_equal_to(Labels::default());
        assert_that!(
            labels(
                &Id::from("el"),
                Some("Name".to_owned()),
                Some(&Id::from("  "))
            )
            .aria_labelledby
        )
        .is_none();
    }
}
