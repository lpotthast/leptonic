// Upstream: react-stately/src/selection/Selection.ts @ 99e6102368
use std::collections::HashSet;

use super::Key;

/// The type of selection allowed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SelectionMode {
    /// No selection allowed (react-aria's default).
    #[default]
    None,
    /// Only one item can be selected at a time.
    Single,
    /// Multiple items can be selected.
    Multiple,
}

/// The selection behavior.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SelectionBehavior {
    /// Clicking an item toggles its selection state.
    #[default]
    Toggle,
    /// Clicking an item replaces the entire selection with that item.
    Replace,
}

/// Controls how disabled items behave in a collection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DisabledBehavior {
    /// Disabled items cannot be focused, selected, or interacted with.
    #[default]
    All,
    /// Disabled items can be focused and have actions, but cannot be selected.
    Selection,
}

/// Focus strategy when auto-focusing items.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FocusStrategy {
    /// Focus the first item.
    #[default]
    First,
    /// Focus the last item.
    Last,
}

/// The selected items of a collection.
///
/// react-aria represents this as `'all' | Set<Key>`. Here it is an enum: [`Selection::All`]
/// stands for "everything that can be selected", including items that aren't loaded or rendered,
/// so selecting all doesn't require knowing every key.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Selection {
    /// Every selectable item.
    All,
    /// The given keys.
    Keys(SelectedKeys),
}

impl Default for Selection {
    fn default() -> Self {
        Self::Keys(SelectedKeys::default())
    }
}

impl Selection {
    /// A selection of exactly `keys`.
    pub fn keys(keys: impl IntoIterator<Item = Key>) -> Self {
        Self::Keys(keys.into_iter().collect())
    }

    /// Whether nothing is selected.
    pub fn is_empty(&self) -> bool {
        matches!(self, Self::Keys(keys) if keys.is_empty())
    }
}

/// An explicit set of selected keys, plus the keys that range selection (Shift+click,
/// Shift+Arrow) extends from (`anchor`) and to (`current`).
///
/// Equality only compares the selected keys: the anchor is navigation state, not part of what is
/// selected.
#[derive(Debug, Clone, Default)]
pub struct SelectedKeys {
    keys: HashSet<Key>,
    anchor: Option<Key>,
    current: Option<Key>,
}

impl SelectedKeys {
    pub fn contains(&self, key: &Key) -> bool {
        self.keys.contains(key)
    }

    pub fn len(&self) -> usize {
        self.keys.len()
    }

    pub fn is_empty(&self) -> bool {
        self.keys.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = &Key> {
        self.keys.iter()
    }

    /// Where range selection starts.
    pub fn anchor(&self) -> Option<&Key> {
        self.anchor.as_ref()
    }

    /// Where the last range selection ended.
    pub fn current(&self) -> Option<&Key> {
        self.current.as_ref()
    }

    pub(crate) fn insert(&mut self, key: Key) {
        self.keys.insert(key);
    }

    pub(crate) fn remove(&mut self, key: &Key) {
        self.keys.remove(key);
    }

    pub(crate) fn with_range(mut self, anchor: Option<Key>, current: Option<Key>) -> Self {
        self.anchor = anchor;
        self.current = current;
        self
    }
}

impl PartialEq for SelectedKeys {
    fn eq(&self, other: &Self) -> bool {
        self.keys == other.keys
    }
}

impl Eq for SelectedKeys {}

impl FromIterator<Key> for SelectedKeys {
    fn from_iter<I: IntoIterator<Item = Key>>(iter: I) -> Self {
        Self {
            keys: iter.into_iter().collect(),
            anchor: None,
            current: None,
        }
    }
}

impl IntoIterator for SelectedKeys {
    type Item = Key;
    type IntoIter = std::collections::hash_set::IntoIter<Key>;

    fn into_iter(self) -> Self::IntoIter {
        self.keys.into_iter()
    }
}
