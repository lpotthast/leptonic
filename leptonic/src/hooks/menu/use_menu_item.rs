use std::hash::Hash;

use leptos::{
    attr,
    attr::{
        custom::{custom_attribute, CustomAttr},
        Attr,
    },
    ev,
    ev::{On, SharedEventCallback},
    prelude::*,
};
use web_sys::{FocusEvent, KeyboardEvent, MouseEvent};

use crate::{
    hooks::{
        focus::{
            use_focus_ring::{use_focus_ring, UseFocusRingInput, UseFocusRingReturn},
            use_focusable::{use_focusable, UseFocusableInput},
        },
        selection::use_selection_state::Selection,
        IntoAttrs,
    },
    utils::{element_capture::ElementCaptureAttr, EventHandler},
};

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/@react-aria/menu/src/useMenuItem.ts

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// No intentional deviations from the react-aria implementation.
//
// =============================================================================

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
    /// Props for the menu item element. Call `.into_attrs()` for view spreading.
    pub item_props: UseMenuItemProps,

    /// Whether this item is currently focused.
    pub is_focused: Signal<bool>,

    /// Whether this item is currently selected.
    pub is_selected: Signal<bool>,

    /// Whether this item is disabled.
    pub is_disabled: Signal<bool>,

    /// Whether the focus ring should be visible (keyboard navigation only).
    pub is_focus_visible: Signal<bool>,
}

/// Props from `use_menu_item` that can be extracted and merged programmatically.
#[derive(Debug)]
pub struct UseMenuItemProps {
    pub role: &'static str,
    pub tabindex: Signal<i32>,
    pub aria_disabled: Signal<Option<&'static str>>,
    pub on_click: EventHandler<MouseEvent>,
    pub on_keydown: EventHandler<KeyboardEvent>,
    pub on_focus: EventHandler<FocusEvent>,
    pub on_blur: EventHandler<FocusEvent>,
    pub on_focusin: EventHandler<FocusEvent>,
    pub on_focusout: EventHandler<FocusEvent>,
    pub on_mouseenter: EventHandler<MouseEvent>,
    pub data_focus_visible: Signal<Option<&'static str>>,
    pub element_capture: ElementCaptureAttr,
}

impl IntoAttrs for UseMenuItemProps {
    type Attrs = UseMenuItemAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            Attr(attr::Role, self.role),
            Attr(attr::Tabindex, self.tabindex),
            Attr(attr::AriaDisabled, self.aria_disabled),
            self.on_click.into_on(ev::click),
            self.on_keydown.into_on(ev::keydown),
            self.on_focus.into_on(ev::focus),
            self.on_blur.into_on(ev::blur),
            self.on_focusin.into_on(ev::focusin),
            self.on_focusout.into_on(ev::focusout),
            self.on_mouseenter.into_on(ev::mouseenter),
            custom_attribute("data-focus-visible", self.data_focus_visible),
            self.element_capture,
        )
    }
}

/// These attributes must be spread onto the target element: `<foo {..attrs} />`
pub type UseMenuItemAttrs = (
    Attr<attr::Role, &'static str>,
    Attr<attr::Tabindex, Signal<i32>>,
    Attr<attr::AriaDisabled, Signal<Option<&'static str>>>,
    On<ev::click, SharedEventCallback<MouseEvent>>,
    On<ev::keydown, SharedEventCallback<KeyboardEvent>>,
    On<ev::focus, SharedEventCallback<FocusEvent>>,
    On<ev::blur, SharedEventCallback<FocusEvent>>,
    On<ev::focusin, SharedEventCallback<FocusEvent>>,
    On<ev::focusout, SharedEventCallback<FocusEvent>>,
    On<ev::mouseenter, SharedEventCallback<MouseEvent>>,
    CustomAttr<&'static str, Signal<Option<&'static str>>>,
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
///     <li {..item.item_props.into_attrs()}>
///         "Copy"
///     </li>
/// }
/// ```
#[allow(clippy::too_many_lines, clippy::needless_pass_by_value)]
pub fn use_menu_item<K>(input: UseMenuItemInput<K>) -> UseMenuItemReturn
where
    K: Hash + Eq + Clone + Send + Sync + 'static,
{
    let UseMenuItemInput {
        key,
        is_disabled,
        focused_key,
        selected_keys,
        on_focus,
        on_action,
        on_close,
        close_on_select,
    } = input;

    // Compute whether this item is focused
    let key_for_focused = key.clone();
    let is_focused = Signal::derive(move || focused_key.get().as_ref() == Some(&key_for_focused));

    // Use focusable to get element capture and focus handle
    let focusable = use_focusable(UseFocusableInput {
        disabled: is_disabled,
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
        if is_disabled.get() {
            Some("true")
        } else {
            None
        }
    });

    // Activate the item
    let key_for_action = key.clone();
    let activate = move || {
        if is_disabled.get_untracked() {
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
        if is_disabled.get_untracked() {
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
        if is_disabled.get_untracked() {
            return;
        }
        on_focus.run(Some(key_for_focus.clone()));
    });

    // Handle mouse enter (for hover focus)
    let key_for_hover = key.clone();
    let handle_mouseenter = move |_e: MouseEvent| {
        if is_disabled.get_untracked() {
            return;
        }
        on_focus.run(Some(key_for_hover.clone()));
    };

    // Use focus ring for keyboard focus visibility
    let UseFocusRingReturn {
        props: focus_ring_props,
        is_focus_visible,
        is_focused: _,
    } = use_focus_ring(UseFocusRingInput {
        disabled: is_disabled,
        within: false,
        auto_focus: false,
        on_focus: Some(focus_callback),
        on_blur: None,
        on_focus_change: None,
    });

    UseMenuItemReturn {
        item_props: UseMenuItemProps {
            role: "menuitem",
            tabindex,
            aria_disabled,
            on_click: EventHandler::new(handle_click),
            on_keydown: EventHandler::new(handle_keydown),
            on_focus: focus_ring_props.on_focus,
            on_blur: focus_ring_props.on_blur,
            on_focusin: focus_ring_props.on_focusin,
            on_focusout: focus_ring_props.on_focusout,
            on_mouseenter: EventHandler::new(handle_mouseenter),
            data_focus_visible: focus_ring_props.data_focus_visible,
            element_capture: focusable.props.element_capture,
        },
        is_focused,
        is_selected,
        is_disabled,
        is_focus_visible,
    }
}
