use leptos::attr::Attribute;
use leptos::ev;
use leptos::ev::{on, On, SharedEventCallback};
use leptos::prelude::*;
use std::hash::Hash;
use web_sys::{FocusEvent, MouseEvent};

use super::use_selection_state::{Selection, SelectionBehavior, SelectionMode};

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/@react-aria/selection/src/useSelectableItem.ts

/// Input parameters for the `use_selectable_item` hook.
#[derive(Clone, Copy)]
pub struct UseSelectableItemInput<K>
where
    K: Hash + Eq + Clone + Send + Sync + 'static,
{
    /// The key for this item.
    pub key: K,

    /// The selection mode.
    pub selection_mode: SelectionMode,

    /// The selection behavior.
    pub selection_behavior: SelectionBehavior,

    /// The current selection.
    pub selected_keys: Signal<Selection<K>>,

    /// The currently focused key.
    pub focused_key: Signal<Option<K>>,

    /// Whether this item is disabled.
    pub is_disabled: Signal<bool>,

    /// Callback to toggle selection.
    pub on_toggle: Callback<K>,

    /// Callback to select this item.
    pub on_select: Callback<K>,

    /// Callback to set focused key.
    pub on_focus: Callback<Option<K>>,

    /// Whether clicking should trigger action instead of selection.
    pub should_select_on_press_up: bool,

    /// Whether to allow drag operations.
    pub allow_drag: bool,
}

/// The return value of the `use_selectable_item` hook.
pub struct UseSelectableItemReturn {
    /// Props for the item element.
    pub item_props: UseSelectableItemAttrs,

    /// Whether this item is currently selected.
    pub is_selected: Signal<bool>,

    /// Whether this item is currently focused.
    pub is_focused: Signal<bool>,

    /// Whether this item is disabled.
    pub is_disabled: Signal<bool>,
}

/// Attributes for a selectable item element.
pub type UseSelectableItemAttrs = (
    On<ev::click, SharedEventCallback<MouseEvent>>,
    On<ev::focus, SharedEventCallback<FocusEvent>>,
    On<ev::mouseenter, SharedEventCallback<MouseEvent>>,
);

/// Manages selection and focus for an individual item in a collection.
///
/// This hook should be used for each item in a selectable collection
/// like a listbox, menu, or table row.
///
/// # Example
///
/// ```ignore
/// let item = use_selectable_item(UseSelectableItemInput {
///     key: "apple".to_string(),
///     selection_mode: SelectionMode::Single,
///     selection_behavior: SelectionBehavior::Toggle,
///     selected_keys: selection.selected_keys,
///     focused_key: collection.focused_key,
///     is_disabled: Signal::derive(|| false),
///     on_toggle: selection.toggle,
///     on_select: selection.select,
///     on_focus: collection.set_focused_key,
///     should_select_on_press_up: false,
///     allow_drag: false,
/// });
///
/// view! {
///     <li
///         role="option"
///         aria-selected=move || item.is_selected.get()
///         class:selected=move || item.is_selected.get()
///         class:focused=move || item.is_focused.get()
///         {..item.item_props}
///     >
///         "Apple"
///     </li>
/// }
/// ```
#[allow(clippy::needless_pass_by_value)]
pub fn use_selectable_item<K>(input: UseSelectableItemInput<K>) -> UseSelectableItemReturn
where
    K: Hash + Eq + Clone + Send + Sync + 'static,
{
    let key = input.key.clone();
    let selection_mode = input.selection_mode;
    let selection_behavior = input.selection_behavior;
    let selected_keys = input.selected_keys;
    let focused_key = input.focused_key;
    let is_disabled_input = input.is_disabled;
    let on_toggle = input.on_toggle;
    let on_select = input.on_select;
    let on_focus = input.on_focus;
    let should_select_on_press_up = input.should_select_on_press_up;

    // Compute whether this item is selected
    let key_for_selected = key.clone();
    let is_selected = Signal::derive(move || {
        let selection = selected_keys.get();
        match selection {
            Selection::Keys(keys) => keys.contains(&key_for_selected),
            Selection::All => true,
        }
    });

    // Compute whether this item is focused
    let key_for_focused = key.clone();
    let is_focused = Signal::derive(move || focused_key.get().as_ref() == Some(&key_for_focused));

    // Handle click
    let key_for_click = key.clone();
    let handle_click = move |_e: MouseEvent| {
        if is_disabled_input.get_untracked() {
            return;
        }

        if selection_mode == SelectionMode::None {
            return;
        }

        if should_select_on_press_up {
            return; // Will be handled on press up instead
        }

        match selection_behavior {
            SelectionBehavior::Toggle => {
                on_toggle.run(key_for_click.clone());
            }
            SelectionBehavior::Replace => {
                on_select.run(key_for_click.clone());
            }
        }
    };

    // Handle focus
    let key_for_focus = key.clone();
    let handle_focus = move |_e: FocusEvent| {
        if is_disabled_input.get_untracked() {
            return;
        }

        on_focus.run(Some(key_for_focus.clone()));
    };

    // Handle mouse enter (for hover focus)
    let key_for_hover = key.clone();
    let handle_mouseenter = move |_e: MouseEvent| {
        if is_disabled_input.get_untracked() {
            return;
        }

        on_focus.run(Some(key_for_hover.clone()));
    };

    UseSelectableItemReturn {
        item_props: (
            on(ev::click, handle_click).into_cloneable(),
            on(ev::focus, handle_focus).into_cloneable(),
            on(ev::mouseenter, handle_mouseenter).into_cloneable(),
        ),
        is_selected,
        is_focused,
        is_disabled: is_disabled_input,
    }
}
