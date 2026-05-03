use std::{fmt::Display, hash::Hash};

use leptos::prelude::*;

pub mod keyboard_delegate;
pub mod list_keyboard_delegate;
pub mod use_selectable_collection;
pub mod use_selectable_item;
pub mod use_selectable_list;
pub mod use_selection_state;
pub mod use_type_select;
pub mod utils;

pub use keyboard_delegate::*;
pub use list_keyboard_delegate::*;
pub use use_selectable_collection::*;
pub use use_selectable_item::*;
pub use use_selectable_list::*;
pub use use_selection_state::*;
pub use use_type_select::*;
pub use utils::*;

// REACT-ARIA DEVIATIONS
//
// - Generic key type with Keyed trait: React-aria uses a fixed
//   `Key = string | number` union type. We use a `SelectionKey` trait
//   (`Hash + Eq + Clone + Display + Send + Sync + 'static`) for keys and a
//   `Keyed` trait for items that yield a key. Types implementing
//   `SelectionKey` are automatically self-keyed via blanket impl. Complex
//   types implement `Keyed` manually to yield a simpler key type, avoiding
//   unnecessary trait bounds on domain objects.

/// Trait alias for the bounds required by selection keys.
///
/// Used by selection state, listbox, select, grid, and similar components.
/// All types that satisfy the bounds automatically implement this trait
/// via the blanket implementation.
pub trait SelectionKey: Hash + Eq + Clone + Display + Send + Sync + 'static {}

impl<T: Hash + Eq + Clone + Display + Send + Sync + 'static> SelectionKey for T {}

/// Trait for types that can yield a [`SelectionKey`].
///
/// Items in a collection (listbox, select, grid, etc.) implement this trait
/// to provide a key for selection tracking. The key is what gets stored in
/// selection state (`HashSet`, `HashMap`), while the item itself can be any type.
///
/// # Blanket implementation
///
/// Types implementing [`SelectionKey`] are automatically self-keyed:
/// `String`, `u64`, and similar types work as both items and keys with no
/// extra code. This blanket impl provides a zero-cost [`items_to_keys`]
/// that returns the input signal directly.
///
/// # Custom implementation
///
/// Complex types that don't implement all `SelectionKey` bounds (e.g., missing
/// `Hash` or `Display`) can implement `Keyed` manually:
///
/// ```
/// use leptonic::hooks::Keyed;
///
/// struct User { id: u64, name: String }
///
/// impl Keyed for User {
///     type Key = u64;
///     fn key(&self) -> u64 { self.id }
/// }
/// ```
///
/// [`items_to_keys`]: Keyed::items_to_keys
pub trait Keyed {
    /// The key type used for selection tracking.
    type Key: SelectionKey;

    /// Extract the selection key from this item.
    fn key(&self) -> Self::Key;

    /// Convert a signal of items into a signal of keys.
    ///
    /// The default implementation maps each item via [`key()`](Keyed::key).
    /// The blanket impl for [`SelectionKey`] types overrides this to return
    /// the input signal directly (zero-cost, no allocation).
    fn items_to_keys(items: Signal<Vec<Self>>) -> Signal<Vec<Self::Key>>
    where
        Self: Sized + Clone + Send + Sync + 'static,
    {
        Signal::derive(move || items.get().iter().map(Keyed::key).collect())
    }
}

/// Blanket implementation: any [`SelectionKey`] type is self-keyed.
impl<T: SelectionKey> Keyed for T {
    type Key = T;

    fn key(&self) -> T {
        self.clone()
    }

    fn items_to_keys(items: Signal<Vec<Self>>) -> Signal<Vec<Self::Key>>
    where
        Self: Sized + Clone + Send + Sync + 'static,
    {
        // Zero-cost: Signal<Vec<T>> IS Signal<Vec<T::Key>> when Key = T.
        items
    }
}
