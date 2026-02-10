use leptos::attr;
use leptos::attr::Attr;
use leptos::ev;
use leptos::ev::{On, SharedEventCallback};
use leptos::prelude::*;
use web_sys::{FocusEvent, KeyboardEvent};

use crate::hooks::focus::use_focus_ring::{use_focus_ring, UseFocusRingInput, UseFocusRingReturn};
use crate::utils::aria::AriaDisabled;
use crate::utils::EventHandler;

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
    /// Props for programmatic merging. Call `.to_attrs()` or `.into_attrs()` for view spreading.
    pub cell_props: UseTableCellProps,

    /// Whether the cell is focused.
    pub is_focused: Signal<bool>,

    /// Whether the focus ring should be visible (keyboard navigation only).
    pub is_focus_visible: Signal<bool>,
}

/// Props from `use_table_cell` that can be extracted and merged programmatically.
#[derive(Debug, Clone)]
pub struct UseTableCellProps {
    pub role: &'static str,
    pub aria_colindex: String,
    pub tabindex: Signal<&'static str>,
    pub aria_disabled: Signal<Option<AriaDisabled>>,
    pub on_keydown: EventHandler<KeyboardEvent>,
    pub on_focus: EventHandler<FocusEvent>,
    pub on_blur: EventHandler<FocusEvent>,
    pub on_focusin: EventHandler<FocusEvent>,
    pub on_focusout: EventHandler<FocusEvent>,
    pub data_focus_visible: Signal<Option<&'static str>>,
}

impl UseTableCellProps {
    /// Convert to spreadable attributes for Leptos views, cloning internally.
    #[must_use]
    pub fn to_attrs(&self) -> UseTableCellAttrs {
        self.clone().into_attrs()
    }

    /// Convert to spreadable attributes for Leptos views, consuming self.
    #[must_use]
    pub fn into_attrs(self) -> UseTableCellAttrs {
        (
            Attr(attr::Role, self.role),
            Attr(attr::AriaColindex, self.aria_colindex),
            Attr(attr::Tabindex, self.tabindex),
            Attr(attr::AriaDisabled, self.aria_disabled),
            self.on_keydown.into_on(ev::keydown),
            self.on_focus.into_on(ev::focus),
            self.on_blur.into_on(ev::blur),
            self.on_focusin.into_on(ev::focusin),
            self.on_focusout.into_on(ev::focusout),
            attr::custom::custom_attribute("data-focus-visible", self.data_focus_visible),
        )
    }
}

/// Attributes for the table cell element.
pub type UseTableCellAttrs = (
    Attr<attr::Role, &'static str>,
    Attr<attr::AriaColindex, String>,
    Attr<attr::Tabindex, Signal<&'static str>>,
    Attr<attr::AriaDisabled, Signal<Option<AriaDisabled>>>,
    On<ev::keydown, SharedEventCallback<KeyboardEvent>>,
    On<ev::focus, SharedEventCallback<FocusEvent>>,
    On<ev::blur, SharedEventCallback<FocusEvent>>,
    On<ev::focusin, SharedEventCallback<FocusEvent>>,
    On<ev::focusout, SharedEventCallback<FocusEvent>>,
    attr::custom::CustomAttr<&'static str, Signal<Option<&'static str>>>,
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
///     <td {..cell.cell_props.into_attrs()}>
///         {data}
///     </td>
/// }
/// ```
#[allow(clippy::needless_pass_by_value)]
pub fn use_table_cell(input: UseTableCellInput) -> UseTableCellReturn {
    let UseTableCellInput {
        column_index,
        is_focused,
        is_disabled: disabled,
        is_interactive,
        on_focus,
        on_focus_next,
        on_focus_previous,
    } = input;

    // Compute tabindex
    let tabindex = Signal::derive(move || if is_focused.get() { "0" } else { "-1" });

    // Compute aria-disabled
    let aria_disabled = Signal::derive(move || disabled.get().then_some(AriaDisabled::True));

    // Handle keyboard navigation
    let handle_keydown = move |e: KeyboardEvent| {
        if disabled.get_untracked() {
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
        disabled,
        within: false,
        auto_focus: false,
        on_focus: on_focus.map(|cb| Callback::new(move |_| cb.run(()))),
        on_blur: None,
        on_focus_change: None,
    });

    // Column index is 1-based for ARIA
    let aria_colindex = (column_index + 1).to_string();

    UseTableCellReturn {
        cell_props: UseTableCellProps {
            role: "gridcell",
            aria_colindex,
            tabindex,
            aria_disabled,
            on_keydown: EventHandler::new(handle_keydown),
            on_focus: focus_ring_props.on_focus,
            on_blur: focus_ring_props.on_blur,
            on_focusin: focus_ring_props.on_focusin,
            on_focusout: focus_ring_props.on_focusout,
            data_focus_visible: focus_ring_props.data_focus_visible,
        },
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
    /// Props for programmatic merging. Call `.to_attrs()` or `.into_attrs()` for view spreading.
    pub cell_props: UseTableCheckboxCellProps,

    /// Props for programmatic merging. Call `.to_attrs()` or `.into_attrs()` for view spreading.
    pub checkbox_props: UseTableCheckboxProps,
}

/// Props from `use_table_checkbox_cell` for the cell element.
#[derive(Debug, Clone)]
pub struct UseTableCheckboxCellProps {
    pub role: &'static str,
}

impl UseTableCheckboxCellProps {
    /// Convert to spreadable attributes for Leptos views, cloning internally.
    #[must_use]
    pub fn to_attrs(&self) -> UseTableCheckboxCellAttrs {
        self.clone().into_attrs()
    }

    /// Convert to spreadable attributes for Leptos views, consuming self.
    #[must_use]
    pub fn into_attrs(self) -> UseTableCheckboxCellAttrs {
        (Attr(attr::Role, self.role),)
    }
}

/// Props from `use_table_checkbox_cell` for the checkbox input element.
#[derive(Debug, Clone)]
pub struct UseTableCheckboxProps {
    pub r#type: &'static str,
    pub checked: Signal<bool>,
    pub aria_label: &'static str,
    pub aria_disabled: Signal<Option<AriaDisabled>>,
    pub on_change: EventHandler<web_sys::Event>,
}

impl UseTableCheckboxProps {
    /// Convert to spreadable attributes for Leptos views, cloning internally.
    #[must_use]
    pub fn to_attrs(&self) -> UseTableCheckboxAttrs {
        self.clone().into_attrs()
    }

    /// Convert to spreadable attributes for Leptos views, consuming self.
    #[must_use]
    pub fn into_attrs(self) -> UseTableCheckboxAttrs {
        (
            Attr(attr::Type, self.r#type),
            Attr(attr::Checked, self.checked),
            Attr(attr::AriaLabel, self.aria_label),
            Attr(attr::AriaDisabled, self.aria_disabled),
            self.on_change.into_on(ev::change),
        )
    }
}

/// Attributes for the checkbox cell.
pub type UseTableCheckboxCellAttrs = (Attr<attr::Role, &'static str>,);

/// Attributes for the checkbox input.
pub type UseTableCheckboxAttrs = (
    Attr<attr::Type, &'static str>,
    Attr<attr::Checked, Signal<bool>>,
    Attr<attr::AriaLabel, &'static str>,
    Attr<attr::AriaDisabled, Signal<Option<AriaDisabled>>>,
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
///     <td {..checkbox.cell_props.into_attrs()}>
///         <input {..checkbox.checkbox_props.into_attrs()} />
///     </td>
/// }
/// ```
pub fn use_table_checkbox_cell(input: UseTableCheckboxCellInput) -> UseTableCheckboxCellReturn {
    let UseTableCheckboxCellInput {
        is_selected,
        is_indeterminate,
        is_disabled: disabled,
        on_change,
    } = input;

    let aria_disabled = Signal::derive(move || disabled.get().then_some(AriaDisabled::True));

    let handle_change = move |_e: web_sys::Event| {
        if disabled.get_untracked() {
            return;
        }
        if let Some(on_change) = on_change {
            on_change.run(!is_selected.get_untracked());
        }
    };

    UseTableCheckboxCellReturn {
        cell_props: UseTableCheckboxCellProps { role: "gridcell" },
        checkbox_props: UseTableCheckboxProps {
            r#type: "checkbox",
            checked: is_selected,
            aria_label: "Select row",
            aria_disabled,
            on_change: EventHandler::new(handle_change),
        },
    }
}
