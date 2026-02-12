use leptos::{
    attr,
    attr::Attr,
    ev,
    ev::{On, SharedEventCallback},
    prelude::*,
};
use web_sys::{FocusEvent, KeyboardEvent, MouseEvent};

use super::use_table::SortDirection;
use crate::{
    hooks::{
        focus::use_focus_ring::{use_focus_ring, UseFocusRingInput, UseFocusRingReturn},
        IntoAttrs,
    },
    utils::{aria::AriaDisabled, EventHandler},
};

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/@react-aria/table/src/useTableColumnHeader.ts

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// No intentional deviations from the react-aria implementation.
//
// =============================================================================

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
    /// Props for programmatic merging. Call `.into_attrs()` for view spreading.
    pub column_props: UseTableColumnHeaderProps,

    /// The column key.
    pub column_key: String,

    /// Whether the column is sortable.
    pub is_sortable: bool,

    /// Whether the focus ring should be visible (keyboard navigation only).
    pub is_focus_visible: Signal<bool>,
}

/// Props from `use_table_column_header` that can be extracted and merged programmatically.
#[derive(Debug)]
pub struct UseTableColumnHeaderProps {
    pub role: &'static str,
    pub aria_colindex: String,
    pub aria_sort: Signal<Option<&'static str>>,
    pub tabindex: Signal<&'static str>,
    pub aria_disabled: Signal<Option<AriaDisabled>>,
    pub on_click: EventHandler<MouseEvent>,
    pub on_keydown: EventHandler<KeyboardEvent>,
    pub on_focus: EventHandler<FocusEvent>,
    pub on_blur: EventHandler<FocusEvent>,
    pub on_focusin: EventHandler<FocusEvent>,
    pub on_focusout: EventHandler<FocusEvent>,
    pub data_focus_visible: Signal<Option<&'static str>>,
}

impl IntoAttrs for UseTableColumnHeaderProps {
    type Attrs = UseTableColumnHeaderAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            Attr(attr::Role, self.role),
            Attr(attr::AriaColindex, self.aria_colindex),
            Attr(attr::AriaSort, self.aria_sort),
            Attr(attr::Tabindex, self.tabindex),
            Attr(attr::AriaDisabled, self.aria_disabled),
            self.on_click.into_on(ev::click),
            self.on_keydown.into_on(ev::keydown),
            self.on_focus.into_on(ev::focus),
            self.on_blur.into_on(ev::blur),
            self.on_focusin.into_on(ev::focusin),
            self.on_focusout.into_on(ev::focusout),
            attr::custom::custom_attribute("data-focus-visible", self.data_focus_visible),
        )
    }
}

/// Attributes for the table column header element.
pub type UseTableColumnHeaderAttrs = (
    Attr<attr::Role, &'static str>,
    Attr<attr::AriaColindex, String>,
    Attr<attr::AriaSort, Signal<Option<&'static str>>>,
    Attr<attr::Tabindex, Signal<&'static str>>,
    Attr<attr::AriaDisabled, Signal<Option<AriaDisabled>>>,
    On<ev::click, SharedEventCallback<MouseEvent>>,
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
///     <th {..column.column_props.into_attrs()}>
///         "Name"
///         {move || if column.is_sortable {
///             // Show sort indicator
///         }}
///     </th>
/// }
/// ```
#[allow(clippy::needless_pass_by_value)]
pub fn use_table_column_header(input: UseTableColumnHeaderInput) -> UseTableColumnHeaderReturn {
    let UseTableColumnHeaderInput {
        column_key,
        column_index,
        is_sortable,
        is_sorted,
        sort_direction,
        is_focused,
        is_disabled: disabled,
        on_sort,
    } = input;

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
    let aria_disabled = Signal::derive(move || disabled.get().then_some(AriaDisabled::True));

    // Handle click for sorting
    let handle_click = move |_e: web_sys::MouseEvent| {
        if disabled.get_untracked() || !is_sortable {
            return;
        }
        if let Some(on_sort) = on_sort {
            on_sort.run(());
        }
    };

    // Handle keyboard for sorting
    let handle_keydown = move |e: KeyboardEvent| {
        if disabled.get_untracked() || !is_sortable {
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
        disabled,
        within: false,
        auto_focus: false,
        on_focus: None,
        on_blur: None,
        on_focus_change: None,
    });

    // Column index is 1-based for ARIA
    let aria_colindex = (column_index + 1).to_string();

    UseTableColumnHeaderReturn {
        column_props: UseTableColumnHeaderProps {
            role: "columnheader",
            aria_colindex,
            aria_sort,
            tabindex,
            aria_disabled,
            on_click: EventHandler::new(handle_click),
            on_keydown: EventHandler::new(handle_keydown),
            on_focus: focus_ring_props.on_focus,
            on_blur: focus_ring_props.on_blur,
            on_focusin: focus_ring_props.on_focusin,
            on_focusout: focus_ring_props.on_focusout,
            data_focus_visible: focus_ring_props.data_focus_visible,
        },
        column_key,
        is_sortable,
        is_focus_visible,
    }
}
