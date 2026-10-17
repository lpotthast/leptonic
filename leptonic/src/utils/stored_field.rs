// No upstream: React keeps a hook's mutable state in refs, which cost nothing to create.
//! [`StoredField`]: one field of a hook's state struct, used like a `StoredValue` of its own.
//!
//! Every `StoredValue` costs 2 allocations (an `Arc` and an arena slot) per hook instance
//! (`documentation/performance.md`, R7). A hook with several pieces of non-reactive state keeps
//! them in one struct in one `StoredValue`, and hands out `StoredField`s (`Copy`, no allocation)
//! where its closures used separate `StoredValue`s: their call sites stay as they were.

use leptos::prelude::*;

/// One field of the state struct `S` in a `StoredValue<S, St>`, with the `StoredValue` methods
/// for that field (`with_value`, `update_value`, `set_value`, `get_value`, and `try_`
/// variants). `Copy`, so closures capture it as they capture the `StoredValue`.
///
/// ```ignore
/// #[derive(Default)]
/// struct State { press: Option<PressState>, is_triggering_event: bool }
///
/// let state = StoredValue::new_local(State::default());
/// let press = StoredField::new(state, |s| &s.press, |s| &mut s.press);
/// let is_triggering_event =
///     StoredField::new(state, |s| &s.is_triggering_event, |s| &mut s.is_triggering_event);
/// is_triggering_event.set_value(true);
/// ```
pub(crate) struct StoredField<S: 'static, T: 'static, St = LocalStorage> {
    store: StoredValue<S, St>,
    field: fn(&S) -> &T,
    field_mut: fn(&mut S) -> &mut T,
}

impl<S, T, St> Clone for StoredField<S, T, St> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<S, T, St> Copy for StoredField<S, T, St> {}

impl<S, T, St> StoredField<S, T, St>
where
    S: 'static,
    T: 'static,
    St: Storage<ArcStoredValue<S>>,
{
    /// The field `field`/`field_mut` projects `store`'s value to.
    pub(crate) fn new(
        store: StoredValue<S, St>,
        field: fn(&S) -> &T,
        field_mut: fn(&mut S) -> &mut T,
    ) -> Self {
        Self {
            store,
            field,
            field_mut,
        }
    }

    /// Reads the field; `None` once the store is disposed.
    pub(crate) fn try_with_value<O>(&self, fun: impl FnOnce(&T) -> O) -> Option<O> {
        let field = self.field;
        self.store.try_with_value(|state| fun(field(state)))
    }

    /// Reads the field. Panics once the store is disposed.
    pub(crate) fn with_value<O>(&self, fun: impl FnOnce(&T) -> O) -> O {
        let field = self.field;
        self.store.with_value(|state| fun(field(state)))
    }

    /// Changes the field; `None` once the store is disposed.
    pub(crate) fn try_update_value<O>(&self, fun: impl FnOnce(&mut T) -> O) -> Option<O> {
        let field_mut = self.field_mut;
        self.store.try_update_value(|state| fun(field_mut(state)))
    }

    /// Changes the field (nothing once the store is disposed).
    pub(crate) fn update_value(&self, fun: impl FnOnce(&mut T)) {
        self.try_update_value(fun);
    }

    /// Replaces the field (nothing once the store is disposed). The previous value is dropped
    /// after the store is released.
    pub(crate) fn set_value(&self, value: T) {
        drop(self.try_update_value(|field| std::mem::replace(field, value)));
    }

    /// A copy of the field; `None` once the store is disposed.
    pub(crate) fn try_get_value(&self) -> Option<T>
    where
        T: Clone,
    {
        self.try_with_value(T::clone)
    }

    /// A copy of the field. Panics once the store is disposed.
    pub(crate) fn get_value(&self) -> T
    where
        T: Clone,
    {
        self.with_value(T::clone)
    }
}

#[cfg(test)]
mod tests {
    use assertr::prelude::*;

    use super::*;
    use crate::testing::with_owner;

    #[derive(Default)]
    struct State {
        count: u32,
        name: Option<String>,
    }

    #[test]
    fn fields_read_and_write_their_part_of_the_state() {
        with_owner(|| {
            let state = StoredValue::new_local(State::default());
            let count = StoredField::new(state, |s| &s.count, |s| &mut s.count);
            let name = StoredField::new(state, |s| &s.name, |s| &mut s.name);

            count.set_value(2);
            count.update_value(|count| *count += 1);
            name.set_value(Some("a".to_owned()));

            assert_that!(count.get_value()).is_equal_to(3);
            assert_that!(name.with_value(Clone::clone)).is_equal_to(Some("a".to_owned()));
            assert_that!(state.with_value(|s| (s.count, s.name.clone())))
                .is_equal_to((3, Some("a".to_owned())));
            assert_that!(name.try_update_value(Option::take).flatten())
                .is_equal_to(Some("a".to_owned()));
            assert_that!(name.get_value()).is_none();
        });
    }

    #[test]
    fn a_disposed_store_reads_as_none() {
        let owner = Owner::new();
        let count = owner.with(|| {
            let state = StoredValue::new_local(State::default());
            StoredField::new(state, |s| &s.count, |s| &mut s.count)
        });
        owner.cleanup();
        assert_that!(count.try_with_value(|count| *count)).is_none();
        assert_that!(count.try_update_value(|count| *count += 1)).is_none();
    }
}
