// No upstream: bindings to app state (react-stately: controlled props, `useControlledState`).
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

    /// What a component's state props (C4: `<x>`, `set_<x>`, `on_<x>_change`) mean for its hook:
    /// with `value`, the state is bound to it and `set_value` receives the changes (without
    /// `set_value`, it is read-only); without `value`, the hook owns the state and `set_value`
    /// receives every change, as `on_change` does.
    pub fn from_state_props(
        value: Option<Signal<T>>,
        set_value: Option<crate::Out<T>>,
        on_change: Option<Callback<T>>,
    ) -> (Option<Self>, Option<Callback<T>>)
    where
        T: Clone,
    {
        match (value, set_value) {
            (Some(value), set_value) => (Some(Self::from_props(value, set_value)), on_change),
            (None, Some(set_value)) => (
                None,
                Some(Callback::new(move |new_value: T| {
                    set_value.set(new_value.clone());
                    if let Some(on_change) = on_change {
                        on_change.run(new_value);
                    }
                })),
            ),
            (None, None) => (None, on_change),
        }
    }

    /// The binding of a component's state props (C4): the readable `value` and the writable
    /// `set_value`. Without `set_value`, the state is read-only: changes go nowhere.
    pub fn from_props(value: Signal<T>, set_value: Option<crate::Out<T>>) -> Self {
        Self::new(
            value,
            Callback::new(move |new_value: T| {
                if let Some(set_value) = set_value {
                    set_value.set(new_value);
                }
            }),
        )
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

#[cfg(test)]
mod tests {
    use assertr::prelude::*;

    use super::*;
    use crate::testing::with_owner;

    #[test]
    fn state_props_with_a_value_bind_it() {
        with_owner(|| {
            let app = RwSignal::new(1);
            let changes = RwSignal::new(Vec::new());
            let (binding, on_change) = ValueBinding::from_state_props(
                Some(app.into()),
                Some(crate::Out::from(app)),
                Some(Callback::new(move |v| changes.update(|c| c.push(v)))),
            );
            let binding = binding.expect("bound");
            binding.set(2);
            assert_that!(app.get_untracked()).is_equal_to(2);
            assert_that!(binding.value.get_untracked()).is_equal_to(2);
            // `on_change` stays the hook's to call.
            assert_that!(on_change.is_some()).is_true();
            assert_that!(changes.get_untracked()).is_empty();
        });
    }

    #[test]
    fn a_value_without_a_setter_is_read_only() {
        with_owner(|| {
            let (binding, _) = ValueBinding::from_state_props(Some(Signal::stored(1)), None, None);
            let binding = binding.expect("bound");
            binding.set(2);
            assert_that!(binding.value.get_untracked()).is_equal_to(1);
        });
    }

    #[test]
    fn a_setter_without_a_value_receives_every_change() {
        with_owner(|| {
            let app = RwSignal::new(0);
            let changes = RwSignal::new(Vec::new());
            let (binding, on_change) = ValueBinding::from_state_props(
                None,
                Some(crate::Out::from(app)),
                Some(Callback::new(move |v| changes.update(|c| c.push(v)))),
            );
            assert_that!(binding.is_none()).is_true();
            on_change.expect("change callback").run(3);
            assert_that!(app.get_untracked()).is_equal_to(3);
            assert_that!(changes.get_untracked()).is_equal_to(vec![3]);
        });
    }
}
