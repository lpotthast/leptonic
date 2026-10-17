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
    /// The reactions collected by the innermost [`sharing_one_effect`], if any.
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

/// Calls `f`; the [`when_true`] reactions registered meanwhile (by the hooks `f` calls) share one
/// effect, created after `f` returned. A nested call shares its own.
pub(crate) fn sharing_one_effect<T>(f: impl FnOnce() -> T) -> T {
    /// Restores the outer collection when dropped, also if `f` panics.
    struct Restore(Option<Vec<Reaction>>);
    impl Drop for Restore {
        fn drop(&mut self) {
            let outer = self.0.take();
            SHARED.with_borrow_mut(|shared| *shared = outer);
        }
    }

    let restore = Restore(SHARED.with_borrow_mut(|shared| shared.replace(Vec::new())));
    let value = f();
    let reactions = SHARED.with_borrow_mut(Option::take).unwrap_or_default();
    drop(restore);
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
    fn shared_reactions_run_and_nested_ones_have_their_own_effect() {
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
            assert_that!(constant_count.load(Ordering::Relaxed)).is_equal_to(0);
        });
    }
}
