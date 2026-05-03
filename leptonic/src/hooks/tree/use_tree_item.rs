use leptos::{
    attr,
    attr::{
        Attr,
        custom::{CustomAttr, custom_attribute},
    },
    ev,
    ev::{On, SharedEventCallback},
    prelude::*,
};
use web_sys::{FocusEvent, KeyboardEvent, MouseEvent};

use crate::{
    hooks::IntoAttrs,
    utils::{
        EventHandler,
        aria::{AriaDisabled, AriaExpanded, AriaRole, AriaSelected},
    },
};

//
// No intentional deviations from the react-aria implementation.
//

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

    /// Called when navigation to the next visible item is requested (ArrowDown).
    pub on_focus_next: Option<Callback<()>>,

    /// Called when navigation to the previous visible item is requested (ArrowUp).
    pub on_focus_previous: Option<Callback<()>>,

    /// Called when navigation to the parent item is requested (ArrowLeft on collapsed/leaf).
    pub on_focus_parent: Option<Callback<()>>,

    /// Called when navigation to the first child is requested (ArrowRight on expanded).
    pub on_focus_first_child: Option<Callback<()>>,

    /// Called when navigation to the first visible item in the tree is requested (Home).
    pub on_focus_first: Option<Callback<()>>,

    /// Called when navigation to the last visible item in the tree is requested (End).
    pub on_focus_last: Option<Callback<()>>,

    /// Called when this item receives DOM focus, so the tree can update its focus tracking.
    pub on_focus_self: Option<Callback<()>>,
}

/// The return value of the `use_tree_item` hook.
#[derive(Debug)]
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
#[derive(Debug)]
pub struct UseTreeItemProps {
    pub role: AriaRole,
    pub aria_expanded: Signal<Option<AriaExpanded>>,
    pub aria_selected: Signal<Option<AriaSelected>>,
    pub aria_disabled: Signal<Option<AriaDisabled>>,
    pub aria_level: String,
    pub aria_setsize: String,
    pub aria_posinset: String,
    pub tabindex: Signal<&'static str>,
    pub on_click: EventHandler<MouseEvent>,
    pub on_keydown: EventHandler<KeyboardEvent>,
    pub on_focus: EventHandler<FocusEvent>,
}

impl IntoAttrs for UseTreeItemProps {
    type Attrs = UseTreeItemAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            Attr(attr::Role, self.role),
            Attr(attr::AriaExpanded, self.aria_expanded),
            Attr(attr::AriaSelected, self.aria_selected),
            Attr(attr::AriaDisabled, self.aria_disabled),
            custom_attribute("aria-level", self.aria_level),
            Attr(attr::AriaSetsize, self.aria_setsize),
            Attr(attr::AriaPosinset, self.aria_posinset),
            Attr(attr::Tabindex, self.tabindex),
            self.on_click.into_on(ev::click),
            self.on_keydown.into_on(ev::keydown),
            self.on_focus.into_on(ev::focus),
        )
    }
}

/// Attributes for the tree item element.
pub type UseTreeItemAttrs = (
    Attr<attr::Role, AriaRole>,
    Attr<attr::AriaExpanded, Signal<Option<AriaExpanded>>>,
    Attr<attr::AriaSelected, Signal<Option<AriaSelected>>>,
    Attr<attr::AriaDisabled, Signal<Option<AriaDisabled>>>,
    CustomAttr<&'static str, String>,
    Attr<attr::AriaSetsize, String>,
    Attr<attr::AriaPosinset, String>,
    Attr<attr::Tabindex, Signal<&'static str>>,
    On<ev::click, SharedEventCallback<MouseEvent>>,
    On<ev::keydown, SharedEventCallback<KeyboardEvent>>,
    On<ev::focus, SharedEventCallback<FocusEvent>>,
);

/// Props from `use_tree_item` for the content element that can be extracted and merged programmatically.
#[derive(Debug)]
pub struct UseTreeItemContentProps {
    pub role: AriaRole,
}

impl IntoAttrs for UseTreeItemContentProps {
    type Attrs = UseTreeItemContentAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (Attr(attr::Role, self.role),)
    }
}

/// Attributes for the tree item content element.
pub type UseTreeItemContentAttrs = (Attr<attr::Role, AriaRole>,);

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
        on_focus_first,
        on_focus_last,
        on_focus_self,
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
        if e.is_composing() {
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
            "Home" => {
                e.prevent_default();
                if let Some(on_first) = on_focus_first {
                    on_first.run(());
                }
            }
            "End" => {
                e.prevent_default();
                if let Some(on_last) = on_focus_last {
                    on_last.run(());
                }
            }
            _ => {}
        }
    };

    let handle_focus = move |_e: web_sys::FocusEvent| {
        if let Some(on_focus_self) = on_focus_self {
            on_focus_self.run(());
        }
    };

    // Convert to 1-based ARIA level (input is 0-based: 0 = root).
    let aria_level = (level + 1).to_string();
    // position_in_set is already 1-based per input contract.
    let aria_posinset = position_in_set.to_string();
    let aria_setsize = set_size.to_string();

    UseTreeItemReturn {
        item_props: UseTreeItemProps {
            role: AriaRole::Treeitem,
            aria_expanded,
            aria_selected,
            aria_disabled,
            aria_level,
            aria_setsize,
            aria_posinset,
            tabindex,
            on_click: EventHandler::new(handle_click),
            on_keydown: EventHandler::new(handle_keydown),
            on_focus: EventHandler::new(handle_focus),
        },
        content_props: UseTreeItemContentProps {
            role: AriaRole::Presentation,
        },
        item_key,
        is_expanded,
        is_selected,
    }
}
