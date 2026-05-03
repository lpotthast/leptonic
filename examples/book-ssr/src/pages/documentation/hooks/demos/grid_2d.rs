use std::collections::HashSet;

use leptonic::hooks::*;
use leptos::prelude::*;

#[component]
pub fn Grid2dDemo() -> impl IntoView {
    let colors = [
        ["#f44336", "#e91e63", "#9c27b0", "#673ab7"],
        ["#3f51b5", "#2196f3", "#03a9f4", "#00bcd4"],
        ["#009688", "#4caf50", "#8bc34a", "#cddc39"],
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
        <div
            {..grid.props.into_attrs()}
            style="margin: 1em 0;"
        >
            <div
                {..row_group.props.into_attrs()}
                style="display: grid; grid-template-columns: repeat(4, 50px); gap: 8px;"
            >
                {colors.iter().enumerate().map(|(row_idx, row)| {
                    let row_hook = use_grid_row(UseGridRowInput {
                        state: grid.state,
                        key: format!("row-{row_idx}"),
                        row_index: row_idx,
                    });
                    let (row_props, row_styles) = row_hook.props.into_parts();

                    view! {
                        <div
                            {..row_props}
                            style=row_styles.add("display", "contents")
                        >
                            {row.iter().enumerate().map(|(col_idx, color)| {
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
                                    .add("position", "relative")
                                    .add("width", "50px")
                                    .add("height", "50px")
                                    .add("background", color.to_string())
                                    .add("border-radius", "4px")
                                    .add("cursor", "pointer");

                                view! {
                                    <div
                                        {..cell_props}
                                        style=cell_styles
                                        style:outline=move || if is_focused.get() { "3px solid #000" } else { "none" }
                                        style:outline-offset="2px"
                                        style:transform=move || if is_selected.get() { "scale(0.9)" } else { "scale(1)" }
                                    >
                                        <div
                                            style="position: absolute; top: 3px; left: 3px; width: 14px; height: 14px; border-radius: 3px; display: flex; align-items: center; justify-content: center; font-size: 10px; line-height: 1; pointer-events: none;"
                                            style:background=move || if is_selected.get() { "rgba(255, 255, 255, 0.95)" } else { "rgba(0, 0, 0, 0.25)" }
                                            style:border=move || if is_selected.get() { "none" } else { "1.5px solid rgba(255, 255, 255, 0.6)" }
                                            style:color=move || if is_selected.get() { "#333" } else { "transparent" }
                                        >
                                            "\u{2713}"
                                        </div>
                                    </div>
                                }
                            }).collect_view()}
                        </div>
                    }
                }).collect_view()}
            </div>
        </div>

        <div style="margin-top: 1em;">
            <strong>"Focused: "</strong>
            { move || {
                focused_key.get().unwrap_or_else(|| "None".to_string())
            }}
        </div>

        <div style="margin-top: 0.5em;">
            <strong>"Selected: "</strong>
            { move || {
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

        <div style="margin-top: 0.5em;">
            <strong>"Last row action: "</strong>
            { move || {
                last_row_action.get().unwrap_or_else(|| "None".to_string())
            }}
        </div>
    }
}
