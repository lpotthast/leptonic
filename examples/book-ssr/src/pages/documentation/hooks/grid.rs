use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::{grid_2d::Grid2dDemo, grid_list::GridListDemo};
use crate::{kit::*, routes};

#[component]
pub fn PageUseGrid() -> impl IntoView {
    view! {
        <DocPage title="Grid Hooks">
            <p>
                <Code inline=true>"use_grid"</Code>" builds two-dimensional grids navigated by row and column (a color "
                "palette, a calendar). "<Code inline=true>"use_grid_list"</Code>" builds lists of interactive rows that may "
                "contain buttons, checkboxes or links. See the "<Link href=routes::doc::Grid.materialize()>"Grid overview"</Link>
                " for concept guidance."
            </p>

            <Section title="use_grid">
                <p>
                    "A grid has rows of cells. Build them as a collection with "<Code inline=true>"b.row(key, text, |r| r.cell(..))"</Code>
                    " (cells may span columns with "<Code inline=true>".col_span(n)"</Code>"); cell keys derive from their row: "
                    <Code inline=true>"Key::cell(&row, column)"</Code>". Selection works on rows."
                </p>

                <Section title="Demo">
                    <p>
                        "Each row is a color palette, each cell a color. Arrow up and down move between palettes, arrow right "
                        "and left between a palette\u{2019}s colors. Space selects the focused palette, Escape clears the selection, "
                        "Enter on a color activates it."
                    </p>

                    <Demo description="Grid of color palettes with row selection and cell actions" source=include_str!("demos/grid_2d.rs")>
                        <Grid2dDemo/>
                    </Demo>
                </Section>

                <Section title="Example">
                    <Code language=Language::Rust>
                        {indoc!(r#"
                            let collection = Memo::new(|_| Arc::new(Collection::build(|b| {
                                b.row("ada", "Ada", |r| { r.cell("Ada"); r.cell("Admin"); });
                                b.row("bob", "Bob", |r| { r.cell("Bob"); r.cell("User"); });
                            })));
                            let state = use_grid_state(UseGridStateInput {
                                collection,
                                selection: SelectionOptions::default(),
                                focus_mode: GridFocusMode::Row,
                            });
                            let UseGridReturn { props, data } = use_grid(UseGridInput {
                                aria_label: "Users".into(),
                                ..UseGridInput::new(state, CapturedElement::new())
                            });

                            // Per row:
                            let row = use_grid_row(UseGridRowInput { grid: data.clone(), key: Key::from("ada") });
                            // Per cell:
                            let cell = use_grid_cell(UseGridCellInput::new(data.clone(), Key::cell(&Key::from("ada"), 0)));
                        "#)}
                    </Code>
                </Section>

                <Section title="Input" id="use-grid-input">
                    <p>"Create the input with "<Code inline=true>"UseGridInput::new(state, element)"</Code>"."</p>
                    <ApiTable kind=ApiKind::Input of="UseGridInput">
                        <ApiRow name="state" ty="GridState">"From "<Code inline=true>"use_grid_state"</Code>"."</ApiRow>
                        <ApiRow name="element" ty="CapturedElement">"The grid element. The props capture it."</ApiRow>
                        <ApiRow name="id" ty="Option<String>" default="None">"The element id, generated when "<Code inline=true>"None"</Code>"."</ApiRow>
                        <ApiRow name="aria_label, aria_labelledby" ty="MaybeProp<String>, Option<String>" default="None">"Names the grid."</ApiRow>
                        <ApiRow name="keyboard_delegate" ty="Option<Signal<Arc<dyn KeyboardDelegate>>>" default="None">"Replaces the grid keyboard navigation."</ApiRow>
                        <ApiRow name="options" ty="CollectionOptions" default="default">"Keyboard and focus behavior."</ApiRow>
                        <ApiRow name="keyboard_navigation_behavior" ty="KeyboardNavigationBehavior" default="Arrow">"How the keyboard reaches interactive children of cells."</ApiRow>
                        <ApiRow name="should_select_on_press_up" ty="bool" default="false">"Select when the press ends instead of when it starts."</ApiRow>
                        <ApiRow name="on_row_action, on_cell_action" ty="Option<Callback<Key>>" default="None">"Called with the key of an activated row or cell."</ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Return" id="use-grid-return">
                    <ApiTable kind=ApiKind::Return of="UseGridReturn">
                        <ApiRow name="props" ty="UseGridProps">
                            "For the grid element: "<Code inline=true>"role=\"grid\""</Code>", labelling, "
                            <Code inline=true>"aria-multiselectable"</Code>", keyboard and focus handling."
                        </ApiRow>
                        <ApiRow name="data" ty="GridData">"Hand this to "<Code inline=true>"use_grid_row"</Code>" and "<Code inline=true>"use_grid_cell"</Code>"."</ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Focus Modes">
                    <DocTable headers=&["Mode", "Behavior"]>
                        <TableRow>
                            <TableCell><Code inline=true>"GridFocusMode::Row"</Code></TableCell>
                            <TableCell>"Arrow up/down move between rows; arrow right enters the cells, arrow left returns to the row."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"GridFocusMode::Cell"</Code></TableCell>
                            <TableCell>"All arrow keys move between cells."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"CellFocusMode::Cell"</Code></TableCell>
                            <TableCell>"A cell receives focus itself."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"CellFocusMode::Child"</Code></TableCell>
                            <TableCell>"A cell moves focus to its first focusable child."</TableCell>
                        </TableRow>
                    </DocTable>
                </Section>

                <Section title="Keyboard">
                    <KeyboardTable>
                        <KeyRow keys="ArrowUp / ArrowDown">"Move between rows (or cells, in cell focus mode)."</KeyRow>
                        <KeyRow keys="ArrowLeft / ArrowRight">"Move between the cells of a row (or their focusable children)."</KeyRow>
                        <KeyRow keys="Home / End">"First or last cell of the row; with "<Keys keys="Control"/>", first or last of the grid."</KeyRow>
                        <KeyRow keys="PageUp / PageDown">"Move by a page."</KeyRow>
                        <KeyRow keys="Space">"Toggle the selection of the row."</KeyRow>
                        <KeyRow keys="Enter">"Activate the row or cell."</KeyRow>
                        <KeyRow keys="Escape">"Clear the selection."</KeyRow>
                        <KeyRow keys="Control + A">"Select all rows (multiple selection)."</KeyRow>
                        <KeyRow keys="Tab">"Leave the grid (it is a single tab stop)."</KeyRow>
                    </KeyboardTable>
                    <p>
                        "Cells can\u{2019}t be selected themselves; presses on a cell select its row. A cell with an action ("
                        <Code inline=true>"on_cell_action"</Code>") owns its presses instead: a click or "<Keys keys="Enter"/>
                        " runs the action, and "<Keys keys="Space"/>" does nothing."
                    </p>
                </Section>
            </Section>

            <Section title="use_grid_state">
                <ApiTable kind=ApiKind::Input of="UseGridStateInput">
                    <ApiRow name="collection" ty="CollectionMemo">"The rows and cells."</ApiRow>
                    <ApiRow name="selection" ty="SelectionOptions">
                        "Row selection, see "<Link href=routes::doc::Collections.materialize()>"Collections"</Link>"."
                    </ApiRow>
                    <ApiRow name="focus_mode" ty="GridFocusMode">"Whether arrow keys move between rows or cells."</ApiRow>
                </ApiTable>
                <p>"Returns a "<Code inline=true>"GridState"</Code>" holding the list state ("<Code inline=true>"list"</Code>") and the focus mode."</p>
            </Section>

            <Section title="use_grid_row">
                <p>"Takes "<Code inline=true>"UseGridRowInput { grid, key }"</Code>"."</p>
                <ApiTable kind=ApiKind::Return of="UseGridRowReturn">
                    <ApiRow name="row_props" ty="PropsWithStyles<UseGridRowProps>">
                        "For the row: "<Code inline=true>"role=\"row\""</Code>", "<Code inline=true>"aria-selected"</Code>", "
                        <Code inline=true>"aria-disabled"</Code>", press and focus handling."
                    </ApiRow>
                    <ApiRow name="is_selected, is_focused, is_disabled, is_pressed" ty="Signal<bool>">"The row\u{2019}s state."</ApiRow>
                    <ApiRow name="allows_selection, has_action" ty="Signal<bool>">"Whether the row can be selected, and whether it has an action."</ApiRow>
                </ApiTable>
            </Section>

            <Section title="use_grid_cell">
                <p>"Create the input with "<Code inline=true>"UseGridCellInput::new(data, cell_key)"</Code>"."</p>
                <ApiTable kind=ApiKind::Input of="UseGridCellInput">
                    <ApiRow name="grid" ty="GridData">"From "<Code inline=true>"use_grid"</Code>"."</ApiRow>
                    <ApiRow name="key" ty="Key">"The cell\u{2019}s key, "<Code inline=true>"Key::cell(&row, column)"</Code>"."</ApiRow>
                    <ApiRow name="id" ty="Option<String>" default="None">"The element id, generated when "<Code inline=true>"None"</Code>"."</ApiRow>
                    <ApiRow name="focus_mode" ty="Option<CellFocusMode>" default="None">"Whether the cell or its first focusable child gets focus."</ApiRow>
                    <ApiRow name="allows_arrow_navigation" ty="bool" default="false">"Let left/right move between the cell\u{2019}s children."</ApiRow>
                    <ApiRow name="should_select_on_press_up" ty="bool" default="false">"Select when the press ends."</ApiRow>
                </ApiTable>
                <p>
                    "Returns "<Code inline=true>"grid_cell_props"</Code>" ("<Code inline=true>"role=\"gridcell\""</Code>", "
                    <Code inline=true>"aria-colindex"</Code>", "<Code inline=true>"aria-colspan"</Code>") and "
                    <Code inline=true>"is_pressed"</Code>". Selection and focus state belong to the row."
                </p>
            </Section>

            <Section title="use_grid_row_group">
                <p>
                    "Returns "<Code inline=true>"row_group_props"</Code>" ("<Code inline=true>"role=\"rowgroup\""</Code>
                    ") for an element grouping rows."
                </p>
            </Section>

            <Section title="use_grid_selection_checkbox">
                <p>
                    "Configures a checkbox selecting a row: "<Code inline=true>"UseGridSelectionCheckboxInput { selection, key }"</Code>
                    " returns a "<Code inline=true>"UseCheckboxInput"</Code>" (with an id, labelled \u{201c}Select\u{201d}) for "
                    <Link href=routes::doc::checkbox::Hook.materialize()>"use_checkbox"</Link>"."
                </p>
            </Section>

            <Section title="use_grid_list">
                <ReactAria hook="useGridList"/>

                <p>
                    "A grid list shows the rows of a collection, like a "<Link href=routes::doc::Listbox.materialize()>"listbox"</Link>
                    ", but its rows may contain interactive elements. Arrow up and down move between rows, arrow left and right "
                    "between a row and its focusable children. Build the rows with "<Code inline=true>"use_list_collection"</Code>
                    " or "<Code inline=true>"use_collection"</Code>", hold their selection with "
                    <Code inline=true>"use_list_state"</Code>", and render each row with "<Code inline=true>"use_grid_list_item"</Code>"."
                </p>

                <Demo description="File list built with use_grid_list, with multiple selection and row actions" source=include_str!("demos/grid_list.rs")>
                    <GridListDemo/>
                </Demo>

                <Section title="Input" id="use-grid-list-input">
                    <p>"Create the input with "<Code inline=true>"UseGridListInput::new(state, element)"</Code>"."</p>
                    <ApiTable kind=ApiKind::Input of="UseGridListInput">
                        <ApiRow name="state" ty="ListState">"The rows and their selection, from "<Code inline=true>"use_list_state"</Code>"."</ApiRow>
                        <ApiRow name="element" ty="CapturedElement">"The grid element. The props capture it."</ApiRow>
                        <ApiRow name="id" ty="Option<String>" default="None">"The element id (row ids derive from it), generated when "<Code inline=true>"None"</Code>"."</ApiRow>
                        <ApiRow name="aria_label" ty="MaybeProp<String>" default="None">"Names the grid list."</ApiRow>
                        <ApiRow name="aria_labelledby" ty="Option<String>" default="None">"The id(s) of elements naming the grid list."</ApiRow>
                        <ApiRow name="layout" ty="ListLayout" default="Stack">
                            <Code inline=true>"Stack"</Code>": one row below the other. "<Code inline=true>"Grid"</Code>
                            ": rows wrap like cards, and up/down find the row in the same column."
                        </ApiRow>
                        <ApiRow name="keyboard_navigation_behavior" ty="KeyboardNavigationBehavior" default="Arrow">
                            <Code inline=true>"Arrow"</Code>": left/right move between a row and its children. "<Code inline=true>"Tab"</Code>
                            ": Tab moves between the children, leaving arrow keys to them (e.g. text inputs)."
                        </ApiRow>
                        <ApiRow name="options" ty="CollectionOptions" default="default">
                            "Keyboard and focus behavior: auto focus, wrapping, Escape, select all, type-ahead."
                        </ApiRow>
                        <ApiRow name="keyboard_delegate" ty="Option<Signal<Arc<dyn KeyboardDelegate>>>" default="None">"Replaces the list keyboard navigation."</ApiRow>
                        <ApiRow name="should_select_on_press_up" ty="bool" default="false">"Select when the press ends instead of when it starts."</ApiRow>
                        <ApiRow name="on_action" ty="Option<Callback<Key>>" default="None">
                            "Called with the key of an activated row: pressed without selection, double-clicked, or Enter with the "
                            <Code inline=true>"Replace"</Code>" selection behavior."
                        </ApiRow>
                        <ApiRow name="tree" ty="Option<TreeExpansion>" default="None">
                            "Makes the rows tree items that expand and collapse. "
                            <Link href=routes::doc::hooks::UseTree.materialize()>"use_tree"</Link>" sets it."
                        </ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Return" id="use-grid-list-return">
                    <ApiTable kind=ApiKind::Return of="UseGridListReturn">
                        <ApiRow name="props" ty="UseGridListProps">
                            "For the grid element: "<Code inline=true>"role=\"grid\""</Code>", label, "
                            <Code inline=true>"aria-multiselectable"</Code>" and the collection\u{2019}s keyboard and focus handlers."
                        </ApiRow>
                        <ApiRow name="data" ty="GridListData">"Hand this to "<Code inline=true>"use_grid_list_item"</Code>" and "<Code inline=true>"use_grid_list_section"</Code>"."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="use_grid_list_item">
                <p>"Create the input with "<Code inline=true>"UseGridListItemInput::new(data, key)"</Code>"."</p>
                <Section title="Input" id="use-grid-list-item-input">
                    <ApiTable kind=ApiKind::Input of="UseGridListItemInput">
                        <ApiRow name="list" ty="GridListData">"The grid list, from "<Code inline=true>"use_grid_list"</Code>"."</ApiRow>
                        <ApiRow name="key" ty="Key">"The row\u{2019}s key in the collection."</ApiRow>
                        <ApiRow name="focus_mode" ty="FocusMode" default="Row">
                            "What receives focus: the row itself, or its first focusable child (e.g. the remove button of a tag)."
                        </ApiRow>
                        <ApiRow name="allows_arrow_navigation" ty="bool" default="false">
                            "Let left/right move between the row\u{2019}s children even with "
                            <Code inline=true>"KeyboardNavigationBehavior::Tab"</Code>"."
                        </ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Return" id="use-grid-list-item-return">
                    <ApiTable kind=ApiKind::Return of="UseGridListItemReturn">
                        <ApiRow name="row_props" ty="PropsWithStyles<UseGridListItemRowProps>">
                            "For the row: "<Code inline=true>"role=\"row\""</Code>", "<Code inline=true>"aria-selected"</Code>", "
                            <Code inline=true>"aria-disabled"</Code>", labelling, and press and keyboard handling."
                        </ApiRow>
                        <ApiRow name="grid_cell_props" ty="UseGridListItemCellProps">
                            "For the single cell inside the row: "<Code inline=true>"role=\"gridcell\""</Code>"."
                        </ApiRow>
                        <ApiRow name="description_props" ty="SlotProps">"For an element describing the row, referenced only while rendered."</ApiRow>
                        <ApiRow name="is_selected, is_focused, is_focus_visible, is_disabled, is_pressed" ty="Signal<bool>">"The row\u{2019}s state."</ApiRow>
                        <ApiRow name="allows_selection, has_action" ty="Signal<bool>">"Whether the row can be selected, and whether it has an action."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="use_grid_list_section">
                <p>
                    "Groups the rows of a collection section ("<Code inline=true>"UseGridListSectionInput { list, key }"</Code>
                    "). Returns "<Code inline=true>"row_props"</Code>" and "<Code inline=true>"row_header_props"</Code>
                    " for a row holding the header, "<Code inline=true>"row_group_props"</Code>" for the element containing the "
                    "section\u{2019}s rows, and the "<Code inline=true>"heading"</Code>" text."
                </p>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::Grid.materialize()>"Grid overview"</Link></li>
                <li><Link href=routes::doc::grid::Atom.materialize()>"Grid atom"</Link></li>
                <li><Link href=routes::doc::grid::Component.materialize()>"Grid component"</Link></li>
                <li><Link href=routes::doc::Table.materialize()>"Table"</Link>" \u{2014} for tabular data with row/column semantics"</li>
                <li><Link href=routes::doc::Collections.materialize()>"Collections"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
