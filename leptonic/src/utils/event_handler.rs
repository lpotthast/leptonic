// TODO: Remove this

use std::{marker::PhantomData, ops::Deref};

use leptos::ev::EventDescriptor;
use web_sys::KeyboardEvent;

pub(crate) fn create_event_handler<E>(
    handler: impl Fn(&NonPropagatingEvent<E::EventType>),
) -> impl Fn(E::EventType)
where
    E: EventDescriptor,
    E::EventType: PropagationStoppable,
{
    move |e: E::EventType| {
        let e = NonPropagatingEvent::new(e);
        handler(&e);
        if e.should_stop_propagation {
            let et = e.deref();
            et.do_stop_propagation();
        }
    }
}

pub(crate) struct NonPropagatingEvent<E> {
    pub(crate) e: E,
    pub(crate) should_stop_propagation: bool,
}

impl<E> NonPropagatingEvent<E> {
    pub(crate) fn new(e: E) -> Self {
        Self {
            e,
            should_stop_propagation: true,
        }
    }

    /// Events are not propagated by default.
    /// By calling this you indicate that this event should be passed to the next handler.
    pub fn continue_propagation(&mut self) {
        self.should_stop_propagation = false;
    }
}

impl<E> Deref for NonPropagatingEvent<E> {
    type Target = E;

    fn deref(&self) -> &Self::Target {
        &self.e
    }
}
