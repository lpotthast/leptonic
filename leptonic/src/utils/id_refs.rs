// No upstream: react-aria joins id lists by hand at every site
// (`[props['aria-describedby'], descriptionId].filter(Boolean).join(' ')`).
//! [`IdRefs`]: the one way to build an ARIA id reference list.

use leptos::prelude::*;

use crate::{
    Id,
    utils::{id::IdListBuilder, probe::probe_sources},
};

/// ARIA id reference lists (`aria-labelledby`, `aria-describedby`, `aria-controls`,
/// `aria-owns`, ...): element ids in order, each once.
///
/// Every place that combines several contributions to such an attribute (a field's description
/// and error message, a long-press description, a tooltip's description, ...) builds it through
/// [`IdRefs::derive`]. A contribution is one id or a space-separated list of ids.
#[derive(Debug, Clone, Copy)]
pub struct IdRefs;

impl IdRefs {
    /// The reference list of `contributions` (each one id or a space-separated list): their
    /// ids in order, each once, without copying ids. Contributions that
    /// can't change are folded once: without reactive ones, the reference is a constant; a single
    /// reactive one next to absent constants is the reference itself (no memo); otherwise a memo
    /// joins them (one present contribution that is a joined list already is passed on as it is).
    pub fn derive(
        contributions: impl IntoIterator<Item = Signal<Option<Id>>>,
    ) -> Signal<Option<Id>> {
        let mut is_reactive = false;
        let mut reactive_count = 0_usize;
        let mut has_fixed = false;
        let contributions: Vec<_> = contributions
            .into_iter()
            .filter(|contribution| {
                let (is_absent, reads_sources) =
                    probe_sources(|| contribution.with(Option::is_none));
                is_reactive |= reads_sources;
                if reads_sources {
                    reactive_count += 1;
                } else if !is_absent {
                    has_fixed = true;
                }
                reads_sources || !is_absent
            })
            .collect();
        if !is_reactive {
            return Signal::stored(join_contributions(&contributions));
        }
        if reactive_count == 1 && !has_fixed {
            return contributions[0];
        }
        Memo::new(move |_| join_contributions(&contributions)).into()
    }
}

/// The present contributions joined; the only present one as it is, if it is joined already.
fn join_contributions(contributions: &[Signal<Option<Id>>]) -> Option<Id> {
    let mut builder = IdListBuilder::default();
    let mut present = 0_usize;
    let mut first = None;
    for contribution in contributions {
        contribution.with(|id| {
            if let Some(id) = id {
                present += 1;
                if first.is_none() {
                    first = Some(id.clone());
                }
                builder.push(id);
            }
        });
    }
    match first {
        Some(first) if present == 1 && first.is_joined() => Some(first),
        _ => builder.finish(),
    }
}

#[cfg(test)]
mod tests {
    use assertr::prelude::*;

    use super::*;
    use crate::{testing::with_owner, utils::probe::is_constant};

    #[test]
    fn derive_joins_present_contributions_and_follows_them() {
        with_owner(|| {
            let a = RwSignal::new(None::<Id>);
            let b = Signal::stored(Some(Id::from("b")));
            let c = Signal::stored(Some(Id::from("c b")));
            let joined = IdRefs::derive([a.into(), b, c]);
            assert_that!(joined.get_untracked()).is_equal_to(Some(Id::from("b c")));

            a.set(Some(Id::from("a")));
            assert_that!(joined.get_untracked()).is_equal_to(Some(Id::from("a b c")));
        });
    }

    #[test]
    fn derive_passes_one_joined_contribution_on() {
        with_owner(|| {
            let id = Id::from(String::from("description-1"));
            let only = RwSignal::new(Some(id.clone()));
            let missing = Signal::stored(None);
            let joined = IdRefs::derive([only.into(), missing]);
            let passed = joined.get_untracked().expect("an id");
            // The same shared string, not a copy.
            assert_that!(passed.as_ptr()).is_equal_to(id.as_ptr());
            assert_that!(is_constant(joined)).is_false();
        });
    }

    #[test]
    fn derive_folds_constant_contributions() {
        with_owner(|| {
            let constant = IdRefs::derive([
                Signal::stored(Some(Id::from("a"))),
                Signal::stored(None),
                Signal::stored(Some(Id::from("b a"))),
            ]);
            assert_that!(constant.get_untracked()).is_equal_to(Some(Id::from("a b")));
            assert_that!(is_constant(constant)).is_true();
            let reactive = RwSignal::new(None::<Id>);
            let joined = IdRefs::derive([
                Signal::stored(Some(Id::from("a"))),
                reactive.into(),
                Signal::stored(None),
            ]);
            assert_that!(joined.get_untracked()).is_equal_to(Some(Id::from("a")));
            reactive.set(Some(Id::from("b")));
            assert_that!(joined.get_untracked()).is_equal_to(Some(Id::from("a b")));
        });
    }
}
