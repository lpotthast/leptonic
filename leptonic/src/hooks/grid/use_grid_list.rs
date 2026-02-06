use leptos::attr;
use leptos::attr::{Attr, Attribute};
use leptos::ev;
use leptos::ev::{on, On, SharedEventCallback};
use leptos::prelude::*;
use uuid::Uuid;
use web_sys::{FocusEvent, KeyboardEvent, MouseEvent};

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/@react-aria/gridlist/src/useGridList.ts

/// The selection mode for grid list items.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum GridListSelectionMode {
    /// No selection allowed.
    #[default]
    None,
    /// Single item selection.
    Single,
    /// Multiple item selection.
    Multiple,
}

/// Input parameters for the `use_grid_list` hook.
#[derive(Debug, Clone)]
pub struct UseGridListInput {
    /// The label for the grid list.
    pub label: Option<String>,

    /// The selection mode.
    pub selection_mode: GridListSelectionMode,

    /// Whether the list is disabled.
    pub is_disabled: Signal<bool>,

    /// The currently selected item keys.
    pub selected_keys: Signal<Vec<String>>,

    /// Callback when selection changes.
    pub on_selection_change: Option<Callback<Vec<String>>>,

    /// Callback when an item is activated.
    pub on_action: Option<Callback<String>>,

    /// Whether to allow empty selection (deselect all).
    pub disallow_empty_selection: bool,
}

impl Default for UseGridListInput {
    fn default() -> Self {
        Self {
            label: None,
            selection_mode: GridListSelectionMode::None,
            is_disabled: Signal::derive(|| false),
            selected_keys: Signal::derive(Vec::new),
            on_selection_change: None,
            on_action: None,
            disallow_empty_selection: false,
        }
    }
}

/// The return value of the `use_grid_list` hook.
#[derive(Debug, Clone)]
pub struct UseGridListReturn {
    /// Props for the grid list container element.
    pub list_props: UseGridListAttrs,

    /// The ID of the grid list.
    pub list_id: String,

    /// The selection mode.
    pub selection_mode: GridListSelectionMode,

    /// The currently focused item key.
    pub focused_key: Signal<Option<String>>,

    /// Set the focused item key.
    pub set_focused_key: Callback<Option<String>>,

    /// Select an item.
    pub select_item: Callback<String>,

    /// Toggle item selection.
    pub toggle_item: Callback<String>,

    /// Clear selection.
    pub clear_selection: Callback<()>,
}

/// Attributes for the grid list container element.
pub type UseGridListAttrs = (
    Attr<attr::Id, String>,
    Attr<attr::Role, &'static str>,
    Attr<attr::AriaLabel, Option<String>>,
    Attr<attr::AriaMultiselectable, Option<&'static str>>,
    Attr<attr::AriaDisabled, Signal<&'static str>>,
    On<ev::keydown, SharedEventCallback<KeyboardEvent>>,
);

/// Provides the behavior and accessibility for a grid list.
///
/// A grid list is a one-dimensional list with keyboard navigation and selection,
/// but with a grid role for accessibility purposes.
///
/// # Example
///
/// ```ignore
/// let list = use_grid_list(UseGridListInput {
///     label: Some("Files".to_string()),
///     selection_mode: GridListSelectionMode::Multiple,
///     ..Default::default()
/// });
///
/// view! {
///     <ul {..list.list_props}>
///         // Grid list items...
///     </ul>
/// }
/// ```
#[allow(clippy::too_many_lines)]
pub fn use_grid_list(input: UseGridListInput) -> UseGridListReturn {
    let list_id = format!("gridlist-{}", Uuid::new_v4());
    let is_disabled = input.is_disabled;
    let selection_mode = input.selection_mode;
    let selected_keys = input.selected_keys;
    let on_selection_change = input.on_selection_change;
    let on_action = input.on_action;
    let disallow_empty_selection = input.disallow_empty_selection;

    // Track focused item
    let (focused_key, set_focused_key_signal) = signal::<Option<String>>(None);

    let set_focused_key = Callback::new(move |key: Option<String>| {
        set_focused_key_signal.set(key);
    });

    // Selection helpers
    let select_item = Callback::new(move |key: String| {
        if selection_mode == GridListSelectionMode::None {
            return;
        }

        let new_selection = if selection_mode == GridListSelectionMode::Single {
            vec![key]
        } else {
            let mut current = selected_keys.get_untracked();
            if !current.contains(&key) {
                current.push(key);
            }
            current
        };

        if let Some(on_change) = on_selection_change {
            on_change.run(new_selection);
        }
    });

    let toggle_item = Callback::new(move |key: String| {
        if selection_mode == GridListSelectionMode::None {
            return;
        }

        let mut current = selected_keys.get_untracked();
        if let Some(pos) = current.iter().position(|k| k == &key) {
            // Don't remove if it would leave empty selection and that's disallowed
            if !disallow_empty_selection || current.len() > 1 {
                current.remove(pos);
            }
        } else if selection_mode == GridListSelectionMode::Single {
            current = vec![key];
        } else {
            current.push(key);
        }

        if let Some(on_change) = on_selection_change {
            on_change.run(current);
        }
    });

    let clear_selection = Callback::new(move |_| {
        if disallow_empty_selection {
            return;
        }
        if let Some(on_change) = on_selection_change {
            on_change.run(vec![]);
        }
    });

    // Compute aria-multiselectable
    let aria_multiselectable = match selection_mode {
        GridListSelectionMode::Multiple => Some("true"),
        GridListSelectionMode::Single => Some("false"),
        GridListSelectionMode::None => None,
    };

    // Compute aria-disabled
    let aria_disabled = Signal::derive(move || if is_disabled.get() { "true" } else { "false" });

    // Handle keyboard navigation
    let handle_keydown = move |e: KeyboardEvent| {
        if is_disabled.get_untracked() {
            return;
        }

        let key = e.key();
        match key.as_str() {
            "Enter" => {
                if let Some(focused) = focused_key.get_untracked() {
                    if let Some(on_action) = on_action {
                        e.prevent_default();
                        on_action.run(focused);
                    }
                }
            }
            " " => {
                if let Some(focused) = focused_key.get_untracked() {
                    if selection_mode != GridListSelectionMode::None {
                        e.prevent_default();
                        let mut current = selected_keys.get_untracked();
                        if let Some(pos) = current.iter().position(|k| k == &focused) {
                            if !disallow_empty_selection || current.len() > 1 {
                                current.remove(pos);
                            }
                        } else if selection_mode == GridListSelectionMode::Single {
                            current = vec![focused];
                        } else {
                            current.push(focused);
                        }
                        if let Some(on_change) = on_selection_change {
                            on_change.run(current);
                        }
                    }
                }
            }
            "Escape" => {
                if selection_mode != GridListSelectionMode::None && !disallow_empty_selection {
                    e.prevent_default();
                    if let Some(on_change) = on_selection_change {
                        on_change.run(vec![]);
                    }
                }
            }
            // Arrow navigation is handled at the item level
            _ => {}
        }
    };

    UseGridListReturn {
        list_props: (
            Attr(attr::Id, list_id.clone()),
            Attr(attr::Role, "grid"),
            Attr(attr::AriaLabel, input.label),
            Attr(attr::AriaMultiselectable, aria_multiselectable),
            Attr(attr::AriaDisabled, aria_disabled),
            on(ev::keydown, handle_keydown).into_cloneable(),
        ),
        list_id,
        selection_mode,
        focused_key: focused_key.into(),
        set_focused_key,
        select_item,
        toggle_item,
        clear_selection,
    }
}

/// Input for a grid list item.
#[derive(Debug, Clone)]
pub struct UseGridListItemInput {
    /// The unique key for this item.
    pub item_key: String,

    /// The item index.
    pub index: usize,

    /// Whether the item is selected.
    pub is_selected: Signal<bool>,

    /// Whether the item is focused.
    pub is_focused: Signal<bool>,

    /// Whether the item is disabled.
    pub is_disabled: Signal<bool>,

    /// Callback when the item is activated.
    pub on_action: Option<Callback<()>>,

    /// Callback when selection changes.
    pub on_selection_change: Option<Callback<bool>>,

    /// Callback to navigate to the next item.
    pub on_focus_next: Option<Callback<()>>,

    /// Callback to navigate to the previous item.
    pub on_focus_previous: Option<Callback<()>>,
}

/// Return value for a grid list item.
#[derive(Debug, Clone)]
pub struct UseGridListItemReturn {
    /// Props for the item element.
    pub item_props: UseGridListItemAttrs,

    /// The item key.
    pub item_key: String,

    /// Whether the item is selected.
    pub is_selected: Signal<bool>,

    /// Whether the item is focused.
    pub is_focused: Signal<bool>,
}

/// Attributes for a grid list item.
pub type UseGridListItemAttrs = (
    Attr<attr::Role, &'static str>,
    Attr<attr::AriaRowindex, String>,
    Attr<attr::AriaSelected, Signal<Option<&'static str>>>,
    Attr<attr::Tabindex, Signal<&'static str>>,
    Attr<attr::AriaDisabled, Signal<&'static str>>,
    On<ev::click, SharedEventCallback<MouseEvent>>,
    On<ev::keydown, SharedEventCallback<KeyboardEvent>>,
    On<ev::focus, SharedEventCallback<FocusEvent>>,
);

/// Provides the behavior and accessibility for a grid list item.
#[allow(clippy::needless_pass_by_value)]
pub fn use_grid_list_item(input: UseGridListItemInput) -> UseGridListItemReturn {
    let item_key = input.item_key.clone();
    let is_selected = input.is_selected;
    let is_focused = input.is_focused;
    let is_disabled = input.is_disabled;
    let on_action = input.on_action;
    let on_selection_change = input.on_selection_change;
    let on_focus_next = input.on_focus_next;
    let on_focus_previous = input.on_focus_previous;

    // Compute aria-selected
    let aria_selected = Signal::derive(move || {
        if on_selection_change.is_some() {
            if is_selected.get() {
                Some("true")
            } else {
                Some("false")
            }
        } else {
            None
        }
    });

    // Compute tabindex
    let tabindex = Signal::derive(move || if is_focused.get() { "0" } else { "-1" });

    // Compute aria-disabled
    let aria_disabled = Signal::derive(move || if is_disabled.get() { "true" } else { "false" });

    // Handle click
    let handle_click = move |e: MouseEvent| {
        if is_disabled.get_untracked() {
            return;
        }

        if let Some(on_selection_change) = on_selection_change {
            let toggle = e.ctrl_key() || e.meta_key();
            if toggle {
                on_selection_change.run(!is_selected.get_untracked());
            } else {
                on_selection_change.run(true);
            }
        }
    };

    // Handle keyboard
    let handle_keydown = move |e: KeyboardEvent| {
        if is_disabled.get_untracked() {
            return;
        }

        let key = e.key();
        match key.as_str() {
            "Enter" => {
                e.prevent_default();
                if let Some(on_action) = on_action {
                    on_action.run(());
                }
            }
            " " => {
                e.prevent_default();
                if let Some(on_selection_change) = on_selection_change {
                    on_selection_change.run(!is_selected.get_untracked());
                }
            }
            "ArrowDown" => {
                e.prevent_default();
                if let Some(on_next) = on_focus_next {
                    on_next.run(());
                }
            }
            "ArrowUp" => {
                e.prevent_default();
                if let Some(on_prev) = on_focus_previous {
                    on_prev.run(());
                }
            }
            _ => {}
        }
    };

    // Handle focus
    let handle_focus = move |_e: FocusEvent| {
        // Focus is managed by parent
    };

    // Row index is 1-based for ARIA
    let aria_rowindex = (input.index + 1).to_string();

    UseGridListItemReturn {
        item_props: (
            Attr(attr::Role, "row"),
            Attr(attr::AriaRowindex, aria_rowindex),
            Attr(attr::AriaSelected, aria_selected),
            Attr(attr::Tabindex, tabindex),
            Attr(attr::AriaDisabled, aria_disabled),
            on(ev::click, handle_click).into_cloneable(),
            on(ev::keydown, handle_keydown).into_cloneable(),
            on(ev::focus, handle_focus).into_cloneable(),
        ),
        item_key,
        is_selected,
        is_focused,
    }
}
