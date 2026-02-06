use leptos::attr;
use leptos::attr::{Attr, Attribute};
use leptos::ev;
use leptos::ev::{on, On, SharedEventCallback};
use leptos::prelude::*;
use web_sys::{FocusEvent, KeyboardEvent};

use crate::hooks::focus::use_focus_ring::{use_focus_ring, UseFocusRingInput, UseFocusRingReturn};
use crate::hooks::{use_grid_cell, UseGridCellInput};
// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/@react-aria/table/src/useTableCell.ts

/// Input parameters for the `use_table_cell` hook.
#[derive(Debug, Clone)]
pub struct UseTableCellInput {
    /// The column index (for aria-colindex).
    pub column_index: usize,

    /// Whether the cell is focused.
    pub is_focused: Signal<bool>,

    /// Whether the cell is disabled.
    pub is_disabled: Signal<bool>,

    /// Whether the cell contains an interactive element.
    pub is_interactive: bool,

    /// Callback when the cell receives focus.
    pub on_focus: Option<Callback<()>>,

    /// Callback when navigating to the next cell.
    pub on_focus_next: Option<Callback<()>>,

    /// Callback when navigating to the previous cell.
    pub on_focus_previous: Option<Callback<()>>,
}

impl Default for UseTableCellInput {
    fn default() -> Self {
        Self {
            column_index: 0,
            is_focused: Signal::derive(|| false),
            is_disabled: Signal::derive(|| false),
            is_interactive: false,
            on_focus: None,
            on_focus_next: None,
            on_focus_previous: None,
        }
    }
}

/// The return value of the `use_table_cell` hook.
pub struct UseTableCellReturn {
    /// Props for the cell element.
    pub cell_props: UseTableCellAttrs,

    /// Whether the cell is focused.
    pub is_focused: Signal<bool>,

    /// Whether the focus ring should be visible (keyboard navigation only).
    pub is_focus_visible: Signal<bool>,
}

/// Attributes for the table cell element.
pub type UseTableCellAttrs = (
    Attr<attr::Role, &'static str>,
    Attr<attr::AriaColindex, String>,
    Attr<attr::Tabindex, Signal<&'static str>>,
    Attr<attr::AriaDisabled, Signal<&'static str>>,
    On<ev::keydown, SharedEventCallback<KeyboardEvent>>,
    On<ev::focus, SharedEventCallback<FocusEvent>>,
    On<ev::blur, SharedEventCallback<FocusEvent>>,
    leptos::attr::custom::CustomAttr<&'static str, Signal<Option<&'static str>>>,
);

/// Provides the behavior and accessibility for a table cell.
///
/// A table cell displays data within a row and column intersection.
///
/// # Example
///
/// ```ignore
/// let cell = use_table_cell(UseTableCellInput {
///     column_index: col_idx,
///     is_focused: is_cell_focused.into(),
///     is_disabled: Signal::derive(|| false),
///     ..Default::default()
/// });
///
/// view! {
///     <td {..cell.cell_props}>
///         {data}
///     </td>
/// }
/// ```
#[allow(clippy::needless_pass_by_value)]
pub fn use_table_cell(input: UseTableCellInput) -> UseTableCellReturn {
    let is_focused = input.is_focused;
    let is_disabled = input.is_disabled;
    let is_interactive = input.is_interactive;
    let on_focus = input.on_focus;
    let on_focus_next = input.on_focus_next;
    let on_focus_previous = input.on_focus_previous;

    let _s = use_grid_cell(UseGridCellInput {
        cell_key: String::new(),
        row_index: 0,
        column_index: 0,
        is_selected: Signal::default(),
        is_focused,
        is_disabled,
        on_navigate: None,
        on_action: None,
        on_selection_change: None,
    });

    // Compute tabindex
    let tabindex = Signal::derive(move || if is_focused.get() { "0" } else { "-1" });

    // Compute aria-disabled
    let aria_disabled = Signal::derive(move || if is_disabled.get() { "true" } else { "false" });

    // Handle keyboard navigation
    let handle_keydown = move |e: KeyboardEvent| {
        if is_disabled.get_untracked() {
            return;
        }

        // If the cell contains interactive content, let it handle events
        if is_interactive {
            return;
        }

        let key = e.key();
        match key.as_str() {
            "ArrowRight" => {
                e.prevent_default();
                if let Some(on_next) = on_focus_next {
                    on_next.run(());
                }
            }
            "ArrowLeft" => {
                e.prevent_default();
                if let Some(on_prev) = on_focus_previous {
                    on_prev.run(());
                }
            }
            _ => {}
        }
    };

    // Use focus ring for keyboard focus visibility with user callback
    let UseFocusRingReturn {
        props: focus_ring_props,
        is_focus_visible,
        is_focused: _,
    } = use_focus_ring(UseFocusRingInput {
        disabled: is_disabled,
        within: false,
        auto_focus: false,
        on_focus: on_focus.map(|cb| Callback::new(move |_| cb.run(()))),
        on_blur: None,
        on_focus_change: None,
    });
    let (handle_focus, handle_blur, data_focus_visible) = focus_ring_props.into_attrs();

    // Column index is 1-based for ARIA
    let aria_colindex = (input.column_index + 1).to_string();

    UseTableCellReturn {
        cell_props: (
            Attr(attr::Role, "gridcell"),
            Attr(attr::AriaColindex, aria_colindex),
            Attr(attr::Tabindex, tabindex),
            Attr(attr::AriaDisabled, aria_disabled),
            on(ev::keydown, handle_keydown).into_cloneable(),
            handle_focus,
            handle_blur,
            data_focus_visible,
        ),
        is_focused,
        is_focus_visible,
    }
}

/// Input for a checkbox cell in a table.
#[derive(Debug, Clone, Copy)]
pub struct UseTableCheckboxCellInput {
    /// Whether the checkbox is selected.
    pub is_selected: Signal<bool>,

    /// Whether the checkbox is indeterminate (for select-all).
    pub is_indeterminate: Signal<bool>,

    /// Whether the checkbox is disabled.
    pub is_disabled: Signal<bool>,

    /// Callback when the checkbox changes.
    pub on_change: Option<Callback<bool>>,
}

/// Return value for a checkbox cell.
pub struct UseTableCheckboxCellReturn {
    /// Props for the checkbox cell.
    pub cell_props: UseTableCheckboxCellAttrs,

    /// Props for the checkbox input.
    pub checkbox_props: UseTableCheckboxAttrs,
}

/// Attributes for the checkbox cell.
pub type UseTableCheckboxCellAttrs = (Attr<attr::Role, &'static str>,);

/// Attributes for the checkbox input.
pub type UseTableCheckboxAttrs = (
    Attr<attr::Type, &'static str>,
    Attr<attr::Checked, Signal<bool>>,
    Attr<attr::AriaLabel, &'static str>,
    Attr<attr::AriaDisabled, Signal<&'static str>>,
    On<ev::change, SharedEventCallback<web_sys::Event>>,
);

/// Provides the behavior for a checkbox selection cell in a table.
///
/// # Example
///
/// ```ignore
/// let checkbox = use_table_checkbox_cell(UseTableCheckboxCellInput {
///     is_selected: is_row_selected.into(),
///     is_indeterminate: Signal::derive(|| false),
///     is_disabled: Signal::derive(|| false),
///     on_change: Some(Callback::new(|checked| { /* update selection */ })),
/// });
///
/// view! {
///     <td {..checkbox.cell_props}>
///         <input {..checkbox.checkbox_props} />
///     </td>
/// }
/// ```
pub fn use_table_checkbox_cell(input: UseTableCheckboxCellInput) -> UseTableCheckboxCellReturn {
    let is_selected = input.is_selected;
    let is_disabled = input.is_disabled;
    let on_change = input.on_change;

    let aria_disabled = Signal::derive(move || if is_disabled.get() { "true" } else { "false" });

    let handle_change = move |_e: web_sys::Event| {
        if is_disabled.get_untracked() {
            return;
        }
        if let Some(on_change) = on_change {
            on_change.run(!is_selected.get_untracked());
        }
    };

    UseTableCheckboxCellReturn {
        cell_props: (Attr(attr::Role, "gridcell"),),
        checkbox_props: (
            Attr(attr::Type, "checkbox"),
            Attr(attr::Checked, is_selected),
            Attr(attr::AriaLabel, "Select row"),
            Attr(attr::AriaDisabled, aria_disabled),
            on(ev::change, handle_change).into_cloneable(),
        ),
    }
}
