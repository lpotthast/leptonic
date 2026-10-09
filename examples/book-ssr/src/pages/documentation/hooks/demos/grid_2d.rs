use leptonic::{
    CapturedElement, IntoAttrs,
    hooks::{
        collections::{
            CollectionOptions, Key, Selection, SelectionMode, SelectionOptions, use_collection,
        },
        grid::{
            GridData, GridFocusMode, UseGridCellInput, UseGridInput, UseGridReturn,
            UseGridRowInput, UseGridStateInput, use_grid, use_grid_cell, use_grid_row,
            use_grid_row_group, use_grid_state,
        },
        gridlist::KeyboardNavigationBehavior,
    },
    leptos_styles::{
        css::{CssColor, rgb},
        property::BackgroundColorProperty,
    },
};
use leptos::prelude::*;

/// A named RGB color.
type Color = (&'static str, (u8, u8, u8));

/// Color palettes: one grid row each, with the palette's name in the first cell and one cell per color.
const PALETTES: [(&str, [Color; 3]); 3] = [
    (
        "Warm",
        [
            ("Red", (0xf4, 0x43, 0x36)),
            ("Orange", (0xff, 0x98, 0x00)),
            ("Amber", (0xff, 0xc1, 0x07)),
        ],
    ),
    (
        "Cool",
        [
            ("Indigo", (0x3f, 0x51, 0xb5)),
            ("Blue", (0x21, 0x96, 0xf3)),
            ("Cyan", (0x00, 0xbc, 0xd4)),
        ],
    ),
    (
        "Green",
        [
            ("Teal", (0x00, 0x96, 0x88)),
            ("Green", (0x4c, 0xaf, 0x50)),
            ("Lime", (0xcd, 0xdc, 0x39)),
        ],
    ),
];

#[component]
pub fn Grid2dDemo() -> impl IntoView {
    // The grid's rows and cells. Cell keys are derived from the row: `Key::cell(&row, column)`.
    let collection = use_collection(|b| {
        for (palette, colors) in PALETTES {
            b.row(palette, palette, |r| {
                r.cell(palette);
                for (color, _) in colors {
                    r.cell(color);
                }
            });
        }
    });
    // App state: the selected palettes, and the cell activated last.
    let selection = RwSignal::new(Selection::default());
    let activated = RwSignal::new(None::<String>);

    let state = use_grid_state(UseGridStateInput {
        selection: SelectionOptions {
            selection_mode: Signal::stored(SelectionMode::Multiple),
            selection: Some(selection.into()),
            ..SelectionOptions::default()
        },
        collection,
        focus_mode: GridFocusMode::Row,
    });
    // The cells have an action, so presses on a cell run it; palettes are selected on the row.
    let UseGridReturn { props, data } = use_grid(UseGridInput {
        aria_label: "Color palettes".into(),
        on_cell_action: Some(Callback::new(move |key: Key| {
            activated.set(collection.with(|c| c.get(&key).map(|node| node.text_value.to_string())));
        })),
        state,
        element: CapturedElement::new(),
        id: None,
        aria_labelledby: Signal::default(),
        keyboard_delegate: None,
        options: CollectionOptions::default(),
        keyboard_navigation_behavior: KeyboardNavigationBehavior::default(),
        should_select_on_press_up: false,
        on_row_action: None,
    });
    let row_group = use_grid_row_group();

    let status = move || {
        let selected = selection.with(|selection| match selection {
            Selection::All => "all".to_owned(),
            Selection::Keys(keys) if keys.is_empty() => "none".to_owned(),
            Selection::Keys(keys) => {
                let mut keys: Vec<String> = keys.iter().map(ToString::to_string).collect();
                keys.sort();
                keys.join(", ")
            }
        });
        let activated = activated.get().unwrap_or_else(|| "none".to_owned());
        format!("Selected palettes: {selected}. Activated: {activated}.")
    };

    view! {
        <div {..props.into_attrs()}>
            <div {..row_group.row_group_props.into_attrs()} class="demo-palette-grid">
                {PALETTES.map(|(palette, colors)| view! { <PaletteRow grid=data.clone() palette colors/> }).collect_view()}
            </div>
        </div>
        <p class="demo-status">{status}</p>
    }
}

/// A row: `use_grid_row` sets `role="row"` and `aria-selected`, which the demo styles.
#[component]
fn PaletteRow(grid: GridData, palette: &'static str, colors: [Color; 3]) -> impl IntoView {
    let row_key = Key::from(palette);
    let row = use_grid_row(UseGridRowInput {
        grid: grid.clone(),
        key: row_key.clone(),
        on_context_menu: None,
    });
    let (row_attrs, row_styles) = row.row_props.into_parts();
    let name = use_grid_cell(UseGridCellInput {
        grid: grid.clone(),
        key: Key::cell(&row_key, 0),
        id: None,
        focus_mode: None,
        allows_arrow_navigation: false,
        should_select_on_press_up: false,
    });
    let (name_attrs, name_styles) = name.grid_cell_props.into_parts();

    view! {
        <div {..row_attrs} class="demo-palette-row" style=row_styles>
            <div {..name_attrs} class="demo-palette-name" style=name_styles>{palette}</div>
            {colors
                .into_iter()
                .enumerate()
                .map(|(i, (color, (r, g, b)))| {
                    view! { <Swatch grid=grid.clone() key=Key::cell(&row_key, i + 1) name=color color=rgb(r, g, b)/> }
                })
                .collect_view()}
        </div>
    }
}

/// A color cell. It shows no text, so it is named by `aria-label`.
#[component]
fn Swatch(grid: GridData, key: Key, name: &'static str, color: CssColor) -> impl IntoView {
    let cell = use_grid_cell(UseGridCellInput {
        grid,
        key,
        id: None,
        focus_mode: None,
        allows_arrow_navigation: false,
        should_select_on_press_up: false,
    });
    let (attrs, styles) = cell.grid_cell_props.into_parts();
    // The color is per-cell data; everything else lives in `.demo-palette-cell`.
    let styles = styles.add(BackgroundColorProperty.declare(color));

    view! { <div {..attrs} aria-label=name title=name class="demo-palette-cell" style=styles></div> }
}
