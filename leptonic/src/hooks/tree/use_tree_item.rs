use leptos::attr;
use leptos::attr::Attr;
use leptos::ev;
use leptos::ev::{On, SharedEventCallback};
use leptos::prelude::*;
use web_sys::{FocusEvent, KeyboardEvent, MouseEvent};

use crate::utils::aria::{AriaDisabled, AriaExpanded, AriaSelected};
use crate::utils::EventHandler;

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// No intentional deviations from the react-aria implementation.
//
// =============================================================================

/// Input parameters for the `use_tree_item` hook.
#[derive(Debug, Clone)]
pub struct UseTreeItemInput {
    /// The unique key for this item.
    pub item_key: String,

    /// The nesting level (0 for root items).
    pub level: usize,

    /// The position in the set (1-based).
    pub position_in_set: usize,

    /// The total number of items in this level.
    pub set_size: usize,

    /// Whether this item has children.
    pub has_children: bool,

    /// Whether this item is expanded.
    pub is_expanded: Signal<bool>,

    /// Whether this item is selected.
    pub is_selected: Signal<bool>,

    /// Whether this item is focused.
    pub is_focused: Signal<bool>,

    /// Whether this item is disabled.
    pub is_disabled: Signal<bool>,

    /// Callback when the item is expanded/collapsed.
    pub on_expand: Option<Callback<bool>>,

    /// Callback when the item is selected.
    pub on_select: Option<Callback<()>>,

    /// Callback when the item is activated.
    pub on_action: Option<Callback<()>>,

    /// Callback to navigate to the next item.
    pub on_focus_next: Option<Callback<()>>,

    /// Callback to navigate to the previous item.
    pub on_focus_previous: Option<Callback<()>>,

    /// Callback to navigate to the parent item.
    pub on_focus_parent: Option<Callback<()>>,

    /// Callback to navigate to the first child.
    pub on_focus_first_child: Option<Callback<()>>,
}

/// The return value of the `use_tree_item` hook.
#[derive(Debug, Clone)]
pub struct UseTreeItemReturn {
    /// Props for the tree item element.
    pub item_props: UseTreeItemProps,

    /// Props for the content element (label area).
    pub content_props: UseTreeItemContentProps,

    /// The item key.
    pub item_key: String,

    /// Whether the item is expanded.
    pub is_expanded: Signal<bool>,

    /// Whether the item is selected.
    pub is_selected: Signal<bool>,
}

/// Props from `use_tree_item` for the item element that can be extracted and merged programmatically.
/// Note: aria-level, aria-setsize, aria-posinset should be set via custom attributes.
#[derive(Debug, Clone)]
pub struct UseTreeItemProps {
    pub role: &'static str,
    pub aria_expanded: Signal<Option<AriaExpanded>>,
    pub aria_selected: Signal<Option<AriaSelected>>,
    pub aria_disabled: Signal<Option<AriaDisabled>>,
    pub tabindex: Signal<&'static str>,
    pub on_click: EventHandler<MouseEvent>,
    pub on_keydown: EventHandler<KeyboardEvent>,
    pub on_focus: EventHandler<FocusEvent>,
}

impl UseTreeItemProps {
    /// Convert to spreadable attributes for Leptos views, cloning internally.
    #[must_use]
    pub fn to_attrs(&self) -> UseTreeItemAttrs {
        self.clone().into_attrs()
    }

    /// Convert to spreadable attributes for Leptos views, consuming self.
    #[must_use]
    pub fn into_attrs(self) -> UseTreeItemAttrs {
        (
            Attr(attr::Role, self.role),
            Attr(attr::AriaExpanded, self.aria_expanded),
            Attr(attr::AriaSelected, self.aria_selected),
            Attr(attr::AriaDisabled, self.aria_disabled),
            Attr(attr::Tabindex, self.tabindex),
            self.on_click.into_on(ev::click),
            self.on_keydown.into_on(ev::keydown),
            self.on_focus.into_on(ev::focus),
        )
    }
}

/// Attributes for the tree item element.
/// Note: aria-level, aria-setsize, aria-posinset should be set via custom attributes.
pub type UseTreeItemAttrs = (
    Attr<attr::Role, &'static str>,
    Attr<attr::AriaExpanded, Signal<Option<AriaExpanded>>>,
    Attr<attr::AriaSelected, Signal<Option<AriaSelected>>>,
    Attr<attr::AriaDisabled, Signal<Option<AriaDisabled>>>,
    Attr<attr::Tabindex, Signal<&'static str>>,
    On<ev::click, SharedEventCallback<MouseEvent>>,
    On<ev::keydown, SharedEventCallback<KeyboardEvent>>,
    On<ev::focus, SharedEventCallback<FocusEvent>>,
);

/// Props from `use_tree_item` for the content element that can be extracted and merged programmatically.
#[derive(Debug, Clone)]
pub struct UseTreeItemContentProps {
    pub role: &'static str,
}

impl UseTreeItemContentProps {
    /// Convert to spreadable attributes for Leptos views, cloning internally.
    #[must_use]
    pub fn to_attrs(&self) -> UseTreeItemContentAttrs {
        self.clone().into_attrs()
    }

    /// Convert to spreadable attributes for Leptos views, consuming self.
    #[must_use]
    pub fn into_attrs(self) -> UseTreeItemContentAttrs {
        (Attr(attr::Role, self.role),)
    }
}

/// Attributes for the tree item content element.
pub type UseTreeItemContentAttrs = (Attr<attr::Role, &'static str>,);

/// Provides the behavior and accessibility for a tree item.
#[allow(clippy::too_many_lines, clippy::needless_pass_by_value)]
pub fn use_tree_item(input: UseTreeItemInput) -> UseTreeItemReturn {
    let UseTreeItemInput {
        item_key,
        level,
        position_in_set,
        set_size,
        has_children,
        is_expanded,
        is_selected,
        is_focused,
        is_disabled: disabled,
        on_expand,
        on_select,
        on_action,
        on_focus_next,
        on_focus_previous,
        on_focus_parent,
        on_focus_first_child,
    } = input;

    // aria-expanded only applies if has children
    let aria_expanded = Signal::derive(move || {
        if has_children {
            Some(AriaExpanded::from(is_expanded.get()))
        } else {
            None
        }
    });

    let aria_selected = Signal::derive(move || {
        if on_select.is_some() {
            Some(AriaSelected::from(is_selected.get()))
        } else {
            None
        }
    });

    let aria_disabled = Signal::derive(move || disabled.get().then_some(AriaDisabled::True));

    let tabindex = Signal::derive(move || if is_focused.get() { "0" } else { "-1" });

    let handle_click = move |_e: web_sys::MouseEvent| {
        if disabled.get_untracked() {
            return;
        }
        if let Some(on_select) = on_select {
            on_select.run(());
        }
    };

    let handle_keydown = move |e: KeyboardEvent| {
        if disabled.get_untracked() {
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
                if let Some(on_select) = on_select {
                    on_select.run(());
                }
            }
            "ArrowRight" => {
                e.prevent_default();
                if has_children {
                    if is_expanded.get_untracked() {
                        // Focus first child
                        if let Some(on_child) = on_focus_first_child {
                            on_child.run(());
                        }
                    } else {
                        // Expand
                        if let Some(on_expand) = on_expand {
                            on_expand.run(true);
                        }
                    }
                }
            }
            "ArrowLeft" => {
                e.prevent_default();
                if has_children && is_expanded.get_untracked() {
                    // Collapse
                    if let Some(on_expand) = on_expand {
                        on_expand.run(false);
                    }
                } else {
                    // Focus parent
                    if let Some(on_parent) = on_focus_parent {
                        on_parent.run(());
                    }
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

    let handle_focus = move |_e: web_sys::FocusEvent| {
        // Focus is managed by parent
    };

    // Note: aria-level, aria-setsize, aria-posinset should be set via custom attributes
    // let aria_level = (input.level + 1).to_string();
    // let aria_setsize = input.set_size.to_string();
    // let aria_posinset = input.position_in_set.to_string();

    UseTreeItemReturn {
        item_props: UseTreeItemProps {
            role: "treeitem",
            aria_expanded,
            aria_selected,
            aria_disabled,
            tabindex,
            on_click: EventHandler::new(handle_click),
            on_keydown: EventHandler::new(handle_keydown),
            on_focus: EventHandler::new(handle_focus),
        },
        content_props: UseTreeItemContentProps {
            role: "presentation",
        },
        item_key,
        is_expanded,
        is_selected,
    }
}
