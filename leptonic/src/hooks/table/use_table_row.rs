use leptos::attr;
use leptos::attr::{Attr, Attribute};
use leptos::ev;
use leptos::ev::{on, On, SharedEventCallback};
use leptos::prelude::*;
use web_sys::{FocusEvent, KeyboardEvent};

use super::use_table::TableSelectionMode;
use crate::hooks::focus::use_focus_ring::{use_focus_ring, UseFocusRingInput, UseFocusRingReturn};
use crate::utils::EventHandler;

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/@react-aria/table/src/useTableRow.ts

/// Input parameters for the `use_table_row` hook.
#[derive(Debug, Clone)]
pub struct UseTableRowInput {
    /// The unique key for this row.
    pub row_key: String,

    /// The row index (for aria-rowindex).
    pub row_index: usize,

    /// Whether the row is selected.
    pub is_selected: Signal<bool>,

    /// Whether the row is focused.
    pub is_focused: Signal<bool>,

    /// Whether the row is disabled.
    pub is_disabled: Signal<bool>,

    /// The selection mode from the parent table.
    pub selection_mode: TableSelectionMode,

    /// Callback when the row is selected/deselected.
    pub on_selection_change: Option<Callback<bool>>,

    /// Callback when the row is activated (Enter/double-click).
    pub on_action: Option<Callback<()>>,

    /// Callback when focus moves to the next row.
    pub on_focus_next: Option<Callback<()>>,

    /// Callback when focus moves to the previous row.
    pub on_focus_previous: Option<Callback<()>>,
}

/// The return value of the `use_table_row` hook.
pub struct UseTableRowReturn {
    /// Props for the row element.
    pub row_props: UseTableRowAttrs,

    /// The row key.
    pub row_key: String,

    /// Whether the row is selected.
    pub is_selected: Signal<bool>,

    /// Whether the row is focused.
    pub is_focused: Signal<bool>,

    /// Whether the focus ring should be visible (keyboard navigation only).
    pub is_focus_visible: Signal<bool>,
}

/// Attributes for the table row element.
pub type UseTableRowAttrs = (
    Attr<attr::Role, &'static str>,
    Attr<attr::AriaRowindex, String>,
    Attr<attr::AriaSelected, Signal<Option<&'static str>>>,
    Attr<attr::AriaDisabled, Signal<&'static str>>,
    Attr<attr::Tabindex, Signal<&'static str>>,
    On<ev::click, SharedEventCallback<web_sys::MouseEvent>>,
    On<ev::dblclick, SharedEventCallback<web_sys::MouseEvent>>,
    On<ev::keydown, SharedEventCallback<KeyboardEvent>>,
    On<ev::focus, SharedEventCallback<FocusEvent>>,
    On<ev::blur, SharedEventCallback<FocusEvent>>,
    attr::custom::CustomAttr<&'static str, Signal<Option<&'static str>>>,
);

/// Provides the behavior and accessibility for a table row.
///
/// A table row represents a single data record and can be selectable.
///
/// # Example
///
/// ```ignore
/// let row = use_table_row(UseTableRowInput {
///     row_key: user.id.clone(),
///     row_index: index,
///     is_selected: is_row_selected.into(),
///     is_focused: is_row_focused.into(),
///     is_disabled: Signal::derive(|| false),
///     selection_mode: TableSelectionMode::Multiple,
///     on_selection_change: Some(Callback::new(|selected| { /* update selection */ })),
///     on_action: Some(Callback::new(|_| { /* open detail view */ })),
///     ..Default::default()
/// });
///
/// view! {
///     <tr {..row.row_props}>
///         // Table cells...
///     </tr>
/// }
/// ```
#[allow(clippy::too_many_lines, clippy::needless_pass_by_value)]
pub fn use_table_row(input: UseTableRowInput) -> UseTableRowReturn {
    let row_key = input.row_key.clone();
    let is_selected = input.is_selected;
    let is_focused = input.is_focused;
    let is_disabled = input.is_disabled;
    let selection_mode = input.selection_mode;
    let on_selection_change = input.on_selection_change;
    let on_action = input.on_action;
    let on_focus_next = input.on_focus_next;
    let on_focus_previous = input.on_focus_previous;

    // Compute aria-selected
    let aria_selected = Signal::derive(move || {
        if selection_mode == TableSelectionMode::None {
            None
        } else if is_selected.get() {
            Some("true")
        } else {
            Some("false")
        }
    });

    // Compute aria-disabled
    let aria_disabled = Signal::derive(move || if is_disabled.get() { "true" } else { "false" });

    // Compute tabindex
    let tabindex = Signal::derive(move || if is_focused.get() { "0" } else { "-1" });

    // Handle click for selection
    let handle_click = move |e: web_sys::MouseEvent| {
        if is_disabled.get_untracked() || selection_mode == TableSelectionMode::None {
            return;
        }

        let should_toggle = e.ctrl_key() || e.meta_key();
        let new_selected = if should_toggle {
            !is_selected.get_untracked()
        } else {
            true
        };

        if let Some(on_change) = on_selection_change {
            on_change.run(new_selected);
        }
    };

    // Handle double-click for action
    let handle_dblclick = EventHandler::new(move |_e: web_sys::MouseEvent| {
        if is_disabled.get_untracked() {
            return;
        }
        if let Some(on_action) = on_action {
            on_action.run(());
        }
    });

    // Handle keyboard
    let handle_keydown = move |e: KeyboardEvent| {
        if is_disabled.get_untracked() {
            return;
        }

        let key = e.key();
        match key.as_str() {
            "Enter" => {
                e.prevent_default();
                if let Some(on_action) = on_action {
                    on_action.run(());
                }
            }
            " " => {
                if selection_mode != TableSelectionMode::None {
                    e.prevent_default();
                    let new_selected = !is_selected.get_untracked();
                    if let Some(on_change) = on_selection_change {
                        on_change.run(new_selected);
                    }
                }
            }
            "ArrowDown" => {
                e.prevent_default();
                if let Some(on_next) = on_focus_next {
                    on_next.run(());
                }
            }
            "ArrowUp" => {
                e.prevent_default();
                if let Some(on_prev) = on_focus_previous {
                    on_prev.run(());
                }
            }
            _ => {}
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
        on_focus: None,
        on_blur: None,
        on_focus_change: None,
    });
    let (on_focus, on_blur, data_focus_visible) = focus_ring_props.into_attrs();

    // Row index is 1-based for ARIA (add 1 for header row)
    let aria_rowindex = (input.row_index + 2).to_string();

    UseTableRowReturn {
        row_props: (
            Attr(attr::Role, "row"),
            Attr(attr::AriaRowindex, aria_rowindex),
            Attr(attr::AriaSelected, aria_selected),
            Attr(attr::AriaDisabled, aria_disabled),
            Attr(attr::Tabindex, tabindex),
            on(ev::click, handle_click).into_cloneable(),
            handle_dblclick.into_on(ev::dblclick),
            on(ev::keydown, handle_keydown).into_cloneable(),
            on_focus,
            on_blur,
            data_focus_visible,
        ),
        row_key,
        is_selected,
        is_focused,
        is_focus_visible,
    }
}

/// Input for the table header row.
#[derive(Debug, Clone, Copy, Default)]
pub struct UseTableHeaderRowInput {
    /// The row index (usually 0 for header).
    pub row_index: usize,
}

/// Return value for the table header row.
pub struct UseTableHeaderRowReturn {
    /// Props for the header row element.
    pub row_props: UseTableHeaderRowAttrs,
}

/// Attributes for the table header row element.
pub type UseTableHeaderRowAttrs = (
    Attr<attr::Role, &'static str>,
    Attr<attr::AriaRowindex, &'static str>,
);

/// Provides the behavior and accessibility for a table header row.
pub fn use_table_header_row(_input: UseTableHeaderRowInput) -> UseTableHeaderRowReturn {
    UseTableHeaderRowReturn {
        row_props: (Attr(attr::Role, "row"), Attr(attr::AriaRowindex, "1")),
    }
}
