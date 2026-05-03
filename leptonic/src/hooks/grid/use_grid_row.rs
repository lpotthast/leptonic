use leptos::{
    attr,
    attr::Attr,
    ev,
    ev::{On, SharedEventCallback},
    prelude::*,
};
use send_wrapper::SendWrapper;
use web_sys::{FocusEvent, MouseEvent};

use super::use_grid::UseGridState;
use crate::{
    hooks::{
        IntoAttrs, PropsWithStyles,
        selection::{
            SelectionKey,
            use_selectable_item::{UseSelectableItemInput, use_selectable_item},
            use_selection_state::SelectionMode,
        },
    },
    utils::{
        EventHandler,
        aria::{AriaDisabled, AriaRole, AriaSelected},
        element_capture::{CapturedElement, ElementCaptureAttr},
        focus::focus_element,
    },
};

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/@react-aria/grid/src/useGridRow.ts

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

/// Input for a grid row.
pub struct UseGridRowInput<K>
where
    K: SelectionKey,
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
    pub props: PropsWithStyles<UseGridRowProps>,

    /// Whether the row is selected.
    pub is_selected: Signal<bool>,

    /// Whether the row is focused.
    pub is_focused: Signal<bool>,

    /// Whether the row is disabled.
    pub is_disabled: Signal<bool>,
}

/// Props from `use_grid_row` that can be extracted and merged programmatically.
#[derive(Debug, Clone)]
pub struct UseGridRowProps {
    pub role: AriaRole,
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

impl IntoAttrs for UseGridRowProps {
    type Attrs = UseGridRowAttrs;

    fn into_attrs(self) -> Self::Attrs {
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
    Attr<attr::Role, AriaRole>,
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
    K: SelectionKey,
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
        is_collection_focused: Signal::derive(|| true),
        is_disabled: state.is_disabled,
        disabled_behavior: state.selection.disabled_behavior,
        disallow_empty_selection: false,
        on_toggle: state.selection.toggle,
        on_replace: state.selection.select,
        on_extend: None,
        on_focus: state.set_focused_key,
        should_select_on_press_up: false,
        should_focus_on_hover: false,
        allow_drag: false,
        allows_different_press_origin: false,
        on_action: None,
        on_double_click: state.on_row_action,
        on_selection_behavior_change: None,
        focus: Some(focus_fn),
        data_key: None,
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
    let (selectable_props, selectable_styles) = selectable.props.into_inner();
    let on_click = selectable_props.press.on_click;
    let on_dblclick = selectable_props.press.on_dblclick;
    let on_focus = selectable_props.on_focus;
    let on_mouseenter = selectable_props.on_mouseenter;

    UseGridRowReturn {
        props: PropsWithStyles::new(
            UseGridRowProps {
                role: AriaRole::Row,
                tabindex,
                aria_rowindex,
                aria_selected,
                aria_disabled: Signal::derive(move || {
                    is_disabled.get().then_some(AriaDisabled::True)
                }),
                element_capture: row_element.attr(),
                on_click,
                on_dblclick,
                on_focus,
                on_mouseenter,
            },
            selectable_styles,
        ),
        is_selected,
        is_focused,
        is_disabled,
    }
}
