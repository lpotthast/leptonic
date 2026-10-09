// Upstream: react-aria/src/table/useTableColumnHeader.ts @ 99e6102368
// Upstream: react-aria-components/test/Table.test.js @ 99e6102368
// Upstream: @adobe/react-spectrum/test/table/TableTests.js @ 99e6102368
use leptos::{
    attr::{self, Attr},
    prelude::*,
};

use super::{ColumnKind, SortDirection, TableData};
use crate::{
    IdRefs, IntoAttrs, PropsWithStyles,
    hooks::{
        collections::{Key, SelectionMode},
        focus::use_focusable::{
            UseFocusableInput, UseFocusableItemAttrs, UseFocusableItemProps, use_focusable,
        },
        grid::{
            CellFocusMode, UseGridCellAttrs, UseGridCellInput, UseGridCellProps, UseGridCellReturn,
            use_grid_cell,
        },
        interactions::use_press::{UsePressAttrs, UsePressInput, UsePressProps, use_press},
    },
    utils::{
        aria::{AriaRole, AriaSort},
        intl_strings::{TableStrings, use_localized_strings},
        platform::device::is_android,
        use_description::use_description,
    },
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - Of `useFocusable`, the header only takes a `FocusableContext`'s handlers, element capture,
//   description and attributes (e.g. from a `TooltipTrigger`); the cell keeps its own tabindex
//   (react-aria merges `useFocusable`'s props first, so the grid cell's tabindex wins there too).
//
// ## DIFFERENT BEHAVIOR
// - Android gets `aria-sort` too (react-aria omits it there, as TalkBack doesn't support it):
//   the server can't know the platform, and the attribute must hydrate. The sort direction is
//   also in the description on Android, as upstream.
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
    /// What a `FocusableContext` (e.g. a `TooltipTrigger` around the header) adds.
    pub focusable: UseFocusableItemProps,
}

pub type UseTableColumnHeaderAttrs = (
    UseGridCellAttrs,
    UsePressAttrs,
    Attr<attr::AriaSort, Signal<Option<AriaSort>>>,
    UseFocusableItemAttrs,
);

impl IntoAttrs for UseTableColumnHeaderProps {
    type Attrs = UseTableColumnHeaderAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            self.cell.into_attrs(),
            self.press.into_attrs(),
            Attr(attr::AriaSort, self.aria_sort),
            self.focusable.into_attrs(),
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
    // The column follows the table (e.g. when columns change).
    let column = {
        let key = key.clone();
        Memo::new(move |_| {
            state.columns.with(|columns| {
                columns
                    .iter()
                    .find(|c| c.key == key)
                    .map_or((false, false), |c| {
                        (c.allows_sorting, c.kind == ColumnKind::SelectionCheckbox)
                    })
            })
        })
    };
    let allows_sorting = Signal::derive(move || column.get().0);
    let is_selection_column = Signal::derive(move || column.get().1);

    let UseGridCellReturn {
        grid_cell_props,
        is_pressed: cell_pressed,
    } = use_grid_cell(UseGridCellInput {
        id: Some(table.column_header_id(&key)),
        focus_mode: Some(CellFocusMode::Child),
        allows_arrow_navigation,
        grid: table.grid.clone(),
        key: key.clone(),
        should_select_on_press_up: false,
    });
    let (mut cell, cell_styles) = grid_cell_props.into_inner();
    cell.role = Signal::stored(AriaRole::Columnheader);

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
            !allows_sorting.get()
                || (is_selection_column.get()
                    && selection.selection_mode() == SelectionMode::Single)
        }),
        on_press: Some(Callback::new(move |_| state.sort(&sort_key, None))),
        ..UsePressInput::default()
    });
    let (press_props, press_styles) = press.props.into_inner();

    let sorted_key = key;
    let aria_sort = Signal::derive(move || {
        allows_sorting.get().then(|| {
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
    // Picks up a `FocusableContext` (e.g. a tooltip on the header).
    let mut focusable: UseFocusableItemProps =
        use_focusable(UseFocusableInput::default()).props.into();

    // Sortable columns are described as such (on Android also with their sort direction).
    let strings = use_localized_strings::<TableStrings>();
    let sort_description = use_description(Signal::derive(move || {
        if !allows_sorting.get() {
            return None;
        }
        let strings = strings.read();
        let sortable = strings.sortable();
        let direction = is_android()
            .then(|| match aria_sort.get() {
                Some(AriaSort::Ascending) => Some(strings.ascending()),
                Some(AriaSort::Descending) => Some(strings.descending()),
                _ => None,
            })
            .flatten();
        Some(match direction {
            Some(direction) => format!("{sortable}, {direction}"),
            None => sortable,
        })
    }));
    let context_describedby = focusable.aria_describedby;
    focusable.aria_describedby = IdRefs::derive([context_describedby, sort_description]);

    UseTableColumnHeaderReturn {
        column_header_props: PropsWithStyles::new(
            UseTableColumnHeaderProps {
                cell,
                press: press_props,
                aria_sort,
                focusable,
            },
            cell_styles.merge(press_styles),
        ),
        is_pressed: Signal::derive(move || cell_pressed.get() || is_pressed.get()),
    }
}
