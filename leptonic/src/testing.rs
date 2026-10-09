// No upstream: leptonic's native test support (react-aria tests run in jsdom, with React's `act`).

//! Native (`--lib`) test support: run hooks inside a reactive owner, with Effects.
//!
//! ```ignore
//! use crate::testing::{flush_effects, with_owner};
//!
//! #[test]
//! fn refocuses_when_the_focused_row_disappears() {
//!     with_owner(|| {
//!         let state = use_grid_state(..);
//!         flush_effects(); // the Effects' first runs
//!         set_rows.set(..);
//!         flush_effects(); // the re-runs the change caused
//!         assert_that!(state.list.selection.focused_key()).is_equal_to(..);
//!     });
//! }
//! ```
//!
//! Effects need the `effects` feature of `reactive_graph` (on in leptonic's test builds through
//! its dev-dependencies) and an async executor. [`with_owner`] installs one whose tasks run on
//! the test's own thread, and only when the test calls [`flush_effects`]: the test decides when
//! Effects run, like React's `act`, and other tests' threads are unaffected.
//!
//! Tests that create Effects under a plain `Owner::new().with(..)` keep working: their Effects
//! never run (with no executor installed yet, `any_spawner`'s `tracing` feature drops the task
//! instead of panicking; with one installed, the task waits in that thread's queue).
//!
//! Leptos' warnings about signals read outside a tracking context (printed to stderr: shown for
//! failed tests, or with `--nocapture`) are suppressed only for the test body's own code, whose
//! reads of state are expected: Effects and the tasks they spawn run in [`flush_effects`], outside
//! that zone, so their untracked reads warn as they would in the browser.

use std::{cell::RefCell, sync::Once};

use any_spawner::{CustomExecutor, Executor, PinnedFuture, PinnedLocalFuture};
use futures::{
    executor::{LocalPool, LocalSpawner},
    task::LocalSpawnExt,
};
use leptos::prelude::Owner;
use reactive_graph::diagnostics::{SpecialNonReactiveZone, SpecialNonReactiveZoneGuard};

thread_local! {
    static POOL: RefCell<LocalPool> = RefCell::new(LocalPool::new());
    static SPAWNER: LocalSpawner = POOL.with(|pool| pool.borrow().spawner());
    /// While a [`with_owner`] body runs: the zone in which its reads of signals outside a tracking
    /// context don't warn. [`flush_effects`] lifts it while Effects and tasks run.
    static TEST_BODY_ZONE: RefCell<Option<SpecialNonReactiveZoneGuard>> = const { RefCell::new(None) };
}

/// Runs every task on the spawning thread's [`POOL`], also the `Send` ones (deterministic tests:
/// no thread pool).
struct ThreadLocalExecutor;

impl CustomExecutor for ThreadLocalExecutor {
    fn spawn(&self, fut: PinnedFuture<()>) {
        self.spawn_local(fut);
    }

    fn spawn_local(&self, fut: PinnedLocalFuture<()>) {
        SPAWNER.with(|spawner| {
            spawner
                .spawn_local(fut)
                .expect("the thread's pool lives as long as the thread");
        });
    }

    fn poll_local(&self) {
        flush_effects();
    }
}

fn init_executor() {
    static INIT: Once = Once::new();
    INIT.call_once(|| {
        Executor::init_custom_executor(ThreadLocalExecutor)
            .expect("no other executor is installed in leptonic's unit tests");
    });
}

/// Runs `f` in a fresh reactive [`Owner`] (as a component body would run), with Effects enabled.
///
/// Effects created in `f` run when `f` calls [`flush_effects`]. Reading signals outside a
/// tracking context is expected in a test body, so `f`'s own code runs without Leptos' warnings
/// about it; the Effects and tasks [`flush_effects`] runs get them. The owner is disposed when `f`
/// returns, which ends its Effects.
pub(crate) fn with_owner<T>(f: impl FnOnce() -> T) -> T {
    init_executor();
    let owner = Owner::new();
    let result = owner.with(|| {
        TEST_BODY_ZONE.set(Some(SpecialNonReactiveZone::enter()));
        let result = f();
        TEST_BODY_ZONE.take();
        result
    });
    drop(owner);
    // Let the disposed Effects' tasks finish, so the next test on this thread starts clean.
    flush_effects();
    result
}

/// Runs every pending Effect (and other spawned task) of this thread until none can make
/// progress: the initial runs of new Effects and the re-runs caused by signal changes.
///
/// # Panics
///
/// When called from inside an Effect.
pub(crate) fn flush_effects() {
    // Effects and tasks run outside the test body's zone: their untracked reads warn.
    let in_test_body = TEST_BODY_ZONE.take().is_some();
    POOL.with(|pool| {
        pool.try_borrow_mut()
            .expect("flush_effects() is not called from inside an Effect")
            .run_until_stalled();
    });
    if in_test_body {
        TEST_BODY_ZONE.set(Some(SpecialNonReactiveZone::enter()));
    }
}

mod tests {
    use assertr::prelude::*;
    use leptos::prelude::*;

    use super::{flush_effects, with_owner};

    #[test]
    fn effects_run_when_flushed() {
        with_owner(|| {
            let source = RwSignal::new(1);
            let seen = RwSignal::new(Vec::new());
            Effect::new(move || {
                let value = source.get();
                seen.update(|seen| seen.push(value));
            });
            assert_that!(seen.get_untracked()).is_empty();

            flush_effects();
            assert_that!(seen.get_untracked()).contains_exactly([1]);

            source.set(2);
            source.set(3);
            flush_effects();
            assert_that!(seen.get_untracked()).contains_exactly([1, 3]);
        });
    }

    #[test]
    fn effects_end_with_their_owner() {
        let source = RwSignal::new(1);
        let runs = StoredValue::new(0);
        with_owner(|| {
            Effect::new(move || {
                source.track();
                runs.update_value(|runs| *runs += 1);
            });
            flush_effects();
        });
        source.set(2);
        flush_effects();
        assert_that!(runs.get_value()).is_equal_to(1);
    }
}
