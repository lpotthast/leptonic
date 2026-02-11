use leptos::prelude::*;
use std::collections::HashSet;
use std::hash::Hash;

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/@react-stately/selection/src/useMultipleSelectionState.ts

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// No intentional deviations from the react-aria implementation.
//
// =============================================================================

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

/// Represents what is currently selected.
#[derive(Debug, Clone)]
pub enum Selection<K: Hash + Eq> {
    /// A set of specific keys are selected.
    Keys(HashSet<K>),
    /// All items are selected.
    All,
}

impl<K: Hash + Eq> PartialEq for Selection<K> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Selection::Keys(a), Selection::Keys(b)) => a == b,
            (Selection::All, Selection::All) => true,
            _ => false,
        }
    }
}

impl<K: Hash + Eq> Eq for Selection<K> {}

impl<K: Hash + Eq> Default for Selection<K> {
    fn default() -> Self {
        Self::Keys(HashSet::new())
    }
}

impl<K: Hash + Eq + Clone> Selection<K> {
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
            Selection::Keys(keys) => keys.clone(),
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
}

/// Input parameters for the `use_selection_state` hook.
#[derive(Clone)]
pub struct UseSelectionStateInput<K>
where
    K: Hash + Eq + Clone + Send + Sync + 'static,
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
}

impl<K: Hash + Eq + Clone + Send + Sync + 'static> Default for UseSelectionStateInput<K> {
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
        }
    }
}

/// The return value of the `use_selection_state` hook.
#[derive(Clone)]
pub struct UseSelectionStateReturn<K>
where
    K: Hash + Eq + Clone + Send + Sync + 'static,
{
    /// The current selection.
    pub selected_keys: Signal<Selection<K>>,

    /// Check (non-reactively) if a specific key is selected.
    pub is_selected: Callback<K, bool>,

    /// Select a key.
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

    /// The selection mode.
    pub selection_mode: SelectionMode,

    /// Whether selection is disabled.
    pub is_disabled: Signal<bool>,
}

// Manual Copy impl to avoid the derive macro adding an unnecessary `K: Copy` bound.
impl<K: Hash + Eq + Clone + Send + Sync + 'static> Copy for UseSelectionStateReturn<K> {}

impl<K: Hash + Eq + Clone + Send + Sync + 'static> UseSelectionStateReturn<K> {
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
    K: Hash + Eq + Clone + Send + Sync + 'static,
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
    } = input;

    // Create internal state if uncontrolled
    let (internal_selection, set_internal_selection) =
        signal(default_selected_keys.unwrap_or_else(|| Selection::Keys(HashSet::new())));

    // Use controlled or internal state
    let selected_keys = selected_keys.unwrap_or_else(|| internal_selection.into());

    let disallow_empty = disallow_empty_selection;

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
                        Selection::Keys(HashSet::new())
                    }
                    _ => {
                        // Select this item
                        Selection::Keys([key].into_iter().collect())
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

        update_selection(Selection::Keys(HashSet::new()));
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

    UseSelectionStateReturn {
        selected_keys,
        is_selected,
        select,
        toggle,
        deselect,
        select_all,
        clear_selection,
        replace_selection,
        selection_mode,
        is_disabled: disabled,
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
        let empty: Selection<&str> = Selection::Keys(HashSet::new());
        assert!(empty.is_empty());

        let not_empty: Selection<&str> = Selection::Keys(["a"].into_iter().collect());
        assert!(!not_empty.is_empty());

        let all: Selection<&str> = Selection::All;
        assert!(!all.is_empty());
    }
}
