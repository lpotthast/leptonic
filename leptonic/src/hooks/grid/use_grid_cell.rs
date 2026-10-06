// Upstream: react-aria/src/grid/useGridCell.ts @ 99e6102368
use leptos::{
    attr::{self, Attr},
    ev,
    ev::{On, SharedEventCallback},
    prelude::*,
};
use send_wrapper::SendWrapper;
use wasm_bindgen::JsCast;
use web_sys::{FocusEvent, KeyboardEvent};

use super::GridData;
use crate::{
    hooks::{
        IntoAttrs, PropsWithStyles,
        collections::{
            FocusStrategy, Key, LinkBehavior, NavigationOptions, UseSelectableItemAttrs,
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

impl UseGridCellInput {
    pub fn new(grid: GridData, key: Key) -> Self {
        Self {
            grid,
            key,
            id: None,
            focus_mode: None,
            allows_arrow_navigation: false,
            should_select_on_press_up: false,
        }
    }
}

/// Return value of [`use_grid_cell`].
pub struct UseGridCellReturn {
    pub grid_cell_props: PropsWithStyles<UseGridCellProps>,
    pub is_pressed: Signal<bool>,
}

/// Props for the cell element.
#[derive(Debug)]
pub struct UseGridCellProps {
    pub role: AriaRole,
    pub aria_colspan: Option<usize>,
    pub aria_colindex: Option<usize>,
    /// For `<td>`/`<th>` cells.
    pub colspan: Option<usize>,
    pub item: UseSelectableItemProps,
    pub on_keydown_capture: EventHandler<KeyboardEvent>,
    pub on_focusin: EventHandler<FocusEvent>,
}

pub type UseGridCellAttrs = (
    Attr<attr::Role, AriaRole>,
    Attr<attr::AriaColspan, Option<usize>>,
    Attr<attr::AriaColindex, Option<usize>>,
    Attr<attr::Colspan, Option<usize>>,
    UseSelectableItemAttrs,
    On<ev::Capture<ev::keydown>, SharedEventCallback<KeyboardEvent>>,
    On<ev::focusin, SharedEventCallback<FocusEvent>>,
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
    let (col_span, col_index) = untrack(|| {
        state.list.collection.with(|c| {
            c.get(&key)
                .map(|n| (n.col_span, n.col_index))
                .unwrap_or_default()
        })
    });

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
        focus: Some(Callback::new(move |()| focus_cell())),
        should_use_virtual_focus: false,
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
        if state.is_keyboard_navigation_disabled.get_untracked() {
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
        let redispatch = |e: &KeyboardEvent| {
            if let Some(parent) = cell.parent_element() {
                let _ = parent.dispatch_event(&clone_keyboard_event(e));
            }
        };

        let key = e.key();
        match key.as_str() {
            "ArrowLeft" | "ArrowRight" => {
                let right = key == "ArrowRight";
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
            "ArrowUp" | "ArrowDown" if !e.alt_key() => {
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
            || state.is_keyboard_navigation_disabled.get_untracked()
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
        if !on_cell && e.key() != "Tab" {
            e.stop_propagation();
            return;
        }
        if e.key() == "Tab"
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

    UseGridCellReturn {
        grid_cell_props: PropsWithStyles::new(
            UseGridCellProps {
                role: AriaRole::Gridcell,
                aria_colspan: col_span,
                aria_colindex: col_index.map(|i| i + 1),
                colspan: col_span,
                item: item_props,
                on_keydown_capture: EventHandler::new(on_keydown_capture),
                on_focusin,
            },
            item_styles,
        ),
        is_pressed,
    }
}

/// A copy of `e` (same key and modifiers) that bubbles, for re-dispatching it elsewhere.
fn clone_keyboard_event(e: &KeyboardEvent) -> KeyboardEvent {
    let init = web_sys::KeyboardEventInit::new();
    init.set_key(&e.key());
    init.set_code(&e.code());
    init.set_location(e.location());
    init.set_repeat(e.repeat());
    init.set_shift_key(e.shift_key());
    init.set_ctrl_key(e.ctrl_key());
    init.set_alt_key(e.alt_key());
    init.set_meta_key(e.meta_key());
    init.set_bubbles(true);
    init.set_cancelable(true);
    KeyboardEvent::new_with_keyboard_event_init_dict(&e.type_(), &init)
        .expect("KeyboardEvent creation should not fail")
}
