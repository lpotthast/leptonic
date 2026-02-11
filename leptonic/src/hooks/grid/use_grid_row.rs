use std::hash::Hash;

use leptos::attr::Attr;
use leptos::ev::{On, SharedEventCallback};
use leptos::prelude::*;
use leptos::{attr, ev};
use web_sys::{FocusEvent, MouseEvent};

use send_wrapper::SendWrapper;

use crate::hooks::selection::use_selectable_item::{use_selectable_item, UseSelectableItemInput};
use crate::hooks::selection::use_selection_state::SelectionMode;
use crate::utils::aria::{AriaDisabled, AriaSelected};
use crate::utils::element_capture::{CapturedElement, ElementCaptureAttr};
use crate::utils::focus::focus_element;
use crate::utils::EventHandler;

use super::use_grid::UseGridState;

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/@react-aria/grid/src/useGridRow.ts

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## OMITTED FEATURES
// - `isVirtualized` — no virtualization support.
// - `shouldSelectOnPressUp` — not configurable per-row; always selects on press down.
// - `isPressed` — not tracked; use `use_press` separately if needed.
// - Deprecated per-row `onAction` prop — use `UseGridInput::on_row_action` instead.
//
// ## DIFFERENT BEHAVIOR
// - Always emits `aria-rowindex` (not only when virtualized).
// - Double-click triggers row action (in addition to Enter at grid level).
// - Shared state struct (`UseGridState<K>`) instead of `gridMap` `WeakMap`.
//
// ## LEPTOS-SPECIFIC ADAPTATIONS
// - `EventHandler<E>` for composable event handler chaining.
//
// =============================================================================

/// Input for a grid row.
pub struct UseGridRowInput<K>
where
    K: Hash + Eq + Clone + Send + Sync + 'static,
{
    /// Shared grid state from `use_grid`.
    pub state: UseGridState<K>,

    /// The unique key for this row.
    pub key: K,

    /// The row index (0-based; converted to 1-based for ARIA).
    pub row_index: usize,
}

/// Return value for a grid row.
pub struct UseGridRowReturn {
    /// Props for the row element.
    pub props: UseGridRowProps,

    /// Whether the row is selected.
    pub is_selected: Signal<bool>,

    /// Whether the row is focused.
    pub is_focused: Signal<bool>,

    /// Whether the row is disabled.
    pub is_disabled: Signal<bool>,
}

/// Props from `use_grid_row` that can be extracted and merged programmatically.
#[derive(Clone)]
pub struct UseGridRowProps {
    pub role: &'static str,
    pub tabindex: Signal<&'static str>,
    pub aria_rowindex: String,
    pub aria_selected: Signal<Option<AriaSelected>>,
    pub aria_disabled: Signal<Option<AriaDisabled>>,
    pub element_capture: ElementCaptureAttr,
    pub on_click: EventHandler<MouseEvent>,
    pub on_dblclick: EventHandler<MouseEvent>,
    pub on_focus: EventHandler<FocusEvent>,
    pub on_mouseenter: EventHandler<MouseEvent>,
}

impl UseGridRowProps {
    /// Convert to spreadable attributes for Leptos views, cloning internally.
    #[must_use]
    pub fn to_attrs(&self) -> UseGridRowAttrs {
        self.clone().into_attrs()
    }

    /// Convert to spreadable attributes for Leptos views, consuming self.
    #[must_use]
    pub fn into_attrs(self) -> UseGridRowAttrs {
        (
            Attr(attr::Role, self.role),
            Attr(attr::Tabindex, self.tabindex),
            Attr(attr::AriaRowindex, self.aria_rowindex),
            Attr(attr::AriaSelected, self.aria_selected),
            Attr(attr::AriaDisabled, self.aria_disabled),
            self.element_capture,
            self.on_click.into_on(ev::click),
            self.on_dblclick.into_on(ev::dblclick),
            self.on_focus.into_on(ev::focus),
            self.on_mouseenter.into_on(ev::mouseenter),
        )
    }
}

/// Attributes for a grid row.
pub type UseGridRowAttrs = (
    Attr<attr::Role, &'static str>,
    Attr<attr::Tabindex, Signal<&'static str>>,
    Attr<attr::AriaRowindex, String>,
    Attr<attr::AriaSelected, Signal<Option<AriaSelected>>>,
    Attr<attr::AriaDisabled, Signal<Option<AriaDisabled>>>,
    ElementCaptureAttr,
    On<ev::click, SharedEventCallback<MouseEvent>>,
    On<ev::dblclick, SharedEventCallback<MouseEvent>>,
    On<ev::focus, SharedEventCallback<FocusEvent>>,
    On<ev::mouseenter, SharedEventCallback<MouseEvent>>,
);

/// Provides the behavior and accessibility for a grid row.
///
/// Reads shared grid state from `UseGridState<K>` (produced by `use_grid`)
/// and delegates selection to `use_selectable_item`.
///
/// # Example
///
/// ```ignore
/// let grid = use_grid(UseGridInput { ... });
///
/// let row = use_grid_row(UseGridRowInput {
///     state: grid.state,
///     key: "row-0".to_string(),
///     row_index: 0,
/// });
///
/// view! {
///     <div {..row.props.into_attrs()}>
///         // Grid cells...
///     </div>
/// }
/// ```
#[allow(clippy::needless_pass_by_value)]
pub fn use_grid_row<K>(input: UseGridRowInput<K>) -> UseGridRowReturn
where
    K: Hash + Eq + Clone + Send + Sync + 'static,
{
    let UseGridRowInput {
        state,
        key,
        row_index,
    } = input;

    let selection_mode = state.selection_mode;

    // --- Element capture for DOM focus synchronization ---
    let row_element = CapturedElement::new();

    let row_element_for_focus = row_element;
    let focus_fn = Callback::new(move |()| {
        if let Some(el) = row_element_for_focus.get_untracked() {
            focus_element(&SendWrapper::take(el), true);
        }
    });

    // --- Delegate selection to use_selectable_item ---
    let selectable = use_selectable_item(UseSelectableItemInput {
        key: key.clone(),
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
        on_double_click: state.on_row_action,
        focus: Some(focus_fn),
    });

    let is_selected = selectable.is_selected;
    let is_focused = selectable.is_focused;
    let is_disabled = selectable.is_disabled;

    // --- ARIA attributes ---
    let aria_selected = Signal::derive(move || {
        if selection_mode == SelectionMode::None {
            None
        } else {
            Some(AriaSelected::from(is_selected.get()))
        }
    });

    let tabindex = Signal::derive(move || if is_focused.get() { "0" } else { "-1" });

    // Index is 1-based for ARIA.
    let aria_rowindex = (row_index + 1).to_string();

    // --- Compose handlers from selectable_item ---
    let on_click = selectable.props.on_click;
    let on_dblclick = selectable.props.on_dblclick;
    let on_focus = selectable.props.on_focus;
    let on_mouseenter = selectable.props.on_mouseenter;

    UseGridRowReturn {
        props: UseGridRowProps {
            role: "row",
            tabindex,
            aria_rowindex,
            aria_selected,
            aria_disabled: Signal::derive(move || is_disabled.get().then_some(AriaDisabled::True)),
            element_capture: row_element.attr(),
            on_click,
            on_dblclick,
            on_focus,
            on_mouseenter,
        },
        is_selected,
        is_focused,
        is_disabled,
    }
}
