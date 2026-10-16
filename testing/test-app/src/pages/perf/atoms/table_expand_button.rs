//! `TableExpandButton`s in a tree table (a `Table` with a tree column, `[role=treegrid]`), for
//! the performance test of `TableExpandButton`
//! (`leptonic/tests/perf_tests/atoms/table_expand_button.rs`): [`SMALL`] parent rows ("Parent
//! <p>", key `p<p>`), each with one child row ("Child <p>", key `c<p>`), all collapsed, in the
//! columns Name (the tree column, its cells with the expand button) and Value, with multiple
//! selection, between two plain buttons (`#perf-before`, `#perf-after`) to tab from and to,
//! mounted at first. Its controls ([`Control`]) mount and unmount the table, set its size while
//! unmounted (`small`, `large`: [`LARGE`] parents) and change the controlled expanded rows
//! (`expand-all`, `collapse-all`).

use std::{collections::HashSet, sync::Arc};

use leptonic::{
    atoms::table::{Table, TableBody, TableCell, TableExpandButton, TableHeader, TableRow},
    hooks::{
        collections::{Key, SelectionMode},
        table::TableCollection,
    },
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

/// The parent rows of the small table (10 rows).
pub const SMALL: usize = 5;
/// The parent rows of the large table (1,000 rows).
pub const LARGE: usize = 500;

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let parents = RwSignal::new(SMALL);
    let expanded = RwSignal::new(HashSet::<Key>::new());
    let table = Memo::new(move |_| {
        Arc::new(TableCollection::build(|t| {
            t.column("name", "Name").row_header();
            t.column("value", "Value");
            t.rows(|b| {
                for p in 0..parents.get() {
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
    });
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("small", move || parents.set(SMALL)),
        Control::action("large", move || parents.set(LARGE)),
        Control::action("expand-all", move || {
            expanded.set(
                (0..parents.get_untracked())
                    .map(|p| Key::from(format!("p{p}")))
                    .collect(),
            );
        }),
        Control::action("collapse-all", move || expanded.set(HashSet::new())),
    ];
    view! {
        <h1>"TableExpandButton performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <Show when=move || mounted.get()>
            <Table
                table=table
                tree_column="name"
                expanded_keys=expanded
                set_expanded_keys=expanded
                selection_mode=SelectionMode::Multiple
                aria_label="Perf tree table"
            >
                <TableHeader />
                <TableBody>
                    <For
                        each=move || {
                            (0..parents.get())
                                .flat_map(|p| [format!("p{p}"), format!("c{p}")])
                                .collect::<Vec<_>>()
                        }
                        key=Clone::clone
                        children=|key: String| {
                            let name = key.clone();
                            let value = format!("Value of {key}");
                            view! {
                                <TableRow key=key>
                                    <TableCell column="name">
                                        <TableExpandButton>"›"</TableExpandButton>
                                        {name.clone()}
                                    </TableCell>
                                    <TableCell column="value">{value.clone()}</TableCell>
                                </TableRow>
                            }
                        }
                    />
                </TableBody>
            </Table>
        </Show>
        <button id="perf-after">"After"</button>
    }
}
