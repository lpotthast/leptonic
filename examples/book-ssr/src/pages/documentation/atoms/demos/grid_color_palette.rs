use itertools::Itertools;
use leptonic::{
    atoms::{
        focus_ring::FocusRing,
        grid::{
            Grid as GridAtom, GridCell as GridCellAtom, GridRow as GridRowAtom,
            GridRowGroup as GridRowGroupAtom,
        },
    },
    hooks::{
        EscapeKeyBehavior, GridCollection, GridFocusMode, GridRow as GridRowData, Selection,
        SelectionBehavior, SelectionMode,
    },
};
use leptos::prelude::*;
use leptos_styles::{
    Style::{
        Background, BorderRadius, Cursor, Display, FlexDirection, FontFamily, FontSize, Gap,
        Height, MarginTop, Outline, OutlineOffset, Padding, Width,
    },
    Styles,
};

fn cell_style(color: &str) -> Styles {
    Styles::builder()
        .with(Width, "50px")
        .with(Height, "50px")
        .with(BorderRadius, "4px")
        .with(Cursor, "pointer")
        .with(
            "transition",
            "transform 0.15s, outline-color 0.15s, box-shadow 0.15s",
        )
        .with(Outline, "3px solid transparent")
        .with(OutlineOffset, "2px")
        .with(Background, color.to_string())
        .build()
}

fn row_style() -> Styles {
    Styles::from([(Display, "flex"), (Gap, "8px")])
}

fn grid_layout_style() -> Styles {
    Styles::from([(Display, "flex"), (FlexDirection, "column"), (Gap, "8px")])
}

fn state_display_style() -> Styles {
    Styles::from([
        (MarginTop, "1em"),
        (Padding, "0.75em 1em"),
        (Background, "#f5f5f5"),
        (BorderRadius, "6px"),
        (FontSize, "0.9em"),
        (FontFamily, "monospace"),
    ])
}

fn format_selection(sel: &Selection<String>) -> String {
    match sel {
        Selection::Keys(keys) => {
            if keys.is_empty() {
                "None".to_string()
            } else {
                let mut sorted: Vec<_> = keys.iter().collect();
                sorted.sort();
                sorted.into_iter().cloned().join(", ")
            }
        }
        Selection::All => "All".to_string(),
    }
}

#[component]
pub fn GridColorPaletteDemo() -> impl IntoView {
    let colors = [
        ["#f44336", "#e91e63", "#9c27b0", "#673ab7"],
        ["#3f51b5", "#2196f3", "#03a9f4", "#00bcd4"],
        ["#009688", "#4caf50", "#8bc34a", "#cddc39"],
    ];

    let collection: Signal<GridCollection<String>> = Signal::stored(GridCollection::new(
        colors
            .iter()
            .enumerate()
            .map(|(ri, row)| GridRowData {
                key: format!("row-{ri}"),
                cells: (0..row.len()).map(|ci| format!("{ri}-{ci}")).collect(),
            })
            .collect(),
    ));

    let (selected, set_selected) = signal(Selection::<String>::default());
    let (last_action, set_last_action) = signal::<Option<String>>(None);

    view! {
        <div style="padding: 1.5em; border: 1px solid #ddd; border-radius: 8px; margin: 1em 0;">
            <GridAtom
                collection
                selection_mode=SelectionMode::Multiple
                selection_behavior=SelectionBehavior::Toggle
                focus_mode=GridFocusMode::Cell
                selected_keys=selected
                on_selection_change=Callback::new(move |sel| set_selected.set(sel))
                escape_key_behavior=EscapeKeyBehavior::ClearSelection
                on_row_action=Callback::new(move |key: String| {
                    set_last_action.set(Some(key));
                })
                label="Color Palette".to_string()
            >
                <GridRowGroupAtom styles=grid_layout_style()>
                    {colors
                        .into_iter()
                        .enumerate()
                        .map(move |(ri, row)| {
                            view! {
                                <GridRowAtom<String> item_key=format!("row-{ri}") row_index=ri styles=row_style()>
                                    {
                                        row
                                            .into_iter()
                                            .enumerate()
                                            .map(move |(ci, color)| {
                                                view! {
                                                    <FocusRing>
                                                        <GridCellAtom<String>
                                                            item_key=format!("{ri}-{ci}")
                                                            row_index=ri
                                                            column_index=ci
                                                            styles=cell_style(color)
                                                        >
                                                            ""
                                                        </GridCellAtom<String>>
                                                    </FocusRing>
                                                }
                                            })
                                            .collect_view()
                                    }
                                </GridRowAtom<String>>
                            }
                        })
                        .collect_view()}
                </GridRowGroupAtom>
            </GridAtom>

            <div style=state_display_style()>
                <div>
                    <strong>"Selected: "</strong>
                    {move || format_selection(&selected.get())}
                </div>
                <div style="margin-top: 0.25em;">
                    <strong>"Last row action: "</strong>
                    {move || last_action.get().unwrap_or_else(|| "None".to_string())}
                </div>
            </div>
        </div>

        <style>
            "article [role='gridcell'][data-focused='true'] { outline-color: #000 !important; }"
            "article [role='gridcell'][data-selected='true'] { transform: scale(0.85); box-shadow: 0 0 0 3px white, 0 0 0 5px #1976d2; }"
        </style>
    }
}
