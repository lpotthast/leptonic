use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::{grid_file_list::GridFileListDemo, grid_messages::GridMessagesDemo};
use crate::{kit::*, routes};

#[component]
pub fn PageAtomGrid() -> impl IntoView {
    view! {
        <DocPage title="Grid Atoms">
            <p>
                "The grid atoms wrap the "<Link href=routes::doc::grid::Hook.materialize()>"grid hooks"</Link>
                " into composable components. Two patterns are supported:"
            </p>
            <ul>
                <li>
                    <strong>"2D Grid"</strong>" \u{2014} "<Code inline=true>"Grid > GridRowGroup > GridRow > GridCell"</Code>
                    " for rows with several cells (message lists, data grids, calendars)"
                </li>
                <li>
                    <strong>"1D Grid List"</strong>" \u{2014} "<Code inline=true>"GridList > GridListItem"</Code>
                    " for single-column lists with grid keyboard semantics (file lists, card galleries)"
                </li>
            </ul>
            <p>
                "Both provide keyboard navigation, row selection and ARIA semantics. Style them through the "
                <Code inline=true>"data-selected"</Code>", "<Code inline=true>"data-focused"</Code>", "
                <Code inline=true>"data-disabled"</Code>" and "<Code inline=true>"data-pressed"</Code>" attributes of rows and list items."
            </p>

            <Section title="2D Grid">
                <p>
                    "A message list with replace selection, like a file manager: a click selects a row ("<Keys keys="Control"/>
                    " + click adds to the selection), Escape clears it, a double click or Enter opens a message. \u{201c}Alan\u{201d} is disabled. Build the rows and cells as a "
                    "collection with "<Code inline=true>"Collection::build"</Code>" and "<Code inline=true>"b.row(key, text, |r| r.cell(..))"</Code>
                    "; cell keys derive from their row: "<Code inline=true>"Key::cell(&row, column)"</Code>"."
                </p>

                <Demo description="Message list grid with row selection, a disabled row and row actions" source=include_str!("demos/grid_messages.rs")>
                    <GridMessagesDemo/>
                </Demo>
            </Section>

            <Section title="Grid List">
                <p>
                    "A file list with replace selection: a click selects a file ("<Keys keys="Control"/>" + click adds to the "
                    "selection), arrow keys move, a double click or Enter opens the file."
                </p>

                <Demo description="File list with selection and row actions" source=include_str!("demos/grid_file_list.rs")>
                    <GridFileListDemo/>
                </Demo>
            </Section>

            <Section title="Data Attributes">
                <p>
                    "All row, cell, and list item atoms expose data attributes for CSS styling. This is the key advantage of atoms "
                    "over raw hooks \u{2014} you can target these attributes with standard CSS selectors."
                </p>

                <Code language=Language::Css>
                    {indoc!(r"
                        /* Focused indicator — visible outline */
                        [data-focused] {
                            outline: 3px solid #1976d2;
                            outline-offset: 2px;
                        }

                        /* Selected indicator — background highlight */
                        [data-selected] {
                            background-color: rgba(25, 118, 210, 0.15);
                        }

                        /* Disabled items — faded out */
                        [data-disabled] {
                            opacity: 0.4;
                            cursor: not-allowed;
                        }
                    ")}
                </Code>
            </Section>

            <Section title="Selection Modes">
                <ul>
                    <li><Code inline=true>"SelectionMode::None"</Code>" \u{2014} Focus only, no selection"</li>
                    <li><Code inline=true>"SelectionMode::Single"</Code>" \u{2014} One row at a time"</li>
                    <li>
                        <Code inline=true>"SelectionMode::Multiple"</Code>
                        " \u{2014} Several rows (Shift + arrow keys extend the selection, Ctrl + A selects all)"
                    </li>
                </ul>

                <p>"Combined with "<Code inline=true>"SelectionBehavior"</Code>":"</p>
                <ul>
                    <li><Code inline=true>"SelectionBehavior::Toggle"</Code>" \u{2014} A click toggles the row"</li>
                    <li><Code inline=true>"SelectionBehavior::Replace"</Code>" \u{2014} A click replaces the selection (hold Ctrl to toggle)"</li>
                </ul>

                <p>
                    "See the "<Link href=routes::doc::grid::Hook.materialize()>"hooks page"</Link>
                    " for a detailed reference on keyboard navigation and ARIA attributes."
                </p>
            </Section>

            <Section title="API Reference">
                <Section title="Grid">
                    <ApiTable kind=ApiKind::Props of="atoms::grid::Grid">
                        <ApiRow name="collection" ty="CollectionMemo">"The rows and their cells."</ApiRow>
                        <ApiRow name="focus_mode" ty="GridFocusMode" default="Row">
                            <Code inline=true>"Row"</Code>": arrow keys move between rows first. "<Code inline=true>"Cell"</Code>
                            ": they move between cells."
                        </ApiRow>
                        <ApiRow name="selection_mode" ty="Signal<SelectionMode>" default="None">"No selection when "<Code inline=true>"None"</Code>"."</ApiRow>
                        <ApiRow name="selection_behavior" ty="SelectionBehavior" default="Toggle">"Whether clicks toggle rows or replace the selection."</ApiRow>
                        <ApiRow name="default_selected_keys" ty="Vec<Key>" default="empty">"The initially selected rows."</ApiRow>
                        <ApiRow name="selection" ty="Option<ValueBinding<Selection>>" default="None">
                            "The selection as app state (e.g. an "<Code inline=true>"RwSignal<Selection>"</Code>"), replacing "<Code inline=true>"default_selected_keys"</Code>"."
                        </ApiRow>
                        <ApiRow name="on_selection_change" ty="Option<Callback<Selection>>" default="None">"Called with the new selection."</ApiRow>
                        <ApiRow name="disabled_keys" ty="Option<Signal<HashSet<Key>>>" default="None">"Rows that can\u{2019}t be selected (or focused)."</ApiRow>
                        <ApiRow name="disabled_behavior" ty="DisabledBehavior" default="All">"What disabling a row prevents."</ApiRow>
                        <ApiRow name="disallow_empty_selection" ty="bool" default="false">"Keep at least one row selected."</ApiRow>
                        <ApiRow name="escape_key_behavior" ty="EscapeKeyBehavior" default="ClearSelection">"What Escape does."</ApiRow>
                        <ApiRow name="should_focus_wrap" ty="bool" default="false">"Arrow keys wrap around at the ends."</ApiRow>
                        <ApiRow name="keyboard_navigation_behavior" ty="KeyboardNavigationBehavior" default="Arrow">"How the keyboard reaches interactive children of cells."</ApiRow>
                        <ApiRow name="on_row_action, on_cell_action" ty="Option<Callback<Key>>" default="None">"Called with the key of an activated row or cell."</ApiRow>
                        <ApiRow name="aria_label, aria_labelledby" ty="MaybeProp<String>, Option<String>" default="None">"Names the grid."</ApiRow>
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">"Classes and styles of the grid element."</ApiRow>
                        <ApiRow name="children" ty="Children">"The rows, directly or in "<Code inline=true>"GridRowGroup"</Code>"s."</ApiRow>
                    </ApiTable>
                </Section>

                <Section title="GridRowGroup">
                    <p>"Groups rows ("<Code inline=true>"role=\"rowgroup\""</Code>"). Props: "<Code inline=true>"classes"</Code>", "<Code inline=true>"styles"</Code>" and the rows as "<Code inline=true>"children"</Code>"."</p>
                </Section>

                <Section title="GridRow">
                    <ApiTable kind=ApiKind::Props of="GridRow">
                        <ApiRow name="key" ty="Key">"The row\u{2019}s key in the collection."</ApiRow>
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">"Classes and styles of the row."</ApiRow>
                        <ApiRow name="children" ty="Children">"The row\u{2019}s cells."</ApiRow>
                    </ApiTable>
                    <p>
                        "The row exposes "<Code inline=true>"data-selected"</Code>", "<Code inline=true>"data-focused"</Code>", "
                        <Code inline=true>"data-disabled"</Code>" and "<Code inline=true>"data-pressed"</Code>"."
                    </p>
                </Section>

                <Section title="GridCell">
                    <ApiTable kind=ApiKind::Props of="GridCell">
                        <ApiRow name="key" ty="Key">"The cell\u{2019}s key: "<Code inline=true>"Key::cell(&row_key, column)"</Code>"."</ApiRow>
                        <ApiRow name="focus_mode" ty="Option<CellFocusMode>" default="None">
                            <Code inline=true>"Cell"</Code>": focus the cell. "<Code inline=true>"Child"</Code>": focus its first focusable child."
                        </ApiRow>
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">"Classes and styles of the cell."</ApiRow>
                        <ApiRow name="children" ty="Children">"The cell\u{2019}s content."</ApiRow>
                    </ApiTable>
                    <p>"The cell exposes "<Code inline=true>"data-pressed"</Code>"; selection and focus state belong to its row."</p>
                </Section>

                <Section title="GridList">
                    <p>
                        "Renders a grid list built with "<Link href=routes::doc::grid::Hook.materialize()>"use_grid_list"</Link>
                        ". Pass the rows as a "<Code inline=true>"collection"</Code>" (e.g. from "
                        <Code inline=true>"use_list_collection"</Code>") and render a "<Code inline=true>"GridListItem"</Code>
                        " per row."
                    </p>
                    <ApiTable kind=ApiKind::Props of="GridList">
                        <ApiRow name="collection" ty="Option<CollectionMemo>" default="None">"The rows. Required unless "<Code inline=true>"state"</Code>" is given."</ApiRow>
                        <ApiRow name="state" ty="Option<ListState>" default="None">"An existing list state, instead of the collection and selection props."</ApiRow>
                        <ApiRow name="selection_mode" ty="Signal<SelectionMode>" default="None">"No selection when "<Code inline=true>"None"</Code>"."</ApiRow>
                        <ApiRow name="selection_behavior" ty="SelectionBehavior" default="Toggle">"Whether clicks toggle rows or replace the selection."</ApiRow>
                        <ApiRow name="default_selected_keys" ty="Vec<Key>" default="empty">"The initially selected rows."</ApiRow>
                        <ApiRow name="selection" ty="Option<ValueBinding<Selection>>" default="None">
                            "The selection as app state (e.g. an "<Code inline=true>"RwSignal<Selection>"</Code>"), replacing "<Code inline=true>"default_selected_keys"</Code>"."
                        </ApiRow>
                        <ApiRow name="on_selection_change" ty="Option<Callback<Selection>>" default="None">"Called with the new selection."</ApiRow>
                        <ApiRow name="disabled_keys" ty="Option<Signal<HashSet<Key>>>" default="None">"Rows that can\u{2019}t be focused or selected."</ApiRow>
                        <ApiRow name="disabled_behavior" ty="DisabledBehavior" default="All">"What disabling a row prevents."</ApiRow>
                        <ApiRow name="disallow_empty_selection" ty="bool" default="false">"Keep at least one row selected."</ApiRow>
                        <ApiRow name="aria_label" ty="MaybeProp<String>" default="None">"Names the grid list."</ApiRow>
                        <ApiRow name="aria_labelledby" ty="Option<String>" default="None">"The id(s) of elements naming the grid list."</ApiRow>
                        <ApiRow name="layout" ty="ListLayout" default="Stack">"Stacked rows, or rows wrapping like cards."</ApiRow>
                        <ApiRow name="keyboard_navigation_behavior" ty="KeyboardNavigationBehavior" default="Arrow">"How the keyboard reaches the rows\u{2019} children."</ApiRow>
                        <ApiRow name="should_focus_wrap" ty="bool" default="false">"Arrow keys wrap around at the ends."</ApiRow>
                        <ApiRow name="auto_focus" ty="Option<AutoFocus>" default="None">"Focus a row when the grid list mounts."</ApiRow>
                        <ApiRow name="escape_key_behavior" ty="EscapeKeyBehavior" default="ClearSelection">"What Escape does."</ApiRow>
                        <ApiRow name="on_action" ty="Option<Callback<Key>>" default="None">"Called with the key of an activated row."</ApiRow>
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">"Classes and styles of the grid element."</ApiRow>
                        <ApiRow name="children" ty="Children">"The rows: a "<Code inline=true>"GridListItem"</Code>" per row."</ApiRow>
                    </ApiTable>
                </Section>

                <Section title="GridListItem">
                    <ApiTable kind=ApiKind::Props of="GridListItem">
                        <ApiRow name="key" ty="Key">"The row\u{2019}s key in the grid list\u{2019}s collection."</ApiRow>
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">"Classes and styles of the row."</ApiRow>
                        <ApiRow name="children" ty="Children">"The row\u{2019}s content."</ApiRow>
                    </ApiTable>
                    <p>
                        "The row exposes "<Code inline=true>"data-selected"</Code>", "<Code inline=true>"data-focused"</Code>", "
                        <Code inline=true>"data-focus-visible"</Code>", "<Code inline=true>"data-disabled"</Code>" and "
                        <Code inline=true>"data-pressed"</Code>" while the state applies."
                    </p>
                </Section>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::Grid.materialize()>"Grid overview"</Link></li>
                <li><Link href=routes::doc::grid::Hook.materialize()>"Grid hooks"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
