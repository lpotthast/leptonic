// Upstream: react-aria/src/menu/useMenuItem.ts @ 99e6102368
use crate::hooks::collections::CloseOnSelect;
use leptos::{
    attr::{self, Attr},
    ev,
    ev::{On, SharedEventCallback},
    prelude::*,
};
use wasm_bindgen::JsCast;
use web_sys::{KeyboardEvent, MouseEvent};

use super::{MenuData, SubmenuTriggerItem};
use crate::{
    hooks::{
        IntoAttrs, PropsWithStyles,
        collections::{
            Key, LinkBehavior, SelectionMode, UseSelectableItemAttrs, UseSelectableItemInput,
            UseSelectableItemProps, UseSelectableItemReturn, use_selectable_item,
        },
        focus::use_focus_visible::{
            Modality, UseFocusVisibleInput, get_modality, set_modality, use_focus_visible,
        },
        interactions::{
            use_hover::{UseHoverAttrs, UseHoverInput, UseHoverProps, use_hover},
            use_keyboard::{UseKeyboardAttrs, UseKeyboardInput, UseKeyboardProps, use_keyboard},
            use_press::{PressEvent, UsePressAttrs, UsePressInput, UsePressProps, use_press},
        },
    },
    utils::{
        CapturedElement, EventAccessors, EventHandler, SlotProps,
        aria::{AriaChecked, AriaDisabled, AriaHasPopup, AriaRole},
        keyboard_shortcut::{KeyboardShortcuts, Shortcut, ShortcutOutcome},
        pointer_type::PointerType,
        use_slot,
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
//
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

/// Return value of [`use_menu_item`].
pub struct UseMenuItemReturn {
    pub props: PropsWithStyles<UseMenuItemProps>,
    pub label_props: SlotProps,
    pub description_props: SlotProps,
    /// For an element showing the item's keyboard shortcut.
    pub keyboard_shortcut_props: SlotProps,
    pub is_focused: Signal<bool>,
    pub is_focus_visible: Signal<bool>,
    pub is_selected: Signal<bool>,
    pub is_pressed: Signal<bool>,
    pub is_disabled: Signal<bool>,
}

/// Props for the menu item element.
#[derive(Debug)]
pub struct UseMenuItemProps {
    pub role: AriaRole,
    pub aria_disabled: Signal<Option<AriaDisabled>>,
    pub aria_checked: Signal<Option<AriaChecked>>,
    pub aria_label: Option<String>,
    pub aria_labelledby: Signal<Option<String>>,
    pub aria_describedby: Signal<Option<String>>,
    pub aria_haspopup: Signal<Option<AriaHasPopup>>,
    pub aria_expanded: Signal<Option<&'static str>>,
    pub aria_controls: Signal<Option<String>>,
    pub item: UseSelectableItemProps,
    pub press: UsePressProps,
    pub hover: UseHoverProps,
    pub keyboard: UseKeyboardProps,
    /// Performs the item's action (all activations end in a click).
    pub on_click: EventHandler<MouseEvent>,
    /// A submenu trigger keeps DOM focus where it is on mouse down.
    pub on_mousedown: EventHandler<MouseEvent>,
}

pub type UseMenuItemAttrs = (
    (
        Attr<attr::Role, AriaRole>,
        Attr<attr::AriaDisabled, Signal<Option<AriaDisabled>>>,
        Attr<attr::AriaChecked, Signal<Option<AriaChecked>>>,
        Attr<attr::AriaLabel, Option<String>>,
        Attr<attr::AriaLabelledby, Signal<Option<String>>>,
        Attr<attr::AriaDescribedby, Signal<Option<String>>>,
        Attr<attr::AriaHaspopup, Signal<Option<AriaHasPopup>>>,
        Attr<attr::AriaExpanded, Signal<Option<&'static str>>>,
        Attr<attr::AriaControls, Signal<Option<String>>>,
    ),
    UseSelectableItemAttrs,
    UsePressAttrs,
    UseHoverAttrs,
    UseKeyboardAttrs,
    On<ev::click, SharedEventCallback<MouseEvent>>,
    On<ev::mousedown, SharedEventCallback<MouseEvent>>,
);

impl IntoAttrs for UseMenuItemProps {
    type Attrs = UseMenuItemAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            (
                Attr(attr::Role, self.role),
                Attr(attr::AriaDisabled, self.aria_disabled),
                Attr(attr::AriaChecked, self.aria_checked),
                Attr(attr::AriaLabel, self.aria_label),
                Attr(attr::AriaLabelledby, self.aria_labelledby),
                Attr(attr::AriaDescribedby, self.aria_describedby),
                Attr(attr::AriaHaspopup, self.aria_haspopup),
                Attr(attr::AriaExpanded, self.aria_expanded),
                Attr(attr::AriaControls, self.aria_controls),
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
    Keyboard { key: &'static str },
    Pointer(PointerType),
}

/// An item of a menu: `menuitem` (or `menuitemradio` / `menuitemcheckbox` in menus with
/// selection), activated by press, Enter or Space. Activating performs the menu's action and
/// closes the menu (see [`UseMenuItemInput::should_close_on_select`]).
#[allow(clippy::too_many_lines)]
pub fn use_menu_item(input: UseMenuItemInput) -> UseMenuItemReturn {
    crate::hooks::track_interaction_modality();
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
    } = menu;
    let selection = state.selection;

    let aria_label = untrack(|| {
        state.collection.with(|c| {
            c.get(&key)
                .and_then(|n| n.aria_label.as_deref().map(str::to_owned))
        })
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
        props: item_props,
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
        on_action: None,
        link_behavior: LinkBehavior::None,
        focus: None,
        should_use_virtual_focus: false,
        on_context_menu: None,
    });
    let (mut item_props, item_styles) = item_props.into_inner();
    if let Some(trigger) = &submenu_trigger {
        // A trigger keeps the item's focus handling only: it isn't selected by a press, and it is
        // named by its own id (the submenu's label). While its submenu is open, Shift+Tab leaves
        // the menu instead of moving to the trigger.
        item_props.id.clone_from(&trigger.id);
        item_props.press = use_press(UsePressInput {
            is_disabled: Signal::stored(true),
            ..UsePressInput::default()
        })
        .props
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
                interaction.set_value(Some(Interaction::Pointer(e.pointer_type.clone())));
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
        ..UsePressInput::default()
    });
    let (press_props, press_styles) = press.props.into_inner();

    // Hovering moves focus, unless the keyboard is in use (or it would leave an open subdialog).
    let is_subdialog_trigger = submenu_trigger
        .as_ref()
        .is_some_and(|trigger| trigger.aria_haspopup.get_untracked() == Some(AriaHasPopup::Dialog));
    let hover = use_hover(UseHoverInput {
        is_disabled,
        on_hover_change: submenu_trigger
            .as_ref()
            .map(|trigger| trigger.on_hover_change),
        on_hover_start: Some(Callback::new(move |_| {
            if get_modality() == Modality::Pointer
                && !(is_subdialog_trigger && is_trigger_expanded.get_untracked())
            {
                selection.set_focused(true);
                selection.set_focused_key(Some(key.get_value()), None);
            }
        })),
        ..UseHoverInput::default()
    })
    .props;

    // Enter and Space activate the item through a click, like a native control.
    let click_target =
        |e: &KeyboardEvent| e.expect_target().dyn_into::<web_sys::HtmlElement>().ok();
    let keyboard = use_keyboard(UseKeyboardInput {
        shortcuts: Some(
            KeyboardShortcuts::new()
                .on(Shortcut::key(" "), move |e| {
                    interaction.set_value(Some(Interaction::Keyboard { key: " " }));
                    if let Some(target) = click_target(e) {
                        target.click();
                    }
                    set_modality(Modality::Keyboard);
                })
                .on(Shortcut::key("Enter"), move |e| {
                    interaction.set_value(Some(Interaction::Keyboard { key: "Enter" }));
                    set_modality(Modality::Keyboard);
                    match click_target(e) {
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
                    }
                }),
        ),
        ..UseKeyboardInput::default()
    })
    .props;
    // A trigger's arrow keys open and close its submenu.
    let keyboard = match &submenu_trigger {
        Some(trigger) => {
            let (item, trigger) = (keyboard, trigger.keyboard.clone());
            UseKeyboardProps {
                on_keydown: EventHandler::new(move |e: KeyboardEvent| {
                    item.on_keydown.call(e.clone());
                    trigger.on_keydown.call(e);
                }),
                on_keyup: EventHandler::new(move |e: KeyboardEvent| {
                    item.on_keyup.call(e.clone());
                    trigger.on_keyup.call(e);
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
                key == "Enter" || mode == SelectionMode::None || is_link
            }
            _ => mode != SelectionMode::Multiple || is_link,
        });
        if should_close && let Some(on_close) = on_close {
            on_close.run(());
        }
        interaction.set_value(None);
    });

    let focus_visible = use_focus_visible(UseFocusVisibleInput::default()).focus_should_be_visible;

    let (aria_haspopup, aria_expanded, aria_controls) = match &submenu_trigger {
        Some(trigger) => {
            let is_open = trigger.is_open;
            (
                trigger.aria_haspopup,
                Signal::derive(move || Some(if is_open.get() { "true" } else { "false" })),
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
        props: PropsWithStyles::new(
            UseMenuItemProps {
                role: match untrack(|| selection.selection_mode()) {
                    _ if is_trigger => AriaRole::Menuitem,
                    SelectionMode::None => AriaRole::Menuitem,
                    SelectionMode::Single => AriaRole::Menuitemradio,
                    SelectionMode::Multiple => AriaRole::Menuitemcheckbox,
                },
                aria_disabled: Signal::derive(move || {
                    is_disabled.get().then_some(AriaDisabled::True)
                }),
                aria_checked: Signal::derive(move || {
                    (!is_trigger && selection.selection_mode() != SelectionMode::None)
                        .then(|| AriaChecked::from(is_selected.get()))
                }),
                aria_label,
                aria_labelledby: label.referenced_id,
                aria_describedby: Signal::derive(move || {
                    let ids: Vec<String> = [description_id.get(), keyboard_id.get()]
                        .into_iter()
                        .flatten()
                        .collect();
                    (!ids.is_empty()).then(|| ids.join(" "))
                }),
                item: item_props,
                press: press_props,
                hover,
                aria_haspopup,
                aria_expanded,
                aria_controls,
                keyboard,
                on_click,
                on_mousedown: EventHandler::new(move |e: MouseEvent| {
                    if is_trigger {
                        e.prevent_default();
                    }
                }),
            },
            item_styles.merge(press_styles),
        ),
        label_props: label.props,
        description_props: description.props,
        keyboard_shortcut_props: keyboard_shortcut.props,
        is_focused,
        is_focus_visible: Signal::derive(move || {
            is_focused.get() && focus_visible.get() && !is_trigger_expanded.get()
        }),
        is_selected,
        is_pressed: press.is_pressed,
        is_disabled,
    }
}
