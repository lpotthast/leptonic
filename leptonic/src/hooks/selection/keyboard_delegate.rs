use super::SelectionKey;

// This is based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/@react-types/shared/src/collections.d.ts
// Specifically the `KeyboardDelegate` interface.

// REACT-ARIA DEVIATIONS
//
// ## API DIFFERENCES
//
// - Uses generic `K` key type instead of React-aria's `Key` (string | number).
// - No `getCollator()` method — collation for search is left to implementors.

/// A delegate that provides keyboard navigation logic for collections.
///
/// Used by keyboard-navigable collections (grids, lists, trees) to abstract
/// navigation behavior. Implementations define how to move between items
/// based on keyboard input.
pub trait KeyboardDelegate<K: SelectionKey> {
    /// Returns the key visually below the given one, or `None` if none.
    fn get_key_below(&self, key: &K) -> Option<K>;

    /// Returns the key visually above the given one, or `None` if none.
    fn get_key_above(&self, key: &K) -> Option<K>;

    /// Returns the key visually to the left of the given one, or `None` if none.
    fn get_key_left_of(&self, key: &K) -> Option<K>;

    /// Returns the key visually to the right of the given one, or `None` if none.
    fn get_key_right_of(&self, key: &K) -> Option<K>;

    /// Returns the first key in the collection.
    ///
    /// If `from_key` is a cell within a row, returns the first cell in that row
    /// (unless `global` is true, which returns the absolute first key).
    fn get_first_key(&self, from_key: Option<&K>, global: bool) -> Option<K>;

    /// Returns the last key in the collection.
    ///
    /// If `from_key` is a cell within a row, returns the last cell in that row
    /// (unless `global` is true, which returns the absolute last key).
    fn get_last_key(&self, from_key: Option<&K>, global: bool) -> Option<K>;

    /// Returns the key one page above the given key, or `None`.
    fn get_key_page_above(&self, _key: &K) -> Option<K> {
        None
    }

    /// Returns the key one page below the given key, or `None`.
    fn get_key_page_below(&self, _key: &K) -> Option<K> {
        None
    }

    /// Returns the next key matching a type-ahead search string, or `None`.
    fn get_key_for_search(&self, _search: &str, _from_key: Option<&K>) -> Option<K> {
        None
    }
}
