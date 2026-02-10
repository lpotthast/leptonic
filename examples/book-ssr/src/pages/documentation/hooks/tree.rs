use crate::pages::documentation::article::Article;
use crate::pages::documentation::toc::Toc;
use indoc::indoc;

use leptonic::components::prelude::*;
use leptonic::hooks::*;
use leptos::prelude::*;

#[component]
pub fn PageUseTree() -> impl IntoView {
    let (expanded, set_expanded) = signal(vec!["documents".to_string()]);
    let (selected, set_selected) = signal(Vec::<String>::new());

    let tree = use_tree(UseTreeInput {
        label: Some("File Browser".to_string()),
        selection_mode: TreeSelectionMode::Single,
        expanded_keys: expanded.into(),
        selected_keys: selected.into(),
        on_expanded_change: Some(Callback::new(move |keys| set_expanded.set(keys))),
        on_selection_change: Some(Callback::new(move |keys| set_selected.set(keys))),
        on_action: Some(Callback::new(|_key| {
            // Handle item activation (e.g., open file)
            // In a real app, you would navigate or open the file here
        })),
        ..Default::default()
    });

    view! {
        <Article>
            <h1 id="use_tree" class="anchor">
                "use_tree"
                <AnchorLink href="#use_tree" description="Direct link to article header"/>
            </h1>

            <p>"Hooks for creating accessible tree components with expandable/collapsible nodes and selection."</p>

            <h2 id="demo" class="anchor">
                "Interactive Demo"
                <AnchorLink href="#demo" description="Direct link to demo"/>
            </h2>

            <div style="margin: 1em 0; max-width: 300px; border: 1px solid #ccc; border-radius: 4px; padding: 0.5em;">
                <ul {..tree.tree_props} style="list-style: none; padding: 0; margin: 0;">
                    <TreeItem
                        key="documents"
                        label="Documents"
                        has_children=true
                        level=0
                        expanded=expanded
                        selected=selected
                        toggle_expanded=tree.toggle_expanded
                        toggle_selected=tree.toggle_selected
                    />
                    <Show when=move || expanded.get().contains(&"documents".to_string())>
                        <TreeItem
                            key="resume.pdf"
                            label="resume.pdf"
                            has_children=false
                            level=1
                            expanded=expanded
                            selected=selected
                            toggle_expanded=tree.toggle_expanded
                            toggle_selected=tree.toggle_selected
                        />
                        <TreeItem
                            key="cover-letter.docx"
                            label="cover-letter.docx"
                            has_children=false
                            level=1
                            expanded=expanded
                            selected=selected
                            toggle_expanded=tree.toggle_expanded
                            toggle_selected=tree.toggle_selected
                        />
                    </Show>
                    <TreeItem
                        key="photos"
                        label="Photos"
                        has_children=true
                        level=0
                        expanded=expanded
                        selected=selected
                        toggle_expanded=tree.toggle_expanded
                        toggle_selected=tree.toggle_selected
                    />
                    <Show when=move || expanded.get().contains(&"photos".to_string())>
                        <TreeItem
                            key="vacation.jpg"
                            label="vacation.jpg"
                            has_children=false
                            level=1
                            expanded=expanded
                            selected=selected
                            toggle_expanded=tree.toggle_expanded
                            toggle_selected=tree.toggle_selected
                        />
                    </Show>
                    <TreeItem
                        key="notes.txt"
                        label="notes.txt"
                        has_children=false
                        level=0
                        expanded=expanded
                        selected=selected
                        toggle_expanded=tree.toggle_expanded
                        toggle_selected=tree.toggle_selected
                    />
                </ul>
            </div>

            <p style="font-size: 0.875em; opacity: 0.7;">
                "Use arrow keys to navigate, Enter to activate, Space to select"
            </p>

            <Code>
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
                        <ul {..tree.tree_props}>
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
                <li><code>"*"</code> " - Expand all siblings"</li>
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
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "use_tree", link: "#use_tree" },
                Toc::Leaf { title: "Demo", link: "#demo" },
                Toc::Leaf { title: "Selection Modes", link: "#selection-modes" },
                Toc::Leaf { title: "Keyboard Navigation", link: "#keyboard-navigation" },
                Toc::Leaf { title: "ARIA Attributes", link: "#aria-attributes" },
                Toc::Leaf { title: "Features", link: "#features" },
            ]
        }/>
    }
}

#[component]
fn TreeItem(
    key: &'static str,
    label: &'static str,
    has_children: bool,
    level: usize,
    expanded: ReadSignal<Vec<String>>,
    selected: ReadSignal<Vec<String>>,
    toggle_expanded: Callback<String>,
    toggle_selected: Callback<String>,
) -> impl IntoView {
    let key_string = key.to_string();
    let key_for_expand = key.to_string();
    let key_for_select = key.to_string();

    let is_expanded = Signal::derive(move || expanded.get().contains(&key_string));
    let is_selected = Signal::derive({
        let key = key.to_string();
        move || selected.get().contains(&key)
    });

    let tree_item = use_tree_item(UseTreeItemInput {
        item_key: key.to_string(),
        level,
        position_in_set: 1,
        set_size: 1,
        has_children,
        is_expanded,
        is_selected,
        is_focused: Signal::derive(|| false),
        is_disabled: Signal::derive(|| false),
        on_expand: Some(Callback::new(move |_expand| {
            toggle_expanded.run(key_for_expand.clone());
        })),
        on_select: Some(Callback::new(move |_| {
            toggle_selected.run(key_for_select.clone());
        })),
        on_action: None,
        on_focus_next: None,
        on_focus_previous: None,
        on_focus_parent: None,
        on_focus_first_child: None,
    });

    let padding = format!("{}em", level as f64 * 1.5);

    view! {
        <li
            {..tree_item.item_props}
            style=format!("padding-left: {}; cursor: pointer; padding: 0.25em 0.5em; border-radius: 4px;", padding)
            style:background=move || if is_selected.get() { "#e3f2fd" } else { "transparent" }
        >
            <div style="display: flex; align-items: center; gap: 0.5em;">
                {if has_children {
                    view! {
                        <span
                            style="width: 1em; text-align: center; cursor: pointer;"
                            on:click=move |e| {
                                e.stop_propagation();
                                toggle_expanded.run(key.to_string());
                            }
                        >
                            { move || if is_expanded.get() { "▼" } else { "▶" } }
                        </span>
                    }.into_any()
                } else {
                    view! { <span style="width: 1em;"></span> }.into_any()
                }}
                <span>{label}</span>
            </div>
        </li>
    }
}
