// Upstream: react-stately/src/table/useTableState.ts @ 99e6102368
use std::sync::Arc;

use leptos::prelude::*;

use super::TableCollection;
use crate::{
    hooks::{
        GridFocusMode, GridState, UseGridStateInput,
        collections::{CollectionMemo, DisabledBehavior, Key, SelectionOptions},
        use_grid_state,
    },
    utils::ValueBinding,
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - Hook-owned sort state (C4): `default_sort_descriptor` and `on_sort_change`, or
//   `sort_descriptor` bound to app state, instead of a controlled `sortDescriptor`. Sorting the
//   rows is up to the caller: rebuild the table collection from the sort descriptor.
// - The table collection is built by the caller (`TableCollection::build_with`, which also adds
//   the selection checkbox column) instead of from JSX children.
// - `UseTableStateInput::new` defaults `disabled_behavior` to `DisabledBehavior::Selection`, as
//   react-aria does for tables.
//
// ## OMITTED FEATURES
// - Tree tables (`treeColumn`, `expandedKeys`): trees are grid lists (`hooks::tree`).
// - `UNSTABLE_useFilteredTableState`.
//
// =============================================================================

/// The direction a table is sorted in.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SortDirection {
    Ascending,
    Descending,
}

impl SortDirection {
    /// The other direction.
    #[must_use]
    pub fn reversed(self) -> Self {
        match self {
            Self::Ascending => Self::Descending,
            Self::Descending => Self::Ascending,
        }
    }
}

/// The column a table is sorted by, and the direction.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SortDescriptor {
    pub column: Key,
    pub direction: SortDirection,
}

/// Input of [`use_table_state`].
#[derive(Debug, Clone)]
pub struct UseTableStateInput {
    /// The columns and rows.
    pub table: Memo<Arc<TableCollection>>,
    pub selection: SelectionOptions,
    pub focus_mode: GridFocusMode,
    /// The initial sorting. Ignored when `sort_descriptor` is bound.
    pub default_sort_descriptor: Option<SortDescriptor>,
    /// The sorting as app state, replacing `default_sort_descriptor`: the table shows it, and
    /// sorting by the user writes it. Setting it to `None` clears the sorting.
    pub sort_descriptor: Option<ValueBinding<Option<SortDescriptor>>>,
    /// Called when the user sorts the table (pressing a sortable column header).
    pub on_sort_change: Option<Callback<SortDescriptor>>,
}

impl UseTableStateInput {
    /// A table without selection, focusing rows, with disabled rows still focusable
    /// (`DisabledBehavior::Selection`).
    pub fn new(table: Memo<Arc<TableCollection>>) -> Self {
        Self {
            table,
            selection: SelectionOptions {
                disabled_behavior: DisabledBehavior::Selection,
                ..SelectionOptions::default()
            },
            focus_mode: GridFocusMode::Row,
            default_sort_descriptor: None,
            sort_descriptor: None,
            on_sort_change: None,
        }
    }
}

/// The state of a table: its grid state (rows, cells, selection, focus) and its sorting.
#[derive(Debug, Clone, Copy)]
pub struct TableState {
    pub grid: GridState,
    /// The columns and rows.
    pub table: Memo<Arc<TableCollection>>,
    /// The current sorting, if any.
    pub sort_descriptor: Signal<Option<SortDescriptor>>,
    binding: ValueBinding<Option<SortDescriptor>>,
    on_sort_change: Option<Callback<SortDescriptor>>,
}

impl TableState {
    /// Sort by `column`: in `direction`, or (when `None`) ascending, or reversed if the table is
    /// already sorted by this column.
    pub fn sort(&self, column: &Key, direction: Option<SortDirection>) {
        let current = self.sort_descriptor.get_untracked();
        let direction = direction.unwrap_or_else(|| match current {
            Some(current) if current.column == *column => current.direction.reversed(),
            _ => SortDirection::Ascending,
        });
        let descriptor = SortDescriptor {
            column: column.clone(),
            direction,
        };
        self.binding.set(Some(descriptor.clone()));
        if let Some(on_sort_change) = self.on_sort_change {
            on_sort_change.run(descriptor);
        }
    }
}

/// Creates the state of a table (see [`TableState`]).
pub fn use_table_state(input: UseTableStateInput) -> TableState {
    let UseTableStateInput {
        table,
        selection,
        focus_mode,
        default_sort_descriptor,
        sort_descriptor,
        on_sort_change,
    } = input;
    let collection: CollectionMemo = Memo::new(move |_| table.with(|t| t.collection().clone()));
    let grid = use_grid_state(UseGridStateInput {
        collection,
        selection,
        focus_mode,
    });
    let binding = sort_descriptor
        .unwrap_or_else(|| ValueBinding::from(RwSignal::new(default_sort_descriptor)));
    TableState {
        grid,
        table,
        sort_descriptor: binding.value,
        binding,
        on_sort_change,
    }
}

#[cfg(test)]
mod tests {
    use assertr::prelude::*;

    use super::*;

    #[test]
    fn sorting_a_column_again_reverses_the_direction() {
        Owner::new().with(|| {
            let table = Memo::new(|_| {
                Arc::new(TableCollection::build(|t| {
                    t.column("name", "Name").allows_sorting();
                    t.column("age", "Age").allows_sorting();
                }))
            });
            let changes = RwSignal::new(Vec::new());
            let state = use_table_state(UseTableStateInput {
                on_sort_change: Some(Callback::new(move |d: SortDescriptor| {
                    changes.update(|c| c.push(d));
                })),
                ..UseTableStateInput::new(table)
            });
            let name = Key::from("name");
            state.sort(&name, None);
            state.sort(&name, None);
            state.sort(&Key::from("age"), None);
            state.sort(&Key::from("age"), Some(SortDirection::Ascending));
            let directions: Vec<(String, SortDirection)> = changes
                .get_untracked()
                .into_iter()
                .map(|d| (d.column.to_string(), d.direction))
                .collect();
            assert_that!(directions).is_equal_to(vec![
                ("name".to_owned(), SortDirection::Ascending),
                ("name".to_owned(), SortDirection::Descending),
                ("age".to_owned(), SortDirection::Ascending),
                ("age".to_owned(), SortDirection::Ascending),
            ]);
            assert_that!(state.sort_descriptor.get_untracked().map(|d| d.direction))
                .is_equal_to(Some(SortDirection::Ascending));
        });
    }

    #[test]
    fn a_bound_sort_descriptor_is_shown_written_and_cleared() {
        Owner::new().with(|| {
            let table = Memo::new(|_| {
                Arc::new(TableCollection::build(|t| {
                    t.column("name", "Name").allows_sorting();
                }))
            });
            let sort = RwSignal::new(None);
            let state = use_table_state(UseTableStateInput {
                sort_descriptor: Some(sort.into()),
                ..UseTableStateInput::new(table)
            });
            let name = Key::from("name");
            state.sort(&name, None);
            assert_that!(sort.get_untracked()).is_equal_to(Some(SortDescriptor {
                column: name.clone(),
                direction: SortDirection::Ascending,
            }));
            // Clearing the app state clears the sorting: the next sort starts ascending again.
            sort.set(None);
            assert_that!(state.sort_descriptor.get_untracked()).is_none();
            state.sort(&name, None);
            assert_that!(state.sort_descriptor.get_untracked().map(|d| d.direction))
                .is_equal_to(Some(SortDirection::Ascending));
        });
    }
}
