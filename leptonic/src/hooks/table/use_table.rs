use leptos::attr;
use leptos::attr::Attr;
use leptos::ev;
use leptos::ev::{On, SharedEventCallback};
use leptos::prelude::*;
use uuid::Uuid;
use web_sys::KeyboardEvent;

use crate::utils::aria::{AriaDisabled, AriaMultiselectable};
use crate::utils::EventHandler;

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/@react-aria/table/src/useTable.ts

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// No intentional deviations from the react-aria implementation.
//
// =============================================================================

/// The selection mode for table rows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TableSelectionMode {
    /// No selection allowed.
    #[default]
    None,
    /// Single row selection.
    Single,
    /// Multiple row selection.
    Multiple,
}

/// The sort direction for a column.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SortDirection {
    /// Ascending order.
    Ascending,
    /// Descending order.
    Descending,
}

impl SortDirection {
    /// Toggles the sort direction.
    #[must_use]
    pub fn toggle(&self) -> Self {
        match self {
            Self::Ascending => Self::Descending,
            Self::Descending => Self::Ascending,
        }
    }
}

/// Input parameters for the `use_table` hook.
#[derive(Debug, Clone)]
pub struct UseTableInput {
    /// The label for the table.
    pub label: Option<String>,

    /// The selection mode.
    pub selection_mode: TableSelectionMode,

    /// Whether the table is disabled.
    pub is_disabled: Signal<bool>,

    /// The currently selected row keys.
    pub selected_keys: Signal<Vec<String>>,

    /// The currently sorted column key.
    pub sorted_column: Signal<Option<String>>,

    /// The current sort direction.
    pub sort_direction: Signal<Option<SortDirection>>,

    /// Callback when selection changes.
    pub on_selection_change: Option<Callback<Vec<String>>>,

    /// Callback when sort changes.
    pub on_sort_change: Option<Callback<(String, SortDirection)>>,

    /// Callback when a row is activated (Enter/double-click).
    pub on_row_action: Option<Callback<String>>,
}

impl Default for UseTableInput {
    fn default() -> Self {
        Self {
            label: None,
            selection_mode: TableSelectionMode::None,
            is_disabled: Signal::derive(|| false),
            selected_keys: Signal::derive(Vec::new),
            sorted_column: Signal::derive(|| None),
            sort_direction: Signal::derive(|| None),
            on_selection_change: None,
            on_sort_change: None,
            on_row_action: None,
        }
    }
}

/// The return value of the `use_table` hook.
pub struct UseTableReturn {
    /// Props for programmatic merging. Call `.to_attrs()` or `.into_attrs()` for view spreading.
    pub table_props: UseTableProps,

    /// The ID of the table.
    pub table_id: String,

    /// The selection mode.
    pub selection_mode: TableSelectionMode,

    /// Whether the table is disabled.
    pub is_disabled: Signal<bool>,

    /// The currently focused row key.
    pub focused_key: Signal<Option<String>>,

    /// Set the focused row key.
    pub set_focused_key: Callback<Option<String>>,

    /// Select a row.
    pub select_row: Callback<String>,

    /// Toggle row selection.
    pub toggle_row: Callback<String>,

    /// Select all rows.
    pub select_all: Callback<Vec<String>>,

    /// Clear selection.
    pub clear_selection: Callback<()>,
}

/// Props from `use_table` that can be extracted and merged programmatically.
#[derive(Debug, Clone)]
pub struct UseTableProps {
    pub id: String,
    pub role: &'static str,
    pub aria_label: Option<String>,
    pub aria_rowcount: Option<String>,
    pub aria_multiselectable: Option<AriaMultiselectable>,
    pub aria_disabled: Signal<Option<AriaDisabled>>,
    pub on_keydown: EventHandler<KeyboardEvent>,
}

impl UseTableProps {
    /// Convert to spreadable attributes for Leptos views, cloning internally.
    #[must_use]
    pub fn to_attrs(&self) -> UseTableAttrs {
        self.clone().into_attrs()
    }

    /// Convert to spreadable attributes for Leptos views, consuming self.
    #[must_use]
    pub fn into_attrs(self) -> UseTableAttrs {
        (
            Attr(attr::Id, self.id),
            Attr(attr::Role, self.role),
            Attr(attr::AriaLabel, self.aria_label),
            Attr(attr::AriaRowcount, self.aria_rowcount),
            Attr(attr::AriaMultiselectable, self.aria_multiselectable),
            Attr(attr::AriaDisabled, self.aria_disabled),
            self.on_keydown.into_on(ev::keydown),
        )
    }
}

/// Attributes for the table element.
pub type UseTableAttrs = (
    Attr<attr::Id, String>,
    Attr<attr::Role, &'static str>,
    Attr<attr::AriaLabel, Option<String>>,
    Attr<attr::AriaRowcount, Option<String>>,
    Attr<attr::AriaMultiselectable, Option<AriaMultiselectable>>,
    Attr<attr::AriaDisabled, Signal<Option<AriaDisabled>>>,
    On<ev::keydown, SharedEventCallback<KeyboardEvent>>,
);

/// Provides the behavior and accessibility for a table.
///
/// A table displays data in rows and columns with support for selection,
/// sorting, and keyboard navigation.
///
/// # Example
///
/// ```ignore
/// let table = use_table(UseTableInput {
///     label: Some("Users".to_string()),
///     selection_mode: TableSelectionMode::Multiple,
///     ..Default::default()
/// });
///
/// view! {
///     <table {..table.table_props.into_attrs()}>
///         <thead>
///             // Table header...
///         </thead>
///         <tbody>
///             // Table rows...
///         </tbody>
///     </table>
/// }
/// ```
#[allow(clippy::too_many_lines)]
pub fn use_table(input: UseTableInput) -> UseTableReturn {
    let UseTableInput {
        label,
        selection_mode,
        is_disabled: disabled,
        selected_keys,
        sorted_column,
        sort_direction,
        on_selection_change,
        on_sort_change,
        on_row_action,
    } = input;

    let table_id = format!("table-{}", Uuid::new_v4());

    // Track focused row
    let (focused_key, set_focused_key_signal) = signal::<Option<String>>(None);

    let set_focused_key = Callback::new(move |key: Option<String>| {
        set_focused_key_signal.set(key);
    });

    // Selection helpers
    let select_row = Callback::new(move |key: String| {
        if selection_mode == TableSelectionMode::None {
            return;
        }

        let new_selection = if selection_mode == TableSelectionMode::Single {
            vec![key]
        } else {
            let mut current = selected_keys.get_untracked();
            if !current.contains(&key) {
                current.push(key);
            }
            current
        };

        if let Some(on_change) = on_selection_change {
            on_change.run(new_selection);
        }
    });

    let toggle_row = Callback::new(move |key: String| {
        if selection_mode == TableSelectionMode::None {
            return;
        }

        let mut current = selected_keys.get_untracked();
        if let Some(pos) = current.iter().position(|k| k == &key) {
            current.remove(pos);
        } else if selection_mode == TableSelectionMode::Single {
            current = vec![key];
        } else {
            current.push(key);
        }

        if let Some(on_change) = on_selection_change {
            on_change.run(current);
        }
    });

    let select_all = Callback::new(move |all_keys: Vec<String>| {
        if selection_mode != TableSelectionMode::Multiple {
            return;
        }

        if let Some(on_change) = on_selection_change {
            on_change.run(all_keys);
        }
    });

    let clear_selection = Callback::new(move |_| {
        if let Some(on_change) = on_selection_change {
            on_change.run(vec![]);
        }
    });

    // Compute aria-multiselectable
    let aria_multiselectable = match selection_mode {
        TableSelectionMode::None => None,
        TableSelectionMode::Single => Some(AriaMultiselectable::False),
        TableSelectionMode::Multiple => Some(AriaMultiselectable::True),
    };

    // Compute aria-disabled
    let aria_disabled = Signal::derive(move || disabled.get().then_some(AriaDisabled::True));

    // Handle keyboard navigation
    let handle_keydown = move |e: KeyboardEvent| {
        if disabled.get_untracked() {
            return;
        }

        let key = e.key();
        match key.as_str() {
            "Enter" => {
                if let Some(focused) = focused_key.get_untracked() {
                    if let Some(on_action) = on_row_action {
                        e.prevent_default();
                        on_action.run(focused);
                    }
                }
            }
            "Escape" => {
                if selection_mode != TableSelectionMode::None {
                    e.prevent_default();
                    if let Some(on_change) = on_selection_change {
                        on_change.run(vec![]);
                    }
                }
            }
            " " => {
                if let Some(focused) = focused_key.get_untracked() {
                    if selection_mode != TableSelectionMode::None {
                        e.prevent_default();
                        let mut current = selected_keys.get_untracked();
                        if let Some(pos) = current.iter().position(|k| k == &focused) {
                            current.remove(pos);
                        } else if selection_mode == TableSelectionMode::Single {
                            current = vec![focused];
                        } else {
                            current.push(focused);
                        }
                        if let Some(on_change) = on_selection_change {
                            on_change.run(current);
                        }
                    }
                }
            }
            "a" | "A" if e.ctrl_key() || e.meta_key() => {
                // Select all - handled at component level with all keys
            }
            _ => {}
        }
    };

    UseTableReturn {
        table_props: UseTableProps {
            id: table_id.clone(),
            role: "grid",
            aria_label: label,
            aria_rowcount: None, // Set by component based on data
            aria_multiselectable,
            aria_disabled,
            on_keydown: EventHandler::new(handle_keydown),
        },
        table_id,
        selection_mode,
        is_disabled: disabled,
        focused_key: focused_key.into(),
        set_focused_key,
        select_row,
        toggle_row,
        select_all,
        clear_selection,
    }
}
