// Upstream: react-aria/src/interactions/createEventHandler.ts @ 99e6102368
//! Stopping propagation by default, with `continue_propagation()` to opt out (react-aria's
//! `createEventHandler`), for press and keyboard events.

use std::{
    fmt,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
};

/// Whether an event continues propagating to its target's ancestors once its handlers ran: by
/// default it doesn't; a handler calls [`Propagation::continue_propagation`] to let it bubble.
///
/// Public so that [`Propagation`] can name it; only the crate creates one. Clones share the state:
/// the hook keeps one and hands a clone to the user's handler.
#[derive(Clone, Default)]
pub struct PropagationControl {
    continued: Arc<AtomicBool>,
}

impl PropagationControl {
    /// A control whose event stops propagating unless a handler continues it.
    pub(crate) fn new() -> Self {
        Self::default()
    }

    /// Lets the event propagate.
    pub(crate) fn continue_propagation(&self) {
        self.continued.store(true, Ordering::Release);
    }

    /// Whether the event stops propagating (the default).
    pub(crate) fn is_propagation_stopped(&self) -> bool {
        !self.continued.load(Ordering::Acquire)
    }
}

impl fmt::Debug for PropagationControl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PropagationControl")
            .field("is_propagation_stopped", &self.is_propagation_stopped())
            .finish()
    }
}

mod sealed {
    pub trait Sealed {}
}

pub(crate) use sealed::Sealed;

/// Events whose propagation the user's handler controls (press and keyboard events, as
/// react-aria's `continuePropagation()`): they stop propagating by default; a handler calls
/// [`continue_propagation()`](Propagation::continue_propagation) to let them bubble.
///
/// This trait is sealed: it cannot be implemented outside the crate.
#[allow(private_bounds)]
pub trait Propagation: sealed::Sealed {
    /// Access the underlying propagation control.
    fn propagation_control(&self) -> &PropagationControl;

    /// Allow parent handlers to also handle this event.
    fn continue_propagation(&self) {
        self.propagation_control().continue_propagation();
    }

    /// Returns `true` when propagation will be stopped (the default).
    fn is_propagation_stopped(&self) -> bool {
        self.propagation_control().is_propagation_stopped()
    }
}
