// No upstream: react-aria's hooks each have a `useEffect` for these resets.
//! Reactions of hooks to a flag becoming `true` (`is_disabled`: end the press, the hover, the
//! focus), sharing one effect when a hook composes several hooks (`documentation/performance.md`,
//! R6).
//!
//! An `Effect` costs ~10 allocations and runs ~4 more each time; a `Button` composing `use_press`,
//! `use_hover` and `use_focus_ring` had one per hook, all reacting to its disabled state.
//! [`when_true`] registers a reaction: for a constant flag nothing (an effect that tracked a
//! constant would never rerun), else an effect; inside [`sharing_one_effect`], the reactions of
//! every hook called share one effect. Reactions are resets ("if disabled, end the hover"), so
//! running all of them when one flag changes is the same as running the one.

use std::cell::RefCell;

use leptos::prelude::*;

use super::probe::probe_sources;

/// A flag and what to do while it is `true`.
type Reaction = (Signal<bool>, Box<dyn Fn()>);

thread_local! {
    /// The reactions collected by the outermost [`sharing_one_effect`] running, if any.
    static SHARED: RefCell<Option<Vec<Reaction>>> = const { RefCell::new(None) };
}

/// Runs `react` after rendering (in an effect) whenever `flag` is or becomes `true`, as
/// `Effect::new(move || if flag.get() { react() })` would: nothing at all for a flag that is
/// constantly `false`. Inside [`sharing_one_effect`], it joins that effect.
pub(crate) fn when_true(flag: Signal<bool>, react: impl Fn() + 'static) {
    if let (false, false) = probe_sources(|| flag.get()) {
        return;
    }
    if SHARED.with_borrow(Option::is_some) {
        let reaction: Reaction = (flag, Box::new(react));
        SHARED.with_borrow_mut(|shared| {
            if let Some(reactions) = shared {
                reactions.push(reaction);
            }
        });
    } else {
        // Unshared, the effect of its own (no box).
        Effect::new(move |_| {
            if flag.get() {
                react();
            }
        });
    }
}

/// Calls `f` and lets the hooks it calls share one effect for their resets: several leptonic
/// hooks end their state when their element becomes disabled (`use_press` ends the press,
/// `use_hover` the hover, `use_focus_ring` the focus), each in an effect of its own. Called in
/// `sharing_one_effect`, they register these resets instead, and one effect, created after `f`
/// returned, runs them when a disabled flag changes (~10 allocations at mount and ~4 per run
/// saved per hook after the first). A reset whose flag never changes costs nothing either way.
/// Nested calls join the outermost one. Leptonic's composing hooks (`use_button`,
/// `use_selectable_item`, ...) share theirs already; use it where you compose several hooks for
/// one element, as leptonic's atoms do (`GridListItem`):
///
/// ```ignore
/// let (item, hover) =
///     sharing_one_effect(|| (use_grid_list_item(item_input), use_hover(hover_input)));
/// ```
pub fn sharing_one_effect<T>(f: impl FnOnce() -> T) -> T {
    /// Ends the collection when dropped, also if `f` panics.
    struct End;
    impl Drop for End {
        fn drop(&mut self) {
            SHARED.with_borrow_mut(|shared| *shared = None);
        }
    }

    if SHARED.with_borrow(Option::is_some) {
        return f();
    }
    SHARED.with_borrow_mut(|shared| *shared = Some(Vec::new()));
    let end = End;
    let value = f();
    let reactions = SHARED.with_borrow_mut(Option::take).unwrap_or_default();
    drop(end);
    if !reactions.is_empty() {
        Effect::new(move |_| {
            for (flag, react) in &reactions {
                if flag.get() {
                    react();
                }
            }
        });
    }
    value
}

#[cfg(test)]
mod tests {
    use std::sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    };

    use assertr::prelude::*;

    use super::*;
    use crate::testing::{flush_effects, with_owner};

    fn counter() -> (Arc<AtomicUsize>, impl Fn() + 'static) {
        let count = Arc::new(AtomicUsize::new(0));
        let counted = Arc::clone(&count);
        (count, move || {
            counted.fetch_add(1, Ordering::Relaxed);
        })
    }

    #[test]
    fn reacts_while_the_flag_is_true() {
        with_owner(|| {
            let flag = RwSignal::new(false);
            let (count, react) = counter();
            when_true(flag.into(), react);
            flush_effects();
            assert_that!(count.load(Ordering::Relaxed)).is_equal_to(0);
            flag.set(true);
            flush_effects();
            assert_that!(count.load(Ordering::Relaxed)).is_equal_to(1);
        });
    }

    #[test]
    fn shared_reactions_run_and_nested_ones_join_the_outer_effect() {
        with_owner(|| {
            let first = RwSignal::new(false);
            let second = RwSignal::new(false);
            let (first_count, react_first) = counter();
            let (second_count, react_second) = counter();
            let (constant_count, react_constant) = counter();
            sharing_one_effect(|| {
                when_true(first.into(), react_first);
                when_true(Signal::stored(false), react_constant);
                sharing_one_effect(|| when_true(second.into(), react_second));
            });
            flush_effects();
            second.set(true);
            flush_effects();
            assert_that!(first_count.load(Ordering::Relaxed)).is_equal_to(0);
            assert_that!(second_count.load(Ordering::Relaxed)).is_equal_to(1);
            first.set(true);
            flush_effects();
            assert_that!(first_count.load(Ordering::Relaxed)).is_equal_to(1);
            // One effect: the nested reaction runs again with the outer one.
            assert_that!(second_count.load(Ordering::Relaxed)).is_equal_to(2);
            assert_that!(constant_count.load(Ordering::Relaxed)).is_equal_to(0);
        });
    }

    /// A hook panicking inside `sharing_one_effect` (and caught) leaves no collection behind, in
    /// which later reactions would wait for an effect that is never created.
    #[test]
    fn a_panic_ends_the_shared_collection() {
        fn failing_hook() {
            when_true(RwSignal::new(false).into(), || {});
            panic!("a hook");
        }
        with_owner(|| {
            let panicked = std::panic::catch_unwind(|| sharing_one_effect(failing_hook));
            assert_that!(panicked.is_err()).is_true();
            assert_that!(SHARED.with_borrow(Option::is_none)).is_true();
        });
    }
}
