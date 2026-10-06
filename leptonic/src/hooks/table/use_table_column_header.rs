// Upstream: react-aria/src/table/useTableColumnHeader.ts @ 99e6102368
use leptos::{
    attr::{self, Attr},
    prelude::*,
};

use super::{ColumnKind, SortDirection, TableData};
use crate::{
    hooks::{
        CellFocusMode, IntoAttrs, PropsWithStyles, UseGridCellAttrs, UseGridCellInput,
        UseGridCellProps, UseGridCellReturn,
        collections::{Key, SelectionMode},
        interactions::use_press::{UsePressAttrs, UsePressInput, UsePressProps, use_press},
        use_grid_cell,
    },
    utils::aria::{AriaRole, AriaSort},
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## OMITTED FEATURES
// - The "sortable" description (`aria-describedby`): it needs localized messages.
// - Android's sort direction description (Android gets `aria-sort` as well).
//
// =============================================================================

/// Input of [`use_table_column_header`].
#[derive(Debug, Clone)]
pub struct UseTableColumnHeaderInput {
    /// The table (from `use_table`).
    pub table: TableData,
    /// The column's key.
    pub key: Key,
    /// Let ArrowLeft/ArrowRight move between the header's children even with
    /// `KeyboardNavigationBehavior::Tab`.
    pub allows_arrow_navigation: bool,
}

impl UseTableColumnHeaderInput {
    pub fn new(table: TableData, key: Key) -> Self {
        Self {
            table,
            key,
            allows_arrow_navigation: false,
        }
    }
}

/// Return value of [`use_table_column_header`].
pub struct UseTableColumnHeaderReturn {
    pub column_header_props: PropsWithStyles<UseTableColumnHeaderProps>,
    pub is_pressed: Signal<bool>,
}

/// Props for the column header element.
#[derive(Debug)]
pub struct UseTableColumnHeaderProps {
    /// Focus and navigation (`use_grid_cell`), with the `columnheader` role.
    pub cell: UseGridCellProps,
    /// Sorting by pressing the header.
    pub press: UsePressProps,
    pub aria_sort: Signal<Option<AriaSort>>,
}

pub type UseTableColumnHeaderAttrs = (
    UseGridCellAttrs,
    UsePressAttrs,
    Attr<attr::AriaSort, Signal<Option<AriaSort>>>,
);

impl IntoAttrs for UseTableColumnHeaderProps {
    type Attrs = UseTableColumnHeaderAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            self.cell.into_attrs(),
            self.press.into_attrs(),
            Attr(attr::AriaSort, self.aria_sort),
        )
    }
}

/// A column header of a table: focusable like a cell (its children get focus first), and
/// sorting the table when pressed if the column allows sorting.
pub fn use_table_column_header(input: UseTableColumnHeaderInput) -> UseTableColumnHeaderReturn {
    let UseTableColumnHeaderInput {
        table,
        key,
        allows_arrow_navigation,
    } = input;
    let state = table.state;
    let selection = state.grid.list.selection;
    let (allows_sorting, is_selection_column) = untrack(|| {
        state.table.with(|t| {
            t.column(&key).map_or((false, false), |c| {
                (c.allows_sorting, c.kind == ColumnKind::SelectionCheckbox)
            })
        })
    });

    let UseGridCellReturn {
        grid_cell_props,
        is_pressed: cell_pressed,
    } = use_grid_cell(UseGridCellInput {
        id: Some(table.column_header_id(&key)),
        focus_mode: Some(CellFocusMode::Child),
        allows_arrow_navigation,
        ..UseGridCellInput::new(table.grid.clone(), key.clone())
    });
    let (mut cell, cell_styles) = grid_cell_props.into_inner();
    cell.role = AriaRole::Columnheader;

    // Without rows, the headers aren't focusable (and lose focus).
    let is_empty = Signal::derive(move || state.table.with(|t| t.size() == 0));
    let cell_tabindex = cell.item.tabindex;
    cell.item.tabindex = Signal::derive(move || {
        if is_empty.get() {
            Some(-1)
        } else {
            cell_tabindex.get()
        }
    });
    let focus_key = key.clone();
    Effect::new(move || {
        if is_empty.get() && untrack(|| selection.is_focused_key(&focus_key)) {
            selection.set_focused_key(None, None);
        }
    });

    let sort_key = key.clone();
    let press = use_press(UsePressInput {
        is_disabled: Signal::derive(move || {
            !allows_sorting
                || (is_selection_column && selection.selection_mode() == SelectionMode::Single)
        }),
        on_press: Some(Callback::new(move |_| state.sort(&sort_key, None))),
        ..UsePressInput::default()
    });
    let (press_props, press_styles) = press.props.into_inner();

    let sorted_key = key;
    let aria_sort = Signal::derive(move || {
        allows_sorting.then(|| {
            state
                .sort_descriptor
                .with(|sort| match sort {
                    Some(sort) if sort.column == sorted_key => Some(sort.direction),
                    _ => None,
                })
                .map_or(AriaSort::None, |direction| match direction {
                    SortDirection::Ascending => AriaSort::Ascending,
                    SortDirection::Descending => AriaSort::Descending,
                })
        })
    });
    let is_pressed = press.is_pressed;

    UseTableColumnHeaderReturn {
        column_header_props: PropsWithStyles::new(
            UseTableColumnHeaderProps {
                cell,
                press: press_props,
                aria_sort,
            },
            cell_styles.merge(press_styles),
        ),
        is_pressed: Signal::derive(move || cell_pressed.get() || is_pressed.get()),
    }
}
