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
use uuid::Uuid;
use web_sys::{FocusEvent, KeyboardEvent, MouseEvent};

use crate::{
    hooks::{
        IntoAttrs,
        focus::{
            use_focus_ring::{UseFocusRingInput, UseFocusRingReturn, use_focus_ring},
            use_focus_visible::{Modality, get_modality},
            use_focusable::{UseFocusableInput, use_focusable},
        },
        selection::{
            SelectionKey,
            use_selection_state::{Selection, SelectionMode},
        },
    },
    utils::{
        EventHandler,
        aria::AriaRole,
        element_capture::ElementCaptureAttr,
        scroll::{ScrollIntoViewportOpts, get_scroll_parent, scroll_into_viewport},
        slot_id::{join_slot_ids, use_slot_id},
    },
};

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/@react-aria/menu/src/useMenuItem.ts

// 1. `close_on_select` is `Option<bool>` with smart defaults instead of `bool`.
//    When `None`, close behavior varies by trigger and selection mode:
//
//      Trigger     | Action-only | Single-select | Multi-select
//      ------------|-------------|---------------|-------------
//      Enter key   | Close       | Close         | Close
//      Space key   | Close       | Don't close   | Don't close
//      Click       | Close       | Close         | Don't close
//
//    This matches react-aria's close matrix from `useMenuItem.ts:224-228`.
//
// 2. `is_pressed` state from `usePress` is not yet tracked. (Deferred)
//
// 3. Drag-from-trigger-to-item pointer behavior is not yet supported. (Deferred)

/// Input parameters for the `use_menu_item` hook.
#[derive(Clone)]
pub struct UseMenuItemInput<K>
where
    K: SelectionKey,
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
    ///
    /// - `Some(true)` — always close after activation.
    /// - `Some(false)` — never close after activation.
    /// - `None` (default) — smart behavior: keyboard Enter always closes;
    ///   multi-select with click stays open; action/single-select always closes.
    pub close_on_select: Option<bool>,

    /// The selection mode of the parent menu. Determines the ARIA role:
    /// - `None` → `"menuitem"` (action)
    /// - `Single` → `"menuitemradio"`
    /// - `Multiple` → `"menuitemcheckbox"`
    pub selection_mode: SelectionMode,

    /// Whether the consumer renders a description slot element.
    /// When `false`, the description ID is excluded from `aria-describedby`
    /// to avoid dangling ARIA references.
    pub has_description: bool,

    /// Whether the consumer renders a keyboard shortcut slot element.
    /// When `false`, the keyboard shortcut ID is excluded from `aria-describedby`
    /// to avoid dangling ARIA references.
    pub has_keyboard_shortcut: bool,
}

/// The return value of the `use_menu_item` hook.
pub struct UseMenuItemReturn {
    /// Props for the menu item element. Call `.into_attrs()` for view spreading.
    pub item_props: UseMenuItemProps,

    /// Props for the label slot element. Spread onto the element containing the item's label text.
    /// This connects the label to the item via `aria-labelledby`.
    pub label_props: UseMenuItemLabelProps,

    /// Props for the description slot element. Spread onto an element containing
    /// supplementary description text. This connects it via `aria-describedby`.
    pub description_props: UseMenuItemDescriptionProps,

    /// Props for the keyboard shortcut slot element. Spread onto an element displaying
    /// a keyboard shortcut hint (e.g., "Ctrl+C"). Connects via `aria-describedby`.
    pub keyboard_shortcut_props: UseMenuItemKeyboardShortcutProps,

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
    pub id: String,
    pub role: AriaRole,
    pub tabindex: Signal<i32>,
    pub aria_disabled: Signal<Option<&'static str>>,
    pub aria_checked: Signal<Option<&'static str>>,
    pub aria_labelledby: Option<String>,
    pub aria_describedby: Signal<Option<String>>,
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
            Attr(attr::Id, self.id),
            Attr(attr::Role, self.role),
            Attr(attr::Tabindex, self.tabindex),
            Attr(attr::AriaDisabled, self.aria_disabled),
            Attr(attr::AriaChecked, self.aria_checked),
            Attr(attr::AriaLabelledby, self.aria_labelledby),
            Attr(attr::AriaDescribedby, self.aria_describedby),
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
    Attr<attr::Id, String>,
    Attr<attr::Role, AriaRole>,
    Attr<attr::Tabindex, Signal<i32>>,
    Attr<attr::AriaDisabled, Signal<Option<&'static str>>>,
    Attr<attr::AriaChecked, Signal<Option<&'static str>>>,
    Attr<attr::AriaLabelledby, Option<String>>,
    Attr<attr::AriaDescribedby, Signal<Option<String>>>,
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

/// Props for the label slot element inside a menu item.
#[derive(Debug)]
pub struct UseMenuItemLabelProps {
    /// The id for the label element.
    pub id: String,
}

impl IntoAttrs for UseMenuItemLabelProps {
    type Attrs = UseMenuItemLabelAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (Attr(attr::Id, self.id),)
    }
}

/// Attributes for the menu item label slot.
pub type UseMenuItemLabelAttrs = (Attr<attr::Id, String>,);

/// Props for the description slot element inside a menu item.
#[derive(Debug)]
pub struct UseMenuItemDescriptionProps {
    /// The id for the description element.
    pub id: String,
}

impl IntoAttrs for UseMenuItemDescriptionProps {
    type Attrs = UseMenuItemDescriptionAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (Attr(attr::Id, self.id),)
    }
}

/// Attributes for the menu item description slot.
pub type UseMenuItemDescriptionAttrs = (Attr<attr::Id, String>,);

/// Props for the keyboard shortcut slot element inside a menu item.
#[derive(Debug)]
pub struct UseMenuItemKeyboardShortcutProps {
    /// The id for the keyboard shortcut element.
    pub id: String,
}

impl IntoAttrs for UseMenuItemKeyboardShortcutProps {
    type Attrs = UseMenuItemKeyboardShortcutAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (Attr(attr::Id, self.id),)
    }
}

/// Attributes for the menu item keyboard shortcut slot.
pub type UseMenuItemKeyboardShortcutAttrs = (Attr<attr::Id, String>,);

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
///     has_description: false,
///     has_keyboard_shortcut: false,
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
    K: SelectionKey,
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
        selection_mode,
        has_description,
        has_keyboard_shortcut,
    } = input;

    // Generate unique IDs for accessible slot elements
    let base_id = Uuid::new_v4();
    let item_id = format!("menuitem-{base_id}");
    let label_id = format!("menuitem-label-{base_id}");
    let description_id = format!("menuitem-desc-{base_id}");
    let keyboard_shortcut_id = format!("menuitem-kbd-{base_id}");

    // Build aria-labelledby and aria-describedby referencing the slot IDs.
    // Use use_slot_id to avoid dangling ARIA references when slots are not rendered.
    let aria_labelledby = Some(label_id.clone());
    let slot_desc = use_slot_id(description_id.clone(), Signal::stored(has_description));
    let slot_kbd = use_slot_id(
        keyboard_shortcut_id.clone(),
        Signal::stored(has_keyboard_shortcut),
    );
    let aria_describedby = join_slot_ids(&[slot_desc, slot_kbd]);

    // Determine ARIA role based on selection mode
    let role: AriaRole = match selection_mode {
        SelectionMode::None => AriaRole::Menuitem,
        SelectionMode::Single => AriaRole::Menuitemradio,
        SelectionMode::Multiple => AriaRole::Menuitemcheckbox,
    };

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
            if let Some(el) = focus_handle.get_element() {
                let el: &web_sys::Element = &el;
                scroll_into_viewport(
                    Some(el),
                    &ScrollIntoViewportOpts {
                        containing_element: Some(get_scroll_parent(el, true)),
                    },
                );
            }
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

    // Compute aria-disabled (ARIA string, not boolean attribute)
    let aria_disabled = Signal::derive(move || {
        if is_disabled.get() {
            Some("true")
        } else {
            None
        }
    });

    // Compute aria-checked for selectable items (radio/checkbox roles)
    let aria_checked: Signal<Option<&'static str>> = match selection_mode {
        SelectionMode::None => Signal::derive(|| None),
        SelectionMode::Single | SelectionMode::Multiple => Signal::derive(move || {
            if is_selected.get() {
                Some("true")
            } else {
                Some("false")
            }
        }),
    };

    // Determine if menu should close after activation.
    // Resolves `close_on_select: Option<bool>` with smart defaults per trigger type.
    // See deviation comment #1 for the full close matrix.
    let should_close_on_enter = close_on_select.unwrap_or(true);
    let should_close_on_space = close_on_select.unwrap_or(selection_mode == SelectionMode::None);
    let should_close_on_click = close_on_select.unwrap_or(
        // Default: multi-select click stays open; action/single-select closes.
        selection_mode != SelectionMode::Multiple,
    );

    // Activate via keyboard (Enter/Space)
    let key_for_keyboard_action = key.clone();
    let activate_keyboard = move |should_close: bool| {
        if is_disabled.get_untracked() {
            return;
        }
        on_action.run(key_for_keyboard_action.clone());
        if should_close {
            if let Some(on_close) = on_close {
                on_close.run(());
            }
        }
    };

    // Activate via click/pointer
    let key_for_click_action = key.clone();
    let activate_click = move || {
        if is_disabled.get_untracked() {
            return;
        }
        on_action.run(key_for_click_action.clone());
        if should_close_on_click {
            if let Some(on_close) = on_close {
                on_close.run(());
            }
        }
    };

    // Handle click
    let handle_click = move |_e: MouseEvent| {
        activate_click();
    };

    // Handle keydown
    // - Enter/Space: activate and stop propagation (prevents double-fire at menu level)
    // - Ignores e.repeat to prevent accidental activation from held-down keys on trigger
    // - Other keys: let them bubble to menu for navigation/type-ahead
    let handle_keydown = move |e: KeyboardEvent| {
        if is_disabled.get_untracked() {
            return;
        }

        match e.key().as_str() {
            "Enter" => {
                if e.repeat() {
                    return;
                }
                e.prevent_default();
                e.stop_propagation();
                activate_keyboard(should_close_on_enter);
            }
            " " => {
                if e.repeat() {
                    return;
                }
                e.prevent_default();
                e.stop_propagation();
                activate_keyboard(should_close_on_space);
            }
            _ => {
                // Let other keys bubble up to the menu for navigation/type-ahead
            }
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

    // Handle mouse enter (for hover focus).
    // Only set focus when in pointer modality to avoid disrupting keyboard navigation.
    // React Aria guards this with `!isFocusVisible()` (useMenuItem.ts:266).
    let key_for_hover = key.clone();
    let handle_mouseenter = move |_e: MouseEvent| {
        if is_disabled.get_untracked() {
            return;
        }
        if get_modality() == Modality::Pointer {
            on_focus.run(Some(key_for_hover.clone()));
        }
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
        is_text_input: false,
        on_focus: Some(focus_callback),
        on_blur: None,
        on_focus_change: None,
    });

    UseMenuItemReturn {
        item_props: UseMenuItemProps {
            id: item_id,
            role,
            tabindex,
            aria_disabled,
            aria_checked,
            aria_labelledby,
            aria_describedby,
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
        label_props: UseMenuItemLabelProps { id: label_id },
        description_props: UseMenuItemDescriptionProps { id: description_id },
        keyboard_shortcut_props: UseMenuItemKeyboardShortcutProps {
            id: keyboard_shortcut_id,
        },
        is_focused,
        is_selected,
        is_disabled,
        is_focus_visible,
    }
}
