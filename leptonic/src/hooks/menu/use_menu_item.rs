use leptos::attr;
use leptos::attr::{Attr, Attribute};
use leptos::ev;
use leptos::ev::{on, On, SharedEventCallback};
use leptos::prelude::*;
use std::hash::Hash;
use web_sys::{FocusEvent, KeyboardEvent, MouseEvent};

use crate::hooks::focus::use_focus_ring::{use_focus_ring, UseFocusRingInput, UseFocusRingReturn};
use crate::hooks::focus::use_focusable::{use_focusable, UseFocusableInput};
use crate::hooks::selection::use_selection_state::Selection;
use crate::utils::element_capture::ElementCaptureAttr;

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/@react-aria/menu/src/useMenuItem.ts

/// Input parameters for the `use_menu_item` hook.
#[derive(Clone)]
pub struct UseMenuItemInput<K>
where
    K: Hash + Eq + Clone + Send + Sync + 'static,
{
    /// The key for this menu item.
    pub key: K,

    /// Whether this item is disabled.
    pub is_disabled: Signal<bool>,

    /// The currently focused key in the menu.
    pub focused_key: Signal<Option<K>>,

    /// The current selection.
    pub selected_keys: Signal<Selection<K>>,

    /// Callback when this item is focused.
    pub on_focus: Callback<Option<K>>,

    /// Callback when this item is selected/activated.
    pub on_action: Callback<K>,

    /// Callback to close the menu.
    pub on_close: Option<Callback<()>>,

    /// Whether the menu should close when this item is selected.
    pub close_on_select: bool,
}

/// The return value of the `use_menu_item` hook.
pub struct UseMenuItemReturn {
    /// Props for the menu item element.
    pub item_props: UseMenuItemAttrs,

    /// Whether this item is currently focused.
    pub is_focused: Signal<bool>,

    /// Whether this item is currently selected.
    pub is_selected: Signal<bool>,

    /// Whether this item is disabled.
    pub is_disabled: Signal<bool>,

    /// Whether the focus ring should be visible (keyboard navigation only).
    pub is_focus_visible: Signal<bool>,
}

/// Attributes for a menu item element.
pub type UseMenuItemAttrs = (
    Attr<attr::Role, &'static str>,
    Attr<attr::Tabindex, Signal<i32>>,
    Attr<attr::AriaDisabled, Signal<Option<&'static str>>>,
    On<ev::click, SharedEventCallback<MouseEvent>>,
    On<ev::keydown, SharedEventCallback<KeyboardEvent>>,
    On<ev::focus, SharedEventCallback<FocusEvent>>,
    On<ev::blur, SharedEventCallback<FocusEvent>>,
    On<ev::mouseenter, SharedEventCallback<MouseEvent>>,
    attr::custom::CustomAttr<&'static str, Signal<Option<&'static str>>>,
    ElementCaptureAttr,
);

/// Provides the behavior and accessibility implementation for a menu item.
///
/// Menu items can be activated by click, Enter, or Space keys.
/// They support keyboard navigation and focus management.
///
/// The hook automatically captures the DOM element through [`ElementCaptureAttr`],
/// so you don't need to create or pass a `NodeRef`. Just spread the `item_props`
/// onto your element and focus management works automatically.
///
/// # Example
///
/// ```ignore
/// let item = use_menu_item(UseMenuItemInput {
///     key: "copy".to_string(),
///     is_disabled: Signal::derive(|| false),
///     focused_key: menu.list.collection.focused_key,
///     selected_keys: menu.list.collection.selection_state.selected_keys,
///     on_focus: menu.list.collection.set_focused_key,
///     on_action: Callback::new(|key| {
///         // Handle action
///     }),
///     on_close: Some(Callback::new(|_| {
///         // Close the menu
///     })),
///     close_on_select: true,
/// });
///
/// view! {
///     <li {..item.item_props}>
///         "Copy"
///     </li>
/// }
/// ```
#[allow(clippy::too_many_lines, clippy::needless_pass_by_value)]
pub fn use_menu_item<K>(input: UseMenuItemInput<K>) -> UseMenuItemReturn
where
    K: Hash + Eq + Clone + Send + Sync + 'static,
{
    let key = input.key.clone();
    let is_disabled_input = input.is_disabled;
    let focused_key = input.focused_key;
    let selected_keys = input.selected_keys;
    let on_focus = input.on_focus;
    let on_action = input.on_action;
    let on_close = input.on_close;
    let close_on_select = input.close_on_select;

    // Compute whether this item is focused
    let key_for_focused = key.clone();
    let is_focused = Signal::derive(move || focused_key.get().as_ref() == Some(&key_for_focused));

    // Use focusable to get element capture and focus handle
    let focusable = use_focusable(UseFocusableInput {
        disabled: is_disabled_input,
        auto_focus: false,
        exclude_from_tab_order: Signal::derive(|| true), // Menu items use roving tabindex
        on_focus: None, // We handle focus through focus_ring's on_focus
        on_blur: None,
        on_focus_change: None,
        on_key_down: None, // We handle keydown separately for menu-specific behavior
        on_key_up: None,
    });

    // Effect to focus the element when is_focused becomes true
    let focus_handle = focusable.focus_handle;
    Effect::new(move |prev_focused: Option<bool>| {
        let currently_focused = is_focused.get();
        let was_focused = prev_focused.unwrap_or(false);

        // Only focus if we just became focused (transition from false to true)
        if currently_focused && !was_focused {
            focus_handle.focus();
        }

        currently_focused
    });

    // Compute whether this item is selected
    let key_for_selected = key.clone();
    let is_selected = Signal::derive(move || match selected_keys.get() {
        Selection::Keys(keys) => keys.contains(&key_for_selected),
        Selection::All => true,
    });

    // Compute tabindex (only focused item should be tabbable)
    let tabindex = Signal::derive(move || if is_focused.get() { 0 } else { -1 });

    // Compute aria-disabled
    let aria_disabled = Signal::derive(move || {
        if is_disabled_input.get() {
            Some("true")
        } else {
            None
        }
    });

    // Activate the item
    let key_for_action = key.clone();
    let activate = move || {
        if is_disabled_input.get_untracked() {
            return;
        }

        on_action.run(key_for_action.clone());

        if close_on_select {
            if let Some(on_close) = on_close {
                on_close.run(());
            }
        }
    };

    // Handle click
    let activate_click = activate.clone();
    let handle_click = move |_e: MouseEvent| {
        activate_click();
    };

    // Handle keydown
    let activate_key = activate.clone();
    let handle_keydown = move |e: KeyboardEvent| {
        if is_disabled_input.get_untracked() {
            return;
        }

        match e.key().as_str() {
            "Enter" | " " => {
                e.prevent_default();
                activate_key();
            }
            _ => {}
        }
    };

    // Handle focus callback
    let key_for_focus = key.clone();
    let focus_callback = Callback::new(move |_: FocusEvent| {
        if is_disabled_input.get_untracked() {
            return;
        }
        on_focus.run(Some(key_for_focus.clone()));
    });

    // Use focus ring for keyboard focus visibility
    let UseFocusRingReturn {
        props: focus_ring_props,
        is_focus_visible,
        is_focused: _,
    } = use_focus_ring(UseFocusRingInput {
        disabled: is_disabled_input,
        within: false,
        auto_focus: false,
        on_focus: Some(focus_callback),
        on_blur: None,
        on_focus_change: None,
    });
    let (handle_focus, handle_blur, data_focus_visible) = focus_ring_props.into_attrs();

    // Handle mouse enter (for hover focus)
    let key_for_hover = key.clone();
    let handle_mouseenter = move |_e: MouseEvent| {
        if is_disabled_input.get_untracked() {
            return;
        }
        on_focus.run(Some(key_for_hover.clone()));
    };

    UseMenuItemReturn {
        item_props: (
            Attr(attr::Role, "menuitem"),
            Attr(attr::Tabindex, tabindex),
            Attr(attr::AriaDisabled, aria_disabled),
            on(ev::click, handle_click).into_cloneable(),
            on(ev::keydown, handle_keydown).into_cloneable(),
            handle_focus,
            handle_blur,
            on(ev::mouseenter, handle_mouseenter).into_cloneable(),
            data_focus_visible,
            focusable.props.element_capture,
        ),
        is_focused,
        is_selected,
        is_disabled: is_disabled_input,
        is_focus_visible,
    }
}
