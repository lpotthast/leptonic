use crate::pages::documentation::article::Article;
use crate::pages::documentation::toc::Toc;
use indoc::indoc;
use itertools::Itertools;
use leptonic::atoms::focus_ring::FocusRing;
use leptonic::atoms::grid::{
    Grid as GridAtom, GridCell as GridCellAtom, GridRow as GridRowAtom,
    GridRowGroup as GridRowGroupAtom,
};
use leptonic::atoms::grid_list::{GridList, GridListItem};

use leptonic::components::prelude::*;
use leptonic::hooks::{
    EscapeKeyBehavior, GridCollection, GridFocusMode, GridRow as GridRowData, Selection,
    SelectionBehavior, SelectionMode,
};
use leptonic::utils::styles::Style::{
    AlignItems, Background, Border, BorderBottom, BorderRadius, Cursor, Display, FlexDirection,
    FontFamily, FontSize, Gap, Height, MarginTop, MaxWidth, Outline, OutlineOffset, Overflow,
    Padding, UserSelect, Width,
};
use leptonic::utils::styles::Styles;
use leptos::prelude::*;
use std::collections::HashSet;

fn cell_style(color: &str) -> Styles {
    Styles::builder()
        .with((Width, "50px"))
        .with((Height, "50px"))
        .with((BorderRadius, "4px"))
        .with((Cursor, "pointer"))
        .with((
            "transition",
            "transform 0.15s, outline-color 0.15s, box-shadow 0.15s",
        ))
        .with((Outline, "3px solid transparent"))
        .with((OutlineOffset, "2px"))
        .with((Background, color.to_string()))
        .build()
}

fn row_style() -> Styles {
    Styles::from([(Display, "flex"), (Gap, "8px")])
}

fn grid_layout_style() -> Styles {
    Styles::from([(Display, "flex"), (FlexDirection, "column"), (Gap, "8px")])
}

fn list_item_style() -> Styles {
    Styles::builder()
        .with((Display, "flex"))
        .with((AlignItems, "center"))
        .with((Padding, "10px 14px"))
        .with((Cursor, "pointer"))
        .with((UserSelect, "none"))
        .with(("transition", "background-color 0.15s, outline-color 0.15s"))
        .with((Outline, "2px solid transparent"))
        .with((OutlineOffset, "-2px"))
        .with((BorderBottom, "1px solid #e0e0e0"))
        .with((Gap, "10px"))
        .build()
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

#[component]
fn GridDemo(children: Children) -> impl IntoView {
    view! {
        <div style="padding: 1.5em; border: 1px solid #ddd; border-radius: 8px; margin: 1em 0;">
            {children()}
        </div>
    }
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
#[allow(clippy::too_many_lines)]
pub fn PageAtomGrid() -> impl IntoView {
    view! {
        <Article>
            <h1 id="grid" class="anchor">
                "Grid Atoms"
                <AnchorLink href="#grid" description="Direct link to article header"/>
            </h1>

            <p>
                "The grid atoms wrap the "
                <a href="/documentation/hooks/grid">"grid hooks"</a>
                " into composable components. Two patterns are supported:"
            </p>
            <ul>
                <li><strong>"2D Grid"</strong>" — "<code>"Grid > GridRowGroup > GridRow > GridCell"</code>" for multi-column layouts (color pickers, calendars)"</li>
                <li><strong>"1D Grid List"</strong>" — "<code>"GridList > GridListItem"</code>" for single-column lists with grid keyboard semantics (file lists, card galleries)"</li>
            </ul>
            <p>
                "Both provide full keyboard navigation, selection, and ARIA accessibility. "
                "Style via "<code>"data-selected"</code>", "<code>"data-focused"</code>", and "<code>"data-disabled"</code>" data attributes on each row, cell, or list item."
            </p>

            // ---- 2D Grid Demo ----

            <h2 id="grid-2d" class="anchor">
                "2D Grid"
                <AnchorLink href="#grid-2d" description="Direct link to 2D grid"/>
            </h2>

            <p>"A color palette grid with multi-selection. Click cells to select, arrow keys to navigate, Space to toggle, Escape to clear, Ctrl+A to select all."</p>

            <ColorPaletteDemo/>

            <Code>
                {indoc!(r#"
                    use leptonic::atoms::grid::*;
                    use leptonic::hooks::*;

                    let collection = Signal::stored(GridCollection::new(vec![
                        GridRow { key: "row-0".into(), cells: vec!["0-0".into(), "0-1".into()] },
                        GridRow { key: "row-1".into(), cells: vec!["1-0".into(), "1-1".into()] },
                    ]));

                    let (selected, set_selected) = signal(Selection::<String>::default());

                    view! {
                        <Grid
                            collection
                            selection_mode=SelectionMode::Multiple
                            selection_behavior=SelectionBehavior::Toggle
                            focus_mode=GridFocusMode::Cell
                            selected_keys=selected
                            on_selection_change=Callback::new(move |sel| set_selected.set(sel))
                            label="Color Palette".to_string()
                        >
                            <GridRowGroup>
                                <GridRow item_key="row-0".to_string() row_index=0>
                                    <GridCell item_key="0-0".to_string() row_index=0 column_index=0>
                                        "Cell content"
                                    </GridCell>
                                </GridRow>
                            </GridRowGroup>
                        </Grid>
                    }
                "#)}
            </Code>

            // ---- Grid List Demo ----

            <h2 id="grid-list" class="anchor">
                "Grid List"
                <AnchorLink href="#grid-list" description="Direct link to grid list"/>
            </h2>

            <p>
                "A 1D list with grid keyboard navigation. Arrow Up/Down to navigate, Space to toggle selection, Enter or double-click to trigger the row action."
            </p>

            <FileListDemo/>

            <Code>
                {indoc!(r#"
                    use leptonic::atoms::grid_list::*;
                    use leptonic::hooks::*;

                    let all_keys = Signal::stored(vec![
                        "file-1".to_string(), "file-2".to_string(),
                    ]);
                    let (selected, set_selected) = signal(Selection::<String>::default());

                    view! {
                        <GridList
                            all_keys
                            selection_mode=SelectionMode::Multiple
                            selection_behavior=SelectionBehavior::Toggle
                            selected_keys=selected
                            on_selection_change=Callback::new(move |sel| set_selected.set(sel))
                            on_action=Callback::new(|key: String| { /* handle action */ })
                            label="Files".to_string()
                        >
                            <GridListItem item_key="file-1".to_string() row_index=0
                                text_value="Document.pdf".to_string()
                            >
                                "Document.pdf"
                            </GridListItem>
                        </GridList>
                    }
                "#)}
            </Code>

            // ---- Data Attributes ----

            <h2 id="data-attributes" class="anchor">
                "Data Attributes"
                <AnchorLink href="#data-attributes" description="Direct link to data attributes"/>
            </h2>

            <p>"All row, cell, and list item atoms expose data attributes for CSS styling. This is the key advantage of atoms over raw hooks — you can target these attributes with standard CSS selectors."</p>

            <Code>
                {indoc!(r#"
                    /* Focused indicator — visible outline */
                    [data-focused="true"] {
                        outline: 3px solid #1976d2;
                        outline-offset: 2px;
                    }

                    /* Selected indicator — background highlight */
                    [data-selected="true"] {
                        background-color: rgba(25, 118, 210, 0.15);
                    }

                    /* Disabled items — faded out */
                    [data-disabled="true"] {
                        opacity: 0.4;
                        cursor: not-allowed;
                    }
                "#)}
            </Code>

            // ---- Selection Modes ----

            <h2 id="selection-modes" class="anchor">
                "Selection Modes"
                <AnchorLink href="#selection-modes" description="Direct link to selection modes"/>
            </h2>

            <ul>
                <li><code>"SelectionMode::None"</code>" — Focus-only, no selection"</li>
                <li><code>"SelectionMode::Single"</code>" — One item at a time"</li>
                <li><code>"SelectionMode::Multiple"</code>" — Multiple items (Shift+Arrow extends, Ctrl+A selects all)"</li>
            </ul>

            <p>"Combined with "<code>"SelectionBehavior"</code>":"</p>
            <ul>
                <li><code>"SelectionBehavior::Toggle"</code>" — Click toggles individual items"</li>
                <li><code>"SelectionBehavior::Replace"</code>" — Click replaces the selection (hold Ctrl to toggle)"</li>
            </ul>

            <p>"See the "<a href="/documentation/hooks/grid">"hooks page"</a>" for a detailed reference on keyboard navigation and ARIA attributes."</p>

            // ---- API Reference ----

            <h2 id="api" class="anchor">
                "API Reference"
                <AnchorLink href="#api" description="Direct link to API"/>
            </h2>

            <h3 id="api-grid">"Grid"</h3>
            <ul>
                <li><code>"collection: Signal<GridCollection<K>>"</code>" — Grid structure (rows and cells)"</li>
                <li><code>"selection_mode: SelectionMode"</code>" — None, Single, or Multiple"</li>
                <li><code>"selection_behavior: SelectionBehavior"</code>" — Toggle or Replace"</li>
                <li><code>"focus_mode: GridFocusMode"</code>" — Row or Cell focus mode"</li>
                <li><code>"selected_keys: Option<Signal<Selection<K>>>"</code>" — Controlled selection"</li>
                <li><code>"on_selection_change: Option<Callback<Selection<K>>>"</code>" — Selection callback"</li>
                <li><code>"disabled_keys: Option<Signal<HashSet<K>>>"</code>" — Disabled row/cell keys"</li>
                <li><code>"disabled: Option<Signal<bool>>"</code>" — Disable entire grid"</li>
                <li><code>"escape_key_behavior: EscapeKeyBehavior"</code>" — ClearSelection or None"</li>
                <li><code>"should_focus_wrap: bool"</code>" — Wrap arrow key navigation"</li>
                <li><code>"on_row_action / on_cell_action: Option<Callback<K>>"</code>" — Enter key callbacks"</li>
                <li><code>"label / labelled_by"</code>" — ARIA labeling"</li>
                <li><code>"classes: Classes, styles: Styles"</code>" — CSS styling"</li>
            </ul>

            <h3 id="api-grid-row-group">"GridRowGroup"</h3>
            <p>"Structural wrapper. Props: "<code>"classes"</code>", "<code>"styles"</code>"."</p>

            <h3 id="api-grid-row">"GridRow"</h3>
            <ul>
                <li><code>"item_key: K"</code>" — Row key (must match collection)"</li>
                <li><code>"row_index: usize"</code>" — 0-based row index"</li>
                <li><code>"classes: Classes, styles: Styles"</code>" — CSS styling"</li>
            </ul>

            <h3 id="api-grid-cell">"GridCell"</h3>
            <ul>
                <li><code>"item_key: K"</code>" — Cell key (must match collection)"</li>
                <li><code>"row_index: usize"</code>" — 0-based row index"</li>
                <li><code>"column_index: usize"</code>" — 0-based column index"</li>
                <li><code>"focus_mode: CellFocusMode"</code>" — Cell (default) or Child"</li>
                <li><code>"classes: Classes, styles: Styles"</code>" — CSS styling"</li>
            </ul>

            <h3 id="api-grid-list">"GridList"</h3>
            <ul>
                <li><code>"all_keys: Signal<Vec<K>>"</code>" — Ordered list of all row keys"</li>
                <li><code>"disabled_keys: Option<Signal<HashSet<K>>>"</code>" — Disabled keys"</li>
                <li>"Same selection, ARIA, and behavior props as Grid"</li>
                <li><code>"on_action: Option<Callback<K>>"</code>" — Enter key / double-click callback"</li>
            </ul>

            <h3 id="api-grid-list-item">"GridListItem"</h3>
            <ul>
                <li><code>"item_key: K"</code>" — Item key"</li>
                <li><code>"row_index: usize"</code>" — 0-based row index"</li>
                <li><code>"disabled: Option<Signal<bool>>"</code>" — Per-item disabled state"</li>
                <li><code>"text_value: Option<String>"</code>" — Accessible text label"</li>
                <li><code>"classes: Classes, styles: Styles"</code>" — CSS styling"</li>
            </ul>

        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "Grid Atoms", link: "#grid" },
                Toc::Leaf { title: "2D Grid", link: "#grid-2d" },
                Toc::Leaf { title: "Grid List", link: "#grid-list" },
                Toc::Leaf { title: "Data Attributes", link: "#data-attributes" },
                Toc::Leaf { title: "Selection Modes", link: "#selection-modes" },
                Toc::Leaf { title: "API Reference", link: "#api" },
            ]
        }/>
    }
}

#[component]
#[allow(clippy::too_many_lines)]
fn ColorPaletteDemo() -> impl IntoView {
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
        <GridDemo>
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
        </GridDemo>

        <style>
            "article [role='gridcell'][data-focused='true'] { outline-color: #000 !important; }"
            "article [role='gridcell'][data-selected='true'] { transform: scale(0.85); box-shadow: 0 0 0 3px white, 0 0 0 5px #1976d2; }"
        </style>
    }
}

#[component]
#[allow(clippy::too_many_lines)]
fn FileListDemo() -> impl IntoView {
    let items = [
        ("doc-1", "Document.pdf", "pdf"),
        ("img-1", "Photo.jpg", "img"),
        ("sheet-1", "Spreadsheet.xlsx", "xls"),
        ("pres-1", "Presentation.pptx", "ppt"),
        ("arch-1", "Archive.zip", "zip"),
    ];

    let all_keys: Signal<Vec<String>> = Signal::stored(
        items
            .iter()
            .map(|(k, _, _)| (*k).to_string())
            .collect::<Vec<_>>(),
    );

    let (selected, set_selected) = signal(Selection::<String>::default());
    let (last_action, set_last_action) = signal::<Option<String>>(None);

    let disabled_keys: Signal<HashSet<String>> = Signal::stored(
        ["arch-1".to_string()]
            .into_iter()
            .collect::<HashSet<String>>(),
    );

    let list_container_style = Styles::from([
        (Border, "1px solid #ccc"),
        (BorderRadius, "6px"),
        (Overflow, "hidden"),
        (MaxWidth, "360px"),
    ]);

    view! {
        <GridDemo>
            <GridList
                all_keys
                disabled_keys
                selection_mode=SelectionMode::Multiple
                selection_behavior=SelectionBehavior::Toggle
                selected_keys=selected
                on_selection_change=Callback::new(move |sel| set_selected.set(sel))
                escape_key_behavior=EscapeKeyBehavior::ClearSelection
                on_action=Callback::new(move |key: String| {
                    set_last_action.set(Some(key));
                })
                label="Files".to_string()
                styles=list_container_style
            >
                {items
                    .iter()
                    .enumerate()
                    .map(|(idx, (key, label, icon))| {
                        let key = (*key).to_string();
                        let label = *label;
                        let icon = *icon;
                        view! {
                            <GridListItem<String>
                                item_key=key
                                row_index=idx
                                text_value=label.to_string()
                                styles=list_item_style()
                            >
                                <span style="width: 24px; text-align: center; font-size: 1.1em;">
                                    {file_icon(icon)}
                                </span>
                                <span>{label}</span>
                            </GridListItem<String>>
                        }
                    })
                    .collect_view()}
            </GridList>

            <p style="font-size: 0.85em; color: #666; margin-top: 0.5em;">
                "\"Archive.zip\" is disabled — it is skipped during keyboard navigation and cannot be selected."
            </p>

            <div style=state_display_style()>
                <div>
                    <strong>"Selected: "</strong>
                    {move || format_selection(&selected.get())}
                </div>
                <div style="margin-top: 0.25em;">
                    <strong>"Last action: "</strong>
                    {move || last_action.get().unwrap_or_else(|| "None".to_string())}
                </div>
            </div>
        </GridDemo>

        <style>
            "article [role='row'][data-focused='true'] { outline-color: #1976d2 !important; }"
            "article [role='row'][data-selected='true'] { background-color: #e3f2fd; }"
            "article [role='row'][data-disabled='true'] { opacity: 0.4; cursor: not-allowed !important; }"
        </style>
    }
}

fn file_icon(kind: &str) -> &'static str {
    match kind {
        "pdf" => "\u{1F4C4}",
        "img" => "\u{1F5BC}",
        "xls" | "ppt" => "\u{1F4CA}",
        "zip" => "\u{1F4E6}",
        _ => "\u{1F4C1}",
    }
}
