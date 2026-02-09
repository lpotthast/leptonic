use leptos::attr;
use leptos::attr::{Attr, Attribute};
use leptos::ev;
use leptos::ev::{on, On, SharedEventCallback};
use leptos::prelude::*;
use web_sys::{FocusEvent, KeyboardEvent};

use super::use_table::SortDirection;
use crate::hooks::focus::use_focus_ring::{use_focus_ring, UseFocusRingInput, UseFocusRingReturn};

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/@react-aria/table/src/useTableColumnHeader.ts

/// Input parameters for the `use_table_column_header` hook.
#[derive(Debug, Clone)]
pub struct UseTableColumnHeaderInput {
    /// The unique key for this column.
    pub column_key: String,

    /// The column index (for aria-colindex).
    pub column_index: usize,

    /// Whether the column is sortable.
    pub is_sortable: bool,

    /// Whether the column is currently sorted.
    pub is_sorted: Signal<bool>,

    /// The current sort direction (if sorted).
    pub sort_direction: Signal<Option<SortDirection>>,

    /// Whether the column header is focused.
    pub is_focused: Signal<bool>,

    /// Whether the table is disabled.
    pub is_disabled: Signal<bool>,

    /// Callback when sort is triggered.
    pub on_sort: Option<Callback<()>>,
}

/// The return value of the `use_table_column_header` hook.
pub struct UseTableColumnHeaderReturn {
    /// Props for the column header element.
    pub column_props: UseTableColumnHeaderAttrs,

    /// The column key.
    pub column_key: String,

    /// Whether the column is sortable.
    pub is_sortable: bool,

    /// Whether the focus ring should be visible (keyboard navigation only).
    pub is_focus_visible: Signal<bool>,
}

/// Attributes for the table column header element.
pub type UseTableColumnHeaderAttrs = (
    Attr<attr::Role, &'static str>,
    Attr<attr::AriaColindex, String>,
    Attr<attr::AriaSort, Signal<Option<&'static str>>>,
    Attr<attr::Tabindex, Signal<&'static str>>,
    Attr<attr::AriaDisabled, Signal<&'static str>>,
    On<ev::click, SharedEventCallback<web_sys::MouseEvent>>,
    On<ev::keydown, SharedEventCallback<KeyboardEvent>>,
    On<ev::focus, SharedEventCallback<FocusEvent>>,
    On<ev::blur, SharedEventCallback<FocusEvent>>,
    On<ev::focusin, SharedEventCallback<FocusEvent>>,
    On<ev::focusout, SharedEventCallback<FocusEvent>>,
    attr::custom::CustomAttr<&'static str, Signal<Option<&'static str>>>,
);

/// Provides the behavior and accessibility for a table column header.
///
/// A column header can be sortable and displays the current sort direction.
///
/// # Example
///
/// ```ignore
/// let column = use_table_column_header(UseTableColumnHeaderInput {
///     column_key: "name".to_string(),
///     column_index: 1,
///     is_sortable: true,
///     is_sorted: is_name_sorted.into(),
///     sort_direction: name_sort_direction.into(),
///     is_focused: is_focused.into(),
///     is_disabled: Signal::derive(|| false),
///     on_sort: Some(Callback::new(|_| { /* toggle sort */ })),
/// });
///
/// view! {
///     <th {..column.column_props}>
///         "Name"
///         {move || if column.is_sortable {
///             // Show sort indicator
///         }}
///     </th>
/// }
/// ```
#[allow(clippy::needless_pass_by_value)]
pub fn use_table_column_header(input: UseTableColumnHeaderInput) -> UseTableColumnHeaderReturn {
    let column_key = input.column_key.clone();
    let is_sortable = input.is_sortable;
    let is_sorted = input.is_sorted;
    let sort_direction = input.sort_direction;
    let is_focused = input.is_focused;
    let is_disabled = input.is_disabled;
    let on_sort = input.on_sort;

    // Compute aria-sort
    let aria_sort = Signal::derive(move || {
        if !is_sortable || !is_sorted.get() {
            None
        } else {
            match sort_direction.get() {
                Some(SortDirection::Ascending) => Some("ascending"),
                Some(SortDirection::Descending) => Some("descending"),
                None => Some("none"),
            }
        }
    });

    // Compute tabindex
    let tabindex = Signal::derive(move || {
        if is_focused.get() && is_sortable {
            "0"
        } else {
            "-1"
        }
    });

    // Compute aria-disabled
    let aria_disabled = Signal::derive(move || if is_disabled.get() { "true" } else { "false" });

    // Handle click for sorting
    let handle_click = move |_e: web_sys::MouseEvent| {
        if is_disabled.get_untracked() || !is_sortable {
            return;
        }
        if let Some(on_sort) = on_sort {
            on_sort.run(());
        }
    };

    // Handle keyboard for sorting
    let handle_keydown = move |e: KeyboardEvent| {
        if is_disabled.get_untracked() || !is_sortable {
            return;
        }

        let key = e.key();
        if key == "Enter" || key == " " {
            e.prevent_default();
            if let Some(on_sort) = on_sort {
                on_sort.run(());
            }
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
    let (on_focus, on_blur, on_focusin, on_focusout, data_focus_visible) =
        focus_ring_props.into_attrs();

    // Column index is 1-based for ARIA
    let aria_colindex = (input.column_index + 1).to_string();

    UseTableColumnHeaderReturn {
        column_props: (
            Attr(attr::Role, "columnheader"),
            Attr(attr::AriaColindex, aria_colindex),
            Attr(attr::AriaSort, aria_sort),
            Attr(attr::Tabindex, tabindex),
            Attr(attr::AriaDisabled, aria_disabled),
            on(ev::click, handle_click).into_cloneable(),
            on(ev::keydown, handle_keydown).into_cloneable(),
            on_focus,
            on_blur,
            on_focusin,
            on_focusout,
            data_focus_visible,
        ),
        column_key,
        is_sortable,
        is_focus_visible,
    }
}
