use leptos::attr;
use leptos::attr::Attr;
use leptos::ev;
use leptos::ev::{On, SharedEventCallback};
use leptos::prelude::*;
use web_sys::{FocusEvent, KeyboardEvent, MouseEvent};

use super::use_table::TableSelectionMode;
use crate::hooks::focus::use_focus_ring::{use_focus_ring, UseFocusRingInput, UseFocusRingReturn};
use crate::utils::aria::{AriaDisabled, AriaSelected};
use crate::utils::EventHandler;

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/@react-aria/table/src/useTableRow.ts

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// No intentional deviations from the react-aria implementation.
//
// =============================================================================

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
    /// Props for programmatic merging. Call `.into_attrs()` for view spreading.
    pub row_props: UseTableRowProps,

    /// The row key.
    pub row_key: String,

    /// Whether the row is selected.
    pub is_selected: Signal<bool>,

    /// Whether the row is focused.
    pub is_focused: Signal<bool>,

    /// Whether the focus ring should be visible (keyboard navigation only).
    pub is_focus_visible: Signal<bool>,
}

/// Props from `use_table_row` that can be extracted and merged programmatically.
#[derive(Debug)]
pub struct UseTableRowProps {
    pub role: &'static str,
    pub aria_rowindex: String,
    pub aria_selected: Signal<Option<AriaSelected>>,
    pub aria_disabled: Signal<Option<AriaDisabled>>,
    pub tabindex: Signal<&'static str>,
    pub on_click: EventHandler<MouseEvent>,
    pub on_dblclick: EventHandler<MouseEvent>,
    pub on_keydown: EventHandler<KeyboardEvent>,
    pub on_focus: EventHandler<FocusEvent>,
    pub on_blur: EventHandler<FocusEvent>,
    pub on_focusin: EventHandler<FocusEvent>,
    pub on_focusout: EventHandler<FocusEvent>,
    pub data_focus_visible: Signal<Option<&'static str>>,
}

impl UseTableRowProps {
    /// Convert to spreadable attributes for Leptos views, consuming self.
    #[must_use]
    pub fn into_attrs(self) -> UseTableRowAttrs {
        (
            Attr(attr::Role, self.role),
            Attr(attr::AriaRowindex, self.aria_rowindex),
            Attr(attr::AriaSelected, self.aria_selected),
            Attr(attr::AriaDisabled, self.aria_disabled),
            Attr(attr::Tabindex, self.tabindex),
            self.on_click.into_on(ev::click),
            self.on_dblclick.into_on(ev::dblclick),
            self.on_keydown.into_on(ev::keydown),
            self.on_focus.into_on(ev::focus),
            self.on_blur.into_on(ev::blur),
            self.on_focusin.into_on(ev::focusin),
            self.on_focusout.into_on(ev::focusout),
            attr::custom::custom_attribute("data-focus-visible", self.data_focus_visible),
        )
    }
}

/// Attributes for the table row element.
pub type UseTableRowAttrs = (
    Attr<attr::Role, &'static str>,
    Attr<attr::AriaRowindex, String>,
    Attr<attr::AriaSelected, Signal<Option<AriaSelected>>>,
    Attr<attr::AriaDisabled, Signal<Option<AriaDisabled>>>,
    Attr<attr::Tabindex, Signal<&'static str>>,
    On<ev::click, SharedEventCallback<MouseEvent>>,
    On<ev::dblclick, SharedEventCallback<MouseEvent>>,
    On<ev::keydown, SharedEventCallback<KeyboardEvent>>,
    On<ev::focus, SharedEventCallback<FocusEvent>>,
    On<ev::blur, SharedEventCallback<FocusEvent>>,
    On<ev::focusin, SharedEventCallback<FocusEvent>>,
    On<ev::focusout, SharedEventCallback<FocusEvent>>,
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
///     <tr {..row.row_props.into_attrs()}>
///         // Table cells...
///     </tr>
/// }
/// ```
#[allow(clippy::too_many_lines, clippy::needless_pass_by_value)]
pub fn use_table_row(input: UseTableRowInput) -> UseTableRowReturn {
    let UseTableRowInput {
        row_key,
        row_index,
        is_selected,
        is_focused,
        is_disabled: disabled,
        selection_mode,
        on_selection_change,
        on_action,
        on_focus_next,
        on_focus_previous,
    } = input;

    // Compute aria-selected
    let aria_selected = Signal::derive(move || {
        if selection_mode == TableSelectionMode::None {
            None
        } else {
            Some(AriaSelected::from(is_selected.get()))
        }
    });

    // Compute aria-disabled
    let aria_disabled = Signal::derive(move || disabled.get().then_some(AriaDisabled::True));

    // Compute tabindex
    let tabindex = Signal::derive(move || if is_focused.get() { "0" } else { "-1" });

    // Handle click for selection
    let handle_click = move |e: web_sys::MouseEvent| {
        if disabled.get_untracked() || selection_mode == TableSelectionMode::None {
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
        if disabled.get_untracked() {
            return;
        }
        if let Some(on_action) = on_action {
            on_action.run(());
        }
    });

    // Handle keyboard
    let handle_keydown = move |e: KeyboardEvent| {
        if disabled.get_untracked() {
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
        disabled,
        within: false,
        auto_focus: false,
        on_focus: None,
        on_blur: None,
        on_focus_change: None,
    });

    // Row index is 1-based for ARIA (add 1 for header row)
    let aria_rowindex = (row_index + 2).to_string();

    UseTableRowReturn {
        row_props: UseTableRowProps {
            role: "row",
            aria_rowindex,
            aria_selected,
            aria_disabled,
            tabindex,
            on_click: EventHandler::new(handle_click),
            on_dblclick: handle_dblclick,
            on_keydown: EventHandler::new(handle_keydown),
            on_focus: focus_ring_props.on_focus,
            on_blur: focus_ring_props.on_blur,
            on_focusin: focus_ring_props.on_focusin,
            on_focusout: focus_ring_props.on_focusout,
            data_focus_visible: focus_ring_props.data_focus_visible,
        },
        row_key,
        is_selected,
        is_focused,
        is_focus_visible,
    }
}
