// Upstream: react-aria/src/selection/useSelectableItem.ts @ 99e6102368
// Upstream: react-aria/test/selection/useSelectableCollection.test.js @ 99e6102368
// Upstream: react-aria-components/test/ListBox.test.js @ 99e6102368
use leptos::{
    attr::{
        self, Attr,
        custom::{CustomAttr, custom_attribute},
    },
    ev,
    prelude::*,
};
use wasm_bindgen::JsCast;
use web_sys::{DragEvent, FocusEvent, MouseEvent, PointerEvent};

use super::{
    ItemElements, Key, SelectionBehavior, SelectionManager, SelectionMode,
    modifiers::{is_ctrl_key_pressed, is_non_contiguous_selection_modifier},
};
use crate::{
    CapturedElement, ElementCaptureAttr, EventHandler, IntoAttrs, Modifiers, OnEvent,
    PropsWithStyles,
    hooks::interactions::{
        use_context_menu::{
            ContextMenuEvent, UseContextMenuAttrs, UseContextMenuInput, UseContextMenuProps,
            UseContextMenuReturn, use_context_menu,
        },
        use_press::{
            LongPress, LongPressEvent, PressEvent, UsePressAttrs, UsePressInput, UsePressProps,
            use_press,
        },
    },
    utils::{
        dom_ext::EventAccessors, focus::focus_safely, focusability::is_tabbable, id::use_id,
        key::KeyboardKey, modifiers::EventModifiers, open_link::is_opening_link,
        pointer_type::PointerType, shadow_dom::get_active_element,
        virtual_focus::move_virtual_focus,
    },
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - The item registers its element in the collection's `ItemElements` registry instead of
//   rendering `data-key` for DOM queries.
// - `is_disabled` is a signal combined with the selection manager's disabled keys.
//
// ## OMITTED FEATURES
// - Client-side router integration: links open through a synthetic click, which Leptos' router
//   intercepts like any other link click.
//
// =============================================================================

/// What happens when a link item is interacted with.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum LinkBehavior {
    /// Pressing the item opens the link; selection (if any) needs a modifier, a checkbox or a
    /// long press.
    #[default]
    Action,
    /// Selecting the item opens the link (e.g. navigation tabs).
    Selection,
    /// Pressing the item opens the link and never selects it.
    Override,
    /// The item is not handled as a link.
    None,
}

/// Moves DOM focus to an item in place of focusing the item element, e.g. to a grid row's cell or
/// child. Called when the item becomes the focused key.
#[derive(Clone)]
pub struct FocusItem(std::sync::Arc<dyn Fn() + Send + Sync>);

impl FocusItem {
    pub fn new(focus: impl Fn() + Send + Sync + 'static) -> Self {
        Self(std::sync::Arc::new(focus))
    }

    /// Moves the focus.
    pub fn focus(&self) {
        (self.0)();
    }
}

impl std::fmt::Debug for FocusItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("FocusItem(..)")
    }
}

/// Input of [`use_selectable_item`].
#[derive(Clone, Debug)]
pub struct UseSelectableItemInput {
    pub selection: SelectionManager,
    pub item_elements: ItemElements,
    pub key: Key,
    /// The item element; the hook's props capture it.
    pub element: CapturedElement,
    /// The element id. Generated when `None`.
    pub id: Option<String>,
    /// The `collection_id` of the collection the item belongs to (from
    /// `use_selectable_collection`).
    pub collection_id: String,
    /// Disables the item, in addition to the selection manager's disabled keys.
    pub is_disabled: Signal<bool>,
    /// Select when the press ends instead of when it starts (menus, select popovers).
    pub should_select_on_press_up: bool,
    /// Allow a press that started elsewhere (e.g. on a menu trigger) to select the item when it
    /// ends on it.
    pub allows_different_press_origin: bool,
    /// The item's action, e.g. opening a detail view. Without a selection mode, pressing performs
    /// it; with one, double-click / Enter does (in `Replace` behavior). Reactive: an item can gain
    /// or lose its action (a tree row toggling once it has children).
    pub on_action: Signal<Option<Callback<()>>>,
    /// Reactive: e.g. a listbox's follows its selection behavior.
    pub link_behavior: Signal<LinkBehavior>,
    /// Focuses the item when it becomes the focused key. `None`: focusing the element.
    pub focus: Option<FocusItem>,
    /// DOM focus stays elsewhere (e.g. in a combo box input); the item is focused virtually.
    pub should_use_virtual_focus: bool,
    /// Called when a context menu is requested on the item (right click, Shift+F10, the
    /// context menu key; a long press on iOS unless it selects): the item's own replaces the
    /// browser's.
    pub on_context_menu: Option<Callback<ContextMenuEvent>>,
}

/// Return value of [`use_selectable_item`].
pub struct UseSelectableItemReturn {
    pub props: PropsWithStyles<UseSelectableItemProps>,
    pub is_pressed: Signal<bool>,
    pub is_selected: Signal<bool>,
    /// Whether the item has keyboard focus within the focused collection.
    pub is_focused: Signal<bool>,
    pub is_disabled: Signal<bool>,
    /// Whether pressing the item can select it.
    pub allows_selection: Signal<bool>,
    /// Whether the item has an action (or link) to perform.
    pub has_action: Signal<bool>,
}

/// Props for the item element.
#[derive(Debug)]
pub struct UseSelectableItemProps {
    pub id: String,
    /// `0` for the focused item, `-1` for others, none for disabled items.
    pub tabindex: Signal<Option<i32>>,
    pub collection_id: String,
    pub element_capture: ElementCaptureAttr,
    pub press: UsePressProps,
    pub on_focus: EventHandler<FocusEvent>,
    pub on_dragstart_capture: EventHandler<DragEvent>,
    pub context_menu: UseContextMenuProps,
}

pub type UseSelectableItemAttrs = (
    Attr<attr::Id, String>,
    Attr<attr::Tabindex, Signal<Option<i32>>>,
    CustomAttr<&'static str, String>,
    ElementCaptureAttr,
    UsePressAttrs,
    OnEvent<ev::focus>,
    OnEvent<ev::Capture<ev::dragstart>>,
    UseContextMenuAttrs,
);

impl IntoAttrs for UseSelectableItemProps {
    type Attrs = UseSelectableItemAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            Attr(attr::Id, self.id),
            Attr(attr::Tabindex, self.tabindex),
            custom_attribute("data-collection", self.collection_id),
            self.element_capture,
            self.press.into_attrs(),
            self.on_focus.into_on(ev::focus),
            self.on_dragstart_capture
                .into_on(ev::capture(ev::dragstart)),
            self.context_menu.into_attrs(),
        )
    }
}

/// Selection, focus and actions for one item of a collection: pressing selects (respecting
/// selection mode/behavior and modifier keys), the focused item gets DOM focus and is the tab
/// stop, and items with an action or link perform it on press, Enter or double-click.
#[allow(clippy::too_many_lines)]
pub fn use_selectable_item(input: UseSelectableItemInput) -> UseSelectableItemReturn {
    let UseSelectableItemInput {
        selection,
        item_elements,
        key,
        element,
        id,
        collection_id,
        is_disabled,
        should_select_on_press_up,
        allows_different_press_origin,
        on_action,
        link_behavior,
        focus,
        should_use_virtual_focus,
        on_context_menu,
    } = input;
    let UseContextMenuReturn {
        props: context_menu_props,
        long_press: context_menu_long_press,
    } = use_context_menu(UseContextMenuInput { on_context_menu });

    let id = id.unwrap_or_else(|| use_id("item"));
    item_elements.register(key.clone(), element);
    let key = StoredValue::new(key);

    // The item's state, as memos: a change of the collection or the selection re-runs only the
    // readers of what actually changed for this item.
    let is_disabled =
        Memo::new(move |_| is_disabled.get() || key.with_value(|k| selection.is_disabled(k)));
    let is_selected = Signal::derive(move || key.with_value(|k| selection.is_selected(k)));
    // Only the focused item tracks whether the collection has focus.
    let is_focused = Signal::derive(move || {
        key.with_value(|k| selection.is_focused_key(k)) && selection.is_focused()
    });
    let is_link = Memo::new(move |_| key.with_value(|k| selection.is_link(k)));

    // -- Which interactions the item supports (react-aria's primary/secondary action model) --
    let allows_selection = Memo::new(move |_| {
        !is_disabled.get()
            && key.with_value(|k| selection.can_select_item(k))
            && !(is_link.get() && link_behavior.get() == LinkBehavior::Override)
    });
    let has_link_action = move || {
        is_link.get()
            && !matches!(
                link_behavior.get(),
                LinkBehavior::Selection | LinkBehavior::None
            )
    };
    let allows_actions =
        move || (on_action.with(Option::is_some) || has_link_action()) && !is_disabled.get();
    // An action performed by a plain press (instead of selecting).
    let has_primary_action = Memo::new(move |_| {
        allows_actions()
            && if selection.selection_behavior() == SelectionBehavior::Replace {
                !allows_selection.get()
            } else {
                !allows_selection.get() || selection.is_empty()
            }
    });
    // An action next to selection: double-click or Enter.
    let has_secondary_action = Memo::new(move |_| {
        allows_actions()
            && allows_selection.get()
            && selection.selection_behavior() == SelectionBehavior::Replace
    });
    let has_action = Memo::new(move |_| has_primary_action.get() || has_secondary_action.get());
    let long_press_enabled = Memo::new(move |_| has_action.get() && allows_selection.get());

    // -- DOM focus follows the focused key --
    Effect::new(move |_| {
        let focused = key.with_value(|k| selection.is_focused_key(k)) && selection.is_focused();
        if !focused {
            return;
        }
        // Focusing the item again moves focus to its other end (a grid row's first or last cell).
        selection.child_focus_strategy();
        if should_use_virtual_focus {
            if let Some(el) = element.get_untracked() {
                move_virtual_focus(Some(&el));
            }
        } else if let Some(focus) = &focus {
            focus.focus();
        } else if let Some(el) = element.get_untracked() {
            let active = el.owner_document().as_ref().and_then(get_active_element);
            if active.as_ref() != Some(&*el) {
                focus_safely(&el);
            }
        }
    });

    // A focused item that becomes disabled loses focus.
    Effect::new(move |_| {
        if is_disabled.get() && key.with_value(|k| selection.is_focused_key(k)) {
            selection.set_focused_key(None, None);
        }
    });

    // -- Selection --
    let on_select = move |pointer_type: &PointerType, modifiers: Modifiers| {
        let key = key.get_value();
        if *pointer_type == PointerType::Keyboard && is_non_contiguous_selection_modifier(modifiers)
        {
            selection.toggle_selection(&key);
            return;
        }
        let mode = untrack(|| selection.selection_mode());
        if mode == SelectionMode::None {
            return;
        }
        if is_link.get_untracked() {
            match link_behavior.get_untracked() {
                LinkBehavior::Selection => {
                    open_link_of(element, &selection, &key, modifiers);
                    // Report the (unchanged) selection, e.g. so a menu closes.
                    selection.set_selected_keys(untrack(|| selection.selected_keys()));
                    return;
                }
                LinkBehavior::Override | LinkBehavior::None => return,
                LinkBehavior::Action => {}
            }
        }
        if mode == SelectionMode::Single {
            if untrack(|| selection.is_selected(&key) && !selection.disallow_empty_selection()) {
                selection.toggle_selection(&key);
            } else {
                selection.replace_selection(&key);
            }
        } else if modifiers.shift_key {
            selection.extend_selection(&key);
        } else if untrack(|| selection.selection_behavior()) == SelectionBehavior::Toggle
            || is_ctrl_key_pressed(modifiers)
            || matches!(pointer_type, PointerType::Touch | PointerType::Virtual)
        {
            selection.toggle_selection(&key);
        } else {
            selection.replace_selection(&key);
        }
    };

    let perform_action = move |modifiers: Modifiers| {
        if let Some(on_action) = on_action.get_untracked() {
            on_action.run(());
            if let Some(el) = element.get_untracked() {
                let init = web_sys::CustomEventInit::new();
                init.set_bubbles(true);
                if let Ok(event) =
                    web_sys::CustomEvent::new_with_event_init_dict(ITEM_ACTION_EVENT, &init)
                {
                    let _ = el.dispatch_event(&event);
                }
            }
        }
        if untrack(has_link_action) {
            open_link_of(element, &selection, &key.get_value(), modifiers);
        }
    };

    // -- Press handling --
    let modality: StoredValue<Option<PointerType>> = StoredValue::new(None);
    let long_press_enabled_on_press_start = StoredValue::new(false);
    let had_primary_action_on_press_start = StoredValue::new(false);
    let is_selection_key = |e: &PressEvent| e.key == Some(KeyboardKey::Space);
    let is_action_key = |e: &PressEvent| e.key == Some(KeyboardKey::Enter);

    let (on_press_start, on_press, on_press_up) = if should_select_on_press_up {
        let on_press_start = Callback::new(move |e: PressEvent| {
            modality.set_value(Some(e.pointer_type));
            long_press_enabled_on_press_start.set_value(long_press_enabled.get_untracked());
            if e.pointer_type == PointerType::Keyboard
                && (!has_action.get_untracked() || is_selection_key(&e))
            {
                on_select(&e.pointer_type, e.modifiers);
            }
        });
        if allows_different_press_origin {
            let on_press_up = Callback::new(move |e: PressEvent| {
                if !has_primary_action.get_untracked()
                    && e.pointer_type == PointerType::Mouse
                    && allows_selection.get_untracked()
                {
                    on_select(&e.pointer_type, e.modifiers);
                }
            });
            let on_press = Callback::new(move |e: PressEvent| {
                if has_primary_action.get_untracked() {
                    perform_action(e.modifiers);
                } else if !matches!(e.pointer_type, PointerType::Keyboard | PointerType::Mouse)
                    && allows_selection.get_untracked()
                {
                    on_select(&e.pointer_type, e.modifiers);
                }
            });
            (on_press_start, on_press, Some(on_press_up))
        } else {
            let on_press = Callback::new(move |e: PressEvent| {
                if has_primary_action.get_untracked()
                    || (has_secondary_action.get_untracked()
                        && e.pointer_type != PointerType::Mouse)
                {
                    if e.pointer_type == PointerType::Keyboard && !is_action_key(&e) {
                        return;
                    }
                    perform_action(e.modifiers);
                } else if e.pointer_type != PointerType::Keyboard
                    && allows_selection.get_untracked()
                {
                    on_select(&e.pointer_type, e.modifiers);
                }
            });
            (on_press_start, on_press, None)
        }
    } else {
        let on_press_start = Callback::new(move |e: PressEvent| {
            modality.set_value(Some(e.pointer_type));
            long_press_enabled_on_press_start.set_value(long_press_enabled.get_untracked());
            let primary = has_primary_action.get_untracked();
            had_primary_action_on_press_start.set_value(primary);
            if allows_selection.get_untracked()
                && ((e.pointer_type == PointerType::Mouse && !primary)
                    || (e.pointer_type == PointerType::Keyboard
                        && (!allows_actions() || is_selection_key(&e))))
            {
                on_select(&e.pointer_type, e.modifiers);
            }
        });
        let on_press = Callback::new(move |e: PressEvent| {
            let has_action = has_action.get_untracked();
            let applies = match e.pointer_type {
                PointerType::Touch | PointerType::Pen | PointerType::Virtual => true,
                PointerType::Keyboard => has_action && is_action_key(&e),
                PointerType::Mouse => had_primary_action_on_press_start.get_value(),
                PointerType::Unknown => false,
            };
            if !applies {
                return;
            }
            if has_action {
                perform_action(e.modifiers);
            } else if allows_selection.get_untracked() {
                on_select(&e.pointer_type, e.modifiers);
            }
        });
        (on_press_start, on_press, None)
    };

    // With virtual focus, pressing an item focuses it (virtually), while DOM focus stays put.
    let virtually_focus = move |e: &PressEvent, on_touch: bool| {
        if should_use_virtual_focus && (e.pointer_type == PointerType::Touch) == on_touch {
            selection.set_focused(true);
            selection.set_focused_key(Some(key.get_value()), None);
        }
    };
    // Without selection or a primary action, pressing does nothing (except focusing the item
    // virtually).
    let press_disabled = Memo::new(move |_| {
        !(allows_selection.get()
            || has_primary_action.get()
            || (should_use_virtual_focus && !is_disabled.get()))
    });
    let press = use_press(UsePressInput {
        is_disabled: press_disabled.into(),
        prevent_focus_on_press: Signal::stored(should_use_virtual_focus),
        on_press: Some(Callback::new(move |e: PressEvent| {
            virtually_focus(&e, true);
            on_press.run(e);
        })),
        on_press_start: Some(Callback::new(move |e: PressEvent| {
            virtually_focus(&e, false);
            on_press_start.run(e);
        })),
        on_press_up,
        // A long press selects (touch selection mode); else, on iOS, it requests the context menu.
        long_press: Some(LongPress {
            on_long_press_start: context_menu_long_press
                .and_then(|context_menu| context_menu.on_long_press_start)
                .map(|on_start| {
                    Callback::new(move |e: LongPressEvent| {
                        if !long_press_enabled.get_untracked() {
                            on_start.run(e);
                        }
                    })
                }),
            on_long_press: Some(Callback::new(move |e: LongPressEvent| {
                if long_press_enabled.get_untracked() {
                    if e.pointer_type == PointerType::Touch {
                        on_select(&e.pointer_type, e.modifiers);
                        selection.set_selection_behavior(SelectionBehavior::Toggle);
                    }
                } else if let Some(on_context_menu) =
                    context_menu_long_press.and_then(|context_menu| context_menu.on_long_press)
                {
                    on_context_menu.run(e);
                }
            })),
            is_disabled: Signal::derive(move || {
                !long_press_enabled.get() && context_menu_long_press.is_none()
            }),
            ..LongPress::default()
        }),
        ..UsePressInput::default()
    });
    let (press_props, press_styles) = press.props.into_inner();

    // Double-click performs the secondary action.
    let on_dblclick = press_props.on_dblclick.chain(move |e: MouseEvent| {
        if has_secondary_action.get_untracked() && modality.get_value() == Some(PointerType::Mouse)
        {
            e.stop_propagation();
            e.prevent_default();
            perform_action(e.modifiers());
        }
    });

    // Link items open through `perform_action`; the native click must not open them a second
    // time.
    let on_click = press_props.on_click.chain(move |e: MouseEvent| {
        if link_behavior.get_untracked() != LinkBehavior::None
            && is_link.get_untracked()
            && !is_opening_link()
        {
            e.prevent_default();
        }
    });
    // react-aria leaves the press props off an item that can't be pressed: its disabled press
    // must not stop the click, which completes the press of an item around it (a grid row
    // pressed on one of its cells).
    let on_click = EventHandler::new(move |e: MouseEvent| {
        if !press_disabled.get_untracked() {
            on_click.call(e);
        }
    });

    // Presses on interactive children (a checkbox, a nested collection) belong to them.
    let child_collection_id = collection_id.clone();
    let is_child_interaction = move |target: &web_sys::Element| {
        let item = element.get_untracked();
        let mut current = Some(target.clone());
        while let Some(el) = current {
            if item.as_ref().is_some_and(|item| **item == el) {
                break;
            }
            if let Some(collection) = el.get_attribute("data-collection") {
                return collection != child_collection_id;
            }
            current = el.parent_element();
        }
        is_tabbable(target)
    };
    let is_child_interaction_for_mouse = is_child_interaction.clone();
    let base_pointerdown = press_props.on_pointerdown;
    let on_pointerdown = EventHandler::new(move |e: PointerEvent| {
        if let Some(target) = e
            .target()
            .and_then(|t| t.dyn_into::<web_sys::Element>().ok())
            && element.get_untracked().is_none_or(|el| *el != target)
            && is_child_interaction(&target)
        {
            e.stop_propagation();
            return;
        }
        base_pointerdown.call(e);
    });
    let base_mousedown = press_props.on_mousedown;
    let on_mousedown = EventHandler::new(move |e: MouseEvent| {
        if let Some(target) = e
            .target()
            .and_then(|t| t.dyn_into::<web_sys::Element>().ok())
            && element.get_untracked().is_none_or(|el| *el != target)
            && is_child_interaction_for_mouse(&target)
        {
            e.stop_propagation();
            return;
        }
        // Disabled items don't take focus when clicked, and with virtual focus no item does.
        if is_disabled.get_untracked() || should_use_virtual_focus {
            e.prevent_default();
        }
        base_mousedown.call(e);
    });

    let on_focus = EventHandler::new(move |e: FocusEvent| {
        if is_disabled.get_untracked() || should_use_virtual_focus {
            return;
        }
        let on_item = element
            .get_untracked()
            .is_some_and(|el| e.expect_target().dyn_ref::<web_sys::Element>() == Some(&*el));
        if on_item {
            selection.set_focused_key(Some(key.get_value()), None);
        }
    });

    let on_dragstart_capture = EventHandler::new(move |e: DragEvent| {
        if modality.get_value() == Some(PointerType::Touch)
            && long_press_enabled_on_press_start.get_value()
        {
            e.prevent_default();
        }
    });

    UseSelectableItemReturn {
        props: PropsWithStyles::new(
            UseSelectableItemProps {
                id,
                tabindex: Signal::derive(move || {
                    (!is_disabled.get() && !should_use_virtual_focus).then(|| {
                        if key.with_value(|k| selection.is_focused_key(k)) {
                            0
                        } else {
                            -1
                        }
                    })
                }),
                collection_id,
                element_capture: element.attr(),
                press: UsePressProps {
                    on_click,
                    on_pointerdown,
                    on_mousedown,
                    on_dblclick,
                    ..press_props
                },
                on_focus,
                on_dragstart_capture,
                context_menu: context_menu_props,
            },
            press_styles,
        ),
        is_pressed: press.is_pressed,
        is_selected,
        is_focused,
        is_disabled: is_disabled.into(),
        allows_selection: allows_selection.into(),
        has_action: has_action.into(),
    }
}

/// Dispatched (bubbling) on the item element after its action ran.
pub const ITEM_ACTION_EVENT: &str = "leptonic-item-action";

/// Opens the link of the item `key` (if it has one) as a click on `element` with `modifiers` would.
fn open_link_of(
    element: CapturedElement,
    selection: &SelectionManager,
    key: &Key,
    modifiers: Modifiers,
) {
    let link = untrack(|| {
        selection
            .collection()
            .with(|c| c.get(key).and_then(|n| n.link.clone()))
    });
    if let (Some(el), Some(link)) = (element.get_untracked(), link) {
        link.open(&el, modifiers);
    }
}
