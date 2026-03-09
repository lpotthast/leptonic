use leptos::{
    attr,
    attr::Attr,
    ev,
    ev::{On, SharedEventCallback},
    prelude::*,
};

use crate::{
    hooks::IntoAttrs,
    utils::{
        aria::{AriaDisabled, AriaRole},
        EventHandler,
    },
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// No intentional deviations from the react-aria implementation.
//
// =============================================================================

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
    /// Props for programmatic merging. Call `.into_attrs()` for view spreading.
    pub cell_props: UseTableCheckboxCellProps,

    /// Props for programmatic merging. Call `.into_attrs()` for view spreading.
    pub checkbox_props: UseTableCheckboxProps,
}

/// Props from `use_table_checkbox_cell` for the cell element.
#[derive(Debug)]
pub struct UseTableCheckboxCellProps {
    pub role: AriaRole,
}

impl IntoAttrs for UseTableCheckboxCellProps {
    type Attrs = UseTableCheckboxCellAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (Attr(attr::Role, self.role),)
    }
}

/// Props from `use_table_checkbox_cell` for the checkbox input element.
#[derive(Debug)]
pub struct UseTableCheckboxProps {
    pub r#type: &'static str,
    pub checked: Signal<bool>,
    pub aria_label: &'static str,
    pub aria_disabled: Signal<Option<AriaDisabled>>,
    pub on_change: EventHandler<web_sys::Event>,
}

impl IntoAttrs for UseTableCheckboxProps {
    type Attrs = UseTableCheckboxAttrs;

    fn into_attrs(self) -> Self::Attrs {
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
pub type UseTableCheckboxCellAttrs = (Attr<attr::Role, AriaRole>,);

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
        cell_props: UseTableCheckboxCellProps {
            role: AriaRole::Gridcell,
        },
        checkbox_props: UseTableCheckboxProps {
            r#type: "checkbox",
            checked: is_selected,
            aria_label: "Select row",
            aria_disabled,
            on_change: EventHandler::new(handle_change),
        },
    }
}
