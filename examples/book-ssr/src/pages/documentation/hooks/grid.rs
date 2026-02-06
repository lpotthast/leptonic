use crate::pages::documentation::article::Article;
use crate::pages::documentation::toc::Toc;
use leptonic::atoms::link::AnchorLink;
use leptonic::components::prelude::*;
use leptonic::hooks::*;
use leptos::prelude::*;
use std::collections::HashSet;

#[component]
pub fn PageUseGrid() -> impl IntoView {
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
                cells: (0..row.len())
                    .map(|ci| format!("{ri}-{ci}"))
                    .collect(),
            })
            .collect(),
    ));

    let (selected, set_selected) = signal(Selection::<String>::default());

    let grid = use_grid(UseGridInput {
        label: Some("Color Palette".to_string()),
        labelled_by: None,
        collection: collection.into(),
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
        on_row_action: None,
        on_cell_action: None,
    });

    let focused_key = grid.focused_key;
    let selection = grid.selection;

    view! {
        <Article>
            <h1 id="use_grid" class="anchor">
                "use_grid"
                <AnchorLink href="#use_grid" description="Direct link to article header"/>
            </h1>

            <p>"Hooks for creating accessible 2D grid layouts with keyboard navigation and selection."</p>

            <h2 id="demo" class="anchor">
                "Interactive Demo"
                <AnchorLink href="#demo" description="Direct link to demo"/>
            </h2>

            <p>"Click to select colors (multi-select enabled). Use arrow keys to navigate, Space to toggle selection, Escape to clear."</p>

            <div
                {..grid.props.into_attrs()}
                style="display: grid; grid-template-columns: repeat(4, 50px); gap: 8px; margin: 1em 0;"
            >
                {colors.iter().enumerate().map(|(row_idx, row)| {
                    row.iter().enumerate().map(|(col_idx, color)| {
                        let color = *color;
                        let cell_key = format!("{row_idx}-{col_idx}");
                        let cell_key_for_focus = cell_key.clone();
                        let cell_key_for_click = cell_key.clone();

                        let is_selected = selection.is_key_selected(cell_key.clone());
                        let is_focused = Signal::derive(move || {
                            focused_key.get().as_ref() == Some(&cell_key_for_focus)
                        });

                        let cell = use_grid_cell(UseGridCellInput {
                            cell_key: cell_key.clone(),
                            row_index: row_idx,
                            column_index: col_idx,
                            is_selected,
                            is_focused,
                            is_disabled: Signal::derive(|| false),
                            on_selection_change: Some(Callback::new(move |_selected: bool| {
                                selection.toggle.run(cell_key_for_click.clone());
                            })),
                            on_navigate: None,
                            on_action: None,
                        });

                        view! {
                            <div
                                {..cell.cell_props.into_attrs()}
                                style=format!("position: relative; width: 50px; height: 50px; background: {}; border-radius: 4px; cursor: pointer;", color)
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
                                    "✓"
                                </div>
                            </div>
                        }
                    }).collect_view()
                }).collect_view()}
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

            <Code>
                {r#"let collection = Signal::stored(GridCollection::new(vec![
    GridRow { key: "row-0".into(), cells: vec!["0-0".into(), "0-1".into()] },
    GridRow { key: "row-1".into(), cells: vec!["1-0".into(), "1-1".into()] },
]));

let grid = use_grid(UseGridInput {
    label: Some("My Grid".to_string()),
    collection: collection.into(),
    selection_mode: SelectionMode::Multiple,
    focus_mode: GridFocusMode::Cell,
    ..Default::default()
});

view! {
    <div {..grid.props.into_attrs()}>
        // Grid cells...
    </div>
}"#}
            </Code>

            <h2 id="grid-vs-grid-list" class="anchor">
                "Grid vs Grid List"
                <AnchorLink href="#grid-vs-grid-list" description="Direct link to grid vs grid list"/>
            </h2>

            <ul>
                <li><code>"use_grid"</code> " - 2D grid with row/column navigation (like a color picker or calendar)"</li>
                <li><code>"use_grid_list"</code> " - 1D list with grid role (like a card gallery with arrow navigation)"</li>
            </ul>

            <h2 id="focus-modes" class="anchor">
                "Focus Modes"
                <AnchorLink href="#focus-modes" description="Direct link to focus modes"/>
            </h2>

            <ul>
                <li><code>"GridFocusMode::Row"</code> " - Arrow up/down navigate rows. Right enters cells, left exits to row."</li>
                <li><code>"GridFocusMode::Cell"</code> " - All arrows navigate cells in 2D."</li>
            </ul>

            <h2 id="selection-modes" class="anchor">
                "Selection Modes"
                <AnchorLink href="#selection-modes" description="Direct link to selection modes"/>
            </h2>

            <ul>
                <li><code>"SelectionMode::None"</code> " - No selection"</li>
                <li><code>"SelectionMode::Single"</code> " - Single cell selection"</li>
                <li><code>"SelectionMode::Multiple"</code> " - Multiple cell selection (Shift+Arrow extends)"</li>
            </ul>

            <h2 id="keyboard-navigation" class="anchor">
                "Keyboard Navigation"
                <AnchorLink href="#keyboard-navigation" description="Direct link to keyboard"/>
            </h2>

            <ul>
                <li><code>"Arrow Up/Down"</code> " - Move between rows (or cells in Cell mode)"</li>
                <li><code>"Arrow Left/Right"</code> " - Move between cells within a row"</li>
                <li><code>"Home"</code> " - First cell in row (Ctrl+Home for absolute first)"</li>
                <li><code>"End"</code> " - Last cell in row (Ctrl+End for absolute last)"</li>
                <li><code>"Page Up/Down"</code> " - Jump by page (if supported)"</li>
                <li><code>"Space"</code> " - Toggle selection"</li>
                <li><code>"Enter"</code> " - Activate row/cell action"</li>
                <li><code>"Escape"</code> " - Clear selection"</li>
                <li><code>"Ctrl+A"</code> " - Select all (Multiple mode)"</li>
                <li><code>"Tab"</code> " - Exit grid (single tab stop)"</li>
            </ul>

            <h2 id="aria-attributes" class="anchor">
                "ARIA Attributes"
                <AnchorLink href="#aria-attributes" description="Direct link to ARIA attributes"/>
            </h2>

            <p>"For the grid container:"</p>
            <ul>
                <li><code>"role=\"grid\""</code></li>
                <li><code>"aria-label"</code></li>
                <li><code>"aria-multiselectable"</code></li>
                <li><code>"aria-disabled"</code></li>
            </ul>

            <p>"For grid cells:"</p>
            <ul>
                <li><code>"role=\"gridcell\""</code></li>
                <li><code>"aria-selected"</code></li>
                <li><code>"aria-rowindex"</code> " / " <code>"aria-colindex"</code></li>
            </ul>
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "use_grid", link: "#use_grid" },
                Toc::Leaf { title: "Demo", link: "#demo" },
                Toc::Leaf { title: "Grid vs Grid List", link: "#grid-vs-grid-list" },
                Toc::Leaf { title: "Focus Modes", link: "#focus-modes" },
                Toc::Leaf { title: "Selection Modes", link: "#selection-modes" },
                Toc::Leaf { title: "Keyboard Navigation", link: "#keyboard-navigation" },
                Toc::Leaf { title: "ARIA Attributes", link: "#aria-attributes" },
            ]
        }/>
    }
}
