//! The state of a table (`use_table_state`) alone, for its performance test
//! (`leptonic/tests/perf_tests/hooks/use_table_state.rs`): [`SMALL`] rows "Row <i>" in the sortable
//! columns Name, Value and Group, with multiple selection, or (`mount-tree`) a tree table of
//! [`SMALL`] / 2 parent rows ("Parent <p>", key `p<p>`) with a child row each (key `c<p>`), created
//! by a mounted component (mounted at first) that shows how many rows are visible (`#perf-rows`),
//! how many columns the table has (`#perf-columns`) and its sort (`#perf-sort`). Its controls
//! ([`Control`]) mount and unmount it, set its size while unmounted (`small`, `large`: [`LARGE`]
//! rows), add and remove a row, show and hide the selection checkboxes, change the selection mode,
//! sort by Value, expand and collapse every row of the tree table and read how many rows are
//! visible (`rows`).

use std::{collections::HashSet, sync::Arc};

use leptonic::{
    ValueBinding,
    hooks::{
        collections::{Key, SelectionMode, SelectionOptions},
        grid::GridFocusMode,
        table::{
            SortDirection, TableCollection, TableState, TableTreeInput, UseTableStateInput,
            use_table_state,
        },
    },
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

/// The rows of the small table.
const SMALL: usize = 10;
/// The rows of the large table.
const LARGE: usize = 1_000;

/// `rows` rows ("Row <i>", key `i`) in the sortable columns `name` (the row header), `value` and
/// `group`.
fn table_collection(rows: RwSignal<usize>) -> Memo<Arc<TableCollection>> {
    Memo::new(move |_| {
        Arc::new(TableCollection::build(|t| {
            for (key, text) in [("name", "Name"), ("value", "Value"), ("group", "Group")] {
                let column = t.column(key, text).allows_sorting();
                if key == "name" {
                    let _ = column.row_header();
                }
            }
            for i in 0..rows.get() {
                t.row(i, format!("Row {i}"), |r| {
                    r.cell(format!("Row {i}"));
                    r.cell(format!("Value {i}"));
                    r.cell(format!("Group {}", i % 3));
                });
            }
        }))
    })
}

/// A tree table of `rows / 2` parent rows with a child row each.
fn tree_collection(rows: RwSignal<usize>) -> Memo<Arc<TableCollection>> {
    Memo::new(move |_| {
        Arc::new(TableCollection::build(|t| {
            let _ = t.column("name", "Name").row_header();
            t.column("value", "Value");
            t.rows(|b| {
                for p in 0..rows.get() / 2 {
                    let _ = b
                        .row(format!("p{p}"), format!("Parent {p}"), |r| {
                            r.cell(format!("Parent {p}"));
                            r.cell(format!("Value {p}"));
                        })
                        .children(|c| {
                            c.row(format!("c{p}"), format!("Child {p}"), |r| {
                                r.cell(format!("Child {p}"));
                                r.cell(format!("Child value {p}"));
                            });
                        });
                }
            });
        }))
    })
}

/// The settings the state is created with.
#[derive(Clone, Copy)]
struct Settings {
    rows: RwSignal<usize>,
    checkboxes: RwSignal<bool>,
    selection_mode: RwSignal<SelectionMode>,
    tree: RwSignal<bool>,
    expanded: RwSignal<HashSet<Key>>,
}

/// Creates the state (into `state`, while mounted) and shows it.
#[component]
fn PerfState(settings: Settings, state: StoredValue<Option<TableState>>) -> impl IntoView {
    let tree = settings.tree.get_untracked();
    let table = use_table_state(UseTableStateInput {
        table: if tree {
            tree_collection(settings.rows)
        } else {
            table_collection(settings.rows)
        },
        show_selection_checkboxes: settings.checkboxes.into(),
        selection: SelectionOptions {
            selection_mode: settings.selection_mode.into(),
            ..SelectionOptions::default()
        },
        focus_mode: GridFocusMode::Row,
        default_sort_descriptor: None,
        sort_descriptor: None,
        on_sort_change: None,
        tree: tree.then(|| TableTreeInput {
            column: Key::from("name"),
            default_expanded_keys: HashSet::new(),
            expanded_keys: Some(ValueBinding::from(settings.expanded)),
            on_expanded_change: None,
        }),
    });
    state.set_value(Some(table));
    on_cleanup(move || state.set_value(None));
    let rows = table.grid.list.collection;
    let columns = table.columns;
    let sort = table.sort_descriptor;
    view! {
        <output id="perf-rows">{move || rows.with(|c| c.items().count())}</output>
        <output id="perf-columns">{move || columns.with(|c| c.len())}</output>
        <output id="perf-sort">
            {move || {
                sort.with(|sort| {
                    sort.as_ref()
                        .map(|sort| match sort.direction {
                            SortDirection::Ascending => "ascending",
                            SortDirection::Descending => "descending",
                        })
                })
            }}
        </output>
    }
}

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let settings = Settings {
        rows: RwSignal::new(SMALL),
        checkboxes: RwSignal::new(false),
        selection_mode: RwSignal::new(SelectionMode::Multiple),
        tree: RwSignal::new(false),
        expanded: RwSignal::new(HashSet::new()),
    };
    let state = StoredValue::new(None::<TableState>);
    let rows = settings.rows;
    let value = Key::from("value");
    let controls = vec![
        Control::action("mount", move || {
            settings.tree.set(false);
            mounted.set(true);
        }),
        Control::action("mount-tree", move || {
            settings.tree.set(true);
            mounted.set(true);
        }),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("small", move || rows.set(SMALL)),
        Control::action("large", move || rows.set(LARGE)),
        Control::action("add-row", move || rows.update(|rows| *rows += 1)),
        Control::action("remove-row", move || rows.update(|rows| *rows -= 1)),
        Control::action("show-checkboxes", move || settings.checkboxes.set(true)),
        Control::action("hide-checkboxes", move || settings.checkboxes.set(false)),
        Control::action("selection-none", move || {
            settings.selection_mode.set(SelectionMode::None);
        }),
        Control::action("selection-multiple", move || {
            settings.selection_mode.set(SelectionMode::Multiple);
        }),
        Control::action("sort", move || {
            if let Some(table) = state.get_value() {
                table.sort(&value, None);
            }
        }),
        Control::action("expand-all", move || {
            settings.expanded.set(
                (0..rows.get_untracked() / 2)
                    .map(|p| Key::from(format!("p{p}")))
                    .collect(),
            );
        }),
        Control::action("collapse-all", move || {
            settings.expanded.set(HashSet::new());
        }),
        Control::read("rows", move || {
            state.get_value().map_or(0.0, |table| {
                let rows = table
                    .grid
                    .list
                    .collection
                    .with_untracked(|c| c.items().count());
                f64::from(u32::try_from(rows).unwrap_or(u32::MAX))
            })
        }),
    ];
    view! {
        <h1>"use_table_state performance"</h1>
        <PerfControls controls />
        <Show when=move || mounted.get()>
            <PerfState settings state />
        </Show>
    }
}
