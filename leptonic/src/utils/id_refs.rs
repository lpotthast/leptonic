// No upstream: react-aria joins id lists by hand at every site
// (`[props['aria-describedby'], descriptionId].filter(Boolean).join(' ')`).
//! [`IdRefs`]: the one way to build an ARIA id reference list.

use std::fmt;

use leptos::prelude::*;

/// An ARIA id reference list (`aria-labelledby`, `aria-describedby`, `aria-controls`,
/// `aria-owns`, ...): element ids in order, each once.
///
/// Every place that combines several contributions to such an attribute (a field's description
/// and error message, a long-press description, a tooltip's description, ...) builds it through
/// this type. A contribution is one id or a space-separated list of ids.
///
/// ```
/// use leptonic::IdRefs;
///
/// let refs: IdRefs = [Some("description"), None, Some("error tooltip"), Some("description")]
///     .into_iter()
///     .flatten()
///     .collect();
/// assert_eq!(refs.into_value().as_deref(), Some("description error tooltip"));
/// assert_eq!(IdRefs::default().into_value(), None);
/// ```
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct IdRefs(Vec<String>);

impl IdRefs {
    /// Adds the ids of `list` (one id or a space-separated list) that aren't in the list yet.
    pub fn push(&mut self, list: &str) {
        for id in list.split_whitespace() {
            if !self.0.iter().any(|known| known == id) {
                self.0.push(id.to_owned());
            }
        }
    }

    /// The ids, in order.
    pub fn ids(&self) -> impl Iterator<Item = &str> {
        self.0.iter().map(String::as_str)
    }

    /// Whether the list has no ids.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// The attribute value: the ids, space-separated; `None` (no attribute) without ids.
    #[must_use]
    pub fn into_value(self) -> Option<String> {
        (!self.is_empty()).then(|| self.0.join(" "))
    }

    /// The attribute value joining reactive contributions, in order, each an optional id or id
    /// list (e.g. a slot's id, present while its element is rendered). Recomputed when a
    /// contribution changes.
    pub fn derive(
        contributions: impl IntoIterator<Item = Signal<Option<String>>>,
    ) -> Signal<Option<String>> {
        let contributions: Vec<_> = contributions.into_iter().collect();
        Memo::new(move |_| {
            let mut refs = IdRefs::default();
            for contribution in &contributions {
                contribution.with(|list| {
                    if let Some(list) = list {
                        refs.push(list);
                    }
                });
            }
            refs.into_value()
        })
        .into()
    }
}

impl<S: AsRef<str>> Extend<S> for IdRefs {
    fn extend<I: IntoIterator<Item = S>>(&mut self, lists: I) {
        for list in lists {
            self.push(list.as_ref());
        }
    }
}

impl<S: AsRef<str>> FromIterator<S> for IdRefs {
    fn from_iter<I: IntoIterator<Item = S>>(lists: I) -> Self {
        let mut refs = Self::default();
        refs.extend(lists);
        refs
    }
}

impl fmt::Display for IdRefs {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0.join(" "))
    }
}

#[cfg(test)]
mod tests {
    use assertr::prelude::*;

    use super::*;
    use crate::testing::with_owner;

    #[test]
    fn splits_lists_and_skips_repeated_ids() {
        let refs: IdRefs = ["a  b", " c ", "b a d"].into_iter().collect();
        assert_that!(refs.ids().collect::<Vec<_>>()).contains_exactly(["a", "b", "c", "d"]);
        assert_that!(refs.to_string()).is_equal_to("a b c d".to_owned());
    }

    #[test]
    fn without_ids_has_no_value() {
        let refs: IdRefs = ["", "   "].into_iter().collect();
        assert_that!(refs.is_empty()).is_true();
        assert_that!(refs.into_value()).is_none();
    }

    #[test]
    fn derive_joins_present_contributions_and_follows_them() {
        with_owner(|| {
            let a = RwSignal::new(None::<String>);
            let b = Signal::stored(Some("b".to_owned()));
            let c = Signal::stored(Some("c b".to_owned()));
            let joined = IdRefs::derive([a.into(), b, c]);
            assert_that!(joined.get_untracked()).is_equal_to(Some("b c".to_owned()));

            a.set(Some("a".to_owned()));
            assert_that!(joined.get_untracked()).is_equal_to(Some("a b c".to_owned()));
        });
    }

    #[test]
    fn derive_without_contributions_is_none() {
        with_owner(|| {
            let joined = IdRefs::derive([Signal::stored(None), Signal::stored(None)]);
            assert_that!(joined.get_untracked()).is_none();
        });
    }
}
