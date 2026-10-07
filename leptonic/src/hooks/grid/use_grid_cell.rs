// Upstream: react-aria/src/grid/useGridCell.ts @ 99e6102368
use leptos::{
    attr::{self, Attr},
    ev,
    ev::{On, SharedEventCallback},
    prelude::*,
};
use send_wrapper::SendWrapper;
use wasm_bindgen::JsCast;
use web_sys::{FocusEvent, KeyboardEvent, PointerEvent};

use super::GridData;
use crate::{
    hooks::{
        IntoAttrs, PropsWithStyles,
        collections::{
            FocusItem, FocusStrategy, Key, LinkBehavior, NavigationOptions, UseSelectableItemAttrs,
            UseSelectableItemInput, UseSelectableItemProps, UseSelectableItemReturn,
            use_selectable_item,
        },
        focus::use_focus_visible::{Modality, get_modality},
        gridlist::KeyboardNavigationBehavior,
    },
    utils::{
        CapturedElement, EventAccessors, EventHandler,
        aria::AriaRole,
        focus::focus_safely,
        focusable_tree_walker::{FocusableTreeWalkerOptions, get_focusable_tree_walker},
        i18n::use_direction,
        key::{KeyboardEventKey, KeyboardKey, redispatch_keyboard_event},
        locale::WritingDirection,
        node_contains,
        owner_alive::OwnerAlive,
        scroll::{ScrollIntoViewportOpts, get_scroll_parent, scroll_into_viewport},
        shadow_dom::get_active_element,
    },
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## OMITTED FEATURES
// - Virtualization (`aria-colindex` from the cell index).
//
// =============================================================================

/// What gets focus when a cell is focused.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CellFocusMode {
    /// The cell itself.
    Cell,
    /// The cell's first focusable child (or last, entering from the right), if it has one.
    Child,
}

/// Input of [`use_grid_cell`].
#[derive(Debug, Clone)]
pub struct UseGridCellInput {
    /// The grid (from `use_grid`).
    pub grid: GridData,
    /// The cell's key ([`Key::cell`]).
    pub key: Key,
    /// The element id. Generated when `None`.
    pub id: Option<String>,
    /// `None`: `Cell` with `KeyboardNavigationBehavior::Tab`, else `Child`.
    pub focus_mode: Option<CellFocusMode>,
    /// Let ArrowLeft/ArrowRight move between the cell's children even with
    /// `KeyboardNavigationBehavior::Tab`.
    pub allows_arrow_navigation: bool,
    /// Select when the press ends instead of when it starts.
    pub should_select_on_press_up: bool,
}

/// Return value of [`use_grid_cell`].
pub struct UseGridCellReturn {
    pub grid_cell_props: PropsWithStyles<UseGridCellProps>,
    pub is_pressed: Signal<bool>,
}

/// Props for the cell element.
#[derive(Debug)]
pub struct UseGridCellProps {
    pub role: Signal<AriaRole>,
    pub aria_colspan: Signal<Option<usize>>,
    pub aria_colindex: Signal<Option<usize>>,
    /// For `<td>`/`<th>` cells.
    pub colspan: Signal<Option<usize>>,
    pub item: UseSelectableItemProps,
    pub on_keydown_capture: EventHandler<KeyboardEvent>,
    pub on_focusin: EventHandler<FocusEvent>,
    /// Briefly removes the cell's `tabindex` on pointer down (see [`use_grid_cell`]).
    pub on_pointerdown: EventHandler<PointerEvent>,
}

pub type UseGridCellAttrs = (
    Attr<attr::Role, Signal<AriaRole>>,
    Attr<attr::AriaColspan, Signal<Option<usize>>>,
    Attr<attr::AriaColindex, Signal<Option<usize>>>,
    Attr<attr::Colspan, Signal<Option<usize>>>,
    UseSelectableItemAttrs,
    On<ev::Capture<ev::keydown>, SharedEventCallback<KeyboardEvent>>,
    On<ev::focusin, SharedEventCallback<FocusEvent>>,
    On<ev::pointerdown, SharedEventCallback<PointerEvent>>,
);

impl IntoAttrs for UseGridCellProps {
    type Attrs = UseGridCellAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            Attr(attr::Role, self.role),
            Attr(attr::AriaColspan, self.aria_colspan),
            Attr(attr::AriaColindex, self.aria_colindex),
            Attr(attr::Colspan, self.colspan),
            self.item.into_attrs(),
            self.on_keydown_capture.into_on(ev::capture(ev::keydown)),
            self.on_focusin.into_on(ev::focusin),
            self.on_pointerdown.into_on(ev::pointerdown),
        )
    }
}

/// A cell of a grid: focusable itself or through its interactive children, with arrow keys
/// moving between the children before moving to the neighboring cell.
#[allow(clippy::too_many_lines)]
pub fn use_grid_cell(input: UseGridCellInput) -> UseGridCellReturn {
    crate::hooks::track_interaction_modality();
    let UseGridCellInput {
        grid,
        key,
        id,
        focus_mode,
        allows_arrow_navigation,
        should_select_on_press_up,
    } = input;
    let GridData {
        state,
        delegate,
        collection_id,
        on_cell_action,
        keyboard_navigation_behavior,
        ..
    } = grid;
    let focus_mode = focus_mode.unwrap_or(
        if keyboard_navigation_behavior == KeyboardNavigationBehavior::Tab {
            CellFocusMode::Cell
        } else {
            CellFocusMode::Child
        },
    );
    let selection = state.list.selection;
    let direction = use_direction();
    // The cell's span and column follow the collection (e.g. when columns change).
    let position = {
        let key = key.clone();
        Memo::new(move |_| {
            state.list.collection.with(|c| {
                c.get(&key)
                    .map(|n| (n.col_span, n.col_index))
                    .unwrap_or_default()
            })
        })
    };

    let element = CapturedElement::new();
    let cell_key = StoredValue::new(key.clone());
    let key_when_focused: StoredValue<Option<Key>> = StoredValue::new(None);
    let last_focused_child: StoredValue<Option<SendWrapper<web_sys::Element>>> =
        StoredValue::new(None);

    let focus_cell = move || {
        let Some(cell) = element.get_untracked() else {
            return;
        };
        let cell: web_sys::Element = (*cell).clone();
        let document = cell.owner_document();
        let active = document.as_ref().and_then(get_active_element);
        let focus_within = active
            .as_ref()
            .is_some_and(|a| cell.contains(Some(a.unchecked_ref())));
        if focus_mode == CellFocusMode::Child {
            if focus_within && active.as_ref() != Some(&cell) {
                return;
            }
            let body = document
                .as_ref()
                .and_then(web_sys::Document::body)
                .map(web_sys::Element::from);
            let should_restore =
                active.is_none() || active == body || active.as_ref() == Some(&cell);
            if should_restore
                && key_when_focused
                    .get_value()
                    .is_some_and(|k| cell_key.with_value(|c| *c == k))
                && let Some(child) = last_focused_child.get_value()
                && cell.contains(Some(child.unchecked_ref()))
            {
                focus_safely(&child);
                return;
            }
            if let Some(mut walker) =
                get_focusable_tree_walker(&cell, FocusableTreeWalkerOptions::default())
            {
                let target =
                    if untrack(|| selection.child_focus_strategy()) == Some(FocusStrategy::Last) {
                        let mut last = None;
                        while let Some(node) = walker.last_child() {
                            last = Some(node);
                        }
                        last
                    } else {
                        walker.first_child()
                    };
                if let Some(target) = target.and_then(|n| n.dyn_into::<web_sys::Element>().ok()) {
                    focus_safely(&target);
                    return;
                }
            }
        }
        let moved = key_when_focused
            .get_value()
            .is_some_and(|k| cell_key.with_value(|c| *c != k));
        if moved || !focus_within {
            focus_safely(&cell);
        }
    };

    let rows = state.list.collection;
    let UseSelectableItemReturn {
        props: item_props,
        is_pressed,
        allows_selection,
        has_action,
        ..
    } = use_selectable_item(UseSelectableItemInput {
        selection,
        item_elements: state.list.item_elements,
        key: key.clone(),
        element,
        id,
        collection_id,
        is_disabled: Signal::derive(move || rows.with(|c| c.size() == 0)),
        should_select_on_press_up,
        allows_different_press_origin: false,
        on_action: on_cell_action.map(|on_cell_action| {
            let key = key.clone();
            Callback::new(move |()| on_cell_action.run(key.clone()))
        }),
        link_behavior: LinkBehavior::Action,
        focus: Some(FocusItem::new(focus_cell)),
        should_use_virtual_focus: false,
        on_context_menu: None,
    });
    let (mut item_props, item_styles) = item_props.into_inner();

    let reveal = move |target: &web_sys::Element| {
        focus_safely(target);
        if let Some(cell) = element.get_untracked() {
            let container = get_scroll_parent(&cell, false);
            scroll_into_viewport(
                Some(target),
                &ScrollIntoViewportOpts {
                    containing_element: Some(container),
                },
            );
        }
    };

    let on_keydown_capture = move |e: KeyboardEvent| {
        if keyboard_navigation_behavior == KeyboardNavigationBehavior::Tab
            && !allows_arrow_navigation
        {
            return;
        }
        if state.is_keyboard_navigation_disabled().get_untracked() {
            return;
        }
        let Some(cell) = element.get_untracked() else {
            return;
        };
        let cell: web_sys::Element = (*cell).clone();
        let target = e.expect_target().dyn_into::<web_sys::Node>().ok();
        if !node_contains(Some(cell.unchecked_ref()), target.as_ref()).unwrap_or(false) {
            return;
        }
        let Some(active) = cell.owner_document().as_ref().and_then(get_active_element) else {
            return;
        };
        let Some(mut walker) =
            get_focusable_tree_walker(&cell, FocusableTreeWalkerOptions::default())
        else {
            return;
        };
        walker.set_current_node(active.unchecked_ref());
        let rtl = direction.get_untracked() == WritingDirection::Rtl;
        // Lets the row and grid handle the key: a nested dispatch of `keydown`, but the copy's
        // path (the row and its ancestors) has no listener still running for `e` (their capture
        // listeners ran before this one, their bubble listeners haven't, as `e` stops here).
        let redispatch = |e: &KeyboardEvent| {
            if let Some(parent) = cell.parent_element() {
                redispatch_keyboard_event(e, &parent);
            }
        };

        let key = e.typed_key();
        match key {
            KeyboardKey::ArrowLeft | KeyboardKey::ArrowRight => {
                let right = key == KeyboardKey::ArrowRight;
                // "Forward" in the reading direction.
                let forward = right != rtl;
                let mut focusable = if forward {
                    walker.next_node()
                } else {
                    walker.previous_node()
                }
                .and_then(|n| n.dyn_into::<web_sys::Element>().ok());
                if focus_mode == CellFocusMode::Child && focusable.as_ref() == Some(&cell) {
                    focusable = None;
                }
                e.prevent_default();
                e.stop_propagation();
                if let Some(focusable) = focusable {
                    reveal(&focusable);
                    return;
                }
                // No child left this way: the grid moves to the neighboring cell, if any.
                let neighbor = cell_key.with_value(|k| {
                    let delegate = delegate.get_untracked();
                    if right {
                        delegate.key_right_of(k, NavigationOptions::default())
                    } else {
                        delegate.key_left_of(k, NavigationOptions::default())
                    }
                });
                if neighbor.as_ref() != Some(&cell_key.get_value()) {
                    redispatch(&e);
                    return;
                }
                // At the edge: wrap within the cell.
                if focus_mode == CellFocusMode::Cell && forward {
                    reveal(&cell);
                } else {
                    walker.set_current_node(cell.unchecked_ref());
                    let target = if forward {
                        walker.first_child()
                    } else {
                        let mut last = None;
                        while let Some(node) = walker.last_child() {
                            last = Some(node);
                        }
                        last
                    };
                    if let Some(target) = target.and_then(|n| n.dyn_into::<web_sys::Element>().ok())
                    {
                        reveal(&target);
                    }
                }
            }
            KeyboardKey::ArrowUp | KeyboardKey::ArrowDown if !e.alt_key() => {
                e.stop_propagation();
                e.prevent_default();
                redispatch(&e);
            }
            _ => {}
        }
    };

    // Tab navigation between the cell's children (`KeyboardNavigationBehavior::Tab`), running
    // before the cell's own key handling.
    let tab_navigation = move |e: &KeyboardEvent| {
        if keyboard_navigation_behavior != KeyboardNavigationBehavior::Tab
            || state.is_keyboard_navigation_disabled().get_untracked()
        {
            return;
        }
        let Some(cell) = element.get_untracked() else {
            return;
        };
        let cell: web_sys::Element = (*cell).clone();
        let on_cell = e
            .target()
            .is_some_and(|t| t.unchecked_ref::<web_sys::Element>() == &cell);
        if !on_cell && e.typed_key() != KeyboardKey::Tab {
            e.stop_propagation();
            return;
        }
        if e.typed_key() == KeyboardKey::Tab
            && let Some(active) = cell.owner_document().as_ref().and_then(get_active_element)
            && let Some(mut walker) = get_focusable_tree_walker(
                &cell,
                FocusableTreeWalkerOptions {
                    tabbable: true,
                    ..FocusableTreeWalkerOptions::default()
                },
            )
        {
            walker.set_current_node(active.unchecked_ref());
            let next = if e.shift_key() {
                walker.previous_node()
            } else {
                walker.next_node()
            };
            if next.is_some() {
                e.stop_propagation();
            }
        }
    };
    let press_keydown = item_props.press.on_keydown;
    item_props.press.on_keydown = EventHandler::new(move |e: KeyboardEvent| {
        tab_navigation(&e);
        if !e.cancel_bubble() {
            press_keydown.call(e);
        }
    });

    // `focusin`: the cell also learns about focus moving to its children (react-aria's `onFocus`
    // bubbles). It runs after the item's own `focus` handler.
    let alive = OwnerAlive::new();
    let on_focusin = EventHandler::new(move |e: FocusEvent| {
        key_when_focused.set_value(Some(cell_key.get_value()));
        let Some(cell) = element.get_untracked() else {
            return;
        };
        let target = e
            .target()
            .and_then(|t| t.dyn_into::<web_sys::Element>().ok());
        if target.as_ref() != Some(&*cell) {
            // A child got focus: remember it, and make the cell the focused key.
            if let Some(target) = &target
                && cell.contains(Some(target.unchecked_ref()))
            {
                last_focused_child.set_value(Some(SendWrapper::new(target.clone())));
            }
            if get_modality() == Modality::Pointer {
                selection.set_focused_key(Some(cell_key.get_value()), None);
            }
            return;
        }
        if focus_mode == CellFocusMode::Child {
            let from_child = e
                .related_target()
                .and_then(|t| t.dyn_into::<web_sys::Node>().ok())
                .is_some_and(|related| cell.contains(Some(&related)));
            if from_child {
                return;
            }
            let alive = alive.clone();
            request_animation_frame(move || {
                // The cell may be gone by the next frame (filtering, removal).
                if !alive.get() {
                    return;
                }
                let still_on_cell = element.get_untracked().is_some_and(|cell| {
                    cell.owner_document()
                        .as_ref()
                        .and_then(get_active_element)
                        .is_some_and(|a| a == *cell)
                });
                if still_on_cell {
                    focus_cell();
                }
            });
        }
    });
    if focus_mode == CellFocusMode::Child
        && keyboard_navigation_behavior == KeyboardNavigationBehavior::Tab
    {
        item_props.tabindex = Signal::stored(Some(-1));
    }

    // When rows select on press up (e.g. draggable rows) and the press goes to the row (the cell
    // can't be selected and has no action), the pointer down's default focus would land on the
    // cell (the closest element with a tabindex) instead of the row: the cell drops its tabindex
    // for a frame (useGridCell.ts).
    let on_pointerdown = {
        let tabindex = item_props.tabindex;
        EventHandler::new(move |e: PointerEvent| {
            if !should_select_on_press_up
                || allows_selection.get_untracked()
                || has_action.get_untracked()
                || tabindex.get_untracked().is_none()
            {
                return;
            }
            let Ok(cell) = e.expect_current_target().dyn_into::<web_sys::Element>() else {
                return;
            };
            let Some(value) = cell.get_attribute("tabindex") else {
                return;
            };
            let _ = cell.remove_attribute("tabindex");
            let cell = SendWrapper::new(cell);
            request_animation_frame(move || {
                let _ = cell.set_attribute("tabindex", &value);
            });
        })
    };

    UseGridCellReturn {
        grid_cell_props: PropsWithStyles::new(
            UseGridCellProps {
                role: Signal::stored(AriaRole::Gridcell),
                aria_colspan: Signal::derive(move || position.get().0),
                aria_colindex: Signal::derive(move || position.get().1.map(|i| i + 1)),
                colspan: Signal::derive(move || position.get().0),
                item: item_props,
                on_keydown_capture: EventHandler::new(on_keydown_capture),
                on_focusin,
                on_pointerdown,
            },
            item_styles,
        ),
        is_pressed,
    }
}
