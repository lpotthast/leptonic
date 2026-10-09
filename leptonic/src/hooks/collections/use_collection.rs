// No upstream: building collections from application data (react-aria builds them from rendered
// children); `Collection` ports react-aria's `BaseCollection`.

use std::sync::Arc;

use leptos::prelude::*;

use super::{Collection, CollectionBuilder, Key};

/// A reactive collection. Cheap to copy; dependents are only notified when the collection
/// actually changes (structural equality).
pub type CollectionMemo = Memo<Arc<Collection>>;

/// The `aria_label` of the node `key` in `collection`, following changes of the collection (an item
/// keeps its rendered element when its label changes).
pub(crate) fn use_node_aria_label(collection: CollectionMemo, key: Key) -> Memo<Option<String>> {
    Memo::new(move |_| {
        collection.with(|c| {
            c.get(&key)
                .and_then(|n| n.aria_label.as_deref().map(str::to_owned))
        })
    })
}

/// Build a collection from reactive data.
///
/// `build` runs whenever a signal it reads changes. A rebuild that produces the same collection
/// does not notify dependents.
///
/// ```ignore
/// let fruits: Signal<Vec<Fruit>> = /* ... */;
/// let collection = use_collection(move |b| {
///     for fruit in fruits.read().iter() {
///         b.item(fruit.id, fruit.name.clone()).disabled(fruit.sold_out);
///     }
/// });
/// ```
///
/// The collection is built from your data, not from rendered elements, so it exists during
/// server-side rendering too. Render items from the same data, in the same order.
pub fn use_collection(
    build: impl Fn(&mut CollectionBuilder) + Send + Sync + 'static,
) -> CollectionMemo {
    Memo::new(move |_| Arc::new(Collection::build(&build)))
}

/// Input of [`use_list_collection`].
pub struct UseListCollectionInput<T, K, Tx, S>
where
    T: Send + Sync + 'static,
    K: Fn(&T) -> Key + Send + Sync + 'static,
    Tx: Fn(&T) -> S + Send + Sync + 'static,
    S: Into<Arc<str>>,
{
    /// The values, in order.
    pub items: Signal<Vec<T>>,
    /// Identifies a value: `Fn(&T) -> Key`.
    pub key: K,
    /// Describes a value in plain text (for type-ahead and filtering): `Fn(&T) -> S`, with `S`
    /// a `String`, `&'static str`, `Arc<str>`, ...
    pub text_value: Tx,
}

/// Build a flat collection (no sections) from a list of values.
pub fn use_list_collection<T, K, Tx, S>(
    input: UseListCollectionInput<T, K, Tx, S>,
) -> CollectionMemo
where
    T: Send + Sync + 'static,
    K: Fn(&T) -> Key + Send + Sync + 'static,
    Tx: Fn(&T) -> S + Send + Sync + 'static,
    S: Into<Arc<str>>,
{
    let UseListCollectionInput {
        items,
        key,
        text_value,
    } = input;
    use_collection(move |b| {
        items.with(|items| {
            for item in items {
                b.item(key(item), text_value(item));
            }
        });
    })
}

#[cfg(test)]
mod tests {
    use assertr::prelude::*;

    use super::*;

    #[test]
    fn rebuilds_when_the_data_changes() {
        crate::testing::with_owner(|| {
            let items = RwSignal::new(vec!["Apple".to_owned(), "Banana".to_owned()]);
            let collection = use_list_collection(UseListCollectionInput {
                items: items.into(),
                key: |s: &String| Key::from(s.as_str()),
                text_value: Clone::clone,
            });
            assert_that!(collection.with_untracked(|c| c.size())).is_equal_to(2);
            items.update(|items| items.push("Cherry".to_owned()));
            assert_that!(collection.with_untracked(|c| c.size())).is_equal_to(3);
        });
    }
}
