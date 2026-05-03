use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::prelude::*;

use crate::pages::documentation::{article::Article, demo_shell::DemoShell, toc::Toc};

use super::demos::{grid_2d::Grid2dDemo, grid_list::GridListDemo};

#[component]
pub fn PageUseGrid() -> impl IntoView {
    view! {
        <Article>
            <h1 id="use_grid" class="anchor">
                "use_grid"
                <AnchorLink href="#use_grid" description="Direct link to article header"/>
            </h1>

            <p>
                "Hooks for creating accessible 2D grid layouts with keyboard navigation and selection. "
                "See the "<Link href=crate::routes::doc::Grid.materialize()>"Grid overview"</Link>" for concept guidance."
            </p>

            <p>
                "Based on react-aria\u{2019}s "
                <LinkExt href="https://react-spectrum.adobe.com/react-aria/useGridList.html" target=LinkTarget::_Blank>
                    "useGridList"
                </LinkExt>
                "."
            </p>

            <h2 id="demo" class="anchor">
                "Interactive Demo"
                <AnchorLink href="#demo" description="Direct link to demo"/>
            </h2>

            <p>"Click to select colors (multi-select enabled). Use arrow keys to navigate, Space to toggle selection, Escape to clear. Double-click a row to trigger the row action."</p>

            <DemoShell source=include_str!("demos/grid_2d.rs")>
                <Grid2dDemo />
            </DemoShell>

            <Code language=Language::Rust>
                {indoc!(r#"
                    let collection = Signal::stored(GridCollection::new(vec![
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
                    }
                "#)}
            </Code>

            <h2 id="grid-list-demo" class="anchor">
                "Grid List Demo"
                <AnchorLink href="#grid-list-demo" description="Direct link to grid list demo"/>
            </h2>

            <p>"A grid list is a 1D list with grid role. Arrow Up/Down navigate rows. Arrow Left/Right navigate focusable children within a row. Click or Space to toggle selection."</p>

            <DemoShell source=include_str!("demos/grid_list.rs")>
                <GridListDemo />
            </DemoShell>

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

            <h2 id="see-also" class="anchor">
                "See Also"
                <AnchorLink href="#see-also" description="Direct link to section: See Also"/>
            </h2>

            <ul>
                <li><Link href=crate::routes::doc::Grid.materialize()>"Grid overview"</Link></li>
                <li><Link href=crate::routes::doc::grid::Atom.materialize()>"Grid atom"</Link></li>
                <li><Link href=crate::routes::doc::grid::Component.materialize()>"Grid component"</Link></li>
                <li><Link href=crate::routes::doc::Table.materialize()>"Table"</Link>" \u{2014} for tabular data with row/column semantics"</li>
                <li><Link href=crate::routes::doc::hooks::Selection.materialize()>"Selection system"</Link></li>
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
                Toc::Leaf { title: "See Also", link: "#see-also" },
            ]
        }/>
    }
}
