use leptos::ev;
use leptos::ev::{On, SharedEventCallback};
use leptos::prelude::*;
use std::collections::HashSet;
use std::hash::Hash;
use web_sys::KeyboardEvent;

use crate::utils::EventHandler;

use super::use_selectable_collection::{
    use_selectable_collection, FocusStrategy, UseSelectableCollectionInput,
    UseSelectableCollectionReturn,
};
use super::use_selection_state::{Selection, SelectionBehavior, SelectionMode};

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/@react-aria/selection/src/useSelectableList.ts

/// Input parameters for the `use_selectable_list` hook.
#[derive(Clone)]
pub struct UseSelectableListInput<K>
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

    /// All available keys in the list.
    pub all_keys: Signal<Vec<K>>,

    /// Whether keyboard navigation should wrap.
    pub should_focus_wrap: bool,

    /// Focus strategy signal. When this becomes Some(strategy), focus moves accordingly.
    /// This is reactive - whenever the signal changes to Some, focus will be applied.
    pub auto_focus: Signal<Option<FocusStrategy>>,

    /// Whether to select items on focus.
    pub select_on_focus: bool,
}

impl<K: Hash + Eq + Clone + Send + Sync + 'static> Default for UseSelectableListInput<K> {
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
            select_on_focus: false,
        }
    }
}

/// The return value of the `use_selectable_list` hook.
#[derive(Clone)]
pub struct UseSelectableListReturn<K>
where
    K: Hash + Eq + Clone + Send + Sync + 'static,
{
    /// The collection state with selection and focus management.
    pub collection: UseSelectableCollectionReturn<K>,

    /// Props for the list container element. Call `.to_attrs()` or `.into_attrs()` for view spreading.
    pub list_props: UseSelectableListProps,

    /// The keyboard event handler callback. Can be called directly to delegate keyboard handling.
    pub on_keydown: Callback<KeyboardEvent>,
}

/// Props from `use_selectable_list` that can be extracted and merged programmatically.
#[derive(Debug, Clone)]
pub struct UseSelectableListProps {
    pub on_keydown: EventHandler<KeyboardEvent>,
}

impl UseSelectableListProps {
    /// Convert to spreadable attributes for Leptos views, cloning internally.
    #[must_use]
    pub fn to_attrs(&self) -> UseSelectableListAttrs {
        self.clone().into_attrs()
    }

    /// Convert to spreadable attributes for Leptos views, consuming self.
    #[must_use]
    pub fn into_attrs(self) -> UseSelectableListAttrs {
        (self.on_keydown.into_on(ev::keydown),)
    }
}

/// Attributes for the list container element.
pub type UseSelectableListAttrs = (On<ev::keydown, SharedEventCallback<KeyboardEvent>>,);

/// Manages keyboard navigation and selection for a list component.
///
/// This hook builds on `use_selectable_collection` to add keyboard event
/// handling for arrow key navigation and selection.
///
/// # Example
///
/// ```ignore
/// let items = vec!["apple", "banana", "cherry"];
/// let all_keys = Signal::derive(move || items.iter().map(|s| s.to_string()).collect());
///
/// let list = use_selectable_list(UseSelectableListInput {
///     selection_mode: SelectionMode::Single,
///     all_keys,
///     ..Default::default()
/// });
///
/// view! {
///     <ul role="listbox" {..list.list_props.into_attrs()}>
///         // Items here
///     </ul>
/// }
/// ```
#[allow(clippy::too_many_lines)]
pub fn use_selectable_list<K>(input: UseSelectableListInput<K>) -> UseSelectableListReturn<K>
where
    K: Hash + Eq + Clone + Send + Sync + 'static,
{
    let UseSelectableListInput {
        selection_mode,
        selection_behavior,
        disabled,
        selected_keys,
        default_selected_keys,
        on_selection_change,
        disabled_keys,
        disallow_empty_selection,
        all_keys,
        should_focus_wrap,
        auto_focus,
        select_on_focus,
    } = input;

    // Create collection state
    let collection = use_selectable_collection(UseSelectableCollectionInput {
        selection_mode,
        selection_behavior,
        disabled,
        selected_keys,
        default_selected_keys,
        on_selection_change,
        disabled_keys,
        disallow_empty_selection,
        all_keys,
        should_focus_wrap,
        auto_focus,
    });

    // Extract individual callbacks for the keyboard handler
    let focus_next = collection.focus_next;
    let focus_previous = collection.focus_previous;
    let focus_first = collection.focus_first;
    let focus_last = collection.focus_last;
    let focused_key = collection.focused_key;

    // Copy the selection callbacks
    let select_cb = collection.selection_state.select;
    let toggle_cb = collection.selection_state.toggle;
    let select_all_cb = collection.selection_state.select_all;
    let clear_selection_cb = collection.selection_state.clear_selection;

    // Handle keyboard navigation - exposed as a callback for delegation
    let on_keydown = Callback::new(move |e: KeyboardEvent| {
        if disabled.get_untracked() {
            return;
        }

        let key = e.key();

        match key.as_str() {
            "ArrowDown" | "ArrowRight" => {
                e.prevent_default();
                focus_next.run(());

                if select_on_focus && selection_mode == SelectionMode::Single {
                    if let Some(focused) = focused_key.get_untracked() {
                        select_cb.run(focused);
                    }
                }
            }
            "ArrowUp" | "ArrowLeft" => {
                e.prevent_default();
                focus_previous.run(());

                if select_on_focus && selection_mode == SelectionMode::Single {
                    if let Some(focused) = focused_key.get_untracked() {
                        select_cb.run(focused);
                    }
                }
            }
            "Home" => {
                e.prevent_default();
                focus_first.run(());

                if select_on_focus && selection_mode == SelectionMode::Single {
                    if let Some(focused) = focused_key.get_untracked() {
                        select_cb.run(focused);
                    }
                }
            }
            "End" => {
                e.prevent_default();
                focus_last.run(());

                if select_on_focus && selection_mode == SelectionMode::Single {
                    if let Some(focused) = focused_key.get_untracked() {
                        select_cb.run(focused);
                    }
                }
            }
            " " | "Enter" => {
                e.prevent_default();
                if let Some(focused) = focused_key.get_untracked() {
                    match selection_behavior {
                        SelectionBehavior::Toggle => {
                            toggle_cb.run(focused);
                        }
                        SelectionBehavior::Replace => {
                            select_cb.run(focused);
                        }
                    }
                }
            }
            "a" if e.ctrl_key() || e.meta_key() => {
                // Select all (for multiple selection)
                if selection_mode == SelectionMode::Multiple {
                    e.prevent_default();
                    select_all_cb.run(Vec::new());
                }
            }
            "Escape" => {
                // Clear selection
                e.prevent_default();
                clear_selection_cb.run(());
            }
            _ => {}
        }
    });

    // Create event handler for list_props using the callback
    let handle_keydown = move |e: KeyboardEvent| {
        on_keydown.run(e);
    };

    UseSelectableListReturn {
        collection,
        list_props: UseSelectableListProps {
            on_keydown: EventHandler::new(handle_keydown),
        },
        on_keydown,
    }
}
