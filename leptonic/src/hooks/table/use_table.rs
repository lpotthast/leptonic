// Upstream: react-aria/src/table/useTable.ts @ 99e6102368
// Upstream: react-aria/src/table/utils.ts @ 99e6102368
// Upstream: react-aria/test/table/useTable.test.tsx @ 99e6102368
// Upstream: react-aria-components/test/Table.test.js @ 99e6102368
// Upstream: @adobe/react-spectrum/test/table/TableTests.js @ 99e6102368
use std::{sync::Arc, time::Duration};

use leptos::prelude::*;

use super::{SortDirection, TableKeyboardDelegate, TableState};
use crate::{
    CapturedElement, IdRefs,
    hooks::{
        collections::{
            CollectionOptions, DomLayoutDelegate, Key, KeyboardDelegate, keyboard_delegate_memo,
        },
        grid::{
            GridData, GridKeyboardDelegate, UseGridInput, UseGridProps, UseGridReturn, use_grid,
        },
        gridlist::KeyboardNavigationBehavior,
    },
    utils::{
        aria::AriaRole,
        filter::{CollatorOptions, use_collator},
        i18n::use_direction,
        id::use_id,
        intl_strings::{TableStrings, use_localized_strings},
        live_announcer::{Assertiveness, announce_with_timeout},
        use_description::use_description,
    },
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - Rows, cells and column headers get the table through the returned `TableData` (react-aria:
//   `WeakMap`s keyed by the state).
//
// ## OMITTED FEATURES
// - Virtualization (`aria-rowcount`).
//
// =============================================================================

/// Input of [`use_table`].
#[derive(Clone)]
pub struct UseTableInput {
    pub state: TableState,
    /// The table element; the hook's props capture it.
    pub element: CapturedElement,
    /// The element id. Generated when `None`.
    pub id: Option<String>,
    pub aria_label: MaybeProp<String>,
    /// Ids of elements labelling the table.
    pub aria_labelledby: Signal<Option<String>>,
    /// Replaces the table keyboard delegate.
    pub keyboard_delegate: Option<Signal<Arc<dyn KeyboardDelegate>>>,
    /// Keyboard and focus behavior.
    pub options: CollectionOptions,
    pub keyboard_navigation_behavior: KeyboardNavigationBehavior,
    /// Select when the press ends instead of when it starts.
    pub should_select_on_press_up: bool,
    /// Called with the key of an activated row.
    pub on_row_action: Option<Callback<Key>>,
    /// Called with the key of an activated cell.
    pub on_cell_action: Option<Callback<Key>>,
}

/// What rows, cells and column headers need to know about their table. Pass it to
/// `use_table_row`, `use_table_cell`, `use_table_column_header`, ...
#[derive(Debug, Clone)]
pub struct TableData {
    pub state: TableState,
    pub grid: GridData,
    /// The table element's id; cell and column header ids derive from it.
    pub id: String,
}

impl TableData {
    /// The id of the column header of `column`.
    pub fn column_header_id(&self, column: &Key) -> String {
        format!("{}-{}", self.id, column.id_fragment())
    }

    /// The id of the cell of `row` in `column`.
    pub fn cell_id(&self, row: &Key, column: &Key) -> String {
        // `%_` occurs in no id fragment: it separates them unambiguously.
        format!(
            "{}-{}%_{}",
            self.id,
            row.id_fragment(),
            column.id_fragment()
        )
    }

    /// The ids of the row header cells of `row`: the row's label. Tracks the table (the row
    /// header columns can change).
    pub fn row_labelledby(&self, row: &Key) -> String {
        self.state.table.with(|t| {
            t.row_header_columns()
                .iter()
                .map(|column| self.cell_id(row, column))
                .collect::<Vec<_>>()
                .join(" ")
        })
    }
}

/// Return value of [`use_table`].
#[derive(Debug)]
pub struct UseTableReturn {
    /// Props for the table element (a grid).
    pub props: UseGridProps,
    pub data: TableData,
}

/// Input of [`use_table_keyboard_delegate`].
#[derive(Debug, Clone, Copy)]
pub struct UseTableKeyboardDelegateInput {
    pub state: TableState,
    /// The table element, in which the delegate measures the rendered rows.
    pub element: CapturedElement,
}

/// The keyboard delegate of a table: a [`TableKeyboardDelegate`] measuring the rendered rows
/// in `element`, with the current locale's reading direction and collation.
pub fn use_table_keyboard_delegate(
    input: UseTableKeyboardDelegateInput,
) -> Signal<Arc<dyn KeyboardDelegate>> {
    let UseTableKeyboardDelegateInput { state, element } = input;
    // One collator per locale, not per read of the delegate.
    let collator = use_collator(CollatorOptions::default());
    let direction = use_direction();
    let grid = state.grid;
    let layout_delegate = Arc::new(DomLayoutDelegate::new(element, grid.list.item_elements));
    keyboard_delegate_memo(move || {
        let collator = collator.get();
        let direction = direction.get();
        let grid_delegate = GridKeyboardDelegate::new(
            grid.list.collection,
            grid.list.selection,
            layout_delegate.clone(),
        )
        .with_direction(direction)
        .with_collator(collator.clone())
        .with_focus_mode(grid.focus_mode);
        Arc::new(TableKeyboardDelegate::new(
            grid_delegate,
            state.table,
            direction,
            Some(collator),
        )) as Arc<dyn KeyboardDelegate>
    })
}

/// A table: a grid with column headers (in one or more header rows) above its body rows,
/// navigated in two dimensions, with sortable columns and selectable rows. Render column headers
/// with `use_table_column_header`, rows with `use_table_row` and cells with `use_table_cell`.
pub fn use_table(input: UseTableInput) -> UseTableReturn {
    let UseTableInput {
        state,
        element,
        id,
        aria_label,
        aria_labelledby,
        keyboard_delegate,
        options,
        keyboard_navigation_behavior,
        should_select_on_press_up,
        on_row_action,
        on_cell_action,
    } = input;
    let id = id.unwrap_or_else(|| use_id("table"));
    let delegate = keyboard_delegate.unwrap_or_else(|| {
        use_table_keyboard_delegate(UseTableKeyboardDelegateInput { state, element })
    });

    let UseGridReturn { props, data } = use_grid(UseGridInput {
        state: state.grid,
        element,
        id: Some(id.clone()),
        aria_label,
        aria_labelledby,
        keyboard_delegate: Some(delegate),
        options,
        keyboard_navigation_behavior,
        should_select_on_press_up,
        on_row_action,
        on_cell_action,
    });

    // The sort, described to the table and announced when it changes (not initially: focusing
    // the table describes it).
    let strings = use_localized_strings::<TableStrings>();
    let sort_description = Memo::new(move |_| {
        state.sort_descriptor.get().map(|sort| {
            let column = state.columns.with(|columns| {
                columns
                    .iter()
                    .find(|column| column.key == sort.column)
                    .map(|column| column.text_value.to_string())
                    .unwrap_or_default()
            });
            let strings = strings.read();
            match sort.direction {
                SortDirection::Ascending => strings.ascending_sort(&column),
                SortDirection::Descending => strings.descending_sort(&column),
            }
        })
    });
    Effect::new(move |previous: Option<Option<String>>| {
        let description = sort_description.get();
        if previous.is_some()
            && let Some(description) = &description
        {
            announce_with_timeout(
                description.clone(),
                Assertiveness::Assertive,
                Duration::from_millis(500),
            );
        }
        description
    });
    // The sort description comes first, then the grid's own (react-aria merges both).
    let props = UseGridProps {
        // A tree table is a tree grid (react-aria: `role="treegrid"` with a tree column).
        role: Signal::stored(if state.tree.is_some() {
            AriaRole::Treegrid
        } else {
            AriaRole::Grid
        }),
        aria_describedby: IdRefs::derive([
            use_description(sort_description.into()),
            props.aria_describedby,
        ]),
        ..props
    };

    UseTableReturn {
        props,
        data: TableData {
            state,
            grid: data,
            id,
        },
    }
}
