use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::{grid_color_palette::GridColorPaletteDemo, grid_file_list::GridFileListDemo};
use crate::pages::documentation::{article::Article, demo_shell::DemoShell, toc::Toc};

#[component]
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

            <DemoShell source=include_str!("demos/grid_color_palette.rs")>
                <GridColorPaletteDemo/>
            </DemoShell>

            // ---- Grid List Demo ----

            <h2 id="grid-list" class="anchor">
                "Grid List"
                <AnchorLink href="#grid-list" description="Direct link to grid list"/>
            </h2>

            <p>
                "A 1D list with grid keyboard navigation. Arrow Up/Down to navigate, Space to toggle selection, Enter or double-click to trigger the row action."
            </p>

            <DemoShell source=include_str!("demos/grid_file_list.rs")>
                <GridFileListDemo/>
            </DemoShell>

            // ---- Data Attributes ----

            <h2 id="data-attributes" class="anchor">
                "Data Attributes"
                <AnchorLink href="#data-attributes" description="Direct link to data attributes"/>
            </h2>

            <p>"All row, cell, and list item atoms expose data attributes for CSS styling. This is the key advantage of atoms over raw hooks — you can target these attributes with standard CSS selectors."</p>

            <Code language=Language::Rust>
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
