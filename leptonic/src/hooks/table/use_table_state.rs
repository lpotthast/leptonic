// Upstream: react-stately/src/table/useTableState.ts @ 99e6102368
// Upstream: react-aria-components/test/Table.test.js @ 99e6102368
// Upstream: @adobe/react-spectrum/test/table/TreeGridTable.test.tsx @ 99e6102368
use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

use leptos::prelude::*;

use super::{Column, ColumnKind, TableCollection};
use crate::{
    ValueBinding,
    hooks::{
        collections::{CollectionMemo, Key, SelectionMode, SelectionOptions},
        grid::{GridFocusMode, GridState, UseGridStateInput, use_grid_state},
        gridlist::{TreeRowPosition, tree_row_positions},
        tree::{TreeExpansion, use_tree_state::use_tree_expansion},
    },
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - Hook-owned sort state (C4): `default_sort_descriptor` and `on_sort_change`, or
//   `sort_descriptor` bound to app state, instead of a controlled `sortDescriptor`. Sorting the
//   rows is up to the caller: rebuild the table collection from the sort descriptor.
// - The table collection is built by the caller (`TableCollection::build`) instead of from JSX
//   children; the state adds the selection checkbox column (`show_selection_checkboxes`, while
//   the selection mode isn't `None`, as react-stately).
// - react-aria defaults a table's `disabledBehavior` to `selection`; here the caller sets
//   `selection.disabled_behavior` (the table atoms use `DisabledBehavior::Selection`).
//
// ## OMITTED FEATURES
// - `expandedKeys: 'all'`.
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
    /// Add a first column of checkboxes selecting the rows (and all rows, in its header), while
    /// the selection mode isn't `None` (see `TableCollection::with_selection_column`).
    pub show_selection_checkboxes: Signal<bool>,
    pub selection: SelectionOptions,
    pub focus_mode: GridFocusMode,
    /// The initial sorting. Ignored when `sort_descriptor` is bound.
    pub default_sort_descriptor: Option<SortDescriptor>,
    /// The sorting as app state, replacing `default_sort_descriptor`: the table shows it, and
    /// sorting by the user writes it. Setting it to `None` clears the sorting.
    pub sort_descriptor: Option<ValueBinding<Option<SortDescriptor>>>,
    /// Called when the user sorts the table (pressing a sortable column header).
    pub on_sort_change: Option<Callback<SortDescriptor>>,
    /// Makes the table a tree table: rows with child rows (`ItemBuilder::children`) expand and
    /// collapse (react-aria-components' `treeColumn`, `expandedKeys`).
    pub tree: Option<TableTreeInput>,
}

/// The settings of a tree table.
#[derive(Debug, Clone)]
pub struct TableTreeInput {
    /// The column showing the hierarchy: its cells hold the expand buttons.
    pub column: Key,
    /// The initially expanded rows. Ignored when `expanded_keys` is bound.
    pub default_expanded_keys: HashSet<Key>,
    /// The expanded rows as app state, replacing `default_expanded_keys`.
    pub expanded_keys: Option<ValueBinding<HashSet<Key>>>,
    /// Called when rows are expanded or collapsed.
    pub on_expanded_change: Option<Callback<HashSet<Key>>>,
}

/// The tree of a tree table.
#[derive(Debug, Clone, Copy)]
pub struct TableTree {
    column: StoredValue<Key>,
    /// Which rows are expanded.
    pub expansion: TreeExpansion,
    /// Every row's position (of all rows, collapsed ones' child rows too), computed once per
    /// table change.
    positions: Memo<Arc<HashMap<Key, TreeRowPosition>>>,
}

impl TableTree {
    /// The column showing the hierarchy.
    pub fn column(&self) -> Key {
        self.column.get_value()
    }

    /// Whether `column` shows the hierarchy.
    pub fn is_tree_column(&self, column: &Key) -> bool {
        self.column.with_value(|tree_column| tree_column == column)
    }

    /// The position of the row `key`: its level, its position among its sibling rows and their
    /// number (tracked).
    pub fn position(&self, key: &Key) -> Option<TreeRowPosition> {
        self.positions.with(|positions| positions.get(key).copied())
    }
}

/// The state of a table: its grid state (rows, cells, selection, focus) and its sorting.
#[derive(Debug, Clone, Copy)]
pub struct TableState {
    pub grid: GridState,
    /// The columns and rows, with the selection checkbox column while it is shown. In a tree
    /// table also the child rows of collapsed rows (the grid's collection,
    /// `grid.list.collection`, holds the visible rows).
    pub table: Memo<Arc<TableCollection>>,
    /// The table's columns: the data columns in order (the selection checkbox column too), then
    /// the column groups. Changes only when the columns do, not with the rows: read it instead
    /// of `table` for what depends only on the columns.
    pub columns: Memo<Arc<[Column]>>,
    /// Set in a tree table.
    pub tree: Option<TableTree>,
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
        show_selection_checkboxes,
        selection,
        focus_mode,
        default_sort_descriptor,
        sort_descriptor,
        on_sort_change,
        tree,
    } = input;
    let tree = tree.map(|tree| {
        let (expansion, _) = use_tree_expansion(
            tree.default_expanded_keys,
            tree.expanded_keys,
            tree.on_expanded_change,
        );
        TableTree {
            column: StoredValue::new(tree.column),
            expansion,
            // Recomputed only when the table changes, and then always a change.
            positions: Memo::new_with_compare(
                move |_| table.with(|t| Arc::new(tree_row_positions(t.collection()))),
                |_, _| true,
            ),
        }
    });
    // The selection checkbox column, while rows can be selected (react-stately).
    let selection_mode = selection.selection_mode;
    let table = Memo::new_with_compare(
        move |_| {
            let shown =
                show_selection_checkboxes.get() && selection_mode.get() != SelectionMode::None;
            table.with(|t| {
                if shown {
                    Arc::new(t.with_selection_column())
                } else {
                    t.clone()
                }
            })
        },
        // The same table (the caller's, unchanged) compares by pointer; a rebuilt one is new.
        |previous, next| {
            previous.is_none_or(|previous| !next.is_some_and(|next| Arc::ptr_eq(previous, next)))
        },
    );
    // A tree table's visible rows: child rows of expanded rows only.
    let collection: CollectionMemo = Memo::new(move |_| match tree {
        Some(tree) => tree
            .expansion
            .expanded_keys
            .with(|expanded| table.with(|t| Arc::new(t.collection().with_expanded(expanded)))),
        None => table.with(|t| t.collection().clone()),
    });
    let grid = use_grid_state(UseGridStateInput {
        collection,
        selection,
        focus_mode,
    })
    .without_navigation_while_empty();
    let binding = sort_descriptor
        .unwrap_or_else(|| ValueBinding::from(RwSignal::new(default_sort_descriptor)));
    let columns = Memo::new(move |_| {
        table.with(|t| {
            let groups = t
                .header_rows()
                .iter()
                .flat_map(|row| t.collection().children(row))
                .filter_map(|node| t.column(&node.key))
                .filter(|column| column.kind == ColumnKind::Group);
            t.columns()
                .chain(groups)
                .cloned()
                .collect::<Arc<[Column]>>()
        })
    });
    TableState {
        grid,
        table,
        columns,
        tree,
        sort_descriptor: binding.value,
        binding,
        on_sort_change,
    }
}

#[cfg(test)]
mod tests {
    use assertr::prelude::*;

    use super::*;
    use crate::hooks::collections::DisabledBehavior;

    #[test]
    fn sorting_a_column_again_reverses_the_direction() {
        crate::testing::with_owner(|| {
            let table = Memo::new(|_| {
                Arc::new(TableCollection::build(|t| {
                    t.column("name", "Name").allows_sorting();
                    t.column("age", "Age").allows_sorting();
                }))
            });
            let changes = RwSignal::new(Vec::new());
            let state = use_table_state(UseTableStateInput {
                show_selection_checkboxes: Signal::stored(false),
                tree: None,
                on_sort_change: Some(Callback::new(move |d: SortDescriptor| {
                    changes.update(|c| c.push(d));
                })),
                table,
                selection: SelectionOptions {
                    disabled_behavior: DisabledBehavior::Selection,
                    ..SelectionOptions::default()
                },
                focus_mode: GridFocusMode::Row,
                default_sort_descriptor: None,
                sort_descriptor: None,
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
        crate::testing::with_owner(|| {
            let table = Memo::new(|_| {
                Arc::new(TableCollection::build(|t| {
                    t.column("name", "Name").allows_sorting();
                }))
            });
            let sort = RwSignal::new(None);
            let state = use_table_state(UseTableStateInput {
                show_selection_checkboxes: Signal::stored(false),
                tree: None,
                sort_descriptor: Some(sort.into()),
                table,
                selection: SelectionOptions {
                    disabled_behavior: DisabledBehavior::Selection,
                    ..SelectionOptions::default()
                },
                focus_mode: GridFocusMode::Row,
                default_sort_descriptor: None,
                on_sort_change: None,
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
