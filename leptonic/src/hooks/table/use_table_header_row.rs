// Upstream: react-aria/src/table/useTableHeaderRow.ts @ 99e6102368
use leptos::{
    attr::{self, Attr},
    prelude::*,
};

use super::TableData;
use crate::{IntoAttrs, hooks::collections::Key, utils::aria::AriaRole};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## ADDITIONS
// - `use_table_header_placeholder`: props for the empty cells that fill header rows above columns
//   without a column group (React Spectrum renders these itself).
//
// ## OMITTED FEATURES
// - Virtualization (`aria-rowindex`).
//
// =============================================================================

/// Props for a header row element.
#[derive(Debug)]
pub struct UseTableHeaderRowProps {
    pub role: AriaRole,
}

pub type UseTableHeaderRowAttrs = (Attr<attr::Role, AriaRole>,);

impl IntoAttrs for UseTableHeaderRowProps {
    type Attrs = UseTableHeaderRowAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (Attr(attr::Role, self.role),)
    }
}

/// A row of column headers (see `TableCollection::header_rows`).
pub fn use_table_header_row() -> UseTableHeaderRowProps {
    UseTableHeaderRowProps {
        role: AriaRole::Row,
    }
}

/// Input of [`use_table_header_placeholder`].
#[derive(Debug, Clone)]
pub struct UseTableHeaderPlaceholderInput {
    /// The table (from `use_table`).
    pub table: TableData,
    /// The placeholder's key (a `NodeKind::Placeholder` node of a header row).
    pub key: Key,
}

/// Props for a header placeholder element.
#[derive(Debug)]
pub struct UseTableHeaderPlaceholderProps {
    pub role: AriaRole,
    pub aria_colindex: Signal<Option<usize>>,
    pub aria_colspan: Signal<Option<usize>>,
    /// For `<th>`/`<td>` placeholders.
    pub colspan: Signal<Option<usize>>,
}

pub type UseTableHeaderPlaceholderAttrs = (
    Attr<attr::Role, AriaRole>,
    Attr<attr::AriaColindex, Signal<Option<usize>>>,
    Attr<attr::AriaColspan, Signal<Option<usize>>>,
    Attr<attr::Colspan, Signal<Option<usize>>>,
);

impl IntoAttrs for UseTableHeaderPlaceholderProps {
    type Attrs = UseTableHeaderPlaceholderAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            Attr(attr::Role, self.role),
            Attr(attr::AriaColindex, self.aria_colindex),
            Attr(attr::AriaColspan, self.aria_colspan),
            Attr(attr::Colspan, self.colspan),
        )
    }
}

/// An empty cell of a header row (a `NodeKind::Placeholder` node), filling the space above
/// columns that aren't in a column group. Not focusable. Its position follows the table.
pub fn use_table_header_placeholder(
    input: UseTableHeaderPlaceholderInput,
) -> UseTableHeaderPlaceholderProps {
    let UseTableHeaderPlaceholderInput { table, key } = input;
    let table = table.state.table;
    let position = Memo::new(move |_| {
        table.with(|t| {
            t.collection()
                .get(&key)
                .map(|n| (n.col_index, n.col_span))
                .unwrap_or_default()
        })
    });
    UseTableHeaderPlaceholderProps {
        role: AriaRole::Gridcell,
        aria_colindex: Signal::derive(move || position.get().0.map(|i| i + 1)),
        aria_colspan: Signal::derive(move || position.get().1),
        colspan: Signal::derive(move || position.get().1),
    }
}
