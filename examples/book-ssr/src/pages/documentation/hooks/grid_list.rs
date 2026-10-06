use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::grid_list::GridListDemo;
use crate::{kit::*, routes};

#[component]
pub fn PageGridListHooks() -> impl IntoView {
    view! {
        <DocPage title="Grid List Hooks">
            <p>
                "The grid list hooks build a list of interactive rows, which may contain buttons, checkboxes or links, "
                "from your own markup. See the "<Link href=routes::doc::GridList.materialize()>"Grid List overview"</Link>
                " for concept guidance and keyboard interaction."
            </p>

            <ReactAria hook="useGridList"/>

            <Section title="Example">
                <p>
                    "Create the rows and their selection as a list state, the grid list, and one "
                    <Code inline=true>"use_grid_list_item"</Code>" per row. Spread the row props onto the row element "
                    "and the cell props onto a single child holding the content:"
                </p>
                <Code language=Language::Rust>
                    {indoc!(r#"
                        use leptonic::{
                            hooks::{*, collections::{SelectionOptions, UseListStateInput}},
                            utils::CapturedElement,
                        };
                        use leptos::prelude::*;

                        let files = use_list_collection(
                            Signal::stored(vec!["Notes.txt", "Photo.jpg"]),
                            |file| Key::from(*file),
                            |file| (*file).to_owned(),
                        );
                        let state = use_list_state(UseListStateInput { collection: files, selection: SelectionOptions::default() });
                        let UseGridListReturn { props, data } = use_grid_list(UseGridListInput {
                            aria_label: "Files".into(),
                            ..UseGridListInput::new(state, CapturedElement::new())
                        });

                        // Per row:
                        let row = use_grid_list_item(UseGridListItemInput::new(data.clone(), Key::from("Notes.txt")));
                        let (row_attrs, row_styles) = row.row_props.into_parts();
                        view! {
                            <div {..row_attrs} style=row_styles>
                                <div {..row.grid_cell_props.into_attrs()}>"Notes.txt"</div>
                            </div>
                        };
                    "#)}
                </Code>
            </Section>

            <Section title="Demo">
                <p>
                    "A file list with the "<Code inline=true>"Replace"</Code>" selection behavior: a click selects a file ("
                    <Keys keys="Control"/>" + click, "<Keys keys="Command"/>" + click on macOS, adds to the selection), and "
                    <Keys keys="Enter"/>" or a double click opens it. "<Keys keys="ArrowRight"/>" moves to a row\u{2019}s "
                    "remove button; removing a file moves the focus to its neighbor."
                </p>

                <Demo description="File list built with use_grid_list, with multiple selection and row actions" source=include_str!("demos/grid_list.rs")>
                    <GridListDemo/>
                </Demo>
            </Section>

            <Section title="use_grid_list">
                <p>
                    "Takes the rows as a list state: build them with "<Code inline=true>"use_list_collection"</Code>" or "
                    <Code inline=true>"use_collection"</Code>" and hold their selection with "
                    <Link href=format!("{}#use-list-state", routes::doc::CollectionState.materialize())>"use_list_state"</Link>
                    ". Spread "<Code inline=true>"props"</Code>" onto the list element and render each row with "
                    <Code inline=true>"use_grid_list_item"</Code>"."
                </p>

                <Section title="Input" id="use-grid-list-input">
                    <p>"Create the input with "<Code inline=true>"UseGridListInput::new(state, element)"</Code>"."</p>
                    <ApiTable kind=ApiKind::Input of="UseGridListInput">
                        <ApiRow name="state" ty="ListState">"The rows and their selection, from "<Code inline=true>"use_list_state"</Code>". Required."</ApiRow>
                        <ApiRow name="element" ty="CapturedElement">"The grid element; the props capture it. Required."</ApiRow>
                        <ApiRow name="id" ty="Option<String>" default="None">"The element id (row ids derive from it), generated when "<Code inline=true>"None"</Code>"."</ApiRow>
                        <ApiRow name="aria_label" ty="MaybeProp<String>" default="None">"Names the grid list."</ApiRow>
                        <ApiRow name="aria_labelledby" ty="Signal<Option<String>>" default="None">"The id(s) of elements naming the grid list."</ApiRow>
                        <ApiRow name="layout" ty="ListLayout" default="Stack">
                            <Code inline=true>"Stack"</Code>": one row below the other. "<Code inline=true>"Grid"</Code>
                            ": rows wrap like cards, and "<Keys keys="ArrowUp"/>" / "<Keys keys="ArrowDown"/>" find the row in the same column."
                        </ApiRow>
                        <ApiRow name="keyboard_navigation_behavior" ty="KeyboardNavigationBehavior" default="Arrow">
                            <Code inline=true>"Arrow"</Code>": "<Keys keys="ArrowLeft"/>" and "<Keys keys="ArrowRight"/>
                            " move between a row and its focusable children. "<Code inline=true>"Tab"</Code>": "<Keys keys="Tab"/>
                            " moves between the children, leaving the arrow keys to them (e.g. text inputs)."
                        </ApiRow>
                        <ApiRow name="options" ty="CollectionOptions" default="CollectionOptions::default()">
                            "Keyboard and focus behavior, see "<Link href=format!("{}#collectionoptions", routes::doc::CollectionState.materialize())>"CollectionOptions"</Link>"."
                        </ApiRow>
                        <ApiRow name="keyboard_delegate" ty="Option<Signal<Arc<dyn KeyboardDelegate>>>" default="None">
                            "Replaces the "<Link href=format!("{}#use-list-keyboard-delegate", routes::doc::CollectionState.materialize())>"list keyboard delegate"</Link>"."
                        </ApiRow>
                        <ApiRow name="should_select_on_press_up" ty="bool" default="false">"Select when the press ends instead of when it starts."</ApiRow>
                        <ApiRow name="on_action" ty="Option<Callback<Key>>" default="None">
                            "Called with the key of an activated row: pressed without selection, double-clicked, or "<Keys keys="Enter"/>" with the "
                            <Code inline=true>"Replace"</Code>" selection behavior."
                        </ApiRow>
                        <ApiRow name="tree" ty="Option<TreeExpansion>" default="None">
                            "Makes the rows tree items that expand and collapse. "
                            <Link href=routes::doc::Tree.materialize()>"use_tree"</Link>" sets it."
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
                <p>"A row with a single cell holding its content."</p>
                <Section title="Input" id="use-grid-list-item-input">
                    <p>"Create the input with "<Code inline=true>"UseGridListItemInput::new(data, key)"</Code>"."</p>
                    <ApiTable kind=ApiKind::Input of="UseGridListItemInput">
                        <ApiRow name="list" ty="GridListData">"The grid list, from "<Code inline=true>"use_grid_list"</Code>". Required."</ApiRow>
                        <ApiRow name="key" ty="Key">"The row\u{2019}s key in the collection. Required."</ApiRow>
                        <ApiRow name="focus_mode" ty="FocusMode" default="Row">
                            "What receives focus: the row itself, or its first focusable child (e.g. the remove button of a tag)."
                        </ApiRow>
                        <ApiRow name="allows_arrow_navigation" ty="bool" default="false">
                            "Let "<Keys keys="ArrowLeft"/>" and "<Keys keys="ArrowRight"/>" move between the row\u{2019}s children even with "
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
                    "Groups the rows of a collection section: a row holding the section\u{2019}s header, and a row group "
                    "with its rows, labelled by the header (or the section\u{2019}s "<Code inline=true>"aria_label"</Code>")."
                </p>
                <Section title="Input" id="use-grid-list-section-input">
                    <ApiTable kind=ApiKind::Input of="UseGridListSectionInput">
                        <ApiRow name="list" ty="GridListData">"The grid list, from "<Code inline=true>"use_grid_list"</Code>". Required."</ApiRow>
                        <ApiRow name="key" ty="Key">"The section\u{2019}s key in the collection. Required."</ApiRow>
                    </ApiTable>
                </Section>
                <Section title="Return" id="use-grid-list-section-return">
                    <ApiTable kind=ApiKind::Return of="UseGridListSectionReturn">
                        <ApiRow name="row_props" ty="UseGridListSectionRowProps">"For the row holding the header ("<Code inline=true>"role=\"row\""</Code>")."</ApiRow>
                        <ApiRow name="row_header_props" ty="UseGridListSectionRowHeaderProps">
                            "For the header cell inside it ("<Code inline=true>"role=\"rowheader\""</Code>", with the id labelling the group)."
                        </ApiRow>
                        <ApiRow name="row_group_props" ty="UseGridListSectionRowGroupProps">
                            "For the element containing the section\u{2019}s rows ("<Code inline=true>"role=\"rowgroup\""</Code>")."
                        </ApiRow>
                        <ApiRow name="heading" ty="Option<String>">"The header text, if the section has a header."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::GridList.materialize()>"Grid List overview"</Link></li>
                <li><Link href=routes::doc::grid_list::Atom.materialize()>"Grid List Atoms"</Link></li>
                <li><Link href=routes::doc::grid::Hook.materialize()>"Grid Hooks"</Link></li>
                <li><Link href=routes::doc::TagGroup.materialize()>"Tag Group Hooks"</Link></li>
                <li><Link href=routes::doc::Tree.materialize()>"Tree Hooks"</Link></li>
                <li><Link href=routes::doc::CollectionState.materialize()>"Collection State"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
