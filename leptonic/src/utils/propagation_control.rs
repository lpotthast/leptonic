use std::{
    fmt,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
};

/// Controls whether an event should continue propagating to parent handlers.
///
/// By default, events in Leptonic stop propagation. Handlers must call
/// [`Propagation::continue_propagation`] to opt in to bubbling.
///
/// This type is publicly visible but cannot be constructed outside the crate.
pub struct PropagationControl {
    state: Arc<AtomicBool>,
    trigger: Arc<dyn Fn() + Send + Sync + 'static>,
}

impl PropagationControl {
    /// Create a new propagation control.
    ///
    /// Returns the control together with a shared [`AtomicBool`] that the caller
    /// can inspect *after* running the user's handler to decide whether native
    /// propagation should be stopped.
    pub(crate) fn new() -> (Self, Arc<AtomicBool>) {
        let state = Arc::new(AtomicBool::new(false));
        let state_for_trigger = state.clone();
        let trigger = Arc::new(move || {
            state_for_trigger.store(true, Ordering::Release);
        });
        let state_clone = state.clone();
        (Self { state, trigger }, state_clone)
    }

    /// Allow parent handlers to also handle this event.
    pub(crate) fn continue_propagation(&self) {
        (self.trigger)();
    }

    // TODO: We should delete this here. This could be an override of an events default `stop_propagation` event...
    /// Warns the caller that `stop_propagation` is the default behavior.
    #[allow(clippy::unused_self)]
    pub(crate) fn stop_propagation(&self) {
        tracing::warn!(
            "stop_propagation is now the default behavior for events in Leptonic. \
             You can use continue_propagation() to revert this behavior."
        );
    }

    /// Returns `true` when propagation will be stopped (the default).
    pub(crate) fn is_propagation_stopped(&self) -> bool {
        !self.state.load(Ordering::Acquire)
    }
}

impl fmt::Debug for PropagationControl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PropagationControl")
            .field("state", &self.state)
            .finish_non_exhaustive()
    }
}

impl Clone for PropagationControl {
    fn clone(&self) -> Self {
        Self {
            state: self.state.clone(),
            trigger: self.trigger.clone(),
        }
    }
}

mod sealed {
    pub trait Sealed {}
}

pub(crate) use sealed::Sealed;

/// Trait for event types that support propagation control.
///
/// Events in Leptonic stop propagation by default. Handlers must call
/// [`continue_propagation()`](Propagation::continue_propagation) to opt in to
/// bubbling — the same model used by react-aria.
///
/// This trait is sealed — it cannot be implemented outside the crate.
#[allow(private_bounds)]
pub trait Propagation: sealed::Sealed {
    /// Access the underlying propagation control.
    fn propagation_control(&self) -> &PropagationControl;

    /// Allow parent handlers to also handle this event.
    fn continue_propagation(&self) {
        self.propagation_control().continue_propagation();
    }

    /// Warns that `stop_propagation` is the default behavior.
    fn stop_propagation(&self) {
        self.propagation_control().stop_propagation();
    }

    /// Returns `true` when propagation will be stopped (the default).
    fn is_propagation_stopped(&self) -> bool {
        self.propagation_control().is_propagation_stopped()
    }
}
