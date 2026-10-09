use std::collections::HashSet;

use leptonic::{
    CapturedElement, I18nProvider, IntoAttrs, Locale, ValueBinding,
    hooks::{
        button::use_button,
        collections::{
            CollectionOptions, DisabledBehavior, EscapeKeyBehavior, Key, Selection, SelectionMode,
            SelectionOptions, use_collection,
        },
        gridlist::{FocusMode, GridListData, KeyboardNavigationBehavior},
        tree::{
            UseTreeInput, UseTreeItemInput, UseTreeItemReturn, UseTreeStateInput, use_tree,
            use_tree_item, use_tree_state,
        },
    },
};
use leptos::prelude::*;

use crate::pages::{Section, atoms::listbox::describe_selection};

/// react-aria-components' `AriaTreeTests` tree: Photos, Projects (Projects-1 (Projects-1A),
/// Projects-2, Projects-3), School (Homework-1 (Homework-1A), Homework-2, Homework-3); School
/// is disabled; then "Notes" without children (`#test-tc-{id}-add-child` gives it one). `empty`:
/// no items at all.
///
/// Shows the selection in `#test-tc-{id}-selection`, the expanded items in
/// `#test-tc-{id}-expanded` (bound app state: `#test-tc-{id}-collapse-all` collapses all) and,
/// with `actions`, the activated items in `#test-tc-{id}-actions`. `with_input` puts a text input
/// ("Photos input", "Projects input") into the rows Photos and Projects.
#[component]
#[allow(clippy::too_many_lines, clippy::fn_params_excessive_bools)]
fn TestTree(
    label: &'static str,
    id: &'static str,
    selection_mode: SelectionMode,
    disabled_behavior: DisabledBehavior,
    #[prop(optional)] actions: bool,
    #[prop(optional)] escape_key_behavior: EscapeKeyBehavior,
    #[prop(optional)] should_select_on_press_up: bool,
    #[prop(optional)] keyboard_navigation_behavior: KeyboardNavigationBehavior,
    #[prop(optional)] with_input: bool,
    #[prop(optional)] empty: bool,
) -> impl IntoView {
    let notes_child = RwSignal::new(false);
    let collection = use_collection(move |b| {
        if empty {
            return;
        }
        b.item("photos", "Photos");
        b.item("projects", "Projects").children(|c| {
            c.item("projects-1", "Projects-1").children(|c| {
                c.item("projects-1A", "Projects-1A");
            });
            c.item("projects-2", "Projects-2");
            c.item("projects-3", "Projects-3");
        });
        b.item("school", "School").children(|c| {
            c.item("homework-1", "Homework-1").children(|c| {
                c.item("homework-1A", "Homework-1A");
            });
            c.item("homework-2", "Homework-2");
            c.item("homework-3", "Homework-3");
        });
        if notes_child.get() {
            b.item("notes", "Notes").children(|c| {
                c.item("draft", "Draft");
            });
        } else {
            b.item("notes", "Notes");
        }
    });
    let expanded = RwSignal::new(HashSet::<Key>::new());
    let selection = RwSignal::new(String::new());
    let activated = RwSignal::new(Vec::<String>::new());
    let state = use_tree_state(UseTreeStateInput {
        collection,
        selection: SelectionOptions {
            selection_mode: Signal::stored(selection_mode),
            disabled_keys: Signal::stored(HashSet::from([Key::from("school")])),
            disabled_behavior,
            on_selection_change: Some(Callback::new(move |s: Selection| {
                selection.set(describe_selection(&s));
            })),
            ..SelectionOptions::default()
        },
        default_expanded_keys: HashSet::new(),
        expanded_keys: Some(ValueBinding::from(expanded)),
        on_expanded_change: None,
    });
    let tree = use_tree(UseTreeInput {
        aria_label: label.into(),
        state,
        element: CapturedElement::new(),
        id: None,
        aria_labelledby: Signal::stored(None),
        options: CollectionOptions {
            escape_key_behavior,
            ..CollectionOptions::default()
        },
        on_action: actions
            .then(|| Callback::new(move |key: Key| activated.update(|a| a.push(key.to_string())))),
        keyboard_navigation_behavior,
        should_select_on_press_up,
    });
    let data = tree.data;
    let visible = state.list.collection;
    let expanded_text = move || {
        let mut keys: Vec<String> = expanded.get().iter().map(ToString::to_string).collect();
        keys.sort();
        keys.join(",")
    };

    view! {
        <div {..tree.props.into_attrs()}>
            <For
                each=move || visible.with(|c| c.items().map(|n| n.key.clone()).collect::<Vec<_>>())
                key=Clone::clone
                let:key
            >
                <TreeItem tree=data.clone() key=key with_input />
            </For>
        </div>
        <div>
            "Selection: " <span id=format!("test-tc-{id}-selection")>{selection}</span>
            " Expanded: " <span id=format!("test-tc-{id}-expanded")>{expanded_text}</span>
            " Actions: " <span id=format!("test-tc-{id}-actions")>{move || activated.get().join(",")}</span>
        </div>
        {(!empty).then(|| view! {
            <button id=format!("test-tc-{id}-collapse-all") on:click=move |_| expanded.set(HashSet::new())>
                "Collapse all"
            </button>
            <button id=format!("test-tc-{id}-add-child") on:click=move |_| notes_child.set(true)>
                "Add a child to Notes"
            </button>
        })}
    }
}

#[component]
fn TreeItem(tree: GridListData, key: Key, with_input: bool) -> impl IntoView {
    let has_input = with_input && matches!(key.as_str(), Some("photos" | "projects"));
    let text = tree
        .state
        .collection
        .with_untracked(|c| c.get(&key).map(|n| n.text_value.to_string()))
        .unwrap_or_default();
    let UseTreeItemReturn {
        item,
        expand_button,
        expand_button_attrs,
        has_child_items,
        ..
    } = use_tree_item(UseTreeItemInput {
        tree,
        key,
        focus_mode: FocusMode::Row,
        allows_arrow_navigation: false,
        on_context_menu: None,
    });
    let (attrs, styles) = item.row_props.into_parts();
    let button = move || {
        has_child_items.get().then(|| {
            let (attrs, styles) = use_button(expand_button.clone()).props.into_parts();
            view! { <button {..attrs} {..expand_button_attrs.clone()} style=styles>"›"</button> }
        })
    };
    let input = has_input.then(|| view! { <input aria-label=format!("{text} input") /> });
    view! {
        <div {..attrs} style=styles>
            <div {..item.grid_cell_props.into_attrs()}>{button} <span>{text.clone()}</span> {input}</div>
        </div>
    }
}

/// Trees from the tree hooks with a disabled item (react-aria-components' `AriaTreeTests`):
/// single selection where disabled items can still be focused and expanded
/// (`DisabledBehavior::Selection`), one where they can't be used at all, and a right-to-left
/// tree; in sections (`goto_sections`) trees with actions (`action`, `selectable-action`),
/// `EscapeKeyBehavior::None` (`escape`), selection on press up (`press-up`), Tab navigation and
/// text inputs (`input`), and no items (`empty`, followed by `#test-tc-after-empty`). Each follows
/// a "Before" button (`#test-tc-before-{id}`).
#[component]
pub fn PageHookTreeCases() -> impl IntoView {
    let rtl: Locale = "ar-AE".parse().expect("a valid locale");
    view! {
        <div id="test-page-hook-tree-cases">
            <h1>"Tree cases"</h1>
            <button id="test-tc-before-selection">"Before"</button>
            <TestTree
                label="Selection tree"
                id="selection"
                selection_mode=SelectionMode::Single
                disabled_behavior=DisabledBehavior::Selection
            />
            <button id="test-tc-before-all">"Before"</button>
            <TestTree
                label="Disabled tree"
                id="all"
                selection_mode=SelectionMode::Single
                disabled_behavior=DisabledBehavior::All
            />
            <button id="test-tc-before-rtl">"Before"</button>
            <div dir="rtl">
                <I18nProvider locale=rtl>
                    <TestTree
                        label="RTL tree"
                        id="rtl"
                        selection_mode=SelectionMode::None
                        disabled_behavior=DisabledBehavior::All
                    />
                </I18nProvider>
            </div>
            <Section name="action">
                <button id="test-tc-before-action">"Before"</button>
                <TestTree
                    label="Action tree"
                    id="action"
                    selection_mode=SelectionMode::None
                    disabled_behavior=DisabledBehavior::All
                    actions=true
                />
            </Section>
            <Section name="selectable-action">
                <button id="test-tc-before-selectable-action">"Before"</button>
                <TestTree
                    label="Selectable action tree"
                    id="selectable-action"
                    selection_mode=SelectionMode::Multiple
                    disabled_behavior=DisabledBehavior::All
                    actions=true
                />
            </Section>
            <Section name="escape">
                <button id="test-tc-before-escape">"Before"</button>
                <TestTree
                    label="Escape tree"
                    id="escape"
                    selection_mode=SelectionMode::Multiple
                    disabled_behavior=DisabledBehavior::All
                    escape_key_behavior=EscapeKeyBehavior::None
                />
            </Section>
            <Section name="press-up">
                <button id="test-tc-before-press-up">"Before"</button>
                <TestTree
                    label="Press up tree"
                    id="press-up"
                    selection_mode=SelectionMode::Single
                    disabled_behavior=DisabledBehavior::All
                    should_select_on_press_up=true
                />
            </Section>
            <Section name="input">
                <button id="test-tc-before-input">"Before"</button>
                <TestTree
                    label="Input tree"
                    id="input"
                    selection_mode=SelectionMode::Multiple
                    disabled_behavior=DisabledBehavior::All
                    keyboard_navigation_behavior=KeyboardNavigationBehavior::Tab
                    with_input=true
                />
            </Section>
            <Section name="empty">
                <button id="test-tc-before-empty">"Before"</button>
                <TestTree
                    label="Empty tree"
                    id="empty"
                    selection_mode=SelectionMode::Single
                    disabled_behavior=DisabledBehavior::All
                    empty=true
                />
                <button id="test-tc-after-empty">"After"</button>
            </Section>
        </div>
    }
}
