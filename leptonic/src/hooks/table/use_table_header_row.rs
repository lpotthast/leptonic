// Upstream: react-aria/src/table/useTableHeaderRow.ts @ 99e6102368
use leptos::prelude::*;

use super::TableData;
use crate::{
    IntoAttrs,
    hooks::collections::Key,
    utils::{
        aria::AriaRole,
        part::{Part, Remount},
        signal_attr::{FixedAttr, SignalAttr, fixed_attr, signal_attr},
    },
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## ADDITIONS
// - `use_table_header_placeholder`: props for the empty cells that fill header rows above columns
//   without a column group (React Spectrum renders these itself).
//
// ## OMITTED FEATURES
// - Virtualization (`aria-rowindex`): not yet, the virtualizer doesn't render tables yet.
//
// =============================================================================

/// Props for a header row element.
#[derive(Debug)]
pub struct UseTableHeaderRowProps {
    pub role: AriaRole,
}

impl Remount for UseTableHeaderRowProps {
    fn remount(&self) -> Self {
        Self { role: self.role }
    }
}

pub type UseTableHeaderRowAttrs = (FixedAttr,);

impl IntoAttrs for UseTableHeaderRowProps {
    type Attrs = UseTableHeaderRowAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (fixed_attr("role", self.role),)
    }
}

/// A row of column headers (see `TableCollection::header_rows`).
pub fn use_table_header_row() -> Part<UseTableHeaderRowProps> {
    crate::metrics::subject!();
    Part::stored(UseTableHeaderRowProps {
        role: AriaRole::Row,
    })
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

impl Remount for UseTableHeaderPlaceholderProps {
    fn remount(&self) -> Self {
        Self {
            role: self.role,
            aria_colindex: self.aria_colindex,
            aria_colspan: self.aria_colspan,
            colspan: self.colspan,
        }
    }
}

pub type UseTableHeaderPlaceholderAttrs = (
    FixedAttr,
    SignalAttr<Option<usize>>,
    SignalAttr<Option<usize>>,
    SignalAttr<Option<usize>>,
);

impl IntoAttrs for UseTableHeaderPlaceholderProps {
    type Attrs = UseTableHeaderPlaceholderAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            fixed_attr("role", self.role),
            signal_attr("aria-colindex", self.aria_colindex),
            signal_attr("aria-colspan", self.aria_colspan),
            signal_attr("colspan", self.colspan),
        )
    }
}

/// An empty cell of a header row (a `NodeKind::Placeholder` node), filling the space above
/// columns that aren't in a column group. Not focusable. Its position follows the table.
pub fn use_table_header_placeholder(
    input: UseTableHeaderPlaceholderInput,
) -> Part<UseTableHeaderPlaceholderProps> {
    crate::metrics::subject!();
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
    Part::stored(UseTableHeaderPlaceholderProps {
        role: AriaRole::Gridcell,
        aria_colindex: Signal::derive(move || position.get().0.map(|i| i + 1)),
        aria_colspan: Signal::derive(move || position.get().1),
        colspan: Signal::derive(move || position.get().1),
    })
}
