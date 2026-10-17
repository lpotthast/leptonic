// No upstream: reading one node of a collection built from application data (react-aria renders
// a collection's nodes from its children, so each node's renderer has it already).
//! One node of a collection, for the renderer of its content (`use_node`, `use_node_aria_label`).

use leptos::prelude::*;

use super::{Key, Node, SelectionManager};

/// Input of [`use_node`].
#[derive(Debug, Clone)]
pub struct UseNodeInput {
    /// The selection manager of the collection (`ListState::selection`, ...).
    pub selection: SelectionManager,
    /// The node's key.
    pub key: Key,
}

/// The node `key` of a collection, for the renderer of the node's content: it notifies when the
/// node's content changes (its text, its value, ...), not when other nodes come, go or move
/// ([`Node::same_content`]), and is `None` while the collection has no such node.
///
/// Render a collection's items with a keyed `<For>` over its keys, and each item's content from
/// its node: adding, removing or filtering other items then renders no item again. The list box,
/// menu and tag group atoms render their items this way.
///
/// ```ignore
/// use leptonic::hooks::collections::{UseNodeInput, use_node};
///
/// let node = use_node(UseNodeInput { selection: state.selection, key: key.clone() });
/// view! { <li>{move || node.get().map(|node| node.text_value.to_string())}</li> }
/// ```
pub fn use_node(input: UseNodeInput) -> Memo<Option<Node>> {
    crate::metrics::subject!();
    let UseNodeInput { selection, key } = input;
    let collection = selection.collection();
    Memo::new_with_compare(
        move |_| {
            // Only this node's changes reach it, not every change of the collection.
            selection.track_node(&key);
            collection.with_untracked(|c| c.get(&key).cloned())
        },
        |previous: Option<&Option<Node>>, current: Option<&Option<Node>>| match (previous, current)
        {
            (Some(Some(previous)), Some(Some(current))) => !previous.same_content(current),
            (previous, current) => previous != current,
        },
    )
}

/// Input of [`use_node_aria_label`].
#[derive(Debug, Clone)]
pub struct UseNodeAriaLabelInput {
    /// The selection manager of the collection (`ListState::selection`, ...).
    pub selection: SelectionManager,
    /// The node's key.
    pub key: Key,
}

/// The `aria_label` of the node `key` of a collection (`ItemBuilder::aria_label`), following
/// changes of the collection: an item keeps its rendered element when its label changes. It
/// notifies only when this node changes, not on every change of the collection.
///
/// Item and section hooks (`use_option`, `use_menu_item`, `use_listbox_section`, ...) label
/// their elements with it; use it in an item hook of your own.
///
/// ```ignore
/// use leptonic::hooks::collections::{UseNodeAriaLabelInput, use_node_aria_label};
///
/// let aria_label = use_node_aria_label(UseNodeAriaLabelInput { selection, key: key.clone() });
/// view! { <li aria-label=move || aria_label.get()>"…"</li> }
/// ```
pub fn use_node_aria_label(input: UseNodeAriaLabelInput) -> Memo<Option<String>> {
    crate::metrics::subject!();
    let UseNodeAriaLabelInput { selection, key } = input;
    let collection = selection.collection();
    Memo::new(move |_| {
        // Only this node's changes reach it, not every change of the collection.
        selection.track_node(&key);
        collection.with_untracked(|c| {
            c.get(&key)
                .and_then(|n| n.aria_label.as_deref().map(str::to_owned))
        })
    })
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use assertr::prelude::*;

    use super::*;
    use crate::{
        hooks::collections::{Collection, CollectionMemo, SelectionOptions},
        testing::with_owner,
    };

    /// A collection of `items` (key and text), with the label of each item its text in capitals.
    fn collection(items: RwSignal<Vec<(&'static str, &'static str)>>) -> CollectionMemo {
        Memo::new(move |_| {
            let items = items.get();
            Arc::new(Collection::build(|b| {
                for (key, text) in &items {
                    let _ = b.item(*key, *text).aria_label(text.to_uppercase());
                }
            }))
        })
    }

    #[test]
    fn a_node_notifies_when_its_content_changes_not_when_other_nodes_do() {
        with_owner(|| {
            let items = RwSignal::new(vec![("a", "Apple"), ("b", "Banana")]);
            let selection = SelectionManager::new(collection(items), SelectionOptions::default());
            let node = use_node(UseNodeInput {
                selection,
                key: Key::from("a"),
            });
            let runs = StoredValue::new(0);
            let text = Memo::new(move |_| {
                runs.update_value(|r| *r += 1);
                node.with(|n| n.as_ref().map(|n| n.text_value.to_string()))
            });
            assert_that!(text.get_untracked()).is_equal_to(Some("Apple".to_owned()));

            // Another node comes, and this one moves: no notification.
            items.set(vec![("c", "Cherry"), ("a", "Apple"), ("b", "Banana")]);
            assert_that!(text.get_untracked()).is_equal_to(Some("Apple".to_owned()));
            assert_that!(runs.get_value()).is_equal_to(1);

            items.set(vec![("c", "Cherry"), ("a", "Apricot")]);
            assert_that!(text.get_untracked()).is_equal_to(Some("Apricot".to_owned()));
            items.set(vec![("c", "Cherry")]);
            assert_that!(text.get_untracked()).is_none();
        });
    }

    #[test]
    fn an_aria_label_follows_its_node() {
        with_owner(|| {
            let items = RwSignal::new(vec![("a", "Apple"), ("b", "Banana")]);
            let selection = SelectionManager::new(collection(items), SelectionOptions::default());
            let label = use_node_aria_label(UseNodeAriaLabelInput {
                selection,
                key: Key::from("a"),
            });
            assert_that!(label.get_untracked()).is_equal_to(Some("APPLE".to_owned()));
            items.set(vec![("a", "Apricot")]);
            assert_that!(label.get_untracked()).is_equal_to(Some("APRICOT".to_owned()));
            items.set(Vec::new());
            assert_that!(label.get_untracked()).is_none();
        });
    }
}
