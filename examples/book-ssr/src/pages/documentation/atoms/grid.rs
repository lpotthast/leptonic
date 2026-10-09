use indoc::indoc;
use leptos::prelude::*;

use super::demos::grid_messages::GridMessagesDemo;
use crate::{kit::*, routes};

#[component]
#[allow(clippy::too_many_lines)]
pub fn PageAtomGrid() -> impl IntoView {
    view! {
        <DocPage title="Grid Atoms">
            <p>
                "The grid atoms render an unstyled grid of rows and cells with keyboard navigation and row selection. "
                "See the "<Link href=routes::doc::Grid.materialize()>"Grid overview"</Link>" for concept guidance."
            </p>

            <Section title="Hooks Used">
                <DocTable headers=&["Atom", "Hooks"]>
                    <TableRow>
                        <TableCell><Code inline=true>"Grid"</Code></TableCell>
                        <TableCell>
                            <Link href=hook_section("use-grid-state")>"use_grid_state"</Link>", "
                            <Link href=hook_section("use-grid")>"use_grid"</Link>
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Code inline=true>"GridRowGroup"</Code></TableCell>
                        <TableCell><Link href=hook_section("use-grid-row-group")>"use_grid_row_group"</Link></TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Code inline=true>"GridRow"</Code></TableCell>
                        <TableCell><Link href=hook_section("use-grid-row")>"use_grid_row"</Link>", "<Link href=routes::doc::focus::UseFocusVisible.materialize()>"use_focus_visible"</Link></TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Code inline=true>"GridCell"</Code></TableCell>
                        <TableCell>
                            <Link href=hook_section("use-grid-cell")>"use_grid_cell"</Link>", "
                            <Link href=routes::doc::focus::UseFocusRing.materialize()>"use_focus_ring"</Link>
                        </TableCell>
                    </TableRow>
                </DocTable>
            </Section>

            <Section title="Example">
                <p>
                    "Build the rows and cells as a collection with "<Code inline=true>"b.row(key, text, |r| r.cell(..))"</Code>
                    " and render one "<Code inline=true>"GridRow"</Code>" per row and one "<Code inline=true>"GridCell"</Code>
                    " per cell, in collection order. Cell keys derive from their row: "
                    <Code inline=true>"Key::cell(&row, column)"</Code>"."
                </p>
                <Code language=Language::Rust>
                    {indoc!(r#"
                        use leptonic::{
                            atoms::grid::{Grid, GridCell, GridRow},
                            hooks::collections::{Key, use_collection},
                        };
                        use leptos::prelude::*;

                        let users = use_collection(|b| {
                            b.row("ada", "Ada", |r| { r.cell("Ada"); r.cell("Admin"); });
                        });
                        let ada = Key::from("ada");

                        view! {
                            <Grid collection=users aria_label="Users">
                                <GridRow key=ada.clone()>
                                    <GridCell key=Key::cell(&ada, 0)>"Ada"</GridCell>
                                    <GridCell key=Key::cell(&ada, 1)>"Admin"</GridCell>
                                </GridRow>
                            </Grid>
                        }
                    "#)}
                </Code>
            </Section>

            <Section title="Demo">
                <p>
                    "A message list with the "<Code inline=true>"Replace"</Code>" selection behavior, like a file manager: "
                    "a click selects a row ("<Keys keys="Control"/>" + click, "<Keys keys="Meta"/>" + click on macOS, "
                    "adds to the selection), "<Keys keys="Escape"/>" clears it, and a double click or "<Keys keys="Enter"/>

                    " opens a message. "<Keys keys="ArrowRight"/>" moves into a row\u{2019}s cells. The message from Alan "
                    "is disabled."
                </p>
                <Demo description="Message list grid with row selection, a disabled row and row actions" source=include_str!("demos/grid_messages.rs")>
                    <GridMessagesDemo/>
                </Demo>
            </Section>

            <Section title="Grid">
                <p>
                    "Creates the grid state and renders the "<Code inline=true>"role=\"grid\""</Code>" element around the rows."
                </p>
                <Section title="Props" id="grid-props">
                    <ApiTable kind=ApiKind::Props of="atoms::grid::Grid">
                        <ApiRow name="collection" ty="CollectionMemo">"The rows and their cells. Required."</ApiRow>
                        <ApiRow name="focus_mode" ty="GridFocusMode" default="Row">
                            <Code inline=true>"Row"</Code>": the arrow keys move between rows, and "<Keys keys="ArrowRight"/>
                            " into a row\u{2019}s cells. "<Code inline=true>"Cell"</Code>": they only move between cells."
                        </ApiRow>
                        <ApiRow name="selection_mode" ty="Signal<SelectionMode>" default="None">
                            <Code inline=true>"None"</Code>", "<Code inline=true>"Single"</Code>" or "<Code inline=true>"Multiple"</Code>" rows."
                        </ApiRow>
                        <ApiRow name="selection_behavior" ty="Signal<SelectionBehavior>" default="Toggle">
                            <Code inline=true>"Toggle"</Code>": a click toggles the row. "<Code inline=true>"Replace"</Code>
                            ": a click replaces the selection, and keyboard focus selects."
                        </ApiRow>
                        <ApiRow name="default_selection" ty="Selection" default="empty">"The initially selected rows."</ApiRow>
                        <ApiRow name="selection" ty="Option<Signal<Selection>>" default="None">
                            "The selection (controlled), replacing "<Code inline=true>"default_selection"</Code>": a value or any signal."
                        </ApiRow>
                        <ApiRow name="set_selection" ty="Option<Out<Selection>>" default="None">
                            "Receives the new selection: an "<Code inline=true>"RwSignal"</Code>", "<Code inline=true>"WriteSignal"</Code>
                            ", closure or "<Code inline=true>"Callback"</Code>"."
                        </ApiRow>
                        <ApiRow name="on_selection_change" ty="Option<Callback<Selection>>" default="None">"Called with the new selection."</ApiRow>
                        <ApiRow name="disabled_keys" ty="Option<Signal<HashSet<Key>>>" default="None">
                            "Rows that can\u{2019}t be selected, nor (with "<Code inline=true>"disabled_behavior"</Code>
                            " "<Code inline=true>"All"</Code>") focused."
                        </ApiRow>
                        <ApiRow name="disabled_behavior" ty="DisabledBehavior" default="All">
                            <Code inline=true>"All"</Code>": disabled rows can\u{2019}t be focused or used. "
                            <Code inline=true>"Selection"</Code>": they only can\u{2019}t be selected."
                        </ApiRow>
                        <ApiRow name="disallow_empty_selection" ty="Signal<bool>" default="false">"Keep at least one row selected."</ApiRow>
                        <ApiRow name="escape_key_behavior" ty="EscapeKeyBehavior" default="ClearSelection">
                            <Code inline=true>"ClearSelection"</Code>": "<Keys keys="Escape"/>" clears the selection. "
                            <Code inline=true>"None"</Code>": the key press bubbles, e.g. to close a dialog."
                        </ApiRow>
                        <ApiRow name="should_focus_wrap" ty="bool" default="false">"Arrow keys wrap around at the ends."</ApiRow>
                        <ApiRow name="keyboard_navigation_behavior" ty="KeyboardNavigationBehavior" default="Arrow">
                            <Code inline=true>"Arrow"</Code>": "<Keys keys="ArrowLeft"/>" and "<Keys keys="ArrowRight"/>
                            " move between the focusable children of cells. "<Code inline=true>"Tab"</Code>": "
                            <Keys keys="Tab"/>" moves between them, and the arrow keys stay with them (e.g. text inputs)."
                        </ApiRow>
                        <ApiRow name="on_row_action, on_cell_action" ty="Option<Callback<Key>>" default="None">
                            "Called with the key of an activated row or cell."
                        </ApiRow>
                        <ApiRow name="aria_label, aria_labelledby" ty="MaybeProp<String>" default="None">
                            "Names the grid. One of them is needed."
                        </ApiRow>
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">"Classes and styles of the grid element."</ApiRow>
                        <ApiRow name="children" ty="Children">"The rows, directly or in "<Code inline=true>"GridRowGroup"</Code>"s."</ApiRow>
                        <ApiRow name="should_select_on_press_up" ty="bool" default="false">"Select when the press ends instead of when it starts."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="GridRowGroup">
                <p>"Groups rows ("<Code inline=true>"role=\"rowgroup\""</Code>"), like "<Code inline=true>"<tbody>"</Code>"."</p>
                <Section title="Props" id="gridrowgroup-props">
                    <ApiTable kind=ApiKind::Props of="GridRowGroup">
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">"Classes and styles of the group."</ApiRow>
                        <ApiRow name="children" ty="Children">"The rows."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="GridRow">
                <p>"A row ("<Code inline=true>"role=\"row\""</Code>"), selected and activated by press."</p>
                <Section title="Props" id="gridrow-props">
                    <ApiTable kind=ApiKind::Props of="GridRow">
                        <ApiRow name="key" ty="Key">"The row\u{2019}s key in the collection. Required."</ApiRow>
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">"Classes and styles of the row."</ApiRow>
                        <ApiRow name="children" ty="Children">"The row\u{2019}s cells."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="GridCell">
                <p>"A cell ("<Code inline=true>"role=\"gridcell\""</Code>"). Presses on a cell select its row, unless the grid has "<Code inline=true>"on_cell_action"</Code>"."</p>
                <Section title="Props" id="gridcell-props">
                    <ApiTable kind=ApiKind::Props of="GridCell">
                        <ApiRow name="key" ty="Key">"The cell\u{2019}s key: "<Code inline=true>"Key::cell(&row, column)"</Code>". Required."</ApiRow>
                        <ApiRow name="focus_mode" ty="Option<CellFocusMode>" default="None">
                            <Code inline=true>"Cell"</Code>": the cell takes focus. "<Code inline=true>"Child"</Code>
                            ": its first focusable child does. "<Code inline=true>"None"</Code>": "<Code inline=true>"Cell"</Code>
                            " with the "<Code inline=true>"Tab"</Code>" navigation behavior, else "<Code inline=true>"Child"</Code>"."
                        </ApiRow>
                        <ApiRow name="allows_arrow_navigation" ty="bool" default="false">
                            "Lets "<Keys keys="ArrowLeft"/>" and "<Keys keys="ArrowRight"/>" move between the cell\u{2019}s children (and "
                            <Keys keys="ArrowUp"/>" and "<Keys keys="ArrowDown"/>" between rows) even with "
                            <Code inline=true>"KeyboardNavigationBehavior::Tab"</Code>"."
                        </ApiRow>
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">"Classes and styles of the cell."</ApiRow>
                        <ApiRow name="children" ty="Children">"The cell\u{2019}s content."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="Data Attributes">
                <p>"Flags are rendered as "<Code inline=true>"data-selected=\"true\""</Code>" while the state applies."</p>
                <ApiTable kind=ApiKind::DataAttributes>
                    <ApiRow name="data-selected" ty="true">"On "<Code inline=true>"GridRow"</Code>": the row is selected."</ApiRow>
                    <ApiRow name="data-focused" ty="true">
                        "On "<Code inline=true>"GridRow"</Code>" and "<Code inline=true>"GridCell"</Code>": it has focus, by "
                        "keyboard or pointer."
                    </ApiRow>
                    <ApiRow name="data-focus-visible" ty="true">
                        "On "<Code inline=true>"GridRow"</Code>" and "<Code inline=true>"GridCell"</Code>": it has keyboard "
                        "focus, which should be shown."
                    </ApiRow>
                    <ApiRow name="data-pressed" ty="true">"On "<Code inline=true>"GridRow"</Code>" and "<Code inline=true>"GridCell"</Code>": being pressed."</ApiRow>
                    <ApiRow name="data-hovered" ty="true">"On "<Code inline=true>"GridRow"</Code>": a pointer is over the row."</ApiRow>
                    <ApiRow name="data-disabled" ty="true">"On "<Code inline=true>"GridRow"</Code>": the row is disabled."</ApiRow>
                </ApiTable>
            </Section>

            <Section title="Styling">
                <p>
                    "The atoms bring no styles. Each renders a "<Code inline=true>"<div>"</Code>" with its default class ("<Code inline=true>"leptonic-Grid"</Code>", "
                    <Code inline=true>"leptonic-GridRowGroup"</Code>", "<Code inline=true>"leptonic-GridRow"</Code>", "<Code inline=true>"leptonic-GridCell"</Code>"), followed by the "
                    <Code inline=true>"classes"</Code>" you pass; the cells hold your content. Lay the grid out yourself (as a CSS table, as in the demo, "
                    "or as a CSS grid) and target the state with the data attributes. Keyboard focus is on a row or, after "
                    <Keys keys="ArrowRight"/>", on a cell, so give both a focus ring. The demo above uses this CSS:"
                </p>
                <Code language=Language::Css>
                    {indoc!(r"
                        .demo-messages { display: table; width: 100%; max-width: 32em; border-collapse: collapse; }
                        .demo-messages > .leptonic-GridRowGroup { display: table-row-group; }
                        .demo-messages-row { display: table-row; cursor: pointer; }
                        .demo-messages-row[data-selected] { background: var(--surface); }
                        .demo-messages-row[data-pressed] { background: var(--border); }
                        .demo-messages-row[data-disabled] { opacity: 0.5; cursor: not-allowed; }
                        .demo-messages-cell { display: table-cell; padding: 0.5rem 1rem; border-bottom: 1px solid var(--border); }
                        .demo-messages-row[data-focus-visible],
                        .demo-messages-cell[data-focus-visible] { outline: 2px solid var(--focus); outline-offset: -2px; }
                    ")}
                </Code>
                <p>
                    "Apps that don\u{2019}t want to style from scratch can load leptonic\u{2019}s optional atom theme, "
                    <Code inline=true>"@use \"leptonic/leptonic-atoms\";"</Code>", which styles the default classes."
                </p>
            </Section>

            <Section title="Composition">
                <p>
                    "The rows and cells find their grid through the "<Code inline=true>"GridData"</Code>" context that "
                    <Code inline=true>"Grid"</Code>" provides, so they may sit in any markup inside it. Interactive content "
                    "in a cell (a button, a link) is reached with "<Keys keys="ArrowRight"/>" from the cell, see "
                    <Code inline=true>"focus_mode"</Code>". For a selection checkbox per row, use "
                    <Link href=hook_section("use-grid-selection-checkbox")>"use_grid_selection_checkbox"</Link>
                    " with a "<Link href=routes::doc::checkbox::Atom.materialize()>"checkbox"</Link>"."
                </p>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::Grid.materialize()>"Grid overview"</Link></li>
                <li><Link href=routes::doc::grid::Hook.materialize()>"Grid Hooks"</Link></li>
                <li><Link href=routes::doc::grid_list::Atom.materialize()>"Grid List Atoms"</Link></li>
                <li><Link href=routes::doc::table::Atom.materialize()>"Table Atoms"</Link></li>
                <li><Link href=routes::doc::CollectionState.materialize()>"Collection State"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}

/// A link target on the grid hook page.
fn hook_section(id: &str) -> String {
    format!("{}#{id}", routes::doc::grid::Hook.materialize())
}
