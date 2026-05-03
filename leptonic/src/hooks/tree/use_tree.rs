use leptos::{
    attr,
    attr::Attr,
    ev,
    ev::{On, SharedEventCallback},
    prelude::*,
};
use uuid::Uuid;
use web_sys::KeyboardEvent;

use crate::{
    hooks::IntoAttrs,
    utils::{
        EventHandler,
        aria::{AriaDisabled, AriaMultiselectable, AriaRole},
    },
};

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/@react-aria/tree/src/useTree.ts

//
// No intentional deviations from the react-aria implementation.
//

/// The selection mode for a tree.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TreeSelectionMode {
    /// No selection allowed.
    #[default]
    None,
    /// Single item selection.
    Single,
    /// Multiple item selection.
    Multiple,
}

/// Input parameters for the `use_tree` hook.
#[derive(Debug, Clone)]
pub struct UseTreeInput {
    /// The label for the tree.
    pub label: Option<String>,

    /// The selection mode.
    pub selection_mode: TreeSelectionMode,

    /// Whether the tree is disabled.
    pub is_disabled: Signal<bool>,

    /// The currently selected item keys.
    pub selected_keys: Signal<Vec<String>>,

    /// The currently expanded item keys.
    pub expanded_keys: Signal<Vec<String>>,

    /// Callback when selection changes.
    pub on_selection_change: Option<Callback<Vec<String>>>,

    /// Callback when expansion changes.
    pub on_expanded_change: Option<Callback<Vec<String>>>,

    /// Callback when an item is activated.
    pub on_action: Option<Callback<String>>,
}

impl Default for UseTreeInput {
    fn default() -> Self {
        Self {
            label: None,
            selection_mode: TreeSelectionMode::None,
            is_disabled: Signal::derive(|| false),
            selected_keys: Signal::derive(Vec::new),
            expanded_keys: Signal::derive(Vec::new),
            on_selection_change: None,
            on_expanded_change: None,
            on_action: None,
        }
    }
}

/// The return value of the `use_tree` hook.
pub struct UseTreeReturn {
    /// Props for the tree element.
    pub tree_props: UseTreeProps,

    /// The ID of the tree.
    pub tree_id: String,

    /// The selection mode.
    pub selection_mode: TreeSelectionMode,

    /// The currently focused item key.
    pub focused_key: Signal<Option<String>>,

    /// Set the focused item.
    pub set_focused_key: Callback<Option<String>>,

    /// Toggle item expansion.
    pub toggle_expanded: Callback<String>,

    /// Toggle item selection.
    pub toggle_selected: Callback<String>,
}

/// Props from `use_tree` that can be extracted and merged programmatically.
#[derive(Debug)]
pub struct UseTreeProps {
    pub id: String,
    pub role: AriaRole,
    pub aria_label: Option<String>,
    pub aria_multiselectable: Option<AriaMultiselectable>,
    pub aria_disabled: Signal<Option<AriaDisabled>>,
    pub on_keydown: EventHandler<KeyboardEvent>,
}

impl IntoAttrs for UseTreeProps {
    type Attrs = UseTreeAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            Attr(attr::Id, self.id),
            Attr(attr::Role, self.role),
            Attr(attr::AriaLabel, self.aria_label),
            Attr(attr::AriaMultiselectable, self.aria_multiselectable),
            Attr(attr::AriaDisabled, self.aria_disabled),
            self.on_keydown.into_on(ev::keydown),
        )
    }
}

/// Attributes for the tree element.
pub type UseTreeAttrs = (
    Attr<attr::Id, String>,
    Attr<attr::Role, AriaRole>,
    Attr<attr::AriaLabel, Option<String>>,
    Attr<attr::AriaMultiselectable, Option<AriaMultiselectable>>,
    Attr<attr::AriaDisabled, Signal<Option<AriaDisabled>>>,
    On<ev::keydown, SharedEventCallback<KeyboardEvent>>,
);

/// Provides the behavior and accessibility for a tree component.
///
/// A tree displays hierarchical data with expandable/collapsible items.
///
/// # Example
///
/// ```ignore
/// let tree = use_tree(UseTreeInput {
///     label: Some("File Browser".to_string()),
///     selection_mode: TreeSelectionMode::Single,
///     ..Default::default()
/// });
///
/// view! {
///     <ul {..tree.tree_props}>
///         // Tree items...
///     </ul>
/// }
/// ```
pub fn use_tree(input: UseTreeInput) -> UseTreeReturn {
    let UseTreeInput {
        label,
        selection_mode,
        is_disabled: disabled,
        selected_keys,
        expanded_keys,
        on_selection_change,
        on_expanded_change,
        on_action,
    } = input;

    let tree_id = format!("tree-{}", Uuid::new_v4());

    // Track focused item
    let (focused_key, set_focused_key_signal) = signal::<Option<String>>(None);

    let set_focused_key = Callback::new(move |key: Option<String>| {
        set_focused_key_signal.set(key);
    });

    let toggle_expanded = Callback::new(move |key: String| {
        let mut current = expanded_keys.get_untracked();
        if let Some(pos) = current.iter().position(|k| k == &key) {
            current.remove(pos);
        } else {
            current.push(key);
        }
        if let Some(on_change) = on_expanded_change {
            on_change.run(current);
        }
    });

    let toggle_selected = Callback::new(move |key: String| {
        if selection_mode == TreeSelectionMode::None {
            return;
        }

        let mut current = selected_keys.get_untracked();
        if let Some(pos) = current.iter().position(|k| k == &key) {
            current.remove(pos);
        } else if selection_mode == TreeSelectionMode::Single {
            current = vec![key];
        } else {
            current.push(key);
        }

        if let Some(on_change) = on_selection_change {
            on_change.run(current);
        }
    });

    let aria_multiselectable = match selection_mode {
        TreeSelectionMode::None => None,
        TreeSelectionMode::Single => Some(AriaMultiselectable::False),
        TreeSelectionMode::Multiple => Some(AriaMultiselectable::True),
    };

    let aria_disabled = Signal::derive(move || disabled.get().then_some(AriaDisabled::True));

    let handle_keydown = move |e: KeyboardEvent| {
        if disabled.get_untracked() {
            return;
        }
        if e.is_composing() {
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
                    if selection_mode != TreeSelectionMode::None {
                        e.prevent_default();
                        toggle_selected.run(focused);
                    }
                }
            }
            _ => {}
        }
    };

    UseTreeReturn {
        tree_props: UseTreeProps {
            id: tree_id.clone(),
            role: AriaRole::Tree,
            aria_label: label,
            aria_multiselectable,
            aria_disabled,
            on_keydown: EventHandler::new(handle_keydown),
        },
        tree_id,
        selection_mode,
        focused_key: focused_key.into(),
        set_focused_key,
        toggle_expanded,
        toggle_selected,
    }
}
