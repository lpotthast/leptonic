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
use crate::utils::EventHandler;

use super::use_grid_list::UseGridListState;

/// Input for a grid list item.
pub struct UseGridListItemInput<K>
where
    K: Hash + Eq + Clone + Send + Sync + 'static,
{
    /// Shared grid list state from `use_grid_list`.
    pub state: UseGridListState<K>,

    /// The unique key for this item.
    pub key: K,

    /// The row index (0-based; converted to 1-based for ARIA).
    pub row_index: usize,

    /// Whether this item is disabled.
    pub is_disabled: Signal<bool>,

    /// Accessible text value for this row.
    pub text_value: Option<String>,
}

/// Return value for a grid list item.
pub struct UseGridListItemReturn {
    /// Props for the row element (`role="row"`).
    pub row_props: UseGridListItemRowProps,

    /// Props for the gridcell element (`role="gridcell"`).
    pub gridcell_props: UseGridListItemGridCellProps,

    /// Whether the item is selected.
    pub is_selected: Signal<bool>,

    /// Whether the item is focused.
    pub is_focused: Signal<bool>,

    /// Whether the item is disabled.
    pub is_disabled: Signal<bool>,
}

/// Props for the row element of a grid list item.
#[derive(Clone)]
pub struct UseGridListItemRowProps {
    pub role: &'static str,
    pub tabindex: Signal<&'static str>,
    pub aria_rowindex: String,
    pub aria_selected: Signal<Option<&'static str>>,
    pub aria_disabled: Signal<&'static str>,
    pub aria_label: Option<String>,
    pub on_keydown: EventHandler<KeyboardEvent>,
    pub on_click: EventHandler<MouseEvent>,
    pub on_dblclick: EventHandler<MouseEvent>,
    pub on_focus: EventHandler<FocusEvent>,
    pub on_mouseenter: EventHandler<MouseEvent>,
}

impl UseGridListItemRowProps {
    /// Convert to spreadable attributes for Leptos views, cloning internally.
    #[must_use]
    pub fn to_attrs(&self) -> UseGridListItemRowAttrs {
        self.clone().into_attrs()
    }

    /// Convert to spreadable attributes for Leptos views, consuming self.
    #[must_use]
    pub fn into_attrs(self) -> UseGridListItemRowAttrs {
        (
            Attr(attr::Role, self.role),
            Attr(attr::Tabindex, self.tabindex),
            Attr(attr::AriaRowindex, self.aria_rowindex),
            Attr(attr::AriaSelected, self.aria_selected),
            Attr(attr::AriaDisabled, self.aria_disabled),
            Attr(attr::AriaLabel, self.aria_label),
            self.on_keydown.into_on(ev::keydown),
            self.on_click.into_on(ev::click),
            self.on_dblclick.into_on(ev::dblclick),
            self.on_focus.into_on(ev::focus),
            self.on_mouseenter.into_on(ev::mouseenter),
        )
    }
}

/// Attributes for the row element of a grid list item.
pub type UseGridListItemRowAttrs = (
    Attr<attr::Role, &'static str>,
    Attr<attr::Tabindex, Signal<&'static str>>,
    Attr<attr::AriaRowindex, String>,
    Attr<attr::AriaSelected, Signal<Option<&'static str>>>,
    Attr<attr::AriaDisabled, Signal<&'static str>>,
    Attr<attr::AriaLabel, Option<String>>,
    On<ev::keydown, SharedEventCallback<KeyboardEvent>>,
    On<ev::click, SharedEventCallback<MouseEvent>>,
    On<ev::dblclick, SharedEventCallback<MouseEvent>>,
    On<ev::focus, SharedEventCallback<FocusEvent>>,
    On<ev::mouseenter, SharedEventCallback<MouseEvent>>,
);

/// Props for the gridcell element of a grid list item.
#[derive(Clone)]
pub struct UseGridListItemGridCellProps {
    pub role: &'static str,
    pub aria_colindex: &'static str,
    pub element_capture: ElementCaptureAttr,
}

impl UseGridListItemGridCellProps {
    /// Convert to spreadable attributes for Leptos views, cloning internally.
    #[must_use]
    pub fn to_attrs(&self) -> UseGridListItemGridCellAttrs {
        self.clone().into_attrs()
    }

    /// Convert to spreadable attributes for Leptos views, consuming self.
    #[must_use]
    pub fn into_attrs(self) -> UseGridListItemGridCellAttrs {
        (
            Attr(attr::Role, self.role),
            Attr(attr::AriaColindex, self.aria_colindex),
            self.element_capture,
        )
    }
}

/// Attributes for the gridcell element of a grid list item.
pub type UseGridListItemGridCellAttrs = (
    Attr<attr::Role, &'static str>,
    Attr<attr::AriaColindex, &'static str>,
    ElementCaptureAttr,
);

/// Provides the behavior and accessibility for a grid list item.
///
/// Each item renders as a row with a single gridcell:
/// ```html
/// <div {..row_props}>      <!-- role="row" -->
///   <div {..gridcell_props}> <!-- role="gridcell" -->
///     <!-- user content with focusable children -->
///   </div>
/// </div>
/// ```
///
/// ArrowLeft/ArrowRight navigate between focusable children within the gridcell.
/// All other keyboard events (ArrowUp/Down, Space, Enter, Home, End, Escape, Ctrl+A)
/// bubble to the grid list container handler.
///
/// # Example
///
/// ```ignore
/// let grid_list = use_grid_list(UseGridListInput { ... });
///
/// let item = use_grid_list_item(UseGridListItemInput {
///     state: grid_list.state,
///     key: "item-0".to_string(),
///     row_index: 0,
///     is_disabled: false.into(),
///     text_value: Some("Item 0".to_string()),
/// });
///
/// view! {
///     <div {..item.row_props.into_attrs()}>
///         <div {..item.gridcell_props.into_attrs()}>
///             "Item content"
///         </div>
///     </div>
/// }
/// ```
#[allow(clippy::needless_pass_by_value, clippy::too_many_lines)]
pub fn use_grid_list_item<K>(input: UseGridListItemInput<K>) -> UseGridListItemReturn
where
    K: Hash + Eq + Clone + Send + Sync + 'static,
{
    let state = input.state;
    let selection_mode = state.selection_mode;

    // --- Delegate selection to use_selectable_item ---
    let selectable = use_selectable_item(UseSelectableItemInput {
        key: input.key.clone(),
        selection_mode: state.selection_mode,
        selection_behavior: state.selection_behavior,
        selected_keys: state.selection.selected_keys,
        focused_key: state.focused_key,
        is_disabled: input.is_disabled,
        on_toggle: state.selection.toggle,
        on_select: state.selection.select,
        on_focus: state.set_focused_key,
        should_select_on_press_up: false,
        allow_drag: false,
        on_double_click: None,
    });

    let is_selected = selectable.is_selected;
    let is_focused = selectable.is_focused;
    let is_disabled = selectable.is_disabled;

    // --- Element capture + FocusManager for within-row child navigation ---
    let scope_element = CapturedElement::new();

    let focus_manager =
        FocusManager::new(move || scope_element.get_untracked().map(SendWrapper::take));

    // --- ARIA attributes ---
    let aria_selected = Signal::derive(move || {
        if selection_mode == SelectionMode::None {
            None
        } else if is_selected.get() {
            Some("true")
        } else {
            Some("false")
        }
    });

    let tabindex = Signal::derive(move || if is_focused.get() { "0" } else { "-1" });

    // Row index is 1-based for ARIA.
    let aria_rowindex = (input.row_index + 1).to_string();

    // --- Row-level keyboard handler (within-row child navigation) ---
    let focus_manager_for_keydown = focus_manager.clone();
    let row_keydown = EventHandler::new(move |e: KeyboardEvent| {
        if state.is_disabled.get_untracked() {
            return;
        }

        let key = e.key();
        match key.as_str() {
            "ArrowRight" => {
                // Try to move focus to the next focusable child inside the gridcell.
                // If successful, stop propagation so the container handler doesn't also process it.
                if focus_manager_for_keydown
                    .focus_next(FocusManagerOptions::default())
                    .is_some()
                {
                    e.stop_propagation();
                    e.prevent_default();
                }
                // Otherwise: no-op. The event may bubble to the container which also ignores
                // ArrowLeft/ArrowRight.
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
            // ArrowUp, ArrowDown, Space, Enter, Home, End, Escape, Ctrl+A:
            // let bubble to grid list container handler.
            _ => {}
        }
    });

    // --- Focus handler ---
    let key_for_focus = input.key.clone();
    let set_focused_key = state.set_focused_key;
    let row_focus = EventHandler::new(move |e: FocusEvent| {
        // If a child element received focus (target != currentTarget), update the grid list's
        // focused key to this row so the container knows which row is active.
        let target = e.target();
        let current_target = e.current_target();
        if target != current_target {
            set_focused_key.run(Some(key_for_focus.clone()));
        }
    });

    // --- Compose handlers ---
    let on_click = selectable.props.on_click;
    let on_dblclick = selectable.props.on_dblclick;
    let on_keydown = row_keydown;
    let on_focus = selectable.props.on_focus.chain(row_focus);
    let on_mouseenter = selectable.props.on_mouseenter;

    UseGridListItemReturn {
        row_props: UseGridListItemRowProps {
            role: "row",
            tabindex,
            aria_rowindex,
            aria_selected,
            aria_disabled: Signal::derive(move || if is_disabled.get() { "true" } else { "false" }),
            aria_label: input.text_value,
            on_keydown,
            on_click,
            on_dblclick,
            on_focus,
            on_mouseenter,
        },
        gridcell_props: UseGridListItemGridCellProps {
            role: "gridcell",
            aria_colindex: "1",
            element_capture: scope_element.attr(),
        },
        is_selected,
        is_focused,
        is_disabled,
    }
}
