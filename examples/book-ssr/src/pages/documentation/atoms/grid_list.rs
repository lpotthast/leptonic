use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::grid_file_list::GridFileListDemo;
use crate::{kit::*, routes};

#[component]
#[allow(clippy::too_many_lines)]
pub fn PageAtomGridList() -> impl IntoView {
    view! {
        <DocPage title="Grid List Atoms">
            <p>
                "The grid list atoms render an unstyled list of interactive rows with keyboard navigation and selection. "
                "See the "<Link href=routes::doc::GridList.materialize()>"Grid List overview"</Link>" for concept guidance."
            </p>

            <Section title="Hooks Used">
                <DocTable headers=&["Atom", "Hooks"]>
                    <TableRow>
                        <TableCell><Code inline=true>"GridList"</Code></TableCell>
                        <TableCell>
                            <Link href=format!("{}#use-list-state", routes::doc::CollectionState.materialize())>"use_list_state"</Link>
                            " (unless you pass a "<Code inline=true>"state"</Code>"), "
                            <Link href=hook_section("use-grid-list")>"use_grid_list"</Link>
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Code inline=true>"GridListItem"</Code></TableCell>
                        <TableCell><Link href=hook_section("use-grid-list-item")>"use_grid_list_item"</Link></TableCell>
                    </TableRow>
                </DocTable>
            </Section>

            <Section title="Example">
                <p>
                    "Build the rows as a collection and render one "<Code inline=true>"GridListItem"</Code>" per row, in "
                    "collection order:"
                </p>
                <Code language=Language::Rust>
                    {indoc!(r#"
                        use leptonic::{
                            atoms::grid_list::{GridList, GridListItem},
                            hooks::{Key, SelectionMode, use_list_collection},
                        };
                        use leptos::prelude::*;

                        let files = use_list_collection(
                            Signal::stored(vec!["Notes.txt", "Photo.jpg"]),
                            |file| Key::from(*file),
                            |file| (*file).to_owned(),
                        );

                        view! {
                            <GridList collection=files selection_mode=SelectionMode::Multiple aria_label="Files">
                                <GridListItem key="Notes.txt">"Notes.txt"</GridListItem>
                                <GridListItem key="Photo.jpg">"Photo.jpg"</GridListItem>
                            </GridList>
                        }
                    "#)}
                </Code>
            </Section>

            <Section title="Demo">
                <p>
                    "A file list with the "<Code inline=true>"Replace"</Code>" selection behavior: a click selects a file ("
                    <Keys keys="Control"/>" + click, "<Keys keys="Command"/>" + click on macOS, adds to the selection), and a "
                    "double click or "<Keys keys="Enter"/>" opens it. "<Keys keys="ArrowRight"/>" moves to a row\u{2019}s "
                    "download button, "<Keys keys="ArrowLeft"/>" back to the row. Archive.zip is disabled."
                </p>
                <Demo description="File list with selection, row actions and a button per row" source=include_str!("demos/grid_file_list.rs")>
                    <GridFileListDemo/>
                </Demo>
            </Section>

            <Section title="GridList">
                <p>
                    "Creates the list state from "<Code inline=true>"collection"</Code>" and the selection props (or takes "
                    "your "<Code inline=true>"state"</Code>") and renders the "<Code inline=true>"role=\"grid\""</Code>" element."
                </p>
                <Section title="Props" id="gridlist-props">
                    <ApiTable kind=ApiKind::Props of="GridList">
                        <ApiRow name="collection" ty="Option<CollectionMemo>" default="None">"The rows. Required unless "<Code inline=true>"state"</Code>" is given."</ApiRow>
                        <ApiRow name="state" ty="Option<ListState>" default="None">
                            "An existing "<Link href=format!("{}#use-list-state", routes::doc::CollectionState.materialize())>"list state"</Link>
                            ", used instead of the collection and selection props."
                        </ApiRow>
                        <ApiRow name="selection_mode" ty="Signal<SelectionMode>" default="None">
                            <Code inline=true>"None"</Code>", "<Code inline=true>"Single"</Code>" or "<Code inline=true>"Multiple"</Code>" rows."
                        </ApiRow>
                        <ApiRow name="selection_behavior" ty="SelectionBehavior" default="Toggle">
                            <Code inline=true>"Toggle"</Code>": a click toggles the row. "<Code inline=true>"Replace"</Code>
                            ": a click replaces the selection, and keyboard focus selects."
                        </ApiRow>
                        <ApiRow name="default_selected_keys" ty="Vec<Key>" default="vec![]">"The initially selected rows."</ApiRow>
                        <ApiRow name="selection" ty="Option<Signal<Selection>>" default="None">
                            "The selection (controlled), replacing "<Code inline=true>"default_selected_keys"</Code>": a value or any signal."
                        </ApiRow>
                        <ApiRow name="set_selection" ty="Option<Out<Selection>>" default="None">
                            "Receives the new selection: an "<Code inline=true>"RwSignal"</Code>", "<Code inline=true>"WriteSignal"</Code>
                            ", closure or "<Code inline=true>"Callback"</Code>"."
                        </ApiRow>
                        <ApiRow name="on_selection_change" ty="Option<Callback<Selection>>" default="None">"Called with the new selection."</ApiRow>
                        <ApiRow name="disabled_keys" ty="Option<Signal<HashSet<Key>>>" default="None">"Rows that can\u{2019}t be selected, nor (by default) focused."</ApiRow>
                        <ApiRow name="disabled_behavior" ty="DisabledBehavior" default="All">
                            <Code inline=true>"All"</Code>": disabled rows can\u{2019}t be focused or used. "
                            <Code inline=true>"Selection"</Code>": they only can\u{2019}t be selected."
                        </ApiRow>
                        <ApiRow name="disallow_empty_selection" ty="bool" default="false">"Keep at least one row selected."</ApiRow>
                        <ApiRow name="aria_label" ty="MaybeProp<String>" default="None">"Names the grid list."</ApiRow>
                        <ApiRow name="aria_labelledby" ty="Option<String>" default="None">"The id(s) of elements naming the grid list."</ApiRow>
                        <ApiRow name="layout" ty="ListLayout" default="Stack">
                            <Code inline=true>"Stack"</Code>": one row below the other. "<Code inline=true>"Grid"</Code>
                            ": rows wrap like cards, and "<Keys keys="ArrowUp"/>" / "<Keys keys="ArrowDown"/>" find the row in the same column."
                        </ApiRow>
                        <ApiRow name="keyboard_navigation_behavior" ty="KeyboardNavigationBehavior" default="Arrow">
                            <Code inline=true>"Arrow"</Code>": "<Keys keys="ArrowLeft"/>" and "<Keys keys="ArrowRight"/>
                            " move between a row and its focusable children. "<Code inline=true>"Tab"</Code>": "<Keys keys="Tab"/>
                            " moves between the children, leaving the arrow keys to them (e.g. text inputs)."
                        </ApiRow>
                        <ApiRow name="should_focus_wrap" ty="bool" default="false">"Arrow keys wrap around at the ends."</ApiRow>
                        <ApiRow name="auto_focus" ty="Option<AutoFocus>" default="None">
                            "Focus a row when the grid list mounts: the "<Code inline=true>"Selected"</Code>", "
                            <Code inline=true>"First"</Code>" or "<Code inline=true>"Last"</Code>" one."
                        </ApiRow>
                        <ApiRow name="escape_key_behavior" ty="EscapeKeyBehavior" default="ClearSelection">
                            <Code inline=true>"ClearSelection"</Code>": "<Keys keys="Escape"/>" clears the selection. "
                            <Code inline=true>"None"</Code>": the key press bubbles."
                        </ApiRow>
                        <ApiRow name="on_action" ty="Option<Callback<Key>>" default="None">"Called with the key of an activated row."</ApiRow>
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">"Classes and styles of the grid element."</ApiRow>
                        <ApiRow name="children" ty="Children">"The rows: a "<Code inline=true>"GridListItem"</Code>" per row."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="GridListItem">
                <p>
                    "A row: an outer "<Code inline=true>"role=\"row\""</Code>" element with a single "
                    <Code inline=true>"role=\"gridcell\""</Code>" holding the children."
                </p>
                <Section title="Props" id="gridlistitem-props">
                    <ApiTable kind=ApiKind::Props of="GridListItem">
                        <ApiRow name="key" ty="Key">"The row\u{2019}s key in the grid list\u{2019}s collection. Required."</ApiRow>
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">"Classes and styles of the row."</ApiRow>
                        <ApiRow name="children" ty="Children">"The row\u{2019}s content."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="Data Attributes">
                <p>
                    "On "<Code inline=true>"GridListItem"</Code>"\u{2019}s row element. Flags are rendered as "
                    <Code inline=true>"data-selected=\"true\""</Code>" while the state applies."
                </p>
                <ApiTable kind=ApiKind::DataAttributes>
                    <ApiRow name="data-selected" ty="true">"The row is selected."</ApiRow>
                    <ApiRow name="data-focused" ty="true">"The row has focus, by keyboard or pointer."</ApiRow>
                    <ApiRow name="data-focus-visible" ty="true">"The row has keyboard focus, which should be shown."</ApiRow>
                    <ApiRow name="data-pressed" ty="true">"The row is being pressed."</ApiRow>
                    <ApiRow name="data-disabled" ty="true">"The row is disabled."</ApiRow>
                </ApiTable>
            </Section>

            <Section title="Styling">
                <p>
                    "The atoms bring no styles. Pass "<Code inline=true>"classes"</Code>" and target the state with "
                    "attribute selectors. Interactive children style their own focus:"
                </p>
                <Code language=Language::Css>
                    {indoc!(r"
                        .my-row[data-focus-visible] { outline: 2px solid var(--focus); outline-offset: -2px; }
                        .my-row[data-selected] { background: var(--surface); }
                        .my-row[data-disabled] { opacity: 0.5; }
                    ")}
                </Code>
            </Section>

            <Section title="Composition">
                <p>
                    "Put any leptonic button, link or checkbox into a row: the arrow keys reach it, and its presses "
                    "don\u{2019}t select the row. For a selection checkbox per row, use "
                    <Link href=format!("{}#use-grid-selection-checkbox", routes::doc::grid::Hook.materialize())>
                        "use_grid_selection_checkbox"
                    </Link>". To share the selection with other parts of your app, create the "
                    <Link href=format!("{}#use-list-state", routes::doc::CollectionState.materialize())>"list state"</Link>
                    " yourself and pass it as "<Code inline=true>"state"</Code>"."
                </p>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::GridList.materialize()>"Grid List overview"</Link></li>
                <li><Link href=routes::doc::grid_list::Hook.materialize()>"Grid List Hooks"</Link></li>
                <li><Link href=routes::doc::grid::Atom.materialize()>"Grid Atoms"</Link></li>
                <li><Link href=routes::doc::listbox::Atom.materialize()>"Listbox Atoms"</Link></li>
                <li><Link href=routes::doc::CollectionState.materialize()>"Collection State"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}

/// A link target on the grid list hook page.
fn hook_section(id: &str) -> String {
    format!("{}#{id}", routes::doc::grid_list::Hook.materialize())
}
