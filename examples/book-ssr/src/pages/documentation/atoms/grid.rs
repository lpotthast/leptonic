use crate::pages::documentation::article::Article;
use crate::pages::documentation::toc::Toc;
use indoc::indoc;
use leptonic::atoms::grid::{
    Grid as GridAtom, GridCell as GridCellAtom, GridRow as GridRowAtom,
    GridRowGroup as GridRowGroupAtom,
};
use leptonic::atoms::grid_list::{GridList, GridListItem};
use leptonic::atoms::link::AnchorLink;
use leptonic::components::prelude::*;
use leptonic::hooks::{
    GridCollection, GridFocusMode, GridRow as GridRowData, SelectionBehavior, SelectionMode,
};
use leptos::prelude::*;
use std::collections::HashSet;

#[component]
fn GridDemo(children: Children) -> impl IntoView {
    view! {
        <div style="padding: 1.5em; border: 1px solid #ddd; border-radius: 8px; margin: 1em 0;">
            { children() }
        </div>
    }
}

#[component]
#[allow(clippy::too_many_lines)]
pub fn PageAtomGrid() -> impl IntoView {
    let (action_log, set_action_log) = signal(String::new());

    view! {
        <Article>
            <h1 id="grid" class="anchor">
                "Grid"
                <AnchorLink href="#grid" description="Direct link to article header"/>
            </h1>

            <p>
                "The Grid atoms wrap the grid hooks into composable components. "
                "Two patterns are supported:"
            </p>
            <ul>
                <li><strong>"2D Grid"</strong>" — "<code>"Grid > GridRowGroup > GridRow > GridCell"</code>" for multi-column layouts"</li>
                <li><strong>"1D Grid List"</strong>" — "<code>"GridList > GridListItem"</code>" for single-column lists with grid semantics"</li>
            </ul>
            <p>
                "Both provide full keyboard navigation, selection, and ARIA accessibility. "
                "Style via "<code>"data-selected"</code>", "<code>"data-focused"</code>", and "<code>"data-disabled"</code>" data attributes."
            </p>

            // ---- 2D Grid ----

            <h2 id="grid-2d" class="anchor">
                "2D Grid"
                <AnchorLink href="#grid-2d" description="Direct link to 2D grid"/>
            </h2>

            <p>"A 3×4 color palette grid with cell-mode focus and single selection."</p>

            <Code>
                {indoc!(r#"
                    use leptonic::atoms::grid::*;

                    let collection = Signal::stored(GridCollection::new(vec![
                        GridRowData { key: "row-0", cells: vec!["0-0", "0-1", "0-2", "0-3"] },
                        GridRowData { key: "row-1", cells: vec!["1-0", "1-1", "1-2", "1-3"] },
                        GridRowData { key: "row-2", cells: vec!["2-0", "2-1", "2-2", "2-3"] },
                    ]));

                    view! {
                        <Grid collection selection_mode=SelectionMode::Single label="Colors".to_string()>
                            <GridRowGroup>
                                // GridRow + GridCell for each row/cell...
                            </GridRowGroup>
                        </Grid>
                    }
                "#)}
            </Code>

            {
                let colors: &[&[&str]] = &[
                    &["#f44336", "#e91e63", "#9c27b0", "#673ab7"],
                    &["#3f51b5", "#2196f3", "#03a9f4", "#00bcd4"],
                    &["#009688", "#4caf50", "#8bc34a", "#cddc39"],
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

                view! {
                    <GridDemo>
                        <GridAtom
                            collection
                            selection_mode=SelectionMode::Single
                            selection_behavior=SelectionBehavior::Toggle
                            focus_mode=GridFocusMode::Cell
                            label="Color Palette".to_string()
                        >
                            <GridRowGroupAtom>
                                {
                                    colors
                                        .iter()
                                        .enumerate()
                                        .map(|(ri, row)| {
                                            let cells = row
                                                .iter()
                                                .enumerate()
                                                .map(|(ci, color)| {
                                                    let color = *color;
                                                    view! {
                                                        <GridCellAtom<String>
                                                            item_key=format!("{ri}-{ci}")
                                                            row_index=ri
                                                            column_index=ci
                                                            styles=format!(
                                                                "width: 48px; height: 48px; background: {color}; border-radius: 6px; \
                                                                 cursor: pointer; outline: 2px solid transparent; outline-offset: 2px; \
                                                                 transition: outline-color 0.15s;"
                                                            )
                                                        >
                                                            ""
                                                        </GridCellAtom<String>>
                                                    }
                                                })
                                                .collect_view();
                                            view! {
                                                <GridRowAtom<String> item_key=format!("row-{ri}") row_index=ri styles="display: flex; gap: 4px; margin-bottom: 4px;">
                                                    {cells}
                                                </GridRowAtom<String>>
                                            }
                                        })
                                        .collect_view();
                                }
                            </GridRowGroupAtom>
                        </GridAtom>
                        <p style="font-size: 0.85em; color: #666; margin-top: 0.5em;">
                            "Click a cell to select it. Use arrow keys to navigate. Space to toggle selection."
                        </p>
                    </GridDemo>
                }
            }

            <style>
                "[data-selected='true'] { outline-color: white !important; box-shadow: 0 0 0 2px var(--brand-color, #2196f3); }"
                "[data-focused='true'] { outline-color: var(--brand-color, #2196f3) !important; }"
            </style>

            // ---- Multi-Selection Grid ----

            <h2 id="multi-select" class="anchor">
                "Multi-Selection Grid"
                <AnchorLink href="#multi-select" description="Direct link to multi-selection"/>
            </h2>

            <p>"Use "<code>"SelectionMode::Multiple"</code>" for multi-select. Ctrl+click or Shift+arrows to extend selection. Ctrl+A to select all. Escape to clear."</p>

            {
                let items: &[&[&str]] = &[
                    &["Alpha", "Beta", "Gamma"],
                    &["Delta", "Epsilon", "Zeta"],
                    &["Eta", "Theta", "Iota"],
                ];

                let collection: Signal<GridCollection<String>> = Signal::stored(GridCollection::new(
                    items
                        .iter()
                        .enumerate()
                        .map(|(ri, row)| GridRowData {
                            key: format!("row-{ri}"),
                            cells: (0..row.len()).map(|ci| format!("{ri}-{ci}")).collect(),
                        })
                        .collect(),
                ));

                let rows = items
                    .iter()
                    .enumerate()
                    .map(|(ri, row)| {
                        let cells = row
                            .iter()
                            .enumerate()
                            .map(|(ci, label)| {
                                let label = *label;
                                view! {
                                    <GridCellAtom<String>
                                        item_key=format!("{ri}-{ci}")
                                        row_index=ri
                                        column_index=ci
                                        styles="padding: 8px 16px; border: 1px solid #ccc; border-radius: 4px; \
                                                cursor: pointer; min-width: 80px; text-align: center; \
                                                transition: background-color 0.15s;"
                                    >
                                        {label}
                                    </GridCellAtom<String>>
                                }
                            })
                            .collect_view();
                        view! {
                            <GridRowAtom<String> item_key=format!("row-{ri}") row_index=ri styles="display: flex; gap: 4px; margin-bottom: 4px;">
                                {cells}
                            </GridRowAtom<String>>
                        }
                    })
                    .collect_view();

                view! {
                    <GridDemo>
                        <GridAtom
                            collection
                            selection_mode=SelectionMode::Multiple
                            selection_behavior=SelectionBehavior::Toggle
                            focus_mode=GridFocusMode::Cell
                            label="Greek Letters".to_string()
                        >
                            <GridRowGroupAtom>
                                {rows}
                            </GridRowGroupAtom>
                        </GridAtom>
                        <p style="font-size: 0.85em; color: #666; margin-top: 0.5em;">
                            "Ctrl+click to toggle multiple cells. Ctrl+A to select all. Escape to clear."
                        </p>
                    </GridDemo>
                }
            }

            // ---- 1D Grid List ----

            <h2 id="grid-list" class="anchor">
                "1D Grid List"
                <AnchorLink href="#grid-list" description="Direct link to grid list"/>
            </h2>

            <p>"A single-column list with grid semantics. Navigate with ArrowUp/ArrowDown."</p>

            <Code>
                {indoc!(r#"
                    use leptonic::atoms::grid_list::*;

                    let items = vec!["Apple", "Banana", "Cherry", "Date", "Elderberry"];
                    let all_keys: Signal<Vec<String>> = Signal::stored(
                        items.iter().map(|s| s.to_string()).collect::<Vec<_>>()
                    ).into();

                    view! {
                        <GridList all_keys selection_mode=SelectionMode::Single label="Fruits".to_string()>
                            // GridListItem for each item...
                        </GridList>
                    }
                "#)}
            </Code>

            {
                let items = ["Apple", "Banana", "Cherry", "Date", "Elderberry"];
                let all_keys: Signal<Vec<String>> = Signal::stored(
                    items.iter().map(|s| (*s).to_string()).collect::<Vec<_>>()
                );

                view! {
                    <GridDemo>
                        <GridList
                            all_keys
                            selection_mode=SelectionMode::Single
                            selection_behavior=SelectionBehavior::Toggle
                            label="Fruits".to_string()
                            styles="max-width: 300px;"
                        >
                            {items
                                .iter()
                                .enumerate()
                                .map(|(idx, item)| {
                                    let key = item.to_string();
                                    let text = item.to_string();
                                    view! {
                                        <GridListItem<String>
                                            item_key=key.clone()
                                            row_index=idx
                                            text_value=text.clone()
                                            styles="padding: 8px 12px; border: 1px solid #ddd; margin-bottom: 2px; \
                                                    border-radius: 4px; cursor: pointer; transition: background-color 0.15s;"
                                        >
                                            {text}
                                        </GridListItem<String>>
                                    }
                                })
                                .collect_view()}
                        </GridList>
                        <p style="font-size: 0.85em; color: #666; margin-top: 0.5em;">
                            "Use ArrowUp/ArrowDown to navigate. Space to toggle selection."
                        </p>
                    </GridDemo>
                }
            }

            // ---- Grid List with Actions ----

            <h2 id="actions" class="anchor">
                "Grid List with Actions"
                <AnchorLink href="#actions" description="Direct link to actions"/>
            </h2>

            <p>"Pass "<code>"on_action"</code>" to handle Enter key or double-click activation."</p>

            {
                let items = ["Open file", "Save file", "Close file"];
                let all_keys: Signal<Vec<String>> = Signal::stored(
                    items.iter().map(|s| (*s).to_string()).collect::<Vec<_>>()
                );

                view! {
                    <GridDemo>
                        <GridList
                            all_keys
                            selection_mode=SelectionMode::Single
                            selection_behavior=SelectionBehavior::Replace
                            on_action=Callback::new(move |key: String| {
                                set_action_log.set(format!("Action: {key}"));
                            })
                            label="Commands".to_string()
                            styles="max-width: 300px;"
                        >
                            {items
                                .iter()
                                .enumerate()
                                .map(|(idx, item)| {
                                    let key = item.to_string();
                                    let text = item.to_string();
                                    view! {
                                        <GridListItem<String>
                                            item_key=key.clone()
                                            row_index=idx
                                            text_value=text.clone()
                                            styles="padding: 8px 12px; border: 1px solid #ddd; margin-bottom: 2px; \
                                                    border-radius: 4px; cursor: pointer;"
                                        >
                                            {text}
                                        </GridListItem<String>>
                                    }
                                })
                                .collect_view()}
                        </GridList>
                        <div style="margin-top: 0.5em; font-size: 0.9em; font-family: monospace;">
                            { move || action_log.get() }
                        </div>
                        <p style="font-size: 0.85em; color: #666; margin-top: 0.5em;">
                            "Press Enter or double-click an item to trigger its action."
                        </p>
                    </GridDemo>
                }
            }

            // ---- Disabled Items ----

            <h2 id="disabled" class="anchor">
                "Disabled Items"
                <AnchorLink href="#disabled" description="Direct link to disabled items"/>
            </h2>

            <p>
                "Use "<code>"disabled_keys"</code>" on the container to disable specific items, "
                "or the "<code>"disabled"</code>" prop on individual "<code>"GridListItem"</code>" components. "
                "Disabled items are skipped during keyboard navigation and cannot be selected."
            </p>

            {
                let items = ["Enabled A", "Disabled B", "Enabled C", "Disabled D", "Enabled E"];
                let all_keys: Signal<Vec<String>> = Signal::stored(
                    items.iter().map(|s| (*s).to_string()).collect::<Vec<_>>()
                );
                let disabled_keys: Signal<HashSet<String>> = Signal::stored(
                    ["Disabled B".to_string(), "Disabled D".to_string()]
                        .into_iter()
                        .collect::<HashSet<String>>(),
                );

                view! {
                    <GridDemo>
                        <GridList
                            all_keys
                            disabled_keys
                            selection_mode=SelectionMode::Multiple
                            selection_behavior=SelectionBehavior::Toggle
                            label="Mixed Items".to_string()
                            styles="max-width: 300px;"
                        >
                            {items
                                .iter()
                                .enumerate()
                                .map(|(idx, item)| {
                                    let key = item.to_string();
                                    let text = item.to_string();
                                    view! {
                                        <GridListItem<String>
                                            item_key=key.clone()
                                            row_index=idx
                                            text_value=text.clone()
                                            styles="padding: 8px 12px; border: 1px solid #ddd; margin-bottom: 2px; \
                                                    border-radius: 4px; cursor: pointer;"
                                        >
                                            {text}
                                        </GridListItem<String>>
                                    }
                                })
                                .collect_view()}
                        </GridList>
                        <p style="font-size: 0.85em; color: #666; margin-top: 0.5em;">
                            "\"Disabled B\" and \"Disabled D\" are skipped during navigation."
                        </p>
                    </GridDemo>
                }
            }

            <style>
                "[data-disabled='true'] { opacity: 0.4; cursor: not-allowed !important; }"
                "[data-selected='true'][role='row'] { background-color: rgba(33, 150, 243, 0.15); }"
                "[data-focused='true'][role='row'] { outline: 2px solid var(--brand-color, #2196f3); outline-offset: -2px; }"
            </style>

            // ---- API Reference ----

            <h2 id="api" class="anchor">
                "API Reference"
                <AnchorLink href="#api" description="Direct link to API"/>
            </h2>

            <h3>"Grid (2D)"</h3>
            <ul>
                <li><code>"collection: Signal<GridCollection<K>>"</code>" - Grid structure (rows and cells)"</li>
                <li><code>"disabled_keys: Option<Signal<HashSet<K>>>"</code>" - Disabled row/cell keys"</li>
                <li><code>"focus_mode: GridFocusMode"</code>" - Row or Cell focus mode"</li>
                <li><code>"selection_mode: SelectionMode"</code>" - None, Single, or Multiple"</li>
                <li><code>"selection_behavior: SelectionBehavior"</code>" - Toggle or Replace"</li>
                <li><code>"selected_keys: Option<Signal<Selection<K>>>"</code>" - Controlled selection"</li>
                <li><code>"on_selection_change: Option<Callback<Selection<K>>>"</code>" - Selection callback"</li>
                <li><code>"disabled: Option<Signal<bool>>"</code>" - Disable entire grid"</li>
                <li><code>"escape_key_behavior: EscapeKeyBehavior"</code>" - ClearSelection or None"</li>
                <li><code>"should_focus_wrap: bool"</code>" - Wrap arrow navigation"</li>
                <li><code>"on_row_action / on_cell_action: Option<Callback<K>>"</code>" - Enter key callbacks"</li>
                <li><code>"label / labelled_by"</code>" - ARIA labeling"</li>
            </ul>

            <h3>"GridRowGroup"</h3>
            <p>"Structural wrapper. Props: "<code>"classes"</code>", "<code>"styles"</code>"."</p>

            <h3>"GridRow"</h3>
            <ul>
                <li><code>"item_key: K"</code>" - Row key (must match collection)"</li>
                <li><code>"row_index: usize"</code>" - 0-based row index"</li>
            </ul>

            <h3>"GridCell"</h3>
            <ul>
                <li><code>"item_key: K"</code>" - Cell key (must match collection)"</li>
                <li><code>"row_index: usize"</code>" - 0-based row index"</li>
                <li><code>"column_index: usize"</code>" - 0-based column index"</li>
                <li><code>"focus_mode: CellFocusMode"</code>" - Cell (default) or Child"</li>
            </ul>

            <h3>"GridList (1D)"</h3>
            <ul>
                <li><code>"all_keys: Signal<Vec<K>>"</code>" - Ordered list of all row keys"</li>
                <li><code>"disabled_keys: Option<Signal<HashSet<K>>>"</code>" - Disabled keys"</li>
                <li><code>"selection_mode / selection_behavior"</code>" - Selection configuration"</li>
                <li><code>"on_action: Option<Callback<K>>"</code>" - Enter key / double-click callback"</li>
                <li>"Same ARIA and behavior props as Grid"</li>
            </ul>

            <h3>"GridListItem"</h3>
            <ul>
                <li><code>"item_key: K"</code>" - Item key"</li>
                <li><code>"row_index: usize"</code>" - 0-based row index"</li>
                <li><code>"disabled: Option<Signal<bool>>"</code>" - Per-item disabled state"</li>
                <li><code>"text_value: Option<String>"</code>" - Accessible text label"</li>
            </ul>

            <h3>"Data Attributes"</h3>
            <p>"All row, cell, and list item atoms expose these data attributes for styling:"</p>
            <ul>
                <li><code>"data-selected=\"true\""</code>" - Present when the item is selected"</li>
                <li><code>"data-focused=\"true\""</code>" - Present when the item has keyboard focus"</li>
                <li><code>"data-disabled=\"true\""</code>" - Present when the item is disabled"</li>
            </ul>

        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "Grid", link: "#grid" },
                Toc::Leaf { title: "2D Grid", link: "#grid-2d" },
                Toc::Leaf { title: "Multi-Selection Grid", link: "#multi-select" },
                Toc::Leaf { title: "1D Grid List", link: "#grid-list" },
                Toc::Leaf { title: "Grid List with Actions", link: "#actions" },
                Toc::Leaf { title: "Disabled Items", link: "#disabled" },
                Toc::Leaf { title: "API Reference", link: "#api" },
            ]
        }/>
    }
}
