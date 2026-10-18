// Upstream: react-aria/src/menu/useMenuItem.ts @ 99e6102368
// Upstream: react-aria/test/menu/useMenu.test.tsx @ 99e6102368
// Upstream: react-aria-components/test/Menu.test.tsx @ 99e6102368
// Upstream: @adobe/react-spectrum/test/menu/Menu.test.js @ 99e6102368
use leptos::{ev, prelude::*};
use wasm_bindgen::JsCast;
use web_sys::{KeyboardEvent, MouseEvent};

use super::{MenuData, SubmenuTriggerItem};
use crate::{
    CapturedElement, EventHandler, Id, IdRefs, IntoAttrs, OnEvent, Part, PropsWithStyles,
    SlotProps,
    hooks::{
        collections::{
            CloseOnSelect, Key, LinkBehavior, SelectionMode, UseNodeAriaLabelInput,
            UseSelectableItemAttrs, UseSelectableItemInput, UseSelectableItemProps,
            UseSelectableItemReturn, use_node_aria_label, use_selectable_item,
        },
        focus::use_focus_visible::{Modality, get_modality, is_focus_visible, set_modality},
        interactions::{
            use_hover::{UseHoverAttrs, UseHoverInput, UseHoverProps, use_hover},
            use_keyboard::{UseKeyboardAttrs, UseKeyboardInput, UseKeyboardProps, use_keyboard},
            use_press::{PressEvent, UsePressAttrs, UsePressInput, UsePressProps, use_press},
        },
    },
    sharing_one_effect, use_slot,
    utils::{
        aria::{AriaChecked, AriaDisabled, AriaExpanded, AriaHasPopup, AriaRole, aria_true},
        dom_ext::EventAccessors,
        key::KeyboardKey,
        keyboard_shortcut::{KeyboardShortcuts, Shortcut, ShortcutOutcome},
        part::Remount,
        pointer_type::PointerType,
        signal_attr::{SignalAttr, signal_attr},
    },
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - Items read their settings from the menu (`MenuData`) and their label, disabled state and
//   link from the collection node. react-aria's deprecated per-item `isDisabled`,
//   `isSelected`, `onAction`, `onClose` and `closeOnSelect` are not offered.
// - Label, description and keyboard shortcut ids are referenced only while those elements are
//   rendered (react-aria: `useSlotId`), detected through element capture.
// - A submenu trigger gets its trigger's behavior as `submenu_trigger` (react-aria: through the
//   props `aria-haspopup`, `aria-expanded`, `onPressStart`, ... from `useSubmenuTrigger`).
//
// ## OMITTED FEATURES
// - Virtual focus.
// - Per-item press, hover, keyboard and focus callbacks.
//
// =============================================================================

/// Input of [`use_menu_item`].
#[derive(Debug, Clone)]
pub struct UseMenuItemInput {
    /// The menu (from `use_menu`).
    pub menu: MenuData,
    /// The item's key in the menu's collection.
    pub key: Key,
    /// Close the menu after the item was activated. `None`: unless the menu allows multiple
    /// selection, or the item was checked with Space.
    pub should_close_on_select: CloseOnSelect,
    /// Makes the item open a submenu (from `use_submenu_trigger`): it has no action, never closes
    /// the menu and isn't selectable.
    pub submenu_trigger: Option<SubmenuTriggerItem>,
}

/// The return value of [`use_menu_item`].
pub struct UseMenuItemReturn {
    /// The menu item element: `let (attrs, styles) = item.attrs();`.
    pub item: Part<PropsWithStyles<UseMenuItemProps>>,
    /// The item's label, which labels the item while it is rendered.
    pub label: Part<SlotProps>,
    /// The item's description, which describes the item while it is rendered.
    pub description: Part<SlotProps>,
    /// An element showing the item's keyboard shortcut.
    pub keyboard_shortcut: Part<SlotProps>,
    pub is_focused: Signal<bool>,
    pub is_focus_visible: Signal<bool>,
    pub is_selected: Signal<bool>,
    pub is_pressed: Signal<bool>,
    pub is_hovered: Signal<bool>,
    pub is_disabled: Signal<bool>,
}

/// Props for the menu item element.
#[derive(Debug)]
pub struct UseMenuItemProps {
    /// Follows the menu's selection mode.
    pub role: Signal<AriaRole>,
    pub aria_disabled: Signal<Option<AriaDisabled>>,
    pub aria_checked: Signal<Option<AriaChecked>>,
    pub aria_label: Signal<Option<String>>,
    pub aria_labelledby: Signal<Option<Id>>,
    pub aria_describedby: Signal<Option<Id>>,
    pub aria_haspopup: Signal<Option<AriaHasPopup>>,
    pub aria_expanded: Signal<Option<AriaExpanded>>,
    pub aria_controls: Signal<Option<Id>>,
    pub item: UseSelectableItemProps,
    pub press: UsePressProps,
    pub hover: UseHoverProps,
    pub keyboard: UseKeyboardProps,
    /// Performs the item's action (all activations end in a click).
    pub on_click: EventHandler<MouseEvent>,
    /// A submenu trigger keeps DOM focus where it is on mouse down.
    pub on_mousedown: EventHandler<MouseEvent>,
}

impl Remount for UseMenuItemProps {
    fn remount(&self) -> Self {
        Self {
            role: self.role,
            aria_disabled: self.aria_disabled,
            aria_checked: self.aria_checked,
            aria_label: self.aria_label,
            aria_labelledby: self.aria_labelledby,
            aria_describedby: self.aria_describedby,
            aria_haspopup: self.aria_haspopup,
            aria_expanded: self.aria_expanded,
            aria_controls: self.aria_controls,
            item: self.item.remount(),
            press: self.press.remount(),
            hover: self.hover.remount(),
            keyboard: self.keyboard.remount(),
            on_click: self.on_click.clone(),
            on_mousedown: self.on_mousedown.clone(),
        }
    }
}

pub type UseMenuItemAttrs = (
    (
        SignalAttr<AriaRole>,
        SignalAttr<Option<AriaDisabled>>,
        SignalAttr<Option<AriaChecked>>,
        SignalAttr<Option<String>>,
        SignalAttr<Option<Id>>,
        SignalAttr<Option<Id>>,
        SignalAttr<Option<AriaHasPopup>>,
        SignalAttr<Option<AriaExpanded>>,
        SignalAttr<Option<Id>>,
    ),
    UseSelectableItemAttrs,
    UsePressAttrs,
    UseHoverAttrs,
    UseKeyboardAttrs,
    OnEvent<ev::click>,
    OnEvent<ev::mousedown>,
);

impl IntoAttrs for UseMenuItemProps {
    type Attrs = UseMenuItemAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            (
                signal_attr("role", self.role),
                signal_attr("aria-disabled", self.aria_disabled),
                signal_attr("aria-checked", self.aria_checked),
                signal_attr("aria-label", self.aria_label),
                signal_attr("aria-labelledby", self.aria_labelledby),
                signal_attr("aria-describedby", self.aria_describedby),
                signal_attr("aria-haspopup", self.aria_haspopup),
                signal_attr("aria-expanded", self.aria_expanded),
                signal_attr("aria-controls", self.aria_controls),
            ),
            self.item.into_attrs(),
            self.press.into_attrs(),
            self.hover.into_attrs(),
            self.keyboard.into_attrs(),
            self.on_click.into_on(ev::click),
            self.on_mousedown.into_on(ev::mousedown),
        )
    }
}

/// How an item was activated, for deciding whether to close the menu.
#[derive(Debug, Clone, PartialEq)]
enum Interaction {
    Keyboard { key: KeyboardKey },
    Pointer(PointerType),
}

/// An item of a menu: `menuitem` (or `menuitemradio` / `menuitemcheckbox` in menus with
/// selection), activated by press, Enter or Space. Activating performs the menu's action and
/// closes the menu (see [`UseMenuItemInput::should_close_on_select`]).
pub fn use_menu_item(input: UseMenuItemInput) -> UseMenuItemReturn {
    crate::metrics::subject!();
    // `use_selectable_item`'s resets and `use_hover`'s react to the disabled state in one effect.
    sharing_one_effect(|| menu_item(input))
}

/// The body of [`use_menu_item`].
fn menu_item(input: UseMenuItemInput) -> UseMenuItemReturn {
    crate::hooks::focus::use_focus_visible::track_interaction_modality();
    let UseMenuItemInput {
        menu,
        key,
        should_close_on_select,
        submenu_trigger,
    } = input;
    let is_trigger = submenu_trigger.is_some();
    let is_trigger_expanded = submenu_trigger
        .as_ref()
        .map_or_else(|| Signal::stored(false), |trigger| trigger.is_open);
    let MenuData {
        state,
        collection_id,
        on_action,
        on_close,
        ..
    } = menu;
    let selection = state.selection;

    if !untrack(|| state.collection.with(|c| c.contains_key(&key))) {
        crate::utils::dev_warn!("use_menu_item: the key {key:?} is not in the menu's collection");
    }
    let aria_label = use_node_aria_label(UseNodeAriaLabelInput {
        selection: state.selection,
        key: key.clone(),
    });

    let label = use_slot("label");
    let description = use_slot("description");
    let keyboard_shortcut = use_slot("keyboard-shortcut");
    let (description_id, keyboard_id) =
        (description.referenced_id, keyboard_shortcut.referenced_id);

    let element = submenu_trigger
        .as_ref()
        .map_or_else(CapturedElement::new, |trigger| trigger.element);
    let UseSelectableItemReturn {
        item: item_props,
        is_selected,
        is_focused,
        is_disabled,
        ..
    } = use_selectable_item(UseSelectableItemInput {
        selection,
        item_elements: state.item_elements,
        key: key.clone(),
        element,
        id: None,
        collection_id,
        is_disabled: Signal::stored(false),
        should_select_on_press_up: true,
        allows_different_press_origin: true,
        on_action: Signal::stored(None),
        link_behavior: Signal::stored(LinkBehavior::None),
        focus: None,
        should_use_virtual_focus: false,
        on_context_menu: None,
    });
    let (mut item_props, item_styles) = item_props.into_props().into_inner();
    if let Some(trigger) = &submenu_trigger {
        // A trigger keeps the item's focus handling only: it isn't selected by a press, and it is
        // named by its own id (the submenu's label). While its submenu is open, Shift+Tab leaves
        // the menu instead of moving to the trigger.
        item_props.id.clone_from(&trigger.id);
        item_props.press = use_press(UsePressInput {
            is_disabled: Signal::stored(true),
            propagation: crate::hooks::interactions::PressPropagation::Stop,
            allow_text_selection_on_press: Signal::stored(false),
            should_cancel_on_pointer_exit: Signal::stored(false),
            prevent_focus_on_press: Signal::stored(false),
            force_is_pressed: Signal::stored(false),
            on_press: None,
            on_press_up: None,
            on_press_start: None,
            on_press_end: None,
            on_press_change: None,
            on_double_press: None,
            long_press: None,
        })
        .target
        .into_props()
        .into_inner()
        .0;
        let tabindex = item_props.tabindex;
        item_props.tabindex = Signal::derive(move || {
            tabindex
                .get()
                .map(|index| if is_trigger_expanded.get() { -1 } else { index })
        });
    }

    let key = StoredValue::new(key);
    let interaction: StoredValue<Option<Interaction>> = StoredValue::new(None);
    let is_pressed_now = StoredValue::new(false);

    // A press that started elsewhere (e.g. on the trigger, dragging into the menu) and ends on
    // the item activates it.
    let press = use_press(UsePressInput {
        is_disabled,
        on_press_start: submenu_trigger
            .as_ref()
            .map(|trigger| trigger.on_press_start),
        on_press: submenu_trigger.as_ref().map(|trigger| trigger.on_press),
        on_press_up: Some(Callback::new(move |e: PressEvent| {
            if e.pointer_type != PointerType::Keyboard {
                interaction.set_value(Some(Interaction::Pointer(e.pointer_type)));
            }
            if e.pointer_type == PointerType::Mouse
                && !is_pressed_now.get_value()
                && let Some(target) = e.target.dyn_ref::<web_sys::HtmlElement>()
            {
                target.click();
            }
        })),
        on_press_change: Some(Callback::new(move |pressed| {
            is_pressed_now.set_value(pressed);
        })),
        propagation: crate::hooks::interactions::PressPropagation::Stop,
        allow_text_selection_on_press: Signal::stored(false),
        should_cancel_on_pointer_exit: Signal::stored(false),
        prevent_focus_on_press: Signal::stored(false),
        force_is_pressed: Signal::stored(false),
        on_press_end: None,
        on_double_press: None,
        long_press: None,
    });
    let (press_props, press_styles) = press.target.into_props().into_inner();

    // Hovering moves focus, unless the keyboard is in use, or the item is the trigger of the
    // open submenu (focus stays in the submenu).
    let hover = use_hover(UseHoverInput {
        is_disabled,
        on_hover_change: submenu_trigger
            .as_ref()
            .map(|trigger| trigger.on_hover_change),
        on_hover_start: Some(Callback::new(move |_| {
            if get_modality() == Some(Modality::Pointer) && !is_trigger_expanded.get_untracked() {
                selection.set_focused(true);
                selection.set_focused_key(Some(key.get_value()), None);
            }
        })),
        on_hover_end: None,
    });
    let hover_props = hover.target.into_props();
    let is_hovered = hover.is_hovered;
    let hover = hover_props;

    // Enter and Space activate the item through a click, like a native control.
    let click_target =
        |e: &KeyboardEvent| e.expect_target().dyn_into::<web_sys::HtmlElement>().ok();
    let keyboard = use_keyboard(UseKeyboardInput {
        shortcuts: Some(
            KeyboardShortcuts::new()
                // The click counts as virtual: the modality is the keyboard's again afterwards
                // (focus moves into an opened submenu, focus rings show).
                .on(Shortcut::new(KeyboardKey::Space), move |e| {
                    interaction.set_value(Some(Interaction::Keyboard {
                        key: KeyboardKey::Space,
                    }));
                    if let Some(target) = click_target(e) {
                        target.click();
                    }
                    set_modality(Modality::Keyboard);
                })
                .on(Shortcut::new(KeyboardKey::Enter), move |e| {
                    interaction.set_value(Some(Interaction::Keyboard {
                        key: KeyboardKey::Enter,
                    }));
                    let outcome = match click_target(e) {
                        // A link navigates by itself on Enter.
                        Some(target) if target.tag_name().eq_ignore_ascii_case("a") => {
                            ShortcutOutcome::Custom {
                                prevent_default: false,
                                continue_propagation: false,
                            }
                        }
                        Some(target) => {
                            target.click();
                            ShortcutOutcome::Handled
                        }
                        None => ShortcutOutcome::Handled,
                    };
                    set_modality(Modality::Keyboard);
                    outcome
                }),
        ),
        ..UseKeyboardInput::default()
    })
    .target
    .into_props();
    // A trigger's arrow keys open and close its submenu.
    let keyboard = match &submenu_trigger {
        Some(trigger) => {
            let (trigger_keydown, trigger_keyup) =
                (trigger.on_keydown.clone(), trigger.on_keyup.clone());
            UseKeyboardProps {
                on_keydown: EventHandler::new(move |e: KeyboardEvent| {
                    keyboard.on_keydown.call(e.clone());
                    trigger_keydown.call(e);
                }),
                on_keyup: EventHandler::new(move |e: KeyboardEvent| {
                    keyboard.on_keyup.call(e.clone());
                    trigger_keyup.call(e);
                }),
            }
        }
        None => keyboard,
    };

    let on_click = EventHandler::new(move |_: MouseEvent| {
        // A trigger has no action and keeps the menu open.
        if is_disabled.get_untracked() || is_trigger {
            return;
        }
        let key = key.get_value();
        if let Some(on_action) = on_action {
            on_action.run(key.clone());
        }
        let mode = untrack(|| selection.selection_mode());
        let is_link = untrack(|| selection.is_link(&key));
        let should_close = should_close_on_select.resolve(|| match interaction.get_value() {
            // Enter always closes; Space only where it doesn't toggle a selection.
            Some(Interaction::Keyboard { key }) => {
                key == KeyboardKey::Enter || mode == SelectionMode::None || is_link
            }
            _ => mode != SelectionMode::Multiple || is_link,
        });
        if should_close && let Some(on_close) = on_close {
            on_close.run(());
        }
        interaction.set_value(None);
    });

    let (aria_haspopup, aria_expanded, aria_controls) = match &submenu_trigger {
        Some(trigger) => {
            let is_open = trigger.is_open;
            (
                trigger.aria_haspopup,
                Signal::derive(move || Some(AriaExpanded::from(is_open.get()))),
                trigger.aria_controls,
            )
        }
        None => (
            Signal::stored(None),
            Signal::stored(None),
            Signal::stored(None),
        ),
    };
    UseMenuItemReturn {
        item: Part::stored(PropsWithStyles::new(
            UseMenuItemProps {
                role: Signal::derive(move || match selection.selection_mode() {
                    _ if is_trigger => AriaRole::Menuitem,
                    SelectionMode::None => AriaRole::Menuitem,
                    SelectionMode::Single => AriaRole::Menuitemradio,
                    SelectionMode::Multiple => AriaRole::Menuitemcheckbox,
                }),
                aria_disabled: aria_true(is_disabled),
                aria_checked: Signal::derive(move || {
                    (!is_trigger && selection.selection_mode() != SelectionMode::None)
                        .then(|| AriaChecked::from(is_selected.get()))
                }),
                aria_label: aria_label.into(),
                aria_labelledby: label.referenced_id,
                aria_describedby: IdRefs::derive([description_id, keyboard_id]),
                item: item_props,
                press: press_props,
                hover,
                aria_haspopup,
                aria_expanded,
                aria_controls,
                keyboard,
                on_click,
                on_mousedown: if is_trigger {
                    EventHandler::new(|e: MouseEvent| e.prevent_default())
                } else {
                    EventHandler::empty()
                },
            },
            item_styles.merge(press_styles),
        )),
        label: label.part,
        description: description.part,
        keyboard_shortcut: keyboard_shortcut.part,
        is_focused,
        // A derive (read once, by the item's element): only the focused item reads the modality.
        is_focus_visible: Signal::derive(move || {
            is_focused.get() && is_focus_visible() && !is_trigger_expanded.get()
        }),
        is_selected,
        is_pressed: press.is_pressed,
        is_hovered,
        is_disabled,
    }
}
