use indoc::indoc;
use leptos::prelude::*;

use super::demos::tree::TreeDemo;
use crate::{kit::*, routes};

#[component]
pub fn PageUseTree() -> impl IntoView {
    view! {
        <DocPage title="Tree Hooks">
            <p>
                "A tree shows hierarchical items, like files and folders, whose children can be expanded and collapsed. "
                <Code inline=true>"use_tree_state"</Code>", "<Code inline=true>"use_tree"</Code>" and "
                <Code inline=true>"use_tree_item"</Code>" build it as a "<Code inline=true>"treegrid"</Code>": a "
                <Link href=routes::doc::GridList.materialize()>"grid list"</Link>" whose rows have levels."
            </p>

            <ReactAria hook="useTree"/>

            <Section title="Demo">
                <p>
                    <Keys keys="ArrowUp"/>" and "<Keys keys="ArrowDown"/>" move between the visible items, "
                    <Keys keys="ArrowRight"/>" expands a folder, "<Keys keys="ArrowLeft"/>" collapses it or moves to its "
                    "parent. "<Keys keys="Space"/>" or a click selects an item."
                </p>
                <Demo description="File tree with expandable folders and single selection" source=include_str!("demos/tree.rs")>
                    <TreeDemo/>
                </Demo>
            </Section>

            <Section title="use_tree_state">
                <p>
                    "Build the tree as a collection of nested items (see "<Link href=routes::doc::CollectionState.materialize()>"Collection State"</Link>
                    "). The state holds the selection and which items are expanded; its list shows the visible items, in order."
                </p>

                <Section title="Input" id="use-tree-state-input">
                    <p>"Pass a "<Code inline=true>"UseTreeStateInput"</Code>" with every field named; the Default column gives the value for fields you don\u{2019}t need."</p>
                    <ApiTable kind=ApiKind::Input of="UseTreeStateInput">
                        <ApiRow name="collection" ty="CollectionMemo">"The items, nested with "<Code inline=true>".children(..)"</Code>". Required."</ApiRow>
                        <ApiRow name="selection" ty="SelectionOptions" default="SelectionOptions::default()">
                            "Selection, see "<Link href=format!("{}#selectionoptions", routes::doc::CollectionState.materialize())>"SelectionOptions"</Link>"."
                        </ApiRow>
                        <ApiRow name="default_expanded_keys" ty="HashSet<Key>" default="empty">"The initially expanded items."</ApiRow>
                        <ApiRow name="on_expanded_change" ty="Option<Callback<HashSet<Key>>>" default="None">"Called with the expanded keys whenever they change."</ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Return" id="use-tree-state-return">
                    <ApiTable kind=ApiKind::Return of="TreeState">
                        <ApiRow name="list" ty="ListState">"The visible items, their selection and focus."</ApiRow>
                        <ApiRow name="expansion" ty="TreeExpansion">
                            "Which items are expanded: "<Code inline=true>"is_expanded(&key)"</Code>", "<Code inline=true>"toggle_key(key)"</Code>" and "
                            "the field "<Code inline=true>"expanded_keys"</Code>"."
                        </ApiRow>
                    </ApiTable>
                    <p>
                        <Code inline=true>"TreeState::set_expanded_keys(keys)"</Code>" replaces the expanded items."
                    </p>
                </Section>

                <Section title="Example" id="use-tree-state-example">
                    <Code language=Language::Rust>
                        {indoc!(r#"
                            use std::collections::HashSet;

                            use leptonic::hooks::{
                                Key, UseTreeStateInput, collections::SelectionOptions, use_collection, use_tree_state,
                            };


                            let collection = use_collection(|b| {
                                b.item("documents", "Documents").children(|c| {
                                    c.item("resume", "resume.pdf");
                                });
                                b.item("notes", "notes.txt");
                            });
                            let state = use_tree_state(UseTreeStateInput {
                                collection,
                                selection: SelectionOptions::default(),
                                default_expanded_keys: HashSet::from([Key::from("documents")]),
                                on_expanded_change: None,
                            });

                            // Render one item per visible entry:
                            let visible = move || state.list.collection.with(|c| c.items().map(|n| n.key.clone()).collect::<Vec<_>>());
                        "#)}
                    </Code>
                </Section>
            </Section>

            <Section title="use_tree">
                <p>"The tree element: a grid list with the "<Code inline=true>"treegrid"</Code>" role, whose rows expand and collapse."</p>
                <Section title="Input" id="use-tree-input">
                    <p>"Pass a "<Code inline=true>"UseTreeInput"</Code>" with every field named; the Default column gives the value for fields you don\u{2019}t need."</p>

                    <ApiTable kind=ApiKind::Input of="UseTreeInput">
                        <ApiRow name="state" ty="TreeState">"From "<Code inline=true>"use_tree_state"</Code>". Required."</ApiRow>
                        <ApiRow name="element" ty="CapturedElement">"The tree element; the props capture it. Required."</ApiRow>
                        <ApiRow name="id" ty="Option<String>" default="None">"The element id, generated when "<Code inline=true>"None"</Code>"."</ApiRow>
                        <ApiRow name="aria_label" ty="MaybeProp<String>" default="None">"Names the tree."</ApiRow>
                        <ApiRow name="aria_labelledby" ty="Option<String>" default="None">"The id(s) of elements naming the tree."</ApiRow>
                        <ApiRow name="options" ty="CollectionOptions" default="CollectionOptions::default()">
                            "Keyboard and focus behavior, see "<Link href=format!("{}#collectionoptions", routes::doc::CollectionState.materialize())>"CollectionOptions"</Link>"."
                        </ApiRow>
                        <ApiRow name="on_action" ty="Option<Callback<Key>>" default="None">"Called with the key of an activated item."</ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Return" id="use-tree-return">
                    <ApiTable kind=ApiKind::Return of="UseGridListReturn">
                        <ApiRow name="props" ty="UseGridListProps">"For the tree element: "<Code inline=true>"role=\"treegrid\""</Code>", labelling, keyboard and focus handling."</ApiRow>
                        <ApiRow name="data" ty="GridListData">"Hand this to "<Code inline=true>"use_tree_item"</Code>" for every visible item."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="use_tree_item">
                <p>"One visible item, with a button expanding and collapsing it."</p>
                <Section title="Input" id="use-tree-item-input">
                    <ApiTable kind=ApiKind::Input of="UseTreeItemInput">
                        <ApiRow name="tree" ty="GridListData">"From "<Code inline=true>"use_tree"</Code>". Required."</ApiRow>
                        <ApiRow name="key" ty="Key">"The item\u{2019}s key. Required."</ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Return" id="use-tree-item-return">
                    <ApiTable kind=ApiKind::Return of="UseTreeItemReturn">
                        <ApiRow name="item" ty="UseGridListItemReturn">
                            "The row: "<Code inline=true>"row_props"</Code>" (with "<Code inline=true>"aria-level"</Code>", "
                            <Code inline=true>"aria-expanded"</Code>", "<Code inline=true>"aria-posinset"</Code>", "
                            <Code inline=true>"aria-setsize"</Code>"), "<Code inline=true>"grid_cell_props"</Code>" and its state, "
                            "as for a "<Link href=format!("{}#use-grid-list-item", routes::doc::grid_list::Hook.materialize())>"grid list row"</Link>"."
                        </ApiRow>
                        <ApiRow name="expand_button" ty="UseButtonInput">
                            "Configuration of a button expanding and collapsing the item, for "
                            <Link href=routes::doc::button::Hook.materialize()>"use_button"</Link>"."
                        </ApiRow>
                        <ApiRow name="expand_button_label" ty="Signal<&'static str>">"\u{201c}Expand\u{201d} or \u{201c}Collapse\u{201d}."</ApiRow>
                        <ApiRow name="is_expanded" ty="Signal<bool>">"Whether the item is expanded."</ApiRow>
                        <ApiRow name="has_child_items" ty="bool">"Whether the item has children (render the expand button only then)."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="Keyboard">
                <KeyboardTable>
                    <KeyRow keys="ArrowDown / ArrowUp">"Focus the next or previous visible item."</KeyRow>
                    <KeyRow keys="ArrowRight">"Expand the focused item (mirrored in right-to-left languages)."</KeyRow>
                    <KeyRow keys="ArrowLeft">"Collapse the focused item, or focus its parent."</KeyRow>
                    <KeyRow keys="Home / End">"Focus the first or last visible item."</KeyRow>
                    <KeyRow keys="Space">"Select or deselect the focused item."</KeyRow>
                    <KeyRow keys="Enter">"Activate the focused item."</KeyRow>
                </KeyboardTable>
                <p>"Pressing a parent item toggles it when the tree has neither selection nor actions."</p>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::Collections.materialize()>"Collections"</Link></li>
                <li><Link href=routes::doc::grid_list::Hook.materialize()>"Grid List Hooks"</Link></li>
                <li><Link href=routes::doc::CollectionState.materialize()>"Collection State"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
