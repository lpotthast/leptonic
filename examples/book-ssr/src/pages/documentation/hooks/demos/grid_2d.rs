use std::sync::Arc;

use leptonic::{
    hooks::{
        GridData, GridFocusMode, IntoAttrs, SelectionMode, UseGridCellInput, UseGridInput,
        UseGridReturn, UseGridRowInput, UseGridStateInput,
        collections::{Collection, Key, Selection, SelectionOptions},
        use_grid, use_grid_cell, use_grid_row, use_grid_row_group, use_grid_state,
    },
    utils::{
        CapturedElement,
        classes::Classes,
        css::{CssColor, rgb},
        style::BackgroundColorProperty,
    },
};
use leptos::prelude::*;

/// An RGB color.
type Rgb = (u8, u8, u8);

/// Color palettes: one grid row each, one cell per color.
const PALETTES: [(&str, [Rgb; 4]); 3] = [
    (
        "Warm",
        [
            (0xf4, 0x43, 0x36),
            (0xe9, 0x1e, 0x63),
            (0xff, 0x98, 0x00),
            (0xff, 0xc1, 0x07),
        ],
    ),
    (
        "Cool",
        [
            (0x3f, 0x51, 0xb5),
            (0x21, 0x96, 0xf3),
            (0x03, 0xa9, 0xf4),
            (0x00, 0xbc, 0xd4),
        ],
    ),
    (
        "Green",
        [
            (0x00, 0x96, 0x88),
            (0x4c, 0xaf, 0x50),
            (0x8b, 0xc3, 0x4a),
            (0xcd, 0xdc, 0x39),
        ],
    ),
];

#[component]
pub fn Grid2dDemo() -> impl IntoView {
    let selected = RwSignal::new(String::from("none"));
    let last_action = RwSignal::new(String::from("none"));

    // The grid's rows and cells. Cell keys are derived from the row: `Key::cell(&row, column)`.
    let collection = Memo::new(|_| {
        Arc::new(Collection::build(|b| {
            for (name, colors) in PALETTES {
                b.row(name, name, |r| {
                    for (i, _) in colors.iter().enumerate() {
                        r.cell(format!("{name} {}", i + 1));
                    }
                });
            }
        }))
    });
    let state = use_grid_state(UseGridStateInput {
        collection,
        selection: SelectionOptions {
            selection_mode: Signal::stored(SelectionMode::Multiple),
            on_selection_change: Some(Callback::new(move |selection: Selection| {
                selected.set(describe(&selection));
            })),
            ..SelectionOptions::default()
        },
        // Arrow up/down move between rows, arrow right enters the cells. The cells have an action (below), so
        // presses on a cell run it; palettes are selected on the row.
        focus_mode: GridFocusMode::Row,
    });
    let UseGridReturn { props, data } = use_grid(UseGridInput {
        aria_label: "Color palettes".into(),
        on_cell_action: Some(Callback::new(move |key: Key| {
            last_action.set(key.to_string());
        })),
        ..UseGridInput::new(state, CapturedElement::new())
    });
    let row_group = use_grid_row_group();

    view! {
        <div {..props.into_attrs()}>
            <div {..row_group.row_group_props.into_attrs()} class="demo-palette-grid">
                {PALETTES.map(|(name, colors)| view! { <PaletteRow grid=data.clone() name colors/> }).collect_view()}
            </div>
        </div>
        <div class="demo-state-display">
            <div><strong>"Selected palettes: "</strong>{selected}</div>
            <div><strong>"Last cell action: "</strong>{last_action}</div>
        </div>
    }
}

#[component]
fn PaletteRow(grid: GridData, name: &'static str, colors: [Rgb; 4]) -> impl IntoView {
    let row_key = Key::from(name);
    let row = use_grid_row(UseGridRowInput {
        grid: grid.clone(),
        key: row_key.clone(),
    });
    let is_selected = row.is_selected;
    let (row_attrs, row_styles) = row.row_props.into_parts();

    view! {
        <div
            {..row_attrs}
            class=Classes::from("demo-palette-row").add_reactive("selected", is_selected)
            style=row_styles
        >
            <span class="demo-palette-name">{name}</span>
            {colors
                .into_iter()
                .enumerate()
                .map(|(column, (r, g, b))| {
                    view! { <Swatch grid=grid.clone() key=Key::cell(&row_key, column) color=rgb(r, g, b)/> }
                })
                .collect_view()}
        </div>
    }
}

#[component]
fn Swatch(grid: GridData, key: Key, color: CssColor) -> impl IntoView {
    let cell = use_grid_cell(UseGridCellInput::new(grid, key));
    let (attrs, styles) = cell.grid_cell_props.into_parts();
    // The color is per-cell data; everything else lives in `.demo-palette-cell`.
    let styles = styles.add(BackgroundColorProperty.declare(color));

    view! { <div {..attrs} class="demo-palette-cell" style=styles></div> }
}

fn describe(selection: &Selection) -> String {
    match selection {
        Selection::All => "all".to_owned(),
        Selection::Keys(keys) if keys.is_empty() => "none".to_owned(),
        Selection::Keys(keys) => {
            let mut keys: Vec<String> = keys.iter().map(ToString::to_string).collect();
            keys.sort();
            keys.join(", ")
        }
    }
}
