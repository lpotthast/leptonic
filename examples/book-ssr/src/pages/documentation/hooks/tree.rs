use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::prelude::*;

use crate::pages::documentation::{article::Article, demo_shell::DemoShell, toc::Toc};

use super::demos::tree::TreeDemo;

#[component]
pub fn PageUseTree() -> impl IntoView {
    view! {
        <Article>
            <h1 id="use_tree" class="anchor">
                "use_tree"
                <AnchorLink href="#use_tree" description="Direct link to article header"/>
            </h1>

            <p>
                "The "<Code inline=true>"use_tree"</Code>" hook is a standalone hook for creating accessible tree components with expandable/collapsible nodes and selection."
            </p>

            <p>
                "Based on react-aria\u{2019}s "
                <LinkExt href="https://react-spectrum.adobe.com/react-aria/useTree.html" target=LinkTarget::_Blank>
                    "useTree"
                </LinkExt>
                "."
            </p>

            <h2 id="demo" class="anchor">
                "Interactive Demo"
                <AnchorLink href="#demo" description="Direct link to demo"/>
            </h2>

            <DemoShell source=include_str!("demos/tree.rs")>
                <TreeDemo />
            </DemoShell>

            <Code language=Language::Rust>
                {indoc!(r#"
                    let (expanded, set_expanded) = signal(vec!["folder1".to_string()]);
                    let (selected, set_selected) = signal(Vec::new());

                    let tree = use_tree(UseTreeInput {
                        label: Some("File Browser".to_string()),
                        selection_mode: TreeSelectionMode::Single,
                        expanded_keys: expanded.into(),
                        selected_keys: selected.into(),
                        on_expanded_change: Some(Callback::new(|keys| set_expanded.set(keys))),
                        on_selection_change: Some(Callback::new(|keys| set_selected.set(keys))),
                        ..Default::default()
                    });

                    view! {
                        <ul {..tree.tree_props.into_attrs()}>
                            // Tree items...
                        </ul>
                    }
                "#)}
            </Code>

            <h2 id="selection-modes" class="anchor">
                "Selection Modes"
                <AnchorLink href="#selection-modes" description="Direct link to selection modes"/>
            </h2>

            <ul>
                <li><code>"TreeSelectionMode::None"</code> " - No selection (default)"</li>
                <li><code>"TreeSelectionMode::Single"</code> " - Single item selection"</li>
                <li><code>"TreeSelectionMode::Multiple"</code> " - Multiple item selection"</li>
            </ul>

            <h2 id="keyboard-navigation" class="anchor">
                "Keyboard Navigation"
                <AnchorLink href="#keyboard-navigation" description="Direct link to keyboard"/>
            </h2>

            <ul>
                <li><code>"Arrow Down"</code> " - Move to next visible item"</li>
                <li><code>"Arrow Up"</code> " - Move to previous visible item"</li>
                <li><code>"Arrow Right"</code> " - Expand node or move to first child"</li>
                <li><code>"Arrow Left"</code> " - Collapse node or move to parent"</li>
                <li><code>"Enter"</code> " - Activate item (on_action callback)"</li>
                <li><code>"Space"</code> " - Toggle selection"</li>
                <li><code>"Home"</code> " - Move to first visible item"</li>
                <li><code>"End"</code> " - Move to last visible item"</li>
            </ul>

            <h2 id="aria-attributes" class="anchor">
                "ARIA Attributes"
                <AnchorLink href="#aria-attributes" description="Direct link to ARIA attributes"/>
            </h2>

            <p>"For the tree container:"</p>
            <ul>
                <li><code>"role=\"tree\""</code></li>
                <li><code>"aria-label"</code></li>
                <li><code>"aria-multiselectable"</code></li>
            </ul>

            <p>"For tree items:"</p>
            <ul>
                <li><code>"role=\"treeitem\""</code></li>
                <li><code>"aria-expanded"</code> " (for parent nodes)"</li>
                <li><code>"aria-selected"</code></li>
                <li><code>"aria-level"</code> " (nesting depth)"</li>
                <li><code>"aria-setsize"</code> " and " <code>"aria-posinset"</code></li>
            </ul>

            <h2 id="features" class="anchor">
                "Features"
                <AnchorLink href="#features" description="Direct link to features"/>
            </h2>

            <ul>
                <li>"Hierarchical data display"</li>
                <li>"Expand/collapse nodes"</li>
                <li>"Multiple selection modes"</li>
                <li>"Full keyboard navigation"</li>
                <li>"Action callbacks"</li>
                <li>"Complete ARIA tree pattern"</li>
            </ul>

            <h2 id="see-also" class="anchor">
                "See Also"
                <AnchorLink href="#see-also" description="Direct link to section: See Also"/>
            </h2>

            <ul>
                <li><Link href=crate::routes::doc::Navigation.materialize()>"Navigation domain"</Link></li>
                <li><Link href=crate::routes::doc::Grid.materialize()>"Grid"</Link></li>
                <li><Link href=crate::routes::doc::hooks::Selection.materialize()>"Selection hooks"</Link></li>
            </ul>
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "use_tree", link: "#use_tree" },
                Toc::Leaf { title: "Demo", link: "#demo" },
                Toc::Leaf { title: "Selection Modes", link: "#selection-modes" },
                Toc::Leaf { title: "Keyboard Navigation", link: "#keyboard-navigation" },
                Toc::Leaf { title: "ARIA Attributes", link: "#aria-attributes" },
                Toc::Leaf { title: "Features", link: "#features" },
                Toc::Leaf { title: "See Also", link: "#see-also" },
            ]
        }/>
    }
}
