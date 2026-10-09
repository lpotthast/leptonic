use std::{collections::HashSet, sync::Arc};

use leptonic::{
    atoms::grid::{Grid, GridCell, GridRow, GridRowGroup},
    hooks::{
        collections::{Collection, CollectionMemo, Key, Selection, SelectionMode},
        grid::{CellFocusMode, GridFocusMode},
    },
};
use leptos::prelude::*;

use super::listbox::describe_selection;

/// Rows with a single cell holding switches, as in react-aria's grid story (`stories/grid`).
const SWITCH_ROWS: [&[&str]; 3] = [
    &["Switch 1", "Switch 2"],
    &["Switch 3", "Switch 4", "Switch 5"],
    &["Switch 6", "Switch 7"],
];

/// A grid of single-cell rows holding switches, in the given focus modes.
#[component]
fn SwitchGrid(
    label: &'static str,
    grid_focus_mode: GridFocusMode,
    cell_focus_mode: CellFocusMode,
) -> impl IntoView {
    let collection: CollectionMemo = Memo::new(|_| {
        Arc::new(Collection::build(|b| {
            for (i, _) in SWITCH_ROWS.iter().enumerate() {
                let text = format!("Item {}", i + 1);
                b.row(i64::try_from(i).expect("small"), text.clone(), |r| {
                    r.cell(text);
                });
            }
        }))
    });
    view! {
        <Grid
            collection=collection
            focus_mode=grid_focus_mode
            selection_mode=SelectionMode::Multiple
            aria_label=label
        >
            {SWITCH_ROWS
                .iter()
                .enumerate()
                .map(|(i, switches)| {
                    let row = Key::from(i64::try_from(i).expect("small"));
                    view! {
                        <GridRow key=row.clone()>
                            <GridCell key=Key::cell(&row, 0) focus_mode=cell_focus_mode>
                                {switches
                                    .iter()
                                    .map(|label| {
                                        view! { <input type="checkbox" role="switch" aria-label=*label /> }
                                    })
                                    .collect_view()}
                            </GridCell>
                        </GridRow>
                    }
                })
                .collect_view()}
        </Grid>
    }
}

/// A grid of fruits in cell focus mode. `with_action` sets `on_cell_action`.
#[component]
fn FruitGrid(label: &'static str, with_action: bool) -> impl IntoView {
    let fruits: CollectionMemo = Memo::new(|_| {
        Arc::new(Collection::build(|b| {
            for (name, color) in FRUITS {
                b.row(name, name, |r| {
                    r.cell(name);
                    r.cell(color);
                });
            }
        }))
    });
    let selection = RwSignal::new(String::new());
    let action = RwSignal::new(String::new());
    let id = label.to_lowercase();
    let on_selection_change = Callback::new(move |s: Selection| {
        selection.set(describe_selection(&s));
    });
    let rows = || {
        FRUITS
            .map(|(name, color)| {
                let row = Key::from(name);
                view! {
                    <GridRow key=row.clone()>
                        <GridCell key=Key::cell(&row, 0)>{name}</GridCell>
                        <GridCell key=Key::cell(&row, 1)>{color}</GridCell>
                    </GridRow>
                }
            })
            .collect_view()
    };
    let grid = if with_action {
        view! {
            <Grid
                collection=fruits
                focus_mode=GridFocusMode::Cell
                selection_mode=SelectionMode::Multiple
                aria_label=label
                on_selection_change=on_selection_change
                on_cell_action=Callback::new(move |key: Key| action.set(key.to_string()))
            >
                {rows()}
            </Grid>
        }
        .into_any()
    } else {
        view! {
            <Grid
                collection=fruits
                focus_mode=GridFocusMode::Cell
                selection_mode=SelectionMode::Multiple
                aria_label=label
                on_selection_change=on_selection_change
            >
                {rows()}
            </Grid>
        }
        .into_any()
    };
    view! {
        {grid}
        <div>
            "Selection: " <span id=format!("test-grid-{id}-selection")>{selection}</span>
            " Action: " <span id=format!("test-grid-{id}-action")>{action}</span>
        </div>
    }
}

const FRUITS: [(&str, &str); 2] = [("Apple", "Red"), ("Banana", "Yellow")];

const USERS: [(&str, &str, &str); 3] = [
    ("Alice", "30", "Admin"),
    ("Bob", "25", "User"),
    ("Dave", "40", "User"),
];

/// Grids in all focus mode combinations, and a selectable multi-column grid ("Users", Bob
/// disabled, Carol's second cell spanning two columns).
#[component]
pub fn PageAtomGrid() -> impl IntoView {
    let users: CollectionMemo = Memo::new(|_| {
        Arc::new(Collection::build(|b| {
            for (name, age, role) in &USERS[..2] {
                b.row(*name, *name, |r| {
                    r.cell(*name);
                    r.cell(*age);
                    r.cell(*role);
                });
            }
            b.row("Carol", "Carol", |r| {
                r.cell("Carol");
                r.cell("On leave").col_span(2);
            });
            let (name, age, role) = USERS[2];
            b.row(name, name, |r| {
                r.cell(name);
                r.cell(age);
                r.cell(role);
            });
        }))
    });
    let selection = RwSignal::new(String::new());
    let cell = |row: &str, column: usize, text: &'static str| {
        view! { <GridCell key=Key::cell(&Key::from(row), column)>{text}</GridCell> }
    };

    view! {
        <div id="test-page-atom-grid">
            <h1>"Grid"</h1>
            <button id="test-grid-before">"Before"</button>
            <SwitchGrid
                label="Row-Cell"
                grid_focus_mode=GridFocusMode::Row
                cell_focus_mode=CellFocusMode::Cell
            />
            <button id="test-grid-before-row-child">"Before"</button>
            <SwitchGrid
                label="Row-Child"
                grid_focus_mode=GridFocusMode::Row
                cell_focus_mode=CellFocusMode::Child
            />
            <button id="test-grid-before-cell-child">"Before"</button>
            <SwitchGrid
                label="Cell-Child"
                grid_focus_mode=GridFocusMode::Cell
                cell_focus_mode=CellFocusMode::Child
            />
            <button id="test-grid-before-cell-cell">"Before"</button>
            <SwitchGrid
                label="Cell-Cell"
                grid_focus_mode=GridFocusMode::Cell
                cell_focus_mode=CellFocusMode::Cell
            />
            <button id="test-grid-before-users">"Before"</button>
            <Grid
                collection=users
                selection_mode=SelectionMode::Multiple
                disabled_keys=Signal::stored(HashSet::from([Key::from("Bob")]))
                aria_label="Users"
                on_selection_change=Callback::new(move |s: Selection| {
                    selection.set(describe_selection(&s));
                })
            >
                <GridRowGroup>
                    {USERS[..2]
                        .iter()
                        .map(|(name, age, role)| {
                            view! {
                                <GridRow key=*name>
                                    {cell(name, 0, name)} {cell(name, 1, age)} {cell(name, 2, role)}
                                </GridRow>
                            }
                        })
                        .collect_view()}
                    <GridRow key="Carol">
                        {cell("Carol", 0, "Carol")} {cell("Carol", 1, "On leave")}
                    </GridRow>
                    <GridRow key="Dave">
                        {cell("Dave", 0, "Dave")} {cell("Dave", 1, "40")} {cell("Dave", 2, "User")}
                    </GridRow>
                </GridRowGroup>
            </Grid>
            <div>"Selection: " <span id="test-grid-selection">{selection}</span></div>
            <button id="test-grid-before-fruits">"Before"</button>
            <FruitGrid label="Fruits" with_action=false />
            <button id="test-grid-before-actions">"Before"</button>
            <FruitGrid label="Actions" with_action=true />
        </div>
    }
}
