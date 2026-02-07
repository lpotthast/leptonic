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
                cells: (0..row.len()).map(|ci| format!("{ri}-{ci}")).collect(),
            })
            .collect(),
    ));

    let (selected, set_selected) = signal(Selection::<String>::default());
    let (last_row_action, set_last_row_action) = signal::<Option<String>>(None);

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
        on_row_action: Some(Callback::new(move |key: String| {
            set_last_row_action.set(Some(key));
        })),
        on_cell_action: None,
    });

    let focused_key = grid.focused_key;

    let row_group = use_grid_row_group();

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

            <p>"Click to select colors (multi-select enabled). Use arrow keys to navigate, Space to toggle selection, Escape to clear. Double-click a row to trigger the row action."</p>

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

                        view! {
                            <div
                                {..row_hook.props.into_attrs()}
                                style="display: contents;"
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

                                    view! {
                                        <div
                                            {..cell.props.into_attrs()}
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
    on_row_action: Some(Callback::new(|key| { /* ... */ })),
    ..Default::default()
});

let row_group = use_grid_row_group();

// Per row:
let row = use_grid_row(UseGridRowInput {
    state: grid.state,
    key: "row-0".to_string(),
    row_index: 0,
});

// Per cell:
let cell = use_grid_cell(UseGridCellInput {
    state: grid.state,
    key: "0-0".to_string(),
    row_index: 0,
    column_index: 0,
    focus_mode: CellFocusMode::Cell,
});

view! {
    <div {..grid.props.into_attrs()}>
        <div {..row_group.props.into_attrs()}>
            <div {..row.props.into_attrs()}>
                <div {..cell.props.into_attrs()}>
                    "Cell content"
                </div>
            </div>
        </div>
    </div>
}"#}
            </Code>

            <h2 id="grid-list-demo" class="anchor">
                "Grid List Demo"
                <AnchorLink href="#grid-list-demo" description="Direct link to grid list demo"/>
            </h2>

            <p>"A grid list is a 1D list with grid role. Arrow Up/Down navigate rows. Arrow Left/Right navigate focusable children within a row. Click or Space to toggle selection."</p>

            <GridListDemo/>

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

            <h2 id="cell-focus-modes" class="anchor">
                "Cell Focus Modes"
                <AnchorLink href="#cell-focus-modes" description="Direct link to cell focus modes"/>
            </h2>

            <ul>
                <li><code>"CellFocusMode::Cell"</code> " - Focus the cell element itself (default)."</li>
                <li><code>"CellFocusMode::Child"</code> " - Automatically focus the first focusable child within the cell."</li>
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
                <li><code>"Arrow Left/Right"</code> " - Move between cells within a row (or focusable children within a cell)"</li>
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

            <p>"For grid row groups:"</p>
            <ul>
                <li><code>"role=\"rowgroup\""</code></li>
            </ul>

            <p>"For grid rows:"</p>
            <ul>
                <li><code>"role=\"row\""</code></li>
                <li><code>"aria-rowindex"</code> " (1-based)"</li>
                <li><code>"aria-selected"</code></li>
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
                Toc::Leaf { title: "Grid List Demo", link: "#grid-list-demo" },
                Toc::Leaf { title: "Grid vs Grid List", link: "#grid-vs-grid-list" },
                Toc::Leaf { title: "Focus Modes", link: "#focus-modes" },
                Toc::Leaf { title: "Cell Focus Modes", link: "#cell-focus-modes" },
                Toc::Leaf { title: "Selection Modes", link: "#selection-modes" },
                Toc::Leaf { title: "Keyboard Navigation", link: "#keyboard-navigation" },
                Toc::Leaf { title: "ARIA Attributes", link: "#aria-attributes" },
            ]
        }/>
    }
}

#[component]
fn GridListDemo() -> impl IntoView {
    let items = vec![
        ("file-1", "Document.pdf"),
        ("file-2", "Photo.jpg"),
        ("file-3", "Spreadsheet.xlsx"),
        ("file-4", "Presentation.pptx"),
        ("file-5", "Archive.zip"),
    ];

    let all_keys = Signal::stored(items.iter().map(|(k, _)| k.to_string()).collect::<Vec<_>>());

    let (list_selected, set_list_selected) = signal(Selection::<String>::default());
    let (last_action, set_last_action) = signal::<Option<String>>(None);

    let grid_list = use_grid_list(UseGridListInput {
        label: Some("Files".to_string()),
        all_keys,
        disabled_keys: Signal::derive(HashSet::new),
        selection_mode: SelectionMode::Multiple,
        selection_behavior: SelectionBehavior::Toggle,
        selected_keys: Some(list_selected.into()),
        on_selection_change: Some(Callback::new(move |sel| set_list_selected.set(sel))),
        escape_key_behavior: EscapeKeyBehavior::ClearSelection,
        on_action: Some(Callback::new(move |key: String| {
            set_last_action.set(Some(key));
        })),
        ..Default::default()
    });

    let list_focused_key = grid_list.focused_key;

    view! {
        <div
            {..grid_list.props.into_attrs()}
            style="margin: 1em 0; border: 1px solid #ccc; border-radius: 4px; overflow: hidden;"
        >
            {items.into_iter().enumerate().map(|(idx, (key, label))| {
                let item = use_grid_list_item(UseGridListItemInput {
                    state: grid_list.state,
                    key: key.to_string(),
                    row_index: idx,
                    is_disabled: false.into(),
                    text_value: Some(label.to_string()),
                });

                let is_selected = item.is_selected;
                let is_focused = item.is_focused;

                view! {
                    <div
                        {..item.row_props.into_attrs()}
                        style="display: flex; align-items: center; padding: 8px 12px; cursor: pointer; user-select: none;"
                        style:background=move || if is_selected.get() { "#e3f2fd" } else { "transparent" }
                        style:outline=move || if is_focused.get() { "2px solid #1976d2" } else { "none" }
                        style:outline-offset="-2px"
                    >
                        <div {..item.gridcell_props.into_attrs()} style="display: flex; align-items: center; gap: 8px; width: 100%;">
                            <span style="width: 20px; text-align: center;">
                                {move || if is_selected.get() { "✓" } else { "" }}
                            </span>
                            <span>{label}</span>
                        </div>
                    </div>
                }
            }).collect_view()}
        </div>

        <div style="margin-top: 1em;">
            <strong>"Focused: "</strong>
            { move || {
                list_focused_key.get().unwrap_or_else(|| "None".to_string())
            }}
        </div>

        <div style="margin-top: 0.5em;">
            <strong>"Selected: "</strong>
            { move || {
                match list_selected.get() {
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
            <strong>"Last action: "</strong>
            { move || {
                last_action.get().unwrap_or_else(|| "None".to_string())
            }}
        </div>

        <Code>
            {r#"let all_keys = Signal::stored(vec!["file-1".into(), "file-2".into()]);

let grid_list = use_grid_list(UseGridListInput {
    label: Some("Files".to_string()),
    all_keys: all_keys.into(),
    selection_mode: SelectionMode::Multiple,
    on_action: Some(Callback::new(|key| { /* ... */ })),
    ..Default::default()
});

// Per item:
let item = use_grid_list_item(UseGridListItemInput {
    state: grid_list.state,
    key: "file-1".to_string(),
    row_index: 0,
    is_disabled: false.into(),
    text_value: Some("Document.pdf".to_string()),
});

view! {
    <div {..grid_list.props.into_attrs()}>
        <div {..item.row_props.into_attrs()}>
            <div {..item.gridcell_props.into_attrs()}>
                "File content"
            </div>
        </div>
    </div>
}"#}
        </Code>
    }
}
