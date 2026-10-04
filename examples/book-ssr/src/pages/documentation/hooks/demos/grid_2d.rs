use std::collections::HashSet;

use leptonic::{
    hooks::*,
    utils::{classes::Classes, css::rgb, style::BackgroundColorProperty},
};
use leptos::prelude::*;

#[component]
pub fn Grid2dDemo() -> impl IntoView {
    let colors = [
        [
            rgb(0xf4, 0x43, 0x36),
            rgb(0xe9, 0x1e, 0x63),
            rgb(0x9c, 0x27, 0xb0),
            rgb(0x67, 0x3a, 0xb7),
        ],
        [
            rgb(0x3f, 0x51, 0xb5),
            rgb(0x21, 0x96, 0xf3),
            rgb(0x03, 0xa9, 0xf4),
            rgb(0x00, 0xbc, 0xd4),
        ],
        [
            rgb(0x00, 0x96, 0x88),
            rgb(0x4c, 0xaf, 0x50),
            rgb(0x8b, 0xc3, 0x4a),
            rgb(0xcd, 0xdc, 0x39),
        ],
    ];

    let collection = Signal::stored(GridCollection::new(
        colors
            .iter()
            .enumerate()
            .map(|(ri, row)| GridRow {
                key: format!("row-{ri}"),
                cells: (0..row.len()).map(|ci| format!("{ri}-{ci}")).collect(),
            })
            .collect(),
    ));

    let (selected, set_selected) = signal(Selection::<String>::default());
    let (last_row_action, set_last_row_action) = signal::<Option<String>>(None);

    let grid = use_grid(UseGridInput {
        label: Some("Color Palette".to_string()),
        labelled_by: None,
        collection,
        disabled_keys: Signal::derive(HashSet::new),
        focus_mode: GridFocusMode::Cell,
        selection_mode: SelectionMode::Multiple,
        selection_behavior: SelectionBehavior::Toggle,
        selected_keys: Some(selected.into()),
        default_selected_keys: None,
        on_selection_change: Some(Callback::new(move |sel| set_selected.set(sel))),
        disallow_empty_selection: false,
        is_disabled: false.into(),
        escape_key_behavior: EscapeKeyBehavior::ClearSelection,
        should_focus_wrap: false,
        on_row_action: Some(Callback::new(move |key: String| {
            set_last_row_action.set(Some(key));
        })),
        on_cell_action: None,
    });

    let focused_key = grid.focused_key;

    let row_group = use_grid_row_group();

    view! {
        <div {..grid.props.into_attrs()} class="demo-my-1">
            <div {..row_group.props.into_attrs()} class="demo-palette-grid-2d">
                {colors
                    .iter()
                    .enumerate()
                    .map(|(row_idx, row)| {
                        let row_hook = use_grid_row(UseGridRowInput {
                            state: grid.state,
                            key: format!("row-{row_idx}"),
                            row_index: row_idx,
                        });
                        let (row_props, row_styles) = row_hook.props.into_parts();

                        view! {
                            <div {..row_props} class="demo-contents" style=row_styles>
                                {row
                                    .iter()
                                    .enumerate()
                                    .map(|(col_idx, color)| {
                                        let color = *color;
                                        let cell = use_grid_cell(UseGridCellInput {
                                            state: grid.state,
                                            key: format!("{row_idx}-{col_idx}"),
                                            row_index: row_idx,
                                            column_index: col_idx,
                                            focus_mode: CellFocusMode::Cell,
                                        });
                                        let is_selected = cell.is_selected;
                                        let is_focused = cell.is_focused;
                                        let (cell_props, cell_styles) = cell.props.into_parts();
                                        let cell_styles = cell_styles
                                            .add(BackgroundColorProperty.declare(color));

                                        // The color is per-cell data; everything else lives in `.demo-palette-cell`.

                                        view! {
                                            <div
                                                {..cell_props}
                                                class=Classes::from("demo-palette-cell")
                                                    .add_reactive("focused", is_focused)
                                                    .add_reactive("selected", is_selected)
                                                style=cell_styles
                                            >
                                                <div class="demo-palette-check">"\u{2713}"</div>
                                            </div>
                                        }
                                    })
                                    .collect_view()}
                            </div>
                        }
                    })
                    .collect_view()}
            </div>
        </div>

        <div class="demo-mt-1">
            <strong>"Focused: "</strong>
            {move || { focused_key.get().unwrap_or_else(|| "None".to_string()) }}
        </div>

        <div class="demo-mt-half">
            <strong>"Selected: "</strong>
            {move || {
                match selected.get() {
                    Selection::Keys(keys) => {
                        if keys.is_empty() {
                            "None".to_string()
                        } else {
                            let mut sorted: Vec<_> = keys.into_iter().collect();
                            sorted.sort();
                            sorted.join(", ")
                        }
                    }
                    Selection::All => "All".to_string(),
                }
            }}
        </div>

        <div class="demo-mt-half">
            <strong>"Last row action: "</strong>
            {move || { last_row_action.get().unwrap_or_else(|| "None".to_string()) }}
        </div>
    }
}
