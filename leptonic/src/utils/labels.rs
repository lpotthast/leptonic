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
// - An `aria_label` counts when given (`Some`), also when empty. React-aria: when non-empty.
//
// ## OMITTED FEATURES
// - `defaultLabel` (no caller needs one).
//
// =============================================================================

use crate::IdRefs;

/// The labelling attributes of an element, from [`labels`].
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Labels {
    /// `aria-label`.
    pub aria_label: Option<String>,
    /// `aria-labelledby`.
    pub aria_labelledby: Option<String>,
}

/// The labelling of the element `id`, named by `aria_label` and/or the elements `aria_labelledby`
/// lists. With both, the element's own id joins the labelling ids (first), so that the label is
/// part of its name; the ids are normalized (each once, single spaces). Upstream: `useLabels`.
pub fn labels(id: &str, aria_label: Option<String>, aria_labelledby: Option<&str>) -> Labels {
    let mut refs = IdRefs::default();
    if aria_label.is_some() && aria_labelledby.is_some_and(|ids| !ids.trim().is_empty()) {
        refs.push(id);
    }
    refs.extend(aria_labelledby);
    Labels {
        aria_label,
        aria_labelledby: refs.into_value(),
    }
}

#[cfg(test)]
mod tests {
    use assertr::prelude::*;

    use super::*;

    // Upstream has no test of `useLabels`; the cases follow its implementation.
    #[test]
    fn label_only() {
        assert_that!(labels("el", Some("Name".to_owned()), None)).is_equal_to(Labels {
            aria_label: Some("Name".to_owned()),
            aria_labelledby: None,
        });
    }

    #[test]
    fn labelledby_only_is_normalized() {
        assert_that!(labels("el", None, Some("  a   b a "))).is_equal_to(Labels {
            aria_label: None,
            aria_labelledby: Some("a b".to_owned()),
        });
    }

    #[test]
    fn both_add_the_own_id_first() {
        assert_that!(labels("el", Some("Name".to_owned()), Some("a el b"))).is_equal_to(Labels {
            aria_label: Some("Name".to_owned()),
            aria_labelledby: Some("el a b".to_owned()),
        });
    }

    #[test]
    fn neither() {
        assert_that!(labels("el", None, None)).is_equal_to(Labels::default());
        assert_that!(labels("el", Some("Name".to_owned()), Some("  ")).aria_labelledby).is_none();
    }
}
