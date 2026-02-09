use std::hash::Hash;

use leptos::attr::Attr;
use leptos::ev::{On, SharedEventCallback};
use leptos::prelude::*;
use leptos::{attr, ev};
use send_wrapper::SendWrapper;
use web_sys::{FocusEvent, KeyboardEvent, MouseEvent};

use crate::hooks::focus::use_focus_manager::{FocusManager, FocusManagerOptions};
use crate::hooks::selection::use_selectable_item::{use_selectable_item, UseSelectableItemInput};
use crate::hooks::selection::use_selection_state::SelectionMode;
use crate::utils::element_capture::{CapturedElement, ElementCaptureAttr};
use crate::utils::focus::focus_element;
use crate::utils::aria::{AriaDisabled, AriaSelected};
use crate::utils::EventHandler;

use super::use_grid::UseGridState;

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/@react-aria/grid/src/useGridCell.ts
//
// ## DEVIATIONS FROM REACT-ARIA
//
// ### Omitted
// - `isVirtualized`, `colSpan`, `keyWhenFocused` — no virtualization support.
// - `onPointerDown` tabindex workaround — no drag support.
// - RTL direction swapping — `ArrowLeft`/`ArrowRight` don't swap based on locale.
// - `isPressed` — not tracked; use `use_press` separately if needed.
//
// ### Different
// - Bubble-phase keydown instead of capture-phase + re-dispatch. ArrowUp/Down
//   bubble naturally to the grid handler.
// - Shared state struct (`UseGridState<K>`) instead of `gridMap` `WeakMap`.
//
// ### Leptos-specific
// - `ElementCaptureAttr` instead of React refs for DOM element access.
// - `FocusManager` from `use_focus_manager` instead of `getFocusableTreeWalker`.
// - `EventHandler<E>` for composable event handler chaining.

/// Controls how focus behaves when a grid cell receives focus.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CellFocusMode {
    /// Focus the cell element itself.
    #[default]
    Cell,
    /// Focus the first focusable child within the cell.
    Child,
}

/// Input for a grid cell.
pub struct UseGridCellInput<K>
where
    K: Hash + Eq + Clone + Send + Sync + 'static,
{
    /// Shared grid state from `use_grid`.
    pub state: UseGridState<K>,

    /// The unique key for this cell.
    pub key: K,

    /// The row index (0-based; converted to 1-based for ARIA).
    pub row_index: usize,

    /// The column index (0-based; converted to 1-based for ARIA).
    pub column_index: usize,

    /// How focus should behave when this cell receives focus.
    pub focus_mode: CellFocusMode,
}

/// Return value for a grid cell.
pub struct UseGridCellReturn {
    /// Props for the cell element.
    pub props: UseGridCellProps,

    /// Whether the cell is selected.
    pub is_selected: Signal<bool>,

    /// Whether the cell is focused.
    pub is_focused: Signal<bool>,

    /// Whether the cell is disabled.
    pub is_disabled: Signal<bool>,
}

/// Props from `use_grid_cell` that can be extracted and merged programmatically.
#[derive(Clone)]
pub struct UseGridCellProps {
    pub role: &'static str,
    pub tabindex: Signal<&'static str>,
    pub aria_rowindex: String,
    pub aria_colindex: String,
    pub aria_selected: Signal<Option<AriaSelected>>,
    pub aria_disabled: Signal<Option<AriaDisabled>>,
    pub element_capture: ElementCaptureAttr,
    pub on_click: EventHandler<MouseEvent>,
    pub on_keydown: EventHandler<KeyboardEvent>,
    pub on_focus: EventHandler<FocusEvent>,
    pub on_mouseenter: EventHandler<MouseEvent>,
}

impl UseGridCellProps {
    /// Convert to spreadable attributes for Leptos views, cloning internally.
    #[must_use]
    pub fn to_attrs(&self) -> UseGridCellAttrs {
        self.clone().into_attrs()
    }

    /// Convert to spreadable attributes for Leptos views, consuming self.
    #[must_use]
    pub fn into_attrs(self) -> UseGridCellAttrs {
        (
            Attr(attr::Role, self.role),
            Attr(attr::Tabindex, self.tabindex),
            Attr(attr::AriaRowindex, self.aria_rowindex),
            Attr(attr::AriaColindex, self.aria_colindex),
            Attr(attr::AriaSelected, self.aria_selected),
            Attr(attr::AriaDisabled, self.aria_disabled),
            self.element_capture,
            self.on_click.into_on(ev::click),
            self.on_keydown.into_on(ev::keydown),
            self.on_focus.into_on(ev::focus),
            self.on_mouseenter.into_on(ev::mouseenter),
        )
    }
}

/// Attributes for a grid cell.
pub type UseGridCellAttrs = (
    Attr<attr::Role, &'static str>,
    Attr<attr::Tabindex, Signal<&'static str>>,
    Attr<attr::AriaRowindex, String>,
    Attr<attr::AriaColindex, String>,
    Attr<attr::AriaSelected, Signal<Option<AriaSelected>>>,
    Attr<attr::AriaDisabled, Signal<Option<AriaDisabled>>>,
    ElementCaptureAttr,
    On<ev::click, SharedEventCallback<MouseEvent>>,
    On<ev::keydown, SharedEventCallback<KeyboardEvent>>,
    On<ev::focus, SharedEventCallback<FocusEvent>>,
    On<ev::mouseenter, SharedEventCallback<MouseEvent>>,
);

/// Provides the behavior and accessibility for a grid cell.
///
/// Reads shared grid state from `UseGridState<K>` (produced by `use_grid`)
/// and delegates selection to `use_selectable_item`.
///
/// Supports within-cell focusable child navigation via `ArrowLeft`/`ArrowRight` keys and a
/// configurable `CellFocusMode`.
///
/// # Example
///
/// ```ignore
/// let grid = use_grid(UseGridInput { ... });
///
/// let cell = use_grid_cell(UseGridCellInput {
///     state: grid.state,
///     key: "0-0".to_string(),
///     row_index: 0,
///     column_index: 0,
///     focus_mode: CellFocusMode::Cell,
/// });
///
/// view! {
///     <div {..cell.props.into_attrs()}>
///         "Cell content"
///     </div>
/// }
/// ```
#[allow(clippy::needless_pass_by_value, clippy::too_many_lines)]
pub fn use_grid_cell<K>(input: UseGridCellInput<K>) -> UseGridCellReturn
where
    K: Hash + Eq + Clone + Send + Sync + 'static,
{
    let state = input.state;
    let cell_focus_mode = input.focus_mode;
    let selection_mode = state.selection_mode;

    // --- Element capture + FocusManager for within-cell navigation ---
    let scope_element = CapturedElement::new();

    // --- Focus callback for DOM focus synchronization ---
    let scope_element_for_focus = scope_element;
    let focus_fn = Callback::new(move |()| {
        if let Some(el) = scope_element_for_focus.get_untracked() {
            let el = SendWrapper::take(el);
            // Don't move focus if it's already within this cell
            // (e.g., user clicked a focusable child). Mirrors react-aria's
            // `!nodeContains(ref.current, document.activeElement)` check.
            if let Some(active) = web_sys::window()
                .and_then(|w| w.document())
                .and_then(|d| d.active_element())
            {
                if el.contains(Some(&active)) && *active != *el {
                    return;
                }
            }
            focus_element(&el, true);
        }
    });

    // --- Delegate selection to use_selectable_item ---
    let selectable = use_selectable_item(UseSelectableItemInput {
        key: input.key.clone(),
        selection_mode: state.selection_mode,
        selection_behavior: state.selection_behavior,
        selected_keys: state.selection.selected_keys,
        focused_key: state.focused_key,
        is_disabled: state.is_disabled,
        on_toggle: state.selection.toggle,
        on_select: state.selection.select,
        on_focus: state.set_focused_key,
        should_select_on_press_up: false,
        allow_drag: false,
        on_double_click: state.on_cell_action,
        focus: Some(focus_fn),
    });

    let is_selected = selectable.is_selected;
    let is_focused = selectable.is_focused;
    let is_disabled = selectable.is_disabled;

    let focus_manager =
        FocusManager::new(move || scope_element.get_untracked().map(SendWrapper::take));

    // --- ARIA attributes ---
    let aria_selected = Signal::derive(move || {
        if selection_mode == SelectionMode::None {
            None
        } else {
            Some(AriaSelected::from(is_selected.get()))
        }
    });

    let tabindex = Signal::derive(move || if is_focused.get() { "0" } else { "-1" });

    // Indices are 1-based for ARIA.
    let aria_rowindex = (input.row_index + 1).to_string();
    let aria_colindex = (input.column_index + 1).to_string();

    // --- Keyboard handler (within-cell arrow navigation) ---
    let focus_manager_for_keydown = focus_manager.clone();
    let cell_keydown = EventHandler::new(move |e: KeyboardEvent| {
        if state.is_disabled.get_untracked() {
            return;
        }

        let key = e.key();
        match key.as_str() {
            "ArrowRight" => {
                // Try to move focus to the next focusable child inside the cell.
                // If successful, stop propagation so the grid handler doesn't also move to the next cell.
                if focus_manager_for_keydown
                    .focus_next(FocusManagerOptions::default())
                    .is_some()
                {
                    e.stop_propagation();
                    e.prevent_default();
                }
                // Otherwise: let the event bubble to the grid's keydown handler for cell-to-cell navigation.
            }
            "ArrowLeft" => {
                if focus_manager_for_keydown
                    .focus_previous(FocusManagerOptions::default())
                    .is_some()
                {
                    e.stop_propagation();
                    e.prevent_default();
                }
            }
            // ArrowUp, ArrowDown, Space, Enter, Home, End, etc.: let bubble to grid handler.
            _ => {}
        }
    });

    // --- Focus handler ---
    let focus_manager_for_focus = focus_manager.clone();
    let key_for_focus = input.key.clone();
    let set_focused_key = state.set_focused_key;
    let cell_focus = EventHandler::new(move |e: FocusEvent| {
        // If a child element received focus (target != currentTarget), update the grid's
        // focused key to this cell so the grid knows which cell is active.
        let target = e.target();
        let current_target = e.current_target();
        if target != current_target {
            set_focused_key.run(Some(key_for_focus.clone()));
        }

        // If the cell itself received focus and focus_mode is Child,
        // redirect focus to the first focusable child.
        if target == current_target && cell_focus_mode == CellFocusMode::Child {
            let fm = focus_manager_for_focus.clone();
            // Use request_animation_frame to defer, avoiding focus loops during the
            // current focus event.
            request_animation_frame(move || {
                fm.focus_first(FocusManagerOptions::default());
            });
        }
    });

    // --- Compose handlers: chain selectable_item handlers with cell-specific handlers ---
    let on_click = selectable.props.on_click;
    // Keydown: cell-specific handler only (selectable_item has no keydown handler).
    // Within-cell navigation may stop_propagation; remaining events bubble to the grid handler.
    let on_keydown = cell_keydown;
    let on_focus = selectable.props.on_focus.chain(cell_focus);
    let on_mouseenter = selectable.props.on_mouseenter;

    UseGridCellReturn {
        props: UseGridCellProps {
            role: "gridcell",
            tabindex,
            aria_rowindex,
            aria_colindex,
            aria_selected,
            aria_disabled: Signal::derive(move || {
                is_disabled.get().then_some(AriaDisabled::True)
            }),
            element_capture: scope_element.attr(),
            on_click,
            on_keydown,
            on_focus,
            on_mouseenter,
        },
        is_selected,
        is_focused,
        is_disabled,
    }
}
