// No upstream: telling deferred callbacks whether their reactive owner still lives (React has no
// disposal of a component's state before its callbacks).

use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};

use leptos::prelude::on_cleanup;

/// Whether the reactive owner that created it is still alive.
///
/// Deferred callbacks (timeouts, animation frames, microtasks, global listeners) can run after
/// their component was unmounted; touching its disposed reactive values then panics. Check
/// [`OwnerAlive::get`] first. Unlike the owner's own signals, this flag is not stored in the
/// reactive arena, so reading it never panics.
#[derive(Debug, Clone)]
pub(crate) struct OwnerAlive(Arc<AtomicBool>);

impl OwnerAlive {
    /// A flag that turns `false` when the current owner is cleaned up.
    pub(crate) fn new() -> Self {
        let alive = Arc::new(AtomicBool::new(true));
        let flag = alive.clone();
        on_cleanup(move || flag.store(false, Ordering::Release));
        Self(alive)
    }

    pub(crate) fn get(&self) -> bool {
        self.0.load(Ordering::Acquire)
    }
}
