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
    utils::{css::rgb, style::BackgroundColorProperty, styles::Styles},
};
use leptos::prelude::*;

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
    // Each cell's color is data, so it is the one inline style here: a typed `background-color`.
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
        <div class="demo-frame">
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
                <GridRowGroupAtom classes="demo-palette-grid">
                    {colors
                        .into_iter()
                        .enumerate()
                        .map(move |(ri, row)| {
                            view! {
                                <GridRowAtom<String> item_key=format!("row-{ri}") row_index=ri classes="demo-palette-row">
                                    {row
                                        .into_iter()
                                        .enumerate()
                                        .map(move |(ci, color)| {
                                            view! {
                                                <FocusRing>
                                                    <GridCellAtom<String>
                                                        item_key=format!("{ri}-{ci}")
                                                        row_index=ri
                                                        column_index=ci
                                                        classes="demo-palette-cell"
                                                        styles=Styles::new()
                                                            .add(BackgroundColorProperty.declare(color))
                                                    >
                                                        ""
                                                    </GridCellAtom<String>>
                                                </FocusRing>
                                            }
                                        })
                                        .collect_view()}
                                </GridRowAtom<String>>
                            }
                        })
                        .collect_view()}
                </GridRowGroupAtom>
            </GridAtom>

            <div class="demo-state-display">
                <div>
                    <strong>"Selected: "</strong>
                    {move || format_selection(&selected.get())}
                </div>
                <div class="demo-mt-quarter">
                    <strong>"Last row action: "</strong>
                    {move || last_action.get().unwrap_or_else(|| "None".to_string())}
                </div>
            </div>
        </div>
    }
}
