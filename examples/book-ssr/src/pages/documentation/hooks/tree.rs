use indoc::indoc;
use leptonic::components::prelude::*;
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
                <Link href=routes::doc::grid::Hook.materialize()>"grid list"</Link>" whose rows have levels."
            </p>

            <ReactAria hook="useTree"/>

            <Section title="Demo">
                <p>
                    "Arrow up and down move between the visible items, arrow right expands a folder, arrow left collapses it or "
                    "moves to its parent. Space or a click selects an item."
                </p>
                <Demo description="File tree with expandable folders and single selection" source=include_str!("demos/tree.rs")>
                    <TreeDemo/>
                </Demo>
            </Section>

            <Section title="use_tree_state">
                <p>
                    "Build the tree as a collection of nested items (see "<Link href=routes::doc::Collections.materialize()>"Collections"</Link>
                    "). The state holds the selection and which items are expanded; its list shows the visible items, in order."
                </p>

                <Code language=Language::Rust>
                    {indoc!(r#"
                        let collection = use_collection(|b| {
                            b.item("documents", "Documents").children(|c| {
                                c.item("resume", "resume.pdf");
                            });
                            b.item("notes", "notes.txt");
                        });
                        let state = use_tree_state(UseTreeStateInput {
                            default_expanded_keys: HashSet::from([Key::from("documents")]),
                            ..UseTreeStateInput::new(collection)
                        });

                        // Render one item per visible entry:
                        let visible = move || state.list.collection.with(|c| c.items().map(|n| n.key.clone()).collect::<Vec<_>>());
                    "#)}
                </Code>

                <Section title="Input" id="use-tree-state-input">
                    <p>"Create the input with "<Code inline=true>"UseTreeStateInput::new(collection)"</Code>"."</p>
                    <ApiTable kind=ApiKind::Input of="UseTreeStateInput">
                        <ApiRow name="collection" ty="CollectionMemo">"The items, nested with "<Code inline=true>".children(..)"</Code>"."</ApiRow>
                        <ApiRow name="selection" ty="SelectionOptions" default="no selection">"Selection mode, behavior and callback."</ApiRow>
                        <ApiRow name="default_expanded_keys" ty="HashSet<Key>" default="empty">"The initially expanded items."</ApiRow>
                        <ApiRow name="on_expanded_change" ty="Option<Callback<HashSet<Key>>>" default="None">"Called with the expanded keys whenever they change."</ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Return" id="use-tree-state-return">
                    <ApiTable kind=ApiKind::Return of="TreeState">
                        <ApiRow name="list" ty="ListState">"The visible items, their selection and focus."</ApiRow>
                        <ApiRow name="expansion" ty="TreeExpansion">
                            <Code inline=true>"is_expanded(&key)"</Code>", "<Code inline=true>"toggle_key(key)"</Code>" and "
                            <Code inline=true>"set_expanded_keys(keys)"</Code>"."
                        </ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="use_tree">
                <p>"Create the input with "<Code inline=true>"UseTreeInput::new(state, element)"</Code>"."</p>
                <Section title="Input" id="use-tree-input">
                    <ApiTable kind=ApiKind::Input of="UseTreeInput">
                        <ApiRow name="state" ty="TreeState">"From "<Code inline=true>"use_tree_state"</Code>"."</ApiRow>
                        <ApiRow name="element" ty="CapturedElement">"The tree element. The props capture it."</ApiRow>
                        <ApiRow name="id" ty="Option<String>" default="None">"The element id, generated when "<Code inline=true>"None"</Code>"."</ApiRow>
                        <ApiRow name="aria_label" ty="MaybeProp<String>" default="None">"Names the tree."</ApiRow>
                        <ApiRow name="aria_labelledby" ty="Option<String>" default="None">"The id(s) of elements naming the tree."</ApiRow>
                        <ApiRow name="options" ty="CollectionOptions" default="default">"Keyboard and focus behavior."</ApiRow>
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
                <Section title="Input" id="use-tree-item-input">
                    <ApiTable kind=ApiKind::Input of="UseTreeItemInput">
                        <ApiRow name="tree" ty="GridListData">"From "<Code inline=true>"use_tree"</Code>"."</ApiRow>
                        <ApiRow name="key" ty="Key">"The item\u{2019}s key."</ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Return" id="use-tree-item-return">
                    <ApiTable kind=ApiKind::Return of="UseTreeItemReturn">
                        <ApiRow name="item" ty="UseGridListItemReturn">
                            "The row: "<Code inline=true>"row_props"</Code>" (with "<Code inline=true>"aria-level"</Code>", "
                            <Code inline=true>"aria-expanded"</Code>", "<Code inline=true>"aria-posinset"</Code>", "
                            <Code inline=true>"aria-setsize"</Code>"), "<Code inline=true>"grid_cell_props"</Code>" and its state, "
                            "as for a "<Link href=routes::doc::grid::Hook.materialize()>"grid list row"</Link>"."
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
                    <KeyRow keys="ArrowRight">"Expand the focused item."</KeyRow>
                    <KeyRow keys="ArrowLeft">"Collapse the focused item, or focus its parent."</KeyRow>
                    <KeyRow keys="Home / End">"Focus the first or last visible item."</KeyRow>
                    <KeyRow keys="Space">"Toggle the selection of the focused item."</KeyRow>
                    <KeyRow keys="Enter">"Activate the focused item."</KeyRow>
                </KeyboardTable>
                <p>"Pressing a parent item toggles it when the tree has neither selection nor actions."</p>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::Collections.materialize()>"Collections"</Link></li>
                <li><Link href=routes::doc::grid::Hook.materialize()>"Grid list hooks"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
