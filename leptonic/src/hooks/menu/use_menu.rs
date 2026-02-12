use leptos::attr;
use leptos::attr::Attr;
use leptos::ev;
use leptos::ev::{On, SharedEventCallback};
use leptos::prelude::*;
use std::collections::HashSet;
use std::hash::Hash;
use web_sys::KeyboardEvent;

use crate::hooks::selection::use_selectable_list::{
    use_selectable_list, UseSelectableListInput, UseSelectableListReturn,
};
use crate::hooks::selection::use_selection_state::{Selection, SelectionBehavior, SelectionMode};
use crate::hooks::selection::use_type_select::{
    use_type_select, UseTypeSelectInput, UseTypeSelectReturn,
};
use crate::utils::EventHandler;

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/@react-aria/menu/src/useMenu.ts

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// No intentional deviations from the react-aria implementation.
//
// =============================================================================

use crate::hooks::selection::use_selectable_collection::FocusStrategy;
use crate::hooks::IntoAttrs;

/// Input parameters for the `use_menu` hook.
#[derive(Clone)]
pub struct UseMenuInput<K>
where
    K: Hash + Eq + Clone + Send + Sync + 'static,
{
    /// Whether keyboard navigation should wrap around.
    pub should_focus_wrap: bool,

    /// An accessibility label for the menu.
    pub aria_label: Option<String>,

    /// All available menu item keys.
    pub all_keys: Signal<Vec<K>>,

    /// Keys that are disabled.
    pub disabled_keys: Signal<HashSet<K>>,

    /// Callback to get the text label for a key (used for type-ahead).
    pub get_key_label: Callback<K, String>,

    /// Handler called when the menu should close.
    pub on_close: Option<Callback<()>>,

    /// Handler called when an action is performed on a menu item.
    pub on_action: Option<Callback<K>>,

    /// Whether the menu is disabled.
    pub disabled: Signal<bool>,

    /// Focus strategy signal. When this becomes Some(strategy), focus moves accordingly.
    /// Connect this to `use_menu_trigger_state().focus_strategy` for proper menu focus behavior.
    pub auto_focus: Signal<Option<FocusStrategy>>,
}

impl<K: Hash + Eq + Clone + Send + Sync + 'static> Default for UseMenuInput<K> {
    fn default() -> Self {
        Self {
            should_focus_wrap: true,
            aria_label: None,
            all_keys: Signal::derive(Vec::new),
            disabled_keys: Signal::derive(HashSet::new),
            get_key_label: Callback::new(|_| String::new()),
            on_close: None,
            on_action: None,
            disabled: Signal::derive(|| false),
            auto_focus: Signal::derive(|| None),
        }
    }
}

/// The return value of the `use_menu` hook.
pub struct UseMenuReturn<K>
where
    K: Hash + Eq + Clone + Send + Sync + 'static,
{
    /// Props for the menu element. Call `.into_attrs()` for view spreading.
    pub menu_props: UseMenuProps,

    /// The selectable list state.
    pub list: UseSelectableListReturn<K>,

    /// The type select state.
    pub type_select: UseTypeSelectReturn,
}

/// Props from `use_menu` that can be extracted and merged programmatically.
#[derive(Debug)]
pub struct UseMenuProps {
    pub role: &'static str,
    pub aria_label: Option<String>,
    pub tabindex: i32,
    pub on_keydown: EventHandler<KeyboardEvent>,
}

impl IntoAttrs for UseMenuProps {
    type Attrs = UseMenuAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            Attr(attr::Role, self.role),
            Attr(attr::AriaLabel, self.aria_label),
            Attr(attr::Tabindex, self.tabindex),
            self.on_keydown.into_on(ev::keydown),
        )
    }
}

/// Attributes for the menu element.
pub type UseMenuAttrs = (
    Attr<attr::Role, &'static str>,
    Attr<attr::AriaLabel, Option<String>>,
    Attr<attr::Tabindex, i32>,
    On<ev::keydown, SharedEventCallback<KeyboardEvent>>,
);

/// Provides the behavior and accessibility implementation for a menu component.
///
/// A menu displays a list of actions or options that a user can choose.
/// This hook handles keyboard navigation, type-ahead selection, and ARIA attributes.
///
/// # Example
///
/// ```ignore
/// let items = vec!["copy", "paste", "cut"];
/// let all_keys = Signal::derive(move || items.iter().map(|s| s.to_string()).collect());
///
/// let menu = use_menu(UseMenuInput {
///     all_keys,
///     get_key_label: Callback::new(|key: String| key.clone()),
///     on_action: Some(Callback::new(|key| {
///         // Handle menu item selection
///     })),
///     on_close: Some(Callback::new(|_| {
///         // Close the menu
///     })),
///     ..Default::default()
/// });
///
/// view! {
///     <ul {..menu.menu_props.into_attrs()}>
///         // Menu items here
///     </ul>
/// }
/// ```
#[allow(clippy::needless_pass_by_value)]
pub fn use_menu<K>(input: UseMenuInput<K>) -> UseMenuReturn<K>
where
    K: Hash + Eq + Clone + Send + Sync + 'static,
{
    let UseMenuInput {
        should_focus_wrap,
        aria_label,
        all_keys,
        disabled_keys,
        get_key_label,
        on_close,
        on_action,
        disabled,
        auto_focus,
    } = input;

    // Create the selectable list
    let list = use_selectable_list(UseSelectableListInput {
        selection_mode: SelectionMode::Single,
        selection_behavior: SelectionBehavior::Replace,
        disabled,
        selected_keys: None,
        default_selected_keys: None,
        on_selection_change: Some(Callback::new(move |selection: Selection<K>| {
            // When an item is selected, trigger the action
            if let Selection::Keys(keys) = selection {
                if let Some(key) = keys.into_iter().next() {
                    if let Some(on_action) = on_action {
                        on_action.run(key);
                    }
                }
            }
        })),
        disabled_keys,
        disallow_empty_selection: false,
        all_keys,
        should_focus_wrap,
        auto_focus,
        select_on_focus: false,
    });

    // Create type-ahead selection
    let type_select = use_type_select(UseTypeSelectInput {
        disabled,
        all_keys,
        get_key_label,
        focused_key: list.collection.focused_key,
        on_focus: list.collection.set_focused_key,
        timeout_ms: 500,
    });

    // Get the keyboard handler callbacks from sub-hooks for delegation
    let list_on_keydown = list.on_keydown;
    let type_select_on_keydown = type_select.on_keydown;

    // Unified keyboard handler that delegates to sub-hook handlers
    let handle_keydown = move |e: KeyboardEvent| {
        if disabled.get_untracked() {
            return;
        }

        let key = e.key();

        match key.as_str() {
            // Menu-specific: close on Escape (override list's Escape which clears selection)
            "Escape" => {
                e.prevent_default();
                if let Some(on_close) = on_close {
                    on_close.run(());
                }
            }
            // Menu-specific: prevent Tab from moving focus out
            "Tab" => {
                e.prevent_default();
            }
            // Menu-specific: close menu after Enter/Space selection
            " " | "Enter" => {
                // Let the list handler process the selection first
                list_on_keydown.run(e);
                // Then close the menu
                if let Some(on_close) = on_close {
                    on_close.run(());
                }
            }
            // Delegate navigation keys to list handler
            "ArrowDown" | "ArrowRight" | "ArrowUp" | "ArrowLeft" | "Home" | "End" => {
                list_on_keydown.run(e);
            }
            // Delegate type-ahead to type_select handler
            _ => {
                type_select_on_keydown.run(e);
            }
        }
    };

    UseMenuReturn {
        menu_props: UseMenuProps {
            role: "menu",
            aria_label,
            tabindex: 0,
            on_keydown: EventHandler::new(handle_keydown),
        },
        list,
        type_select,
    }
}
