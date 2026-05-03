use leptonic::hooks::*;
use leptos::prelude::*;

#[component]
fn TreeItem(
    key: &'static str,
    label: &'static str,
    has_children: bool,
    level: usize,
    position_in_set: usize,
    set_size: usize,
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
        position_in_set,
        set_size,
        has_children,
        is_expanded,
        is_selected,
        is_focused: Signal::derive(|| false),
        is_disabled: Signal::derive(|| false),
        on_expand: Some(Callback::new(move |_expand| {
            toggle_expanded.run(key_for_expand.clone());
        })),
        on_select: Some(Callback::new(move |()| {
            toggle_selected.run(key_for_select.clone());
        })),
        on_action: None,
        on_focus_next: None,
        on_focus_previous: None,
        on_focus_parent: None,
        on_focus_first_child: None,
        on_focus_first: None,
        on_focus_last: None,
        on_focus_self: None,
    });

    let padding = format!(
        "{}em",
        f64::from(u32::try_from(level).unwrap_or(u32::MAX)) * 1.5
    );

    view! {
        <li
            {..tree_item.item_props.into_attrs()}
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
                            { move || if is_expanded.get() { "\u{25bc}" } else { "\u{25b6}" } }
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

#[component]
pub fn TreeDemo() -> impl IntoView {
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
        })),
        ..Default::default()
    });

    view! {
        <div style="margin: 1em 0; max-width: 300px; border: 1px solid #ccc; border-radius: 4px; padding: 0.5em;">
            <ul {..tree.tree_props.into_attrs()} style="list-style: none; padding: 0; margin: 0;">
                <TreeItem
                    key="documents"
                    label="Documents"
                    has_children=true
                    level=0
                    position_in_set=1
                    set_size=3
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
                        position_in_set=1
                        set_size=2
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
                        position_in_set=2
                        set_size=2
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
                    position_in_set=2
                    set_size=3
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
                        position_in_set=1
                        set_size=1
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
                    position_in_set=3
                    set_size=3
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
    }
}
