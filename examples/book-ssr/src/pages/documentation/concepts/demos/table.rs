use std::sync::Arc;

use leptonic::{
    atoms::table::{Table, TableBody, TableCell, TableHeader, TableRow},
    hooks::{SelectionMode, SortDescriptor, SortDirection, TableCollection, collections::Key},
};
use leptos::prelude::*;

/// Name, moons, mean distance from the sun (million km).
const PLANETS: [(&str, u32, u32); 4] = [
    ("Mercury", 0, 58),
    ("Venus", 0, 108),
    ("Earth", 1, 150),
    ("Mars", 2, 228),
];

#[component]
pub fn TableConceptDemo() -> impl IntoView {
    // The table only tracks which column it is sorted by. Sorting the rows is up to you.
    let sort = RwSignal::new(SortDescriptor {
        column: Key::from("distance"),
        direction: SortDirection::Ascending,
    });
    let planets = Memo::new(move |_| {
        let mut planets = PLANETS.to_vec();
        sort.with(|sort| {
            match sort.column.as_str() {
                Some("name") => planets.sort_by_key(|(name, _, _)| *name),
                Some("moons") => planets.sort_by_key(|(_, moons, _)| *moons),
                _ => planets.sort_by_key(|(_, _, distance)| *distance),
            }
            if sort.direction == SortDirection::Descending {
                planets.reverse();
            }
        });
        planets
    });
    let table = Memo::new(move |_| {
        Arc::new(TableCollection::build(|t| {
            t.column("name", "Planet").row_header().allows_sorting();
            t.column("moons", "Moons").allows_sorting();
            t.column("distance", "Distance (million km)")
                .allows_sorting();
            for (name, moons, distance) in planets.get() {
                t.row(name, name, |r| {
                    r.cell(name);
                    r.cell(moons.to_string());
                    r.cell(distance.to_string());
                });
            }
        }))
    });

    view! {
        <div class="demo-table-scroll">
            <Table
                table=table
                selection_mode=SelectionMode::Single
                default_sort_descriptor=sort.get_untracked()
                on_sort_change=Callback::new(move |descriptor| sort.set(descriptor))
                aria_label="Planets"
                classes=["demo-table", "demo-atom-table"]
            >
                <TableHeader/>
                <TableBody>
                    <For each=move || planets.get() key=|(name, _, _)| *name let:planet>
                        <TableRow key=planet.0 classes="demo-atom-table-row">
                            <TableCell column="name">{planet.0}</TableCell>
                            <TableCell column="moons">{planet.1}</TableCell>
                            <TableCell column="distance">{planet.2}</TableCell>
                        </TableRow>
                    </For>
                </TableBody>
            </Table>
        </div>
    }
}
