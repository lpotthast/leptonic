//! Two-way bindings of a component's value to app state.

use leptos::prelude::*;

/// A value living outside a component (app state), which the component reads and sets: Leptos'
/// `bind:value` for leptonic's hooks and atoms. Create it from an `RwSignal` or a signal pair,
/// or with [`ValueBinding::new`] for any other storage.
#[derive(Debug)]
pub struct ValueBinding<T: Send + Sync + 'static> {
    pub value: Signal<T>,
    set_value: Callback<T>,
}

impl<T: Send + Sync + 'static> Clone for ValueBinding<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T: Send + Sync + 'static> Copy for ValueBinding<T> {}

impl<T: Send + Sync + 'static> ValueBinding<T> {
    pub fn new(value: Signal<T>, set_value: Callback<T>) -> Self {
        Self { value, set_value }
    }

    pub fn set(&self, value: T) {
        self.set_value.run(value);
    }
}

impl<T: Clone + Send + Sync + 'static> From<RwSignal<T>> for ValueBinding<T> {
    fn from(signal: RwSignal<T>) -> Self {
        Self::new(signal.into(), Callback::new(move |value| signal.set(value)))
    }
}

impl<T: Clone + Send + Sync + 'static> From<(ReadSignal<T>, WriteSignal<T>)> for ValueBinding<T> {
    fn from((read, write): (ReadSignal<T>, WriteSignal<T>)) -> Self {
        Self::new(read.into(), Callback::new(move |value| write.set(value)))
    }
}
