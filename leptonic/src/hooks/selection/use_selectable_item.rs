use leptos::ev;
use leptos::ev::{On, SharedEventCallback};
use leptos::prelude::*;
use std::hash::Hash;
use web_sys::{FocusEvent, MouseEvent};

use super::use_selection_state::{Selection, SelectionBehavior, SelectionMode};
use crate::utils::EventHandler;

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/@react-aria/selection/src/useSelectableItem.ts
//
// ## DEVIATIONS FROM REACT-ARIA
//
// In react-aria's `useSelectableItem`, double-click triggers the `onAction`
// callback provided to the collection (e.g. `useListBox`). We expose it (`on_double_click`)
// directly on the item input so callers can wire it without a full collection.

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

    /// Called when the item is activated (e.g., by a double-click).
    pub on_double_click: Option<Callback<K>>,

    /// Callback to set focused key.
    pub on_focus: Callback<Option<K>>,

    /// Whether clicking should trigger action instead of selection.
    pub should_select_on_press_up: bool,

    /// Whether to allow drag operations.
    pub allow_drag: bool,

    /// Optional callback to programmatically focus this item's DOM element.
    ///
    /// When provided, an Effect is created that calls this callback each time
    /// `is_focused` transitions from `false` to `true`.
    ///
    /// For grid cells with child focus mode, this can be a custom function
    /// that focuses a child element instead of the cell itself.
    ///
    /// Mirrors react-aria's `useSelectableItem` `focus` parameter.
    pub focus: Option<Callback<()>>,
}

/// Props from `use_selectable_item` that can be extracted and merged programmatically.
#[derive(Clone)]
pub struct UseSelectableItemProps {
    pub on_click: EventHandler<MouseEvent>,
    pub on_dblclick: EventHandler<MouseEvent>,
    pub on_focus: EventHandler<FocusEvent>,
    pub on_mouseenter: EventHandler<MouseEvent>,
}

impl UseSelectableItemProps {
    /// Convert to spreadable attributes for Leptos views, cloning internally.
    #[must_use]
    pub fn to_attrs(&self) -> UseSelectableItemAttrs {
        (
            self.on_click.to_on(ev::click),
            self.on_dblclick.to_on(ev::dblclick),
            self.on_focus.to_on(ev::focus),
            self.on_mouseenter.to_on(ev::mouseenter),
        )
    }

    /// Convert to spreadable attributes for Leptos views, consuming self.
    #[must_use]
    pub fn into_attrs(self) -> UseSelectableItemAttrs {
        (
            self.on_click.into_on(ev::click),
            self.on_dblclick.into_on(ev::dblclick),
            self.on_focus.into_on(ev::focus),
            self.on_mouseenter.into_on(ev::mouseenter),
        )
    }
}

/// The return value of the `use_selectable_item` hook.
pub struct UseSelectableItemReturn {
    /// Props for the item element. Call `.to_attrs()` or `.into_attrs()` for view spreading.
    pub props: UseSelectableItemProps,

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
    On<ev::dblclick, SharedEventCallback<MouseEvent>>,
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
///         {..item.props.into_attrs()}
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
    let UseSelectableItemInput {
        key,
        selection_mode,
        selection_behavior,
        selected_keys,
        focused_key,
        is_disabled,
        on_toggle,
        on_select,
        on_double_click,
        on_focus,
        should_select_on_press_up,
        allow_drag,
        focus,
    } = input;

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

    // --- DOM focus synchronization (mirrors react-aria useSelectableItem) ---
    if let Some(focus_fn) = focus {
        Effect::new(move |prev_focused: Option<bool>| {
            let currently_focused = is_focused.get();
            let was_focused = prev_focused.unwrap_or(false);
            if currently_focused && !was_focused {
                focus_fn.run(());
            }
            currently_focused
        });
    }

    // Handle click
    let key_for_click = key.clone();
    let handle_click = move |_e: MouseEvent| {
        if is_disabled.get_untracked() {
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
        if is_disabled.get_untracked() {
            return;
        }

        on_focus.run(Some(key_for_focus.clone()));
    };

    // Handle double-click (action)
    let key_for_dblclick = key.clone();
    let handle_dblclick = move |_e: MouseEvent| {
        if is_disabled.get_untracked() {
            return;
        }
        if let Some(on_action) = on_double_click {
            on_action.run(key_for_dblclick.clone());
        }
    };

    // Handle mouse enter (for hover focus)
    let handle_mouseenter = move |_e: MouseEvent| {
        if is_disabled.get_untracked() {
            return;
        }

        on_focus.run(Some(key.clone()));
    };

    UseSelectableItemReturn {
        props: UseSelectableItemProps {
            on_click: EventHandler::new(handle_click),
            on_dblclick: EventHandler::new(handle_dblclick),
            on_focus: EventHandler::new(handle_focus),
            on_mouseenter: EventHandler::new(handle_mouseenter),
        },
        is_selected,
        is_focused,
        is_disabled,
    }
}
