use std::{
    collections::HashSet,
    ops::{Deref, DerefMut},
};

use leptos::prelude::*;

use super::SelectionKey;

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/@react-stately/selection/src/useMultipleSelectionState.ts

// REACT-ARIA DEVIATIONS
//
// No intentional deviations from the react-aria implementation.

/// The type of selection allowed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SelectionMode {
    /// No selection allowed.
    None,
    /// Only one item can be selected at a time.
    #[default]
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
///
/// Mirrors react-aria's `DisabledBehavior` type from `@react-types/shared`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DisabledBehavior {
    /// Disabled items cannot be focused, selected, or interacted with.
    #[default]
    All,
    /// Disabled items can be focused and have actions, but cannot be selected.
    Selection,
}

/// Focus strategy when auto-focusing items.
///
/// Mirrors react-aria's `FocusStrategy` type used by `SelectionManager.setFocusedKey`
/// and `autoFocus` options.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FocusStrategy {
    /// Focus the first item.
    #[default]
    First,
    /// Focus the last item.
    Last,
}

/// A set of selected keys with anchor/current tracking for range selection.
///
/// Mirrors react-aria's `Selection` class which extends `Set<Key>` with
/// `anchorKey` and `currentKey` fields used by `extendSelection` for
/// Shift+Arrow/Click range selection.
///
/// Implements `Deref<Target=HashSet<K>>` and `DerefMut` so that all `HashSet`
/// methods (`.contains()`, `.insert()`, `.remove()`, `.len()`, `.iter()`, etc.)
/// work directly on `SelectionSet<K>`.
#[derive(Debug, Clone)]
pub struct SelectionSet<K: SelectionKey> {
    /// The selected keys.
    pub keys: HashSet<K>,
    /// The key where range selection started (the fixed end of a Shift+Arrow range).
    pub anchor_key: Option<K>,
    /// The key where range selection currently extends to (the moving end).
    pub current_key: Option<K>,
}

impl<K: SelectionKey> SelectionSet<K> {
    /// Create an empty selection set.
    pub fn new() -> Self {
        Self {
            keys: HashSet::new(),
            anchor_key: None,
            current_key: None,
        }
    }

    /// Create a selection set containing a single key, with anchor and current set to that key.
    pub fn from_single(key: K) -> Self {
        Self {
            keys: [key.clone()].into_iter().collect(),
            anchor_key: Some(key.clone()),
            current_key: Some(key),
        }
    }

    /// Create a selection set with explicit anchor/current keys.
    pub fn with_anchor_current(
        keys: HashSet<K>,
        anchor_key: Option<K>,
        current_key: Option<K>,
    ) -> Self {
        Self {
            keys,
            anchor_key,
            current_key,
        }
    }
}

impl<K: SelectionKey> Default for SelectionSet<K> {
    fn default() -> Self {
        Self::new()
    }
}

impl<K: SelectionKey> From<HashSet<K>> for SelectionSet<K> {
    fn from(keys: HashSet<K>) -> Self {
        Self {
            keys,
            anchor_key: None,
            current_key: None,
        }
    }
}

impl<K: SelectionKey> FromIterator<K> for SelectionSet<K> {
    fn from_iter<I: IntoIterator<Item = K>>(iter: I) -> Self {
        Self {
            keys: iter.into_iter().collect(),
            anchor_key: None,
            current_key: None,
        }
    }
}

impl<K: SelectionKey> IntoIterator for SelectionSet<K> {
    type Item = K;
    type IntoIter = std::collections::hash_set::IntoIter<K>;

    fn into_iter(self) -> Self::IntoIter {
        self.keys.into_iter()
    }
}

impl<'a, K: SelectionKey> IntoIterator for &'a SelectionSet<K> {
    type Item = &'a K;
    type IntoIter = std::collections::hash_set::Iter<'a, K>;

    fn into_iter(self) -> Self::IntoIter {
        self.keys.iter()
    }
}

impl<K: SelectionKey> Deref for SelectionSet<K> {
    type Target = HashSet<K>;

    fn deref(&self) -> &Self::Target {
        &self.keys
    }
}

impl<K: SelectionKey> DerefMut for SelectionSet<K> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.keys
    }
}

impl<K: SelectionKey> PartialEq for SelectionSet<K> {
    fn eq(&self, other: &Self) -> bool {
        self.keys == other.keys
    }
}

impl<K: SelectionKey> Eq for SelectionSet<K> {}

/// Represents what is currently selected.
#[derive(Debug, Clone)]
pub enum Selection<K: SelectionKey> {
    /// A set of specific keys are selected, with optional range selection tracking.
    Keys(SelectionSet<K>),
    /// All items are selected.
    All,
}

impl<K: SelectionKey> PartialEq for Selection<K> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Selection::Keys(a), Selection::Keys(b)) => a == b,
            (Selection::All, Selection::All) => true,
            _ => false,
        }
    }
}

impl<K: SelectionKey> Eq for Selection<K> {}

impl<K: SelectionKey> Default for Selection<K> {
    fn default() -> Self {
        Self::Keys(SelectionSet::new())
    }
}

impl<K: SelectionKey> Selection<K> {
    /// Check if a specific key is selected.
    pub fn contains(&self, key: &K, all_keys: &[K]) -> bool {
        match self {
            Selection::Keys(keys) => keys.contains(key),
            Selection::All => all_keys.contains(key),
        }
    }

    /// Get the selected keys as a set.
    pub fn selected_keys(&self, all_keys: &[K]) -> HashSet<K> {
        match self {
            Selection::Keys(keys) => keys.keys.clone(),
            Selection::All => all_keys.iter().cloned().collect(),
        }
    }

    /// Check if nothing is selected.
    pub fn is_empty(&self) -> bool {
        match self {
            Selection::Keys(keys) => keys.is_empty(),
            Selection::All => false,
        }
    }

    /// Get the anchor key for range selection, if any.
    pub fn anchor_key(&self) -> Option<&K> {
        match self {
            Selection::Keys(keys) => keys.anchor_key.as_ref(),
            Selection::All => None,
        }
    }

    /// Get the current key for range selection, if any.
    pub fn current_key(&self) -> Option<&K> {
        match self {
            Selection::Keys(keys) => keys.current_key.as_ref(),
            Selection::All => None,
        }
    }
}

/// Input parameters for the `use_selection_state` hook.
#[derive(Clone)]
pub struct UseSelectionStateInput<K>
where
    K: SelectionKey,
{
    /// The selection mode.
    pub selection_mode: SelectionMode,

    /// The selection behavior.
    pub selection_behavior: SelectionBehavior,

    /// Whether selection is disabled.
    pub disabled: Signal<bool>,

    /// The controlled selected keys.
    pub selected_keys: Option<Signal<Selection<K>>>,

    /// The default selected keys (uncontrolled).
    pub default_selected_keys: Option<Selection<K>>,

    /// Callback when selection changes.
    pub on_selection_change: Option<Callback<Selection<K>>>,

    /// Keys that cannot be selected.
    pub disabled_keys: Signal<HashSet<K>>,

    /// Whether to allow empty selection.
    pub disallow_empty_selection: bool,

    /// How disabled items behave in the collection.
    pub disabled_behavior: DisabledBehavior,
}

impl<K: SelectionKey> Default for UseSelectionStateInput<K> {
    fn default() -> Self {
        Self {
            selection_mode: SelectionMode::Single,
            selection_behavior: SelectionBehavior::Toggle,
            disabled: Signal::derive(|| false),
            selected_keys: None,
            default_selected_keys: None,
            on_selection_change: None,
            disabled_keys: Signal::derive(HashSet::new),
            disallow_empty_selection: false,
            disabled_behavior: DisabledBehavior::default(),
        }
    }
}

/// The return value of the `use_selection_state` hook.
///
/// Mirrors react-aria's `MultipleSelectionState` + parts of `SelectionManager`.
/// Owns focused key and is-focused state in addition to selection state.
#[derive(Clone)]
pub struct UseSelectionStateReturn<K>
where
    K: SelectionKey,
{
    /// The current selection.
    pub selected_keys: Signal<Selection<K>>,

    /// Check (non-reactively) if a specific key is selected.
    pub is_selected: Callback<K, bool>,

    /// Select a key (adds to selection in Multiple mode, replaces in Single mode).
    pub select: Callback<K>,

    /// Toggle a key's selection.
    pub toggle: Callback<K>,

    /// Deselect a key.
    pub deselect: Callback<K>,

    /// Select all keys.
    pub select_all: Callback<Vec<K>>,

    /// Clear selection.
    pub clear_selection: Callback<()>,

    /// Replace selection with specific keys.
    pub replace_selection: Callback<Selection<K>>,

    /// Replace selection with a single key, setting anchor and current to that key.
    /// Unlike `select` (which adds in Multiple mode), this always replaces the entire selection.
    pub replace_selection_single: Callback<K>,

    /// The selection mode.
    pub selection_mode: SelectionMode,

    /// The selection behavior (reactive — can be changed at runtime via `set_selection_behavior`).
    pub selection_behavior: Signal<SelectionBehavior>,

    /// Set the selection behavior at runtime (e.g., touch long press switches Replace → Toggle).
    pub set_selection_behavior: Callback<SelectionBehavior>,

    /// Whether selection is disabled.
    pub is_disabled: Signal<bool>,

    /// Whether empty selection is disallowed.
    pub disallow_empty_selection: bool,

    /// The currently focused key in the collection.
    pub focused_key: Signal<Option<K>>,

    /// Set the focused key, optionally with a child focus strategy.
    pub set_focused_key: Callback<(Option<K>, Option<FocusStrategy>)>,

    /// The child focus strategy (first or last child of a composite item).
    pub child_focus_strategy: Signal<Option<FocusStrategy>>,

    /// Whether the collection itself is focused (has focus within).
    pub is_focused: Signal<bool>,

    /// Set whether the collection is focused.
    pub set_focused: Callback<bool>,

    /// Check whether a key can be selected (not disabled, selection mode allows it).
    pub can_select_item: Callback<K, bool>,

    /// Keys that cannot be selected.
    pub disabled_keys: Signal<HashSet<K>>,

    /// How disabled items behave in the collection.
    pub disabled_behavior: DisabledBehavior,
}

// Manual Copy impl to avoid the derive macro adding an unnecessary `K: Copy` bound.
impl<K: SelectionKey> Copy for UseSelectionStateReturn<K> {}

impl<K: SelectionKey> UseSelectionStateReturn<K> {
    /// Returns a reactive `Signal<bool>` that tracks whether the given key is selected.
    ///
    /// Unlike the `is_selected` callback (which uses untracked access for imperative contexts),
    /// this method creates a derived signal that properly subscribes to selection changes.
    pub fn is_key_selected(&self, key: K) -> Signal<bool> {
        let selected_keys = self.selected_keys;
        let is_disabled = self.is_disabled;
        Signal::derive(move || {
            if is_disabled.get() {
                return false;
            }
            match selected_keys.get() {
                Selection::Keys(ref keys) => keys.contains(&key),
                Selection::All => true,
            }
        })
    }
}

/// Manages the selection state for a collection of items.
///
/// This hook provides a consistent API for managing single or multiple
/// selection across various components like lists, menus, and tables.
///
/// # Example
///
/// ```ignore
/// let selection = use_selection_state(UseSelectionStateInput {
///     selection_mode: SelectionMode::Multiple,
///     selection_behavior: SelectionBehavior::Toggle,
///     disabled: Signal::derive(|| false),
///     ..Default::default()
/// });
///
/// // Check if an item is selected
/// let is_item_selected = selection.is_selected.run("item-1".to_string());
///
/// // Toggle an item's selection
/// selection.toggle.run("item-1".to_string());
/// ```
#[allow(clippy::too_many_lines, clippy::needless_pass_by_value)]
pub fn use_selection_state<K>(input: UseSelectionStateInput<K>) -> UseSelectionStateReturn<K>
where
    K: SelectionKey,
{
    let UseSelectionStateInput {
        selection_mode,
        selection_behavior,
        disabled,
        selected_keys,
        default_selected_keys,
        on_selection_change,
        disabled_keys,
        disallow_empty_selection,
        disabled_behavior,
    } = input;

    // Create internal state if uncontrolled
    let (internal_selection, set_internal_selection) =
        signal(default_selected_keys.unwrap_or_else(|| Selection::Keys(SelectionSet::new())));

    // Use controlled or internal state
    let selected_keys = selected_keys.unwrap_or_else(|| internal_selection.into());

    let disallow_empty = disallow_empty_selection;

    // Reactive selection behavior (allows runtime switching, e.g., touch long press).
    let (selection_behavior_rw, set_selection_behavior_signal) = signal(selection_behavior);
    let selection_behavior_signal: Signal<SelectionBehavior> = selection_behavior_rw.into();

    // Focused key state (mirrors SelectionManager.focusedKey)
    let (focused_key, set_focused_key_signal) = signal::<Option<K>>(None);
    let (child_focus_strategy, set_child_focus_strategy) = signal::<Option<FocusStrategy>>(None);
    let (is_focused, set_is_focused_signal) = signal(false);

    let set_focused_key =
        Callback::new(move |(key, strategy): (Option<K>, Option<FocusStrategy>)| {
            set_focused_key_signal.set(key);
            set_child_focus_strategy.set(strategy);
        });

    let set_focused = Callback::new(move |focused: bool| {
        set_is_focused_signal.set(focused);
    });

    // Check if a key can be selected
    let can_select_item = Callback::new(move |key: K| -> bool {
        if selection_mode == SelectionMode::None {
            return false;
        }
        !disabled_keys.get_untracked().contains(&key)
    });

    // Helper to update selection
    let update_selection = move |new_selection: Selection<K>| {
        // Update internal state
        set_internal_selection.set(new_selection.clone());

        // Call callback
        if let Some(on_change) = on_selection_change {
            on_change.run(new_selection);
        }
    };

    // Check if a key is selected
    let is_selected = Callback::new(move |key: K| -> bool {
        if disabled.get_untracked() {
            return false;
        }
        match selected_keys.get_untracked() {
            Selection::Keys(keys) => keys.contains(&key),
            Selection::All => true,
        }
    });

    // Select a key
    let select = Callback::new(move |key: K| {
        if disabled.get_untracked() || selection_mode == SelectionMode::None {
            return;
        }

        // Check if key is disabled
        if disabled_keys.get_untracked().contains(&key) {
            return;
        }

        let current = selected_keys.get_untracked();

        let new_selection = match selection_mode {
            SelectionMode::None => return,
            SelectionMode::Single => Selection::Keys([key].into_iter().collect()),
            SelectionMode::Multiple => match current {
                Selection::Keys(mut keys) => {
                    keys.insert(key);
                    Selection::Keys(keys)
                }
                Selection::All => Selection::All,
            },
        };

        update_selection(new_selection);
    });

    // Toggle a key's selection
    let toggle = Callback::new(move |key: K| {
        if disabled.get_untracked() || selection_mode == SelectionMode::None {
            return;
        }

        // Check if key is disabled
        if disabled_keys.get_untracked().contains(&key) {
            return;
        }

        let current = selected_keys.get_untracked();

        let new_selection = match selection_mode {
            SelectionMode::None => return,
            SelectionMode::Single => {
                match &current {
                    Selection::Keys(keys) if keys.contains(&key) => {
                        // Already selected, toggle off (if allowed)
                        if disallow_empty {
                            return;
                        }
                        Selection::Keys(SelectionSet::new())
                    }
                    _ => {
                        // Select this item (with anchor/current set)
                        Selection::Keys(SelectionSet::from_single(key))
                    }
                }
            }
            SelectionMode::Multiple => match current {
                Selection::Keys(mut keys) => {
                    if keys.contains(&key) {
                        if disallow_empty && keys.len() == 1 {
                            return;
                        }
                        keys.remove(&key);
                    } else {
                        keys.anchor_key = Some(key.clone());
                        keys.current_key = Some(key.clone());
                        keys.insert(key);
                    }
                    Selection::Keys(keys)
                }
                Selection::All => Selection::All,
            },
        };

        update_selection(new_selection);
    });

    // Deselect a key
    let deselect = Callback::new(move |key: K| {
        if disabled.get_untracked() {
            return;
        }

        let current = selected_keys.get_untracked();

        let new_selection = match current {
            Selection::Keys(mut keys) => {
                if disallow_empty && keys.len() == 1 && keys.contains(&key) {
                    return;
                }
                keys.remove(&key);
                Selection::Keys(keys)
            }
            Selection::All => Selection::All, // Can't deselect individual items from All
        };

        update_selection(new_selection);
    });

    // Select all keys
    let select_all = Callback::new(move |_all_keys: Vec<K>| {
        if disabled.get_untracked() || selection_mode != SelectionMode::Multiple {
            return;
        }

        update_selection(Selection::All);
    });

    // Clear selection
    let clear_selection = Callback::new(move |_: ()| {
        if disabled.get_untracked() || disallow_empty {
            return;
        }

        update_selection(Selection::Keys(SelectionSet::new()));
    });

    // Replace selection
    let replace_selection = Callback::new(move |new_selection: Selection<K>| {
        if disabled.get_untracked() {
            return;
        }

        // Validate based on mode
        match selection_mode {
            SelectionMode::None => return,
            SelectionMode::Single => {
                if let Selection::Keys(ref keys) = new_selection {
                    if keys.len() > 1 {
                        return; // Single mode can't have multiple selections
                    }
                }
            }
            SelectionMode::Multiple => {}
        }

        if disallow_empty && new_selection.is_empty() {
            return;
        }

        update_selection(new_selection);
    });

    // Replace selection with a single key, setting anchor and current.
    // Mirrors react-aria's SelectionManager.replaceSelection(key).
    let replace_selection_single = Callback::new(move |key: K| {
        if disabled.get_untracked() || selection_mode == SelectionMode::None {
            return;
        }

        if disabled_keys.get_untracked().contains(&key) {
            return;
        }

        let new_selection = Selection::Keys(SelectionSet::from_single(key));
        update_selection(new_selection);
    });

    UseSelectionStateReturn {
        selected_keys,
        is_selected,
        select,
        toggle,
        deselect,
        select_all,
        clear_selection,
        replace_selection,
        replace_selection_single,
        selection_mode,
        selection_behavior: selection_behavior_signal,
        set_selection_behavior: Callback::new(move |b: SelectionBehavior| {
            set_selection_behavior_signal.set(b);
        }),
        is_disabled: disabled,
        disallow_empty_selection,
        focused_key: focused_key.into(),
        set_focused_key,
        child_focus_strategy: child_focus_strategy.into(),
        is_focused: is_focused.into(),
        set_focused,
        can_select_item,
        disabled_keys,
        disabled_behavior,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_selection_contains() {
        let all_keys = vec!["a", "b", "c"];

        let selection: Selection<&str> = Selection::Keys(["a", "b"].into_iter().collect());
        assert!(selection.contains(&"a", &all_keys));
        assert!(selection.contains(&"b", &all_keys));
        assert!(!selection.contains(&"c", &all_keys));

        let all_selection: Selection<&str> = Selection::All;
        assert!(all_selection.contains(&"a", &all_keys));
        assert!(all_selection.contains(&"b", &all_keys));
        assert!(all_selection.contains(&"c", &all_keys));
    }

    #[test]
    fn test_selection_is_empty() {
        let empty: Selection<&str> = Selection::Keys(SelectionSet::new());
        assert!(empty.is_empty());

        let not_empty: Selection<&str> = Selection::Keys(["a"].into_iter().collect());
        assert!(!not_empty.is_empty());

        let all: Selection<&str> = Selection::All;
        assert!(!all.is_empty());
    }
}
