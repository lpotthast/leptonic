// No upstream: React re-renders instead of tracking reactive sources.
//! Finding out whether a reactive value can change, so that constants cost no reactive machinery
//! (`documentation/performance.md`, R2).
//!
//! Leptos renders every `Signal`, `MaybeProp` or closure in an attribute through a `RenderEffect`
//! (~10 allocations at mount), also when it is a constant (`Signal::stored`, an absent
//! `MaybeProp`, a derive over constants): such an effect tracks nothing and never runs again.
//! [`probe_sources`] evaluates a value once under an observer that only records whether it read a
//! reactive source; [`SignalAttr`](super::signal_attr::SignalAttr) renders an attribute without
//! an effect, and hooks skip effects reacting to inputs, when they read none. A computation that
//! read no source can't be notified, so skipping the effect is exactly what the effect would have
//! done.

use std::sync::{Arc, Mutex, Weak};

use leptos::{
    prelude::*,
    reactive::graph::{AnySource, AnySubscriber, ReactiveNode, Source, Subscriber, WithObserver},
};

/// The observer of [`probe_sources`]: records the sources read under it, and reacts to nothing.
struct Probe {
    sources: Mutex<Vec<AnySource>>,
}

impl ReactiveNode for Probe {
    fn mark_dirty(&self) {}

    fn mark_check(&self) {}

    fn mark_subscribers_check(&self) {}

    fn update_if_necessary(&self) -> bool {
        false
    }
}

impl Subscriber for Probe {
    fn add_source(&self, source: AnySource) {
        if let Ok(mut sources) = self.sources.lock() {
            sources.push(source);
        }
    }

    fn clear_sources(&self, _subscriber: &AnySubscriber) {}
}

thread_local! {
    /// One probe per thread, kept forever: probing allocates nothing (its source buffer grows once).
    static PROBE: (Arc<Probe>, AnySubscriber) = {
        let probe = Arc::new(Probe {
            sources: Mutex::new(Vec::new()),
        });
        // Subscribers are identified by address, as Leptos' own.
        let id = Arc::as_ptr(&probe).cast::<()>() as usize;
        let weak: Weak<dyn Subscriber + Send + Sync> = Arc::downgrade(&probe) as _;
        (probe, AnySubscriber(id, weak))
    };
}

/// Evaluates `f` once and tells whether it read a reactive source (a signal, memo or trigger,
/// directly or through a derived signal). `false` means the value can never change: nothing
/// would notify a computation that tracked nothing.
///
/// `f` runs untracked by the current observer, and the sources it read don't keep a subscription.
/// Allocates nothing for a value without sources (and nothing for a reactive one, unless the
/// sources' subscriber sets grow). Re-entrant: `f` may probe too.
///
/// ```ignore
/// let (value, is_reactive) = probe_sources(|| signal.get());
/// ```
pub fn probe_sources<T>(f: impl FnOnce() -> T) -> (T, bool) {
    // The probing itself is not generic (`performance.md` R13): only this wrapper is instantiated
    // per value type.
    let mut f = Some(f);
    let mut value = None;
    let is_reactive = probe_dyn(&mut || value = f.take().map(|f| f()));
    match value {
        Some(value) => (value, is_reactive),
        None => unreachable!("`probe_dyn` calls its function once"),
    }
}

/// [`probe_sources`] of a type-erased function.
fn probe_dyn(f: &mut dyn FnMut()) -> bool {
    PROBE.with(|(probe, subscriber)| {
        let start = probe.sources.lock().map_or(0, |sources| sources.len());
        subscriber.with_observer(f);
        // Unsubscribe from what `f` read, one source at a time outside the lock (a source may
        // call back), without collecting them.
        let mut is_reactive = false;
        while let Some(source) = probe
            .sources
            .lock()
            .ok()
            .and_then(|mut sources| (sources.len() > start).then(|| sources.pop()).flatten())
        {
            is_reactive = true;
            source.remove_subscriber(subscriber);
        }
        is_reactive
    })
}

/// Whether `signal` can never change: it read no reactive source (`Signal::stored`, a derive of
/// constants). Evaluates it once.
pub fn is_constant<T: Send + Sync + 'static>(signal: Signal<T>) -> bool
where
    Signal<T>: With<Value = T>,
{
    !probe_sources(|| signal.with(|_| ())).1
}

/// `a || b` as a signal, folding constants: a constant `false` side drops out and a constant
/// `true` side is the result, so that only two reactive sides derive a new signal (2
/// allocations, held until the owner is disposed).
pub fn either(a: Signal<bool>, b: Signal<bool>) -> Signal<bool> {
    let (a_value, a_is_reactive) = probe_sources(|| a.get());
    if !a_is_reactive {
        return if a_value { a } else { b };
    }
    let (b_value, b_is_reactive) = probe_sources(|| b.get());
    if !b_is_reactive {
        return if b_value { b } else { a };
    }
    Signal::derive(move || a.get() || b.get())
}

#[cfg(test)]
mod tests {
    use assertr::prelude::*;

    use super::*;
    use crate::testing::{flush_effects, with_owner};

    #[test]
    fn constants_have_no_sources() {
        with_owner(|| {
            let stored = Signal::stored(3);
            let derived_of_constants = Signal::derive(move || stored.get() + 1);
            assert_that!(probe_sources(|| stored.get())).is_equal_to((3, false));
            assert_that!(probe_sources(|| derived_of_constants.get())).is_equal_to((4, false));
            let absent = MaybeProp::<String>::default();
            assert_that!(probe_sources(|| absent.get())).is_equal_to((None, false));
        });
    }

    #[test]
    fn signals_and_derives_of_signals_have_sources() {
        with_owner(|| {
            let source = RwSignal::new(1);
            let derived = Signal::derive(move || source.get() * 2);
            let memo = Memo::new(move |_| source.get() + 1);
            assert_that!(probe_sources(|| source.get())).is_equal_to((1, true));
            assert_that!(probe_sources(|| derived.get())).is_equal_to((2, true));
            assert_that!(probe_sources(|| memo.get())).is_equal_to((2, true));
            assert_that!(is_constant(Signal::<i32>::from(source))).is_false();
        });
    }

    /// A condition that skips the reactive read reads no source.
    #[test]
    fn only_what_was_read_counts() {
        with_owner(|| {
            let disabled = Signal::stored(true);
            let pending = RwSignal::new(false);
            let interactions_disabled = Signal::derive(move || disabled.get() || pending.get());
            assert_that!(is_constant(interactions_disabled)).is_true();
        });
    }

    /// Probing neither subscribes the current observer nor leaves the probe subscribed: an effect
    /// that only probed a signal doesn't rerun when it changes.
    #[test]
    fn probing_leaves_no_subscription() {
        with_owner(|| {
            let source = RwSignal::new(0);
            let runs = StoredValue::new(0);
            Effect::new(move || {
                runs.update_value(|runs| *runs += 1);
                let _ = probe_sources(|| source.get());
            });
            flush_effects();
            source.set(1);
            flush_effects();
            assert_that!(runs.get_value()).is_equal_to(1);
        });
    }

    /// The outer observer keeps tracking what it reads after a probe.
    #[test]
    fn probing_inside_an_effect_keeps_its_own_tracking() {
        with_owner(|| {
            let probed = RwSignal::new(0);
            let tracked = RwSignal::new(0);
            let runs = StoredValue::new(0);
            Effect::new(move || {
                runs.update_value(|runs| *runs += 1);
                let _ = probe_sources(|| probed.get());
                tracked.track();
            });
            flush_effects();
            tracked.set(1);
            flush_effects();
            assert_that!(runs.get_value()).is_equal_to(2);
        });
    }

    #[test]
    fn either_folds_constants() {
        with_owner(|| {
            let source = RwSignal::new(false);
            let reactive = Signal::from(source);
            let yes = Signal::stored(true);
            let no = Signal::stored(false);
            assert_that!(either(no, reactive)).is_equal_to(reactive);
            assert_that!(either(reactive, no)).is_equal_to(reactive);
            assert_that!(either(yes, reactive)).is_equal_to(yes);
            assert_that!(either(reactive, yes)).is_equal_to(yes);

            let both = either(reactive, Signal::derive(move || !source.get()));
            assert_that!(both).is_not_equal_to(reactive);
            assert_that!(both.get()).is_true();
            assert_that!(is_constant(both)).is_false();
        });
    }

    #[test]
    fn nested_probes_report_their_own_reads() {
        with_owner(|| {
            let source = RwSignal::new(1);
            let constant = Signal::stored(2);
            let ((inner, inner_reactive), outer_reactive) =
                probe_sources(|| probe_sources(|| constant.get()));
            assert_that!((inner, inner_reactive, outer_reactive)).is_equal_to((2, false, false));
            let (((), inner_reactive), outer_reactive) = probe_sources(|| {
                let inner = probe_sources(|| ());
                source.track();
                ((), inner.1)
            });
            assert_that!((inner_reactive, outer_reactive)).is_equal_to((false, true));
        });
    }
}
