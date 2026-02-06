use leptos::prelude::*;
use std::collections::HashSet;
use std::hash::Hash;

use super::use_selection_state::{
    use_selection_state, Selection, SelectionBehavior, SelectionMode, UseSelectionStateInput,
    UseSelectionStateReturn,
};

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/@react-aria/selection/src/useSelectableCollection.ts

/// Focus strategy when items change.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FocusStrategy {
    /// Focus the first item.
    #[default]
    First,
    /// Focus the last item.
    Last,
}

/// Input parameters for the `use_selectable_collection` hook.
#[derive(Clone)]
pub struct UseSelectableCollectionInput<K>
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

    /// All available keys in the collection.
    pub all_keys: Signal<Vec<K>>,

    /// Whether keyboard navigation should wrap.
    pub should_focus_wrap: bool,

    /// Focus strategy signal. When this becomes Some(strategy), focus moves accordingly.
    /// This is reactive - whenever the signal changes to Some, focus will be applied.
    pub auto_focus: Signal<Option<FocusStrategy>>,
}

impl<K: Hash + Eq + Clone + Send + Sync + 'static> Default for UseSelectableCollectionInput<K> {
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
            all_keys: Signal::derive(Vec::new),
            should_focus_wrap: true,
            auto_focus: Signal::derive(|| None),
        }
    }
}

/// The return value of the `use_selectable_collection` hook.
#[derive(Clone, Copy)]
pub struct UseSelectableCollectionReturn<K>
where
    K: Hash + Eq + Clone + Send + Sync + 'static,
{
    /// The selection state.
    pub selection_state: UseSelectionStateReturn<K>,

    /// The currently focused key.
    pub focused_key: Signal<Option<K>>,

    /// Set the focused key.
    pub set_focused_key: Callback<Option<K>>,

    /// Focus the next item.
    pub focus_next: Callback<()>,

    /// Focus the previous item.
    pub focus_previous: Callback<()>,

    /// Focus the first item.
    pub focus_first: Callback<()>,

    /// Focus the last item.
    pub focus_last: Callback<()>,

    /// Whether keyboard navigation should wrap.
    pub should_focus_wrap: bool,
}

/// Manages a collection of selectable items with keyboard navigation.
///
/// This hook combines selection state management with focus navigation,
/// providing a complete solution for list-like components.
///
/// # Example
///
/// ```ignore
/// let items = vec!["apple", "banana", "cherry"];
/// let all_keys = Signal::derive(move || items.iter().map(|s| s.to_string()).collect());
///
/// let collection = use_selectable_collection(UseSelectableCollectionInput {
///     selection_mode: SelectionMode::Single,
///     all_keys,
///     ..Default::default()
/// });
///
/// // Navigate with keyboard
/// collection.focus_next.run(());
///
/// // Get current focus
/// let focused = collection.focused_key.get();
/// ```
#[allow(clippy::too_many_lines)]
pub fn use_selectable_collection<K>(
    input: UseSelectableCollectionInput<K>,
) -> UseSelectableCollectionReturn<K>
where
    K: Hash + Eq + Clone + Send + Sync + 'static,
{
    let all_keys = input.all_keys;
    let should_focus_wrap = input.should_focus_wrap;
    let disabled_keys = input.disabled_keys;
    let disabled = input.disabled;
    let auto_focus = input.auto_focus;

    // Create selection state
    let selection_state = use_selection_state(UseSelectionStateInput {
        selection_mode: input.selection_mode,
        selection_behavior: input.selection_behavior,
        disabled: input.disabled,
        selected_keys: input.selected_keys,
        default_selected_keys: input.default_selected_keys,
        on_selection_change: input.on_selection_change,
        disabled_keys: input.disabled_keys,
        disallow_empty_selection: input.disallow_empty_selection,
    });

    // Create focused key state
    let (focused_key, set_focused_key_signal) = signal::<Option<K>>(None);

    // Set focused key callback
    let set_focused_key = Callback::new(move |key: Option<K>| {
        set_focused_key_signal.set(key);
    });

    // Get next valid key (skipping disabled)
    let get_next_key = move |current: Option<&K>, direction: i32| -> Option<K> {
        let keys = all_keys.get_untracked();
        if keys.is_empty() {
            return None;
        }

        let disabled = disabled_keys.get_untracked();

        // Find current index
        let current_idx = current
            .and_then(|k| keys.iter().position(|key| key == k))
            .unwrap_or(if direction > 0 { usize::MAX } else { 0 });

        let len = keys.len();
        let mut idx = current_idx;
        let mut checked = 0;

        loop {
            // Move in direction
            if direction > 0 {
                idx = if idx == usize::MAX {
                    0
                } else if idx + 1 >= len {
                    if should_focus_wrap {
                        0
                    } else {
                        return None;
                    }
                } else {
                    idx + 1
                };
            } else {
                idx = if idx == 0 {
                    if should_focus_wrap {
                        len - 1
                    } else {
                        return None;
                    }
                } else {
                    idx - 1
                };
            }

            checked += 1;

            // Check if valid (not disabled)
            let key = &keys[idx];
            if !disabled.contains(key) {
                return Some(key.clone());
            }

            // Prevent infinite loop
            if checked >= len {
                return None;
            }
        }
    };

    // Focus next
    let focus_next = Callback::new(move |_: ()| {
        if disabled.get_untracked() {
            return;
        }

        let current = focused_key.get_untracked();
        if let Some(next) = get_next_key(current.as_ref(), 1) {
            set_focused_key_signal.set(Some(next));
        }
    });

    // Focus previous
    let focus_previous = Callback::new(move |_: ()| {
        if disabled.get_untracked() {
            return;
        }

        let current = focused_key.get_untracked();
        if let Some(prev) = get_next_key(current.as_ref(), -1) {
            set_focused_key_signal.set(Some(prev));
        }
    });

    // Focus first
    let focus_first = Callback::new(move |_: ()| {
        if disabled.get_untracked() {
            return;
        }

        let keys = all_keys.get_untracked();
        let disabled_set = disabled_keys.get_untracked();

        for key in keys {
            if !disabled_set.contains(&key) {
                set_focused_key_signal.set(Some(key));
                return;
            }
        }
    });

    // Focus last
    let focus_last = Callback::new(move |_: ()| {
        if disabled.get_untracked() {
            return;
        }

        let keys = all_keys.get_untracked();
        let disabled_set = disabled_keys.get_untracked();

        for key in keys.into_iter().rev() {
            if !disabled_set.contains(&key) {
                set_focused_key_signal.set(Some(key));
                return;
            }
        }
    });

    // Reactive auto-focus: whenever auto_focus signal changes to Some, apply the focus strategy
    Effect::new(move |_| {
        if let Some(strategy) = auto_focus.get() {
            // Wait for items to be available
            let keys = all_keys.get();
            if keys.is_empty() {
                return;
            }

            match strategy {
                FocusStrategy::First => focus_first.run(()),
                FocusStrategy::Last => focus_last.run(()),
            }
        }
    });

    UseSelectableCollectionReturn {
        selection_state,
        focused_key: focused_key.into(),
        set_focused_key,
        focus_next,
        focus_previous,
        focus_first,
        focus_last,
        should_focus_wrap,
    }
}
