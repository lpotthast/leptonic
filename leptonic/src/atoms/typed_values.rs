// No upstream: the typed value props of the selection atoms over the key-based hooks.
//! The value props of selection atoms (`RadioGroup`, `Select`, ...) are typed
//! ([`SelectionValue`]); their hooks work with collection [`Key`]s. These adapters convert at the
//! boundary.

use std::{collections::HashSet, hash::BuildHasher, sync::Arc};

use leptos::prelude::*;

use crate::{
    Out,
    hooks::{
        collections::{Key, SelectionValue},
        form::ValidateFn,
        select::SelectMode,
    },
    utils::dev_warn,
};

/// A typed selection (one value or a set) and its key-based form.
pub(crate) trait Keyed: Clone + Send + Sync + 'static {
    type Keys: Clone + PartialEq + Send + Sync + 'static;

    fn to_keys(&self) -> Self::Keys;
    /// Keys of no value of the type are dropped (with a warning in debug builds).
    fn from_keys(keys: &Self::Keys) -> Self;
}

/// The value of `key`; a key of no value (an item's `value` that doesn't match the group's type,
/// e.g. `value="small"` in a `RadioGroup<Size>` whose keys are `"s"`, `"m"`, ...) warns in debug
/// builds, as the app's state silently misses the selection otherwise.
fn value_of<V: SelectionValue>(key: &Key) -> Option<V> {
    let value = V::from_key(key);
    if value.is_none() {
        dev_warn!(
            "the key {key:?} is no value of the group's type `{}` (`SelectionValue::from_key`); \
             the selection doesn't reach the app",
            std::any::type_name::<V>()
        );
    }
    value
}

impl<V: SelectionValue> Keyed for Option<V> {
    type Keys = Option<Key>;

    fn to_keys(&self) -> Option<Key> {
        self.as_ref().map(V::to_key)
    }

    fn from_keys(keys: &Option<Key>) -> Self {
        keys.as_ref().and_then(value_of)
    }
}

impl<V: SelectionValue> Keyed for HashSet<V> {
    type Keys = HashSet<Key>;

    fn to_keys(&self) -> HashSet<Key> {
        self.iter().map(V::to_key).collect()
    }

    fn from_keys(keys: &HashSet<Key>) -> Self {
        keys.iter().filter_map(value_of).collect()
    }
}

mod sealed {
    pub trait Sealed {}
}

/// The value of a [`Select`](super::select::Select) or [`ComboBox`](super::combobox::ComboBox),
/// whose shape is the selection mode: `Option<V>` selects one value, `HashSet<V>` any number.
pub trait SelectedValues: Clone + Default + Send + Sync + 'static + sealed::Sealed {
    /// The type of one selected value.
    type Value: SelectionValue;
    /// The selection mode this shape stands for.
    const MODE: SelectMode;

    /// The selected values' keys.
    fn to_key_list(&self) -> Vec<Key>;
    /// The values of `keys` (keys of no value of the type are dropped, with a warning in debug
    /// builds).
    fn from_key_list(keys: &[Key]) -> Self;
}

impl<V: SelectionValue> sealed::Sealed for Option<V> {}

impl<V: SelectionValue> SelectedValues for Option<V> {
    type Value = V;
    const MODE: SelectMode = SelectMode::Single;

    fn to_key_list(&self) -> Vec<Key> {
        self.iter().map(V::to_key).collect()
    }

    fn from_key_list(keys: &[Key]) -> Self {
        keys.first().and_then(value_of)
    }
}

impl<V: SelectionValue, S> sealed::Sealed for HashSet<V, S> {}

impl<V: SelectionValue, S: BuildHasher + Clone + Default + Send + Sync + 'static> SelectedValues
    for HashSet<V, S>
{
    type Value = V;
    const MODE: SelectMode = SelectMode::Multiple;

    fn to_key_list(&self) -> Vec<Key> {
        self.iter().map(V::to_key).collect()
    }

    fn from_key_list(keys: &[Key]) -> Self {
        keys.iter().filter_map(value_of).collect()
    }
}

/// The typed state props of an atom, as the key-based ones (`K`) its hooks take.
pub(crate) struct KeyedStateProps<K: Send + Sync + 'static> {
    pub default_value: Option<K>,
    pub value: Option<Signal<K>>,
    pub set_value: Option<Out<K>>,
    pub on_change: Option<Callback<K>>,
    pub validate: Option<ValidateFn<K>>,
}

/// Converts an atom's typed state props (C4) and validator to their key-based form.
pub(crate) fn keyed_state_props<T: Keyed>(
    default_value: Option<T>,
    value: Option<Signal<T>>,
    set_value: Option<Out<T>>,
    on_change: Option<Callback<T>>,
    validate: Option<ValidateFn<T>>,
) -> KeyedStateProps<T::Keys> {
    convert_state_props(
        StateProps {
            default_value,
            value,
            set_value,
            on_change,
            validate,
        },
        T::to_keys,
        T::from_keys,
    )
}

/// [`keyed_state_props`] for a [`SelectedValues`] shape: the hooks take a list of keys.
pub(crate) fn selected_state_props<S: SelectedValues>(
    default_value: Option<S>,
    value: Option<Signal<S>>,
    set_value: Option<Out<S>>,
    on_change: Option<Callback<S>>,
    validate: Option<ValidateFn<S>>,
) -> KeyedStateProps<Vec<Key>> {
    convert_state_props(
        StateProps {
            default_value,
            value,
            set_value,
            on_change,
            validate,
        },
        S::to_key_list,
        |keys: &Vec<Key>| S::from_key_list(keys),
    )
}

/// An atom's typed state props.
struct StateProps<T: Send + Sync + 'static> {
    default_value: Option<T>,
    value: Option<Signal<T>>,
    set_value: Option<Out<T>>,
    on_change: Option<Callback<T>>,
    validate: Option<ValidateFn<T>>,
}

fn convert_state_props<T, K>(
    props: StateProps<T>,
    to_keys: fn(&T) -> K,
    from_keys: fn(&K) -> T,
) -> KeyedStateProps<K>
where
    T: Send + Sync + 'static,
    K: Clone + PartialEq + Send + Sync + 'static,
{
    let StateProps {
        default_value,
        value,
        set_value,
        on_change,
        validate,
    } = props;
    KeyedStateProps {
        default_value: default_value.as_ref().map(to_keys),
        // A memo: converted once per change, however often the keys are read.
        value: value.map(|value| Memo::new(move |_| value.with(to_keys)).into()),
        set_value: set_value.map(|set_value| {
            let set_value = Arc::new(set_value);
            Out::from(move |keys: K| set_value.set(from_keys(&keys)))
        }),
        on_change: on_change
            .map(|on_change| Callback::new(move |keys: K| on_change.run(from_keys(&keys)))),
        validate: validate.map(|validate| -> ValidateFn<K> {
            Arc::new(move |keys: &K| validate(&from_keys(keys)))
        }),
    }
}

#[cfg(test)]
mod tests {
    use assertr::prelude::*;

    use super::*;

    #[test]
    fn the_shape_is_the_selection_mode() {
        assert_that!(<Option<u32> as SelectedValues>::MODE).is_equal_to(SelectMode::Single);
        assert_that!(<HashSet<u32> as SelectedValues>::MODE).is_equal_to(SelectMode::Multiple);
    }

    #[test]
    fn selected_values_convert_to_keys_and_back() {
        let keys = vec![Key::from(25), Key::from(10)];
        assert_that!(Some(25_u32).to_key_list()).is_equal_to(vec![Key::from(25)]);
        assert_that!(None::<u32>.to_key_list()).is_equal_to(Vec::<Key>::new());
        assert_that!(<Option<u32>>::from_key_list(&keys)).is_equal_to(Some(25));
        assert_that!(<Option<u32>>::from_key_list(&[])).is_none();
        // Keys of no `u32` are dropped.
        assert_that!(<HashSet<u32>>::from_key_list(&[
            Key::from("x"),
            keys[0].clone(),
            keys[1].clone()
        ]))
        .is_equal_to(HashSet::from([25, 10]));
    }
}
