// Upstream: react-aria/src/table/useTableRow.ts @ 99e6102368
use leptos::{
    attr::{self, Attr},
    prelude::*,
};

use super::TableData;
use crate::hooks::{
    IntoAttrs, PropsWithStyles, UseGridRowAttrs, UseGridRowInput, UseGridRowProps,
    UseGridRowReturn, collections::Key, use_grid_row,
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## OMITTED FEATURES
// - Tree tables (expansion keys, `aria-expanded`/`aria-level`/..., the expand button): not built
//   yet (planned; trees are grid lists meanwhile, `hooks::tree`).
// - Virtualization (`aria-rowindex`), synthetic link props.
//
// =============================================================================

/// Input of [`use_table_row`].
#[derive(Debug, Clone)]
pub struct UseTableRowInput {
    /// The table (from `use_table`).
    pub table: TableData,
    /// The row's key.
    pub key: Key,
    /// Called when a context menu is requested on the row (right click, Shift+F10, the context
    /// menu key; a long press on iOS unless it selects).
    pub on_context_menu: Option<Callback<crate::hooks::ContextMenuEvent>>,
}

/// Return value of [`use_table_row`].
pub struct UseTableRowReturn {
    pub row_props: PropsWithStyles<UseTableRowProps>,
    pub is_selected: Signal<bool>,
    pub is_focused: Signal<bool>,
    pub is_disabled: Signal<bool>,
    pub is_pressed: Signal<bool>,
    pub allows_selection: Signal<bool>,
    pub has_action: Signal<bool>,
}

/// Props for the row element.
#[derive(Debug)]
pub struct UseTableRowProps {
    pub row: UseGridRowProps,
    /// The row header cells label the row.
    pub aria_labelledby: Signal<String>,
}

pub type UseTableRowAttrs = (UseGridRowAttrs, Attr<attr::AriaLabelledby, Signal<String>>);

impl IntoAttrs for UseTableRowProps {
    type Attrs = UseTableRowAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            self.row.into_attrs(),
            Attr(attr::AriaLabelledby, self.aria_labelledby),
        )
    }
}

/// A body row of a table, labelled by its row header cells.
pub fn use_table_row(input: UseTableRowInput) -> UseTableRowReturn {
    let UseTableRowInput {
        table,
        key,
        on_context_menu,
    } = input;
    let aria_labelledby = {
        let table = table.clone();
        let key = key.clone();
        Signal::derive(move || table.row_labelledby(&key))
    };
    let UseGridRowReturn {
        row_props,
        is_selected,
        is_focused,
        is_disabled,
        is_pressed,
        allows_selection,
        has_action,
    } = use_grid_row(UseGridRowInput {
        grid: table.grid,
        key,
        on_context_menu,
    });
    let (row, styles) = row_props.into_inner();
    UseTableRowReturn {
        row_props: PropsWithStyles::new(
            UseTableRowProps {
                row,
                aria_labelledby,
            },
            styles,
        ),
        is_selected,
        is_focused,
        is_disabled,
        is_pressed,
        allows_selection,
        has_action,
    }
}
