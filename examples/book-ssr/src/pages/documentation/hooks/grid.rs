use indoc::indoc;
use leptos::prelude::*;

use super::demos::grid_2d::Grid2dDemo;
use crate::{kit::*, routes};

#[component]
#[allow(clippy::too_many_lines)]
pub fn PageUseGrid() -> impl IntoView {
    view! {
        <DocPage title="Grid Hooks">
            <p>
                "The grid hooks build a two-dimensional grid of rows and cells from your own markup. See the "
                <Link href=routes::doc::Grid.materialize()>"Grid overview"</Link>" for concept guidance and keyboard "
                "interaction."
            </p>

            <ReactAriaSource path="grid/useGrid.ts"/>

            <Section title="Example">
                <p>
                    "Build the rows and cells as a collection with "<Code inline=true>"b.row(key, text, |r| r.cell(..))"</Code>
                    " (see "<Link href=format!("{}#collectionbuilder", routes::doc::CollectionState.materialize())>"CollectionBuilder"</Link>
                    "); cell keys derive from their row: "<Code inline=true>"Key::cell(&row, column)"</Code>". Then create "
                    "the state, the grid, and one row and cell hook per element:"
                </p>
                <Code language=Language::Rust>
                    {indoc!(r#"
                        use leptonic::{
                            CapturedElement,
                            hooks::{
                                collections::{
                                    CollectionOptions,
                                    Key,
                                    SelectionOptions,
                                    use_collection,
                                },
                                grid::{
                                    GridFocusMode,
                                    UseGridCellInput,
                                    UseGridInput,
                                    UseGridReturn,
                                    UseGridRowInput,
                                    UseGridStateInput,
                                    use_grid,
                                    use_grid_cell,
                                    use_grid_row,
                                    use_grid_state,
                                },
                                gridlist::KeyboardNavigationBehavior,
                            },
                        };
                        use leptos::prelude::*;

                        let collection = use_collection(|b| {
                            b.row("ada", "Ada", |r| { r.cell("Ada"); r.cell("Admin"); });
                            b.row("bob", "Bob", |r| { r.cell("Bob"); r.cell("User"); });
                        });
                        let state = use_grid_state(UseGridStateInput {
                            collection,
                            selection: SelectionOptions::default(),
                            focus_mode: GridFocusMode::Row,
                        });
                        let UseGridReturn { props, data } = use_grid(UseGridInput {
                            state,
                            element: CapturedElement::new(),
                            id: None,
                            aria_label: "Users".into(),
                            aria_labelledby: None,
                            keyboard_delegate: None,
                            options: CollectionOptions::default(),
                            keyboard_navigation_behavior: KeyboardNavigationBehavior::Arrow,
                            should_select_on_press_up: false,
                            on_row_action: None,
                            on_cell_action: None,
                        });

                        // Per row:
                        let row = use_grid_row(UseGridRowInput { grid: data.clone(), key: Key::from("ada"), on_context_menu: None });
                        let (row_attrs, row_styles) = row.row_props.into_parts();
                        // Per cell:
                        let cell = use_grid_cell(UseGridCellInput {
                            grid: data.clone(),
                            key: Key::cell(&Key::from("ada"), 0),
                            id: None,
                            focus_mode: None,
                            allows_arrow_navigation: false,
                            should_select_on_press_up: false,
                        });
                        let (cell_attrs, cell_styles) = cell.grid_cell_props.into_parts();
                    "#)}
                </Code>
            </Section>

            <Section title="Demo">
                <p>
                    "Each row is a color palette: its name, then its colors. "<Keys keys="ArrowUp"/>" and "
                    <Keys keys="ArrowDown"/>" move between palettes, "<Keys keys="ArrowRight"/>" and "
                    <Keys keys="ArrowLeft"/>" between a palette\u{2019}s cells. "<Keys keys="Space"/>" selects the "
                    "focused palette, "<Keys keys="Enter"/>" on a cell activates it."
                </p>
                <Demo description="Grid of color palettes with row selection and cell actions" source=include_str!("demos/grid_2d.rs")>
                    <Grid2dDemo/>
                </Demo>
            </Section>

            <Section title="use_grid_state">
                <p>
                    "Holds the rows and cells, the row selection and the focus. Pass a "
                    <Code inline=true>"UseGridStateInput"</Code>" with every field named; the Default column gives the value "
                    "for fields you don\u{2019}t need (rows focused first, no selection)."
                </p>
                <Section title="Input" id="use-grid-state-input">
                    <ApiTable kind=ApiKind::Input of="UseGridStateInput">
                        <ApiRow name="collection" ty="CollectionMemo">"The rows and their cells. Required."</ApiRow>
                        <ApiRow name="selection" ty="SelectionOptions" default="SelectionOptions::default()">
                            "Row selection, see "<Link href=format!("{}#selectionoptions", routes::doc::CollectionState.materialize())>"SelectionOptions"</Link>"."
                        </ApiRow>
                        <ApiRow name="focus_mode" ty="GridFocusMode" default="Row">
                            "What the arrow keys move between, see "<AnchorLink href="#focus-modes">"Focus Modes"</AnchorLink>"."
                        </ApiRow>
                    </ApiTable>
                </Section>
                <Section title="Return" id="use-grid-state-return">
                    <ApiTable kind=ApiKind::Return of="GridState">
                        <ApiRow name="list" ty="ListState">
                            "The rows\u{2019} "<Link href=format!("{}#liststate", routes::doc::CollectionState.materialize())>"list state"</Link>
                            ": collection, selection and focus."
                        </ApiRow>
                        <ApiRow name="focus_mode" ty="GridFocusMode">"The focus mode."</ApiRow>
                    </ApiTable>
                    <p>
                        <Code inline=true>"set_keyboard_navigation_disabled(true)"</Code>" lets the grid ignore navigation keys "
                        "(e.g. while the arrow keys resize a table column), "<Code inline=true>"false"</Code>" handles them again; "
                        <Code inline=true>"is_keyboard_navigation_disabled()"</Code>" reads it as a "<Code inline=true>"Signal<bool>"</Code>
                        " (for tables also "<Code inline=true>"true"</Code>" while they have no rows)."
                    </p>
                </Section>
            </Section>

            <Section title="use_grid">
                <p>"Keyboard navigation, selection and ARIA attributes for the grid element."</p>
                <Section title="Input" id="use-grid-input">
                    <p>"Pass a "<Code inline=true>"UseGridInput"</Code>" with every field named; the Default column gives the value for fields you don\u{2019}t need."</p>
                    <ApiTable kind=ApiKind::Input of="UseGridInput">
                        <ApiRow name="state" ty="GridState">"From "<Code inline=true>"use_grid_state"</Code>". Required."</ApiRow>
                        <ApiRow name="element" ty="CapturedElement">"The grid element; the props capture it. Required."</ApiRow>
                        <ApiRow name="id" ty="Option<String>" default="None">"The element id, generated when "<Code inline=true>"None"</Code>"."</ApiRow>
                        <ApiRow name="aria_label, aria_labelledby" ty="MaybeProp<String>, Signal<Option<String>>" default="None">"Names the grid."</ApiRow>
                        <ApiRow name="keyboard_delegate" ty="Option<Signal<Arc<dyn KeyboardDelegate>>>" default="None">
                            "Replaces the "<Link href=format!("{}#use-grid-keyboard-delegate", routes::doc::CollectionState.materialize())>"grid keyboard delegate"</Link>"."
                        </ApiRow>
                        <ApiRow name="options" ty="CollectionOptions" default="CollectionOptions::default()">
                            "Keyboard and focus behavior, see "<Link href=format!("{}#collectionoptions", routes::doc::CollectionState.materialize())>"CollectionOptions"</Link>"."
                        </ApiRow>
                        <ApiRow name="keyboard_navigation_behavior" ty="KeyboardNavigationBehavior" default="Arrow">
                            <Code inline=true>"Arrow"</Code>": "<Keys keys="ArrowLeft"/>" and "<Keys keys="ArrowRight"/>" move "
                            "between the focusable children of a cell. "<Code inline=true>"Tab"</Code>": "<Keys keys="Tab"/>
                            " moves between them, and the arrow keys stay with them (e.g. text inputs in cells)."
                        </ApiRow>
                        <ApiRow name="should_select_on_press_up" ty="bool" default="false">"Select when the press ends instead of when it starts."</ApiRow>
                        <ApiRow name="on_row_action, on_cell_action" ty="Option<Callback<Key>>" default="None">
                            "Called with the key of an activated row or cell ("<Keys keys="Enter"/>", a click without "
                            "selection, or a double click with the "<Code inline=true>"Replace"</Code>" selection behavior)."
                        </ApiRow>
                    </ApiTable>
                </Section>
                <Section title="Return" id="use-grid-return">
                    <ApiTable kind=ApiKind::Return of="UseGridReturn">
                        <ApiRow name="props" ty="UseGridProps">
                            "For the grid element: "<Code inline=true>"role=\"grid\""</Code>", labelling, "
                            <Code inline=true>"aria-multiselectable"</Code>", keyboard and focus handling."
                        </ApiRow>
                        <ApiRow name="data" ty="GridData">
                            "Hand it to "<Code inline=true>"use_grid_row"</Code>" and "<Code inline=true>"use_grid_cell"</Code>"."
                        </ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="use_grid_row_group">
                <p>"Takes no input. Groups rows, like "<Code inline=true>"<tbody>"</Code>"."</p>
                <Section title="Return" id="use-grid-row-group-return">
                    <ApiTable kind=ApiKind::Return of="UseGridRowGroupReturn">
                        <ApiRow name="row_group_props" ty="UseGridRowGroupProps">
                            "For the element grouping rows: "<Code inline=true>"role=\"rowgroup\""</Code>"."
                        </ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="use_grid_row">
                <p>"A row: selected and activated by press, the tab stop while it is focused."</p>
                <Section title="Input" id="use-grid-row-input">
                    <ApiTable kind=ApiKind::Input of="UseGridRowInput">
                        <ApiRow name="grid" ty="GridData">"From "<Code inline=true>"use_grid"</Code>". Required."</ApiRow>
                        <ApiRow name="key" ty="Key">"The row\u{2019}s key in the collection. Required."</ApiRow>
                        <ApiRow name="on_context_menu" ty="Option<Callback<ContextMenuEvent>>" default="None">
                            "Called when a context menu is requested on the row (right click, "<Keys keys="Shift + F10"/>", the context menu key, a long press on iOS); the row\u{2019}s menu then replaces the browser\u{2019}s."
                        </ApiRow>
                    </ApiTable>
                </Section>
                <Section title="Return" id="use-grid-row-return">
                    <ApiTable kind=ApiKind::Return of="UseGridRowReturn">
                        <ApiRow name="row_props" ty="PropsWithStyles<UseGridRowProps>">
                            "For the row: "<Code inline=true>"role=\"row\""</Code>", "<Code inline=true>"aria-selected"</Code>", "
                            <Code inline=true>"aria-disabled"</Code>", press and focus handling. Spread with "
                            <Code inline=true>"into_parts()"</Code>"."
                        </ApiRow>
                        <ApiRow name="is_selected, is_focused, is_disabled, is_pressed" ty="Signal<bool>">"The row\u{2019}s state."</ApiRow>
                        <ApiRow name="allows_selection, has_action" ty="Signal<bool>">
                            "Whether the row can be selected, and whether it has an action."
                        </ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="use_grid_cell">
                <p>
                    "A cell: focusable itself or through its interactive children. Cells aren\u{2019}t selected "
                    "themselves; presses on a cell select its row. A cell of a grid with "<Code inline=true>"on_cell_action"</Code>
                    " owns its presses instead: a click or "<Keys keys="Enter"/>" runs the action."
                </p>
                <Section title="Input" id="use-grid-cell-input">
                    <p>"Pass a "<Code inline=true>"UseGridCellInput"</Code>" with every field named; the Default column gives the value for fields you don\u{2019}t need."</p>

                    <ApiTable kind=ApiKind::Input of="UseGridCellInput">
                        <ApiRow name="grid" ty="GridData">"From "<Code inline=true>"use_grid"</Code>". Required."</ApiRow>
                        <ApiRow name="key" ty="Key">"The cell\u{2019}s key, "<Code inline=true>"Key::cell(&row, column)"</Code>". Required."</ApiRow>
                        <ApiRow name="id" ty="Option<String>" default="None">"The element id, generated when "<Code inline=true>"None"</Code>"."</ApiRow>
                        <ApiRow name="focus_mode" ty="Option<CellFocusMode>" default="None">
                            "What gets focus, see "<AnchorLink href="#focus-modes">"Focus Modes"</AnchorLink>". "
                            <Code inline=true>"None"</Code>": "<Code inline=true>"Cell"</Code>" with the "<Code inline=true>"Tab"</Code>
                            " navigation behavior, else "<Code inline=true>"Child"</Code>"."
                        </ApiRow>
                        <ApiRow name="allows_arrow_navigation" ty="bool" default="false">
                            "Let "<Keys keys="ArrowLeft"/>" and "<Keys keys="ArrowRight"/>" move between the cell\u{2019}s "
                            "children even with the "<Code inline=true>"Tab"</Code>" navigation behavior."
                        </ApiRow>
                        <ApiRow name="should_select_on_press_up" ty="bool" default="false">"Select when the press ends."</ApiRow>
                    </ApiTable>
                </Section>
                <Section title="Return" id="use-grid-cell-return">
                    <ApiTable kind=ApiKind::Return of="UseGridCellReturn">
                        <ApiRow name="grid_cell_props" ty="PropsWithStyles<UseGridCellProps>">
                            "For the cell: "<Code inline=true>"role=\"gridcell\""</Code>", "<Code inline=true>"aria-colindex"</Code>", "
                            <Code inline=true>"aria-colspan"</Code>", press, focus and key handling."
                        </ApiRow>
                        <ApiRow name="is_pressed" ty="Signal<bool>">"Whether the cell is being pressed."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="use_grid_selection_checkbox">
                <p>
                    "Configures a checkbox selecting a row (of a grid, a grid list or a table): it returns a "
                    <Code inline=true>"UseCheckboxInput"</Code>" (with an id, labelled \u{201c}Select\u{201d}) for "
                    <Link href=routes::doc::checkbox::Hook.materialize()>"use_checkbox"</Link>"."
                </p>
                <Section title="Input" id="use-grid-selection-checkbox-input">
                    <ApiTable kind=ApiKind::Input of="UseGridSelectionCheckboxInput">
                        <ApiRow name="selection" ty="SelectionManager">"The grid state\u{2019}s selection ("<Code inline=true>"state.list.selection"</Code>"). Required."</ApiRow>
                        <ApiRow name="key" ty="Key">"The row the checkbox selects. Required."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="use_grid_selection_announcement">
                <p>
                    "Announces selection changes through the "
                    <Link href=routes::doc::screen_readers::LiveAnnouncer.materialize()>"live announcer"</Link>
                    ", as many screen readers don\u{2019}t: \u{201c}Inbox selected.\u{201d}, \u{201c}Inbox not selected.\u{201d}, "
                    "\u{201c}Drafts selected. 2 items selected.\u{201d}, \u{201c}All items selected.\u{201d}, in the locale\u{2019}s "
                    "language. "<Code inline=true>"use_grid"</Code>" and "
                    <Link href=routes::doc::grid_list::Hook.materialize()>"use_grid_list"</Link>" call it; call it yourself "
                    "only for a collection you build from lower-level hooks. It returns nothing."
                </p>
                <Section title="Input" id="use-grid-selection-announcement-input">
                    <ApiTable kind=ApiKind::Input of="UseGridSelectionAnnouncementInput">
                        <ApiRow name="selection" ty="SelectionManager">"The collection\u{2019}s selection. Required."</ApiRow>
                        <ApiRow name="collection" ty="CollectionMemo">"The rows, for their texts. Required."</ApiRow>
                        <ApiRow name="get_row_text" ty="Option<GetRowText>" default="None">
                            "The text announced for a row ("<Code inline=true>"None"</Code>" from the function: nothing). "
                            "Default: the row\u{2019}s text value."
                        </ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="use_highlight_selection_description">
                <p>
                    "Describes how to select in a collection whose rows perform an action when pressed and select with the "
                    <Code inline=true>"Replace"</Code>" behavior: on touch devices, \u{201c}Long press to enter selection "
                    "mode.\u{201d} It returns the id of the description while there is one, for the collection\u{2019}s "
                    <Code inline=true>"aria-describedby"</Code>" ("<Code inline=true>"Signal<Option<String>>"</Code>"). "
                    <Code inline=true>"use_grid"</Code>" and "<Code inline=true>"use_grid_list"</Code>" call it."
                </p>
                <Section title="Input" id="use-highlight-selection-description-input">
                    <ApiTable kind=ApiKind::Input of="UseHighlightSelectionDescriptionInput">
                        <ApiRow name="selection" ty="SelectionManager">"The collection\u{2019}s selection. Required."</ApiRow>
                        <ApiRow name="has_item_actions" ty="bool">"Whether the rows (or cells) have actions. Required."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="Focus Modes">
                <DocTable headers=&["Mode", "Behavior"]>
                    <TableRow>
                        <TableCell><Code inline=true>"GridFocusMode::Row"</Code></TableCell>
                        <TableCell>
                            <Keys keys="ArrowUp"/>" and "<Keys keys="ArrowDown"/>" move between rows; "<Keys keys="ArrowRight"/>
                            " enters the cells, "<Keys keys="ArrowLeft"/>" returns to the row."
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Code inline=true>"GridFocusMode::Cell"</Code></TableCell>
                        <TableCell>"All arrow keys move between cells; rows never take focus."</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Code inline=true>"CellFocusMode::Cell"</Code></TableCell>
                        <TableCell>"A cell receives focus itself."</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Code inline=true>"CellFocusMode::Child"</Code></TableCell>
                        <TableCell>
                            "A cell passes focus to its first focusable child (its last one when entered from the right), "
                            "if it has one."
                        </TableCell>
                    </TableRow>
                </DocTable>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::Grid.materialize()>"Grid overview"</Link></li>
                <li><Link href=routes::doc::grid::Atom.materialize()>"Grid Atoms"</Link></li>
                <li><Link href=routes::doc::grid_list::Hook.materialize()>"Grid List Hooks"</Link></li>
                <li><Link href=routes::doc::table::Hook.materialize()>"Table Hooks"</Link></li>
                <li><Link href=routes::doc::CollectionState.materialize()>"Collection State"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
