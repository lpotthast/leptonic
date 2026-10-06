use std::sync::Arc;

use leptonic::{
    atoms::table::{Table, TableBody, TableCell, TableHeader, TableRow},
    hooks::{SelectionMode, TableCollection, collections::Selection},
};
use leptos::prelude::*;

/// Name and number of moons.
const PLANETS: [(&str, u32); 3] = [("Venus", 0), ("Earth", 1), ("Mars", 2)];

#[component]
pub fn TableConceptDemo() -> impl IntoView {
    // The columns and rows: a key and a text per column, a key per row.
    let table = Memo::new(|_| {
        Arc::new(TableCollection::build(|t| {
            t.column("name", "Planet").row_header();
            t.column("moons", "Moons");
            for (name, moons) in PLANETS {
                t.row(name, name, |r| {
                    r.cell(name);
                    r.cell(moons.to_string());
                });
            }
        }))
    });
    let selection = RwSignal::new(Selection::default());

    view! {
        <Table table selection_mode=SelectionMode::Single selection=selection set_selection=selection aria_label="Planets" classes=["demo-table", "demo-atom-table"]>
            <TableHeader/>
            <TableBody>
                {PLANETS
                    .map(|(name, moons)| view! {
                        <TableRow key=name classes="demo-atom-table-row">
                            <TableCell column="name">{name}</TableCell>
                            <TableCell column="moons">{moons}</TableCell>
                        </TableRow>
                    })
                    .collect_view()}
            </TableBody>
        </Table>
        <p class="demo-status">
            {move || selection.with(|selection| match selection {
                Selection::Keys(keys) => keys.iter().next().map_or_else(|| "No planet selected.".to_owned(), |key| format!("Selected: {key}.")),
                Selection::All => "All planets selected.".to_owned(),
            })}
        </p>
    }
}
