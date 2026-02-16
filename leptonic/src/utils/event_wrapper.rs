use std::fmt;

use super::propagation_control::{PropagationControl, Sealed};
use crate::utils::Propagation;

/// Generic wrapper for any DOM event providing the standard propagation-control API
/// from react-aria's `createEventHandler`.
///
/// By default, events in Leptonic stop propagation. Handlers must call
/// [`continue_propagation()`](Propagation::continue_propagation) to opt in to
/// bubbling — the same model used by react-aria.
pub struct EventWrapper<E: AsRef<web_sys::Event>> {
    event: E,
    propagation: PropagationControl,
}

impl<E: AsRef<web_sys::Event>> Sealed for EventWrapper<E> {}
impl<E: AsRef<web_sys::Event>> Propagation for EventWrapper<E> {
    fn propagation_control(&self) -> &PropagationControl {
        &self.propagation
    }
}

impl<E: AsRef<web_sys::Event>> EventWrapper<E> {
    /// Create a new event wrapper.
    ///
    /// Returns the wrapper together with a shared `AtomicBool` that the caller
    /// can inspect *after* running the user's handler to decide whether native
    /// propagation should be stopped.
    pub(crate) fn new(event: E) -> (Self, std::sync::Arc<std::sync::atomic::AtomicBool>) {
        let (propagation, state) = PropagationControl::new();
        (Self { event, propagation }, state)
    }

    /// Access the inner DOM event.
    pub fn event(&self) -> &E {
        &self.event
    }

    /// Prevent the browser's default action for this event.
    pub fn prevent_default(&self) {
        self.event.as_ref().prevent_default();
    }

    /// Whether `prevent_default()` has been called.
    pub fn is_default_prevented(&self) -> bool {
        self.event.as_ref().default_prevented()
    }

    /// Get the event target.
    pub fn target(&self) -> Option<web_sys::EventTarget> {
        self.event.as_ref().target()
    }

    /// Get the current target.
    pub fn current_target(&self) -> Option<web_sys::EventTarget> {
        self.event.as_ref().current_target()
    }
}

impl<E: AsRef<web_sys::Event> + fmt::Debug> fmt::Debug for EventWrapper<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("EventWrapper")
            .field("event", &self.event)
            .field("propagation", &self.propagation)
            .finish()
    }
}

impl<E: AsRef<web_sys::Event> + Clone> Clone for EventWrapper<E> {
    fn clone(&self) -> Self {
        Self {
            event: self.event.clone(),
            propagation: self.propagation.clone(),
        }
    }
}
