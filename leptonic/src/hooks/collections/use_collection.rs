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

/// Build a flat collection (no sections) from a list of values.
///
/// `key` identifies each value, `text_value` describes it in plain text (for type-ahead and
/// filtering).
pub fn use_list_collection<T: Send + Sync + 'static>(
    items: Signal<Vec<T>>,
    key: impl Fn(&T) -> Key + Send + Sync + 'static,
    text_value: impl Fn(&T) -> String + Send + Sync + 'static,
) -> CollectionMemo {
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
        let owner = Owner::new();
        owner.with(|| {
            let items = RwSignal::new(vec!["Apple".to_owned(), "Banana".to_owned()]);
            let collection = use_list_collection(
                items.into(),
                |s: &String| Key::from(s.as_str()),
                Clone::clone,
            );
            assert_that!(collection.with_untracked(|c| c.size())).is_equal_to(2);
            items.update(|items| items.push("Cherry".to_owned()));
            assert_that!(collection.with_untracked(|c| c.size())).is_equal_to(3);
        });
    }
}
