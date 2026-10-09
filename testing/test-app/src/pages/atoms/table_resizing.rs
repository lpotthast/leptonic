use std::{collections::HashMap, sync::Arc};

use leptonic::{
    atoms::table::{ResizableTableContainer, Table, TableBody, TableCell, TableHeader, TableRow},
    hooks::{
        collections::Key,
        table::{ColumnBound, ColumnSize, TableCollection},
    },
};
use leptos::prelude::*;

/// The rows of react-aria's resizing tests.
const POKEMON: [[&str; 6]; 4] = [
    ["1", "Charizard", "Fire, Flying", "5'7\"", "200lbs", "67"],
    ["2", "Blastoise", "Water", "5'3\"", "188lbs", "56"],
    ["3", "Venusaur", "Grass, Poison", "6'7\"", "220lbs", "83"],
    ["4", "Pikachu", "Electric", "1'4\"", "13lbs", "100"],
];

const COLUMNS: [(&str, &str); 5] = [
    ("name", "Name"),
    ("type", "Type"),
    ("height", "Height"),
    ("weight", "Weight"),
    ("level", "Level"),
];

/// The column sizes `sizes`, in column order (`100 1fr 1fr 1fr 5fr`).
fn describe(sizes: &HashMap<Key, ColumnSize>) -> String {
    COLUMNS
        .iter()
        .map(|(key, _)| match sizes.get(&Key::from(*key)) {
            Some(ColumnSize::Px(px)) => px.to_string(),
            Some(ColumnSize::Fr(fr)) => format!("{fr}fr"),
            Some(ColumnSize::Percent(percent)) => format!("{percent}%"),
            None => "-".to_owned(),
        })
        .collect::<Vec<_>>()
        .join(" ")
}

/// Resizable tables in 900 pixel wide containers, like react-aria's resizing tests (with its
/// `TableWithSomeResizingFRsControlled` column setup).
#[component]
pub fn PageAtomTableResizing() -> impl IntoView {
    view! {
        <div id="test-page-atom-table-resizing">
            <h1>"Table column resizing"</h1>
            <button id="test-resizing-before">"Before"</button>
            // Name, Type, Height and Weight at 1fr, Level at 5fr.
            <ResizingTable id="pokemon" label="Pokemon" level=ColumnSize::Fr(5.0) min_width=None />
            // Level at 4fr.
            <ResizingTable id="ratios" label="Ratios" level=ColumnSize::Fr(4.0) min_width=None />
            // Level at 4fr, all columns at least 100 pixels wide.
            <ResizingTable
                id="minimums"
                label="Minimums"
                level=ColumnSize::Fr(4.0)
                min_width=Some(ColumnBound::Px(100.0))
            />
        </div>
    }
}

#[component]
fn ResizingTable(
    id: &'static str,
    label: &'static str,
    level: ColumnSize,
    min_width: Option<ColumnBound>,
) -> impl IntoView {
    let table = Memo::new(move |_| {
        Arc::new(TableCollection::build(|t| {
            for (key, text) in COLUMNS {
                let mut column = t.column(key, text).allows_resizing();
                if key == "name" {
                    column = column.row_header();
                }
                if key == "level" {
                    column = column.default_width(level);
                }
                if let Some(min_width) = min_width {
                    column.min_width(min_width);
                }
            }
            for [key, name, kind, height, weight, level] in POKEMON {
                t.row(key, name, |r| {
                    r.cell(name);
                    r.cell(kind);
                    r.cell(height);
                    r.cell(weight);
                    r.cell(level);
                });
            }
        }))
    });
    let on_resize_start = RwSignal::new(String::new());
    let on_resize = RwSignal::new(String::new());
    let on_resize_count = RwSignal::new(0_u32);
    let on_resize_end = RwSignal::new(String::new());
    let on_resize_end_count = RwSignal::new(0_u32);

    view! {
        <ResizableTableContainer
            classes="test-resizable-container"
            on_resize_start=Callback::new(move |sizes: HashMap<Key, ColumnSize>| {
                on_resize_start.set(describe(&sizes));
            })
            on_resize=Callback::new(move |sizes: HashMap<Key, ColumnSize>| {
                on_resize.set(describe(&sizes));
                on_resize_count.update(|c| *c += 1);
            })
            on_resize_end=Callback::new(move |sizes: HashMap<Key, ColumnSize>| {
                on_resize_end.set(describe(&sizes));
                on_resize_end_count.update(|c| *c += 1);
            })
        >
            <Table table=table aria_label=label>
                <TableHeader />
                <TableBody>
                    {POKEMON
                        .iter()
                        .map(|[key, name, kind, height, weight, level]| {
                            view! {
                                <TableRow key=*key>
                                    <TableCell column="name">{*name}</TableCell>
                                    <TableCell column="type">{*kind}</TableCell>
                                    <TableCell column="height">{*height}</TableCell>
                                    <TableCell column="weight">{*weight}</TableCell>
                                    <TableCell column="level">{*level}</TableCell>
                                </TableRow>
                            }
                        })
                        .collect_view()}
                </TableBody>
            </Table>
        </ResizableTableContainer>
        <div>"onResizeStart: " <span id=format!("test-{id}-resize-start")>{on_resize_start}</span></div>
        <div>
            "onResize: " <span id=format!("test-{id}-resize")>{on_resize}</span> " ("
            <span id=format!("test-{id}-resize-count")>{on_resize_count}</span> ")"
        </div>
        <div>
            "onResizeEnd: " <span id=format!("test-{id}-resize-end")>{on_resize_end}</span> " ("
            <span id=format!("test-{id}-resize-end-count")>{on_resize_end_count}</span> ")"
        </div>
    }
}
