use leptonic::{
    atoms::listbox::{
        ListBox, ListBoxItem, ListBoxItemDescription, ListBoxItemLabel, ListBoxItems,
        ListBoxSection, ListBoxSectionHeading,
    },
    hooks::collections::{
        AutoFocus, CollectionMemo, EscapeKeyBehavior, ItemLink, Key, ListLayout, Selection,
        SelectionBehavior, SelectionMode, UseListCollectionInput, use_collection,
        use_list_collection,
    },
};
use leptos::prelude::*;

use super::listbox::describe_selection;
use crate::pages::Section;

const ANIMALS: [&str; 3] = ["Cat", "Dog", "Kangaroo"];

fn animals() -> CollectionMemo {
    use_list_collection(UseListCollectionInput {
        items: Signal::stored(ANIMALS.to_vec()),
        key: |animal: &&str| Key::from(*animal),
        text_value: |animal: &&str| (*animal).to_owned(),
    })
}

/// The options of the surrounding listbox's collection.
#[component]
fn Items() -> impl IntoView {
    view! { <ListBoxItems let:node>{node.text_value.to_string()}</ListBoxItems> }
}

/// A log of selection changes, shown in `#<id>`: each change's selected keys (sorted,
/// comma-separated), changes separated by `|`.
fn change_log(id: &'static str) -> (Callback<Selection>, impl IntoView) {
    let changes = RwSignal::new(Vec::<String>::new());
    let on_change = Callback::new(move |selection: Selection| {
        changes.update(|c| c.push(describe_selection(&selection)));
    });
    let view = view! {
        <div>"Changes: " <span id=id>{move || changes.get().join("|")}</span></div>
    };
    (on_change, view)
}

/// A log of actions (keys separated by `,`), shown in `#<id>`.
fn action_log(id: &'static str) -> (Callback<Key>, impl IntoView) {
    let actions = RwSignal::new(Vec::<String>::new());
    let on_action = Callback::new(move |key: Key| actions.update(|a| a.push(key.to_string())));
    let view = view! {
        <div>"Actions: " <span id=id>{move || actions.get().join(",")}</span></div>
    };
    (on_action, view)
}

/// ListBox selection, as react-aria-components' and React Spectrum's `ListBox` tests and
/// `useSelectableCollection.test.js` render them (animals Cat, Dog, Kangaroo unless noted). Each
/// listbox is a section of its own (`?only=<name>`); selection changes are logged in
/// `#<name>-changes`, actions in `#<name>-actions`.
/// - `single`: single selection, default Dog. `single-controlled`: single selection fixed to
///   Dog (the selection is never written).
/// - `multi-default`: multiple selection, default Cat and Kangaroo. `multi-controlled`: the same,
///   fixed.
/// - `escape-none`: multiple selection, default Cat, Escape doesn't clear.
/// - `replace`: multiple selection, replace behavior, a button `#replace-before` before it;
///   `replace-default`: the same with the default Dog and Kangaroo, between the buttons
///   `#replace-default-before` and `#replace-default-after`.
/// - `replace-single`: single selection, replace behavior.
/// - `toggle-action`: multiple selection (toggle) and an action.
/// - `links-multiple`, `links-replace`: link options "One" (`#lbs-one`) and "Two" (`#lbs-two`)
///   with multiple selection, toggle and replace behavior.
/// - `slots`: Cat with a label "Cat" and a description "Meows".
/// - `dynamic`: sections Veggies (Lettuce, Tomato) and Protein (Ham, Tuna); `#dynamic-insert`
///   inserts Onion after Lettuce, `#dynamic-reverse` reverses the veggies, `#dynamic-move` moves
///   Ham to the veggies' end.
/// - `grid`: options "Item 0" ... "Item 8" in a 3-column grid, single selection.
/// - `typeahead`: "Foo Bar", "Foo Baz", "Bar", "Zoo", single selection.
/// - `attrs`: a listbox and an option with classes and a data attribute.
/// - `autofocus-selected` (no selection), `autofocus-first`, `autofocus-last` (replace
///   behavior: focusing selects), `autofocus-selected-default` (default Dog, replace behavior):
///   listboxes focused when they mount.
#[component]
#[allow(clippy::too_many_lines)]
pub fn PageAtomListBoxSelection() -> impl IntoView {
    let (single_change, single_log) = change_log("single-changes");
    let (single_controlled_change, single_controlled_log) = change_log("single-controlled-changes");
    let (multi_default_change, multi_default_log) = change_log("multi-default-changes");
    let (multi_controlled_change, multi_controlled_log) = change_log("multi-controlled-changes");
    let (escape_none_change, escape_none_log) = change_log("escape-none-changes");
    let (replace_change, replace_log) = change_log("replace-changes");
    let (replace_default_change, replace_default_log) = change_log("replace-default-changes");
    let (replace_single_change, replace_single_log) = change_log("replace-single-changes");
    let (toggle_action_change, toggle_action_log) = change_log("toggle-action-changes");
    let (toggle_action, toggle_action_actions) = action_log("toggle-action-actions");
    let (links_multiple_change, links_multiple_log) = change_log("links-multiple-changes");
    let (links_replace_change, links_replace_log) = change_log("links-replace-changes");
    let (grid_change, grid_log) = change_log("grid-changes");
    let (typeahead_change, typeahead_log) = change_log("typeahead-changes");
    let (autofocus_first_change, autofocus_first_log) = change_log("autofocus-first-changes");
    let (autofocus_last_change, autofocus_last_log) = change_log("autofocus-last-changes");
    let (autofocus_default_change, autofocus_default_log) =
        change_log("autofocus-selected-default-changes");

    let links = use_collection(|b| {
        let _ = b.item("one", "One").link(ItemLink::new("#lbs-one"));
        let _ = b.item("two", "Two").link(ItemLink::new("#lbs-two"));
    });
    let slots = use_collection(|b| {
        b.item("cat", "Cat");
        b.item("dog", "Dog");
    });

    let veggies = RwSignal::new(vec!["Lettuce", "Tomato"]);
    let protein = RwSignal::new(vec!["Ham", "Tuna"]);
    let dynamic = use_collection(move |b| {
        b.section("veggies", |s| {
            s.header("veggies-header", "Veggies");
            for item in veggies.get() {
                s.item(item, item);
            }
        });
        b.section("protein", |s| {
            s.header("protein-header", "Protein");
            for item in protein.get() {
                s.item(item, item);
            }
        });
    });
    let dynamic_section = move |key: &'static str, items: RwSignal<Vec<&'static str>>| {
        view! {
            <ListBoxSection key=key>
                <ListBoxSectionHeading />
                <For each=move || items.get() key=|item| *item let:item>
                    <ListBoxItem key=item>{item}</ListBoxItem>
                </For>
            </ListBoxSection>
        }
    };

    let grid = use_list_collection(UseListCollectionInput {
        items: Signal::stored((0..9).collect::<Vec<u32>>()),
        key: |i: &u32| Key::from(*i),
        text_value: |i: &u32| format!("Item {i}"),
    });
    let typeahead = use_list_collection(UseListCollectionInput {
        items: Signal::stored(vec!["Foo Bar", "Foo Baz", "Bar", "Zoo"]),
        key: |item: &&str| Key::from(*item),
        text_value: |item: &&str| (*item).to_owned(),
    });
    let dog = || Selection::keys([Key::from("Dog")]);
    let cat_and_kangaroo = || Selection::keys([Key::from("Cat"), Key::from("Kangaroo")]);

    view! {
        <h1>"ListBox selection"</h1>
        <style>
            "#grid [role=listbox] { display: grid; grid-template-columns: 100px 100px 100px; }"
        </style>

        <Section name="single">
            <div id="single">
                <ListBox
                    collection=animals()
                    selection_mode=SelectionMode::Single
                    default_selection=dog()
                    on_selection_change=single_change
                    aria_label="Single"
                >
                    <Items />
                </ListBox>
            </div>
            {single_log}
        </Section>
        <Section name="single-controlled">
            <div id="single-controlled">
                <ListBox
                    collection=animals()
                    selection_mode=SelectionMode::Single
                    selection=dog()
                    on_selection_change=single_controlled_change
                    aria_label="Single controlled"
                >
                    <Items />
                </ListBox>
            </div>
            {single_controlled_log}
        </Section>
        <Section name="multi-default">
            <div id="multi-default">
                <ListBox
                    collection=animals()
                    selection_mode=SelectionMode::Multiple
                    default_selection=cat_and_kangaroo()
                    on_selection_change=multi_default_change
                    aria_label="Multiple default"
                >
                    <Items />
                </ListBox>
            </div>
            {multi_default_log}
        </Section>
        <Section name="multi-controlled">
            <div id="multi-controlled">
                <ListBox
                    collection=animals()
                    selection_mode=SelectionMode::Multiple
                    selection=cat_and_kangaroo()
                    on_selection_change=multi_controlled_change
                    aria_label="Multiple controlled"
                >
                    <Items />
                </ListBox>
            </div>
            {multi_controlled_log}
        </Section>
        <Section name="escape-none">
            <div id="escape-none">
                <ListBox
                    collection=animals()
                    selection_mode=SelectionMode::Multiple
                    default_selection=Selection::keys([Key::from("Cat")])
                    escape_key_behavior=EscapeKeyBehavior::None
                    on_selection_change=escape_none_change
                    aria_label="Escape none"
                >
                    <Items />
                </ListBox>
            </div>
            {escape_none_log}
        </Section>
        <Section name="replace">
            <button id="replace-before">"Before"</button>
            <div id="replace">
                <ListBox
                    collection=animals()
                    selection_mode=SelectionMode::Multiple
                    selection_behavior=SelectionBehavior::Replace
                    on_selection_change=replace_change
                    aria_label="Replace"
                >
                    <Items />
                </ListBox>
            </div>
            {replace_log}
        </Section>
        <Section name="replace-default">
            <button id="replace-default-before">"Before"</button>
            <div id="replace-default">
                <ListBox
                    collection=animals()
                    selection_mode=SelectionMode::Multiple
                    selection_behavior=SelectionBehavior::Replace
                    default_selection=Selection::keys([Key::from("Dog"), Key::from("Kangaroo")])
                    on_selection_change=replace_default_change
                    aria_label="Replace default"
                >
                    <Items />
                </ListBox>
            </div>
            <button id="replace-default-after">"After"</button>
            {replace_default_log}
        </Section>
        <Section name="replace-single">
            <div id="replace-single">
                <ListBox
                    collection=animals()
                    selection_mode=SelectionMode::Single
                    selection_behavior=SelectionBehavior::Replace
                    on_selection_change=replace_single_change
                    aria_label="Replace single"
                >
                    <Items />
                </ListBox>
            </div>
            {replace_single_log}
        </Section>
        <Section name="toggle-action">
            <div id="toggle-action">
                <ListBox
                    collection=animals()
                    selection_mode=SelectionMode::Multiple
                    on_selection_change=toggle_action_change
                    on_action=toggle_action
                    aria_label="Toggle with action"
                >
                    <Items />
                </ListBox>
            </div>
            {toggle_action_log}
            {toggle_action_actions}
        </Section>
        <Section name="links-multiple">
            <div id="links-multiple">
                <ListBox
                    collection=links
                    selection_mode=SelectionMode::Multiple
                    on_selection_change=links_multiple_change
                    aria_label="Multiple links"
                >
                    <Items />
                </ListBox>
            </div>
            {links_multiple_log}
        </Section>
        <Section name="links-replace">
            <div id="links-replace">
                <ListBox
                    collection=links
                    selection_mode=SelectionMode::Multiple
                    selection_behavior=SelectionBehavior::Replace
                    on_selection_change=links_replace_change
                    aria_label="Replace links"
                >
                    <Items />
                </ListBox>
            </div>
            {links_replace_log}
        </Section>
        <Section name="slots">
            <div id="slots">
                <ListBox collection=slots aria_label="Slots">
                    <ListBoxItem key="cat">
                        <ListBoxItemLabel>"Cat"</ListBoxItemLabel>
                        <ListBoxItemDescription>"Meows"</ListBoxItemDescription>
                    </ListBoxItem>
                    <ListBoxItem key="dog">"Dog"</ListBoxItem>
                </ListBox>
            </div>
        </Section>
        <Section name="dynamic">
            <button
                id="dynamic-insert"
                on:click=move |_| veggies.update(|v| v.insert(1, "Onion"))
            >
                "Insert"
            </button>
            <button id="dynamic-reverse" on:click=move |_| veggies.update(|v| v.reverse())>
                "Reverse"
            </button>
            <button
                id="dynamic-move"
                on:click=move |_| {
                    protein.update(|p| p.retain(|item| *item != "Ham"));
                    veggies.update(|v| v.push("Ham"));
                }
            >
                "Move"
            </button>
            <div id="dynamic">
                <ListBox
                    collection=dynamic
                    selection_mode=SelectionMode::Multiple
                    aria_label="Dynamic"
                >
                    {dynamic_section("veggies", veggies)}
                    {dynamic_section("protein", protein)}
                </ListBox>
            </div>
        </Section>
        <Section name="grid">
            <div id="grid">
                <ListBox
                    collection=grid
                    layout=ListLayout::Grid
                    selection_mode=SelectionMode::Single
                    on_selection_change=grid_change
                    aria_label="Grid"
                >
                    <Items />
                </ListBox>
            </div>
            {grid_log}
        </Section>
        <Section name="typeahead">
            <div id="typeahead">
                <ListBox
                    collection=typeahead
                    selection_mode=SelectionMode::Single
                    on_selection_change=typeahead_change
                    aria_label="Type-ahead"
                >
                    <Items />
                </ListBox>
            </div>
            {typeahead_log}
        </Section>
        <Section name="attrs">
            <div id="attrs">
                <ListBox
                    collection=animals()
                    aria_label="Attributes"
                    classes="test-custom-listbox"
                    attr:data-test="listbox"
                >
                    <ListBoxItem
                        key="Cat"
                        classes="test-custom-option"
                        attr:data-test="option"
                    >
                        "Cat"
                    </ListBoxItem>
                    <ListBoxItem key="Dog">"Dog"</ListBoxItem>
                    <ListBoxItem key="Kangaroo">"Kangaroo"</ListBoxItem>
                </ListBox>
            </div>
        </Section>
        <Section name="autofocus-selected">
            <div id="autofocus-selected">
                <ListBox
                    collection=animals()
                    selection_mode=SelectionMode::Single
                    auto_focus=AutoFocus::Selected
                    aria_label="Autofocus"
                >
                    <Items />
                </ListBox>
            </div>
        </Section>
        <Section name="autofocus-first">
            <div id="autofocus-first">
                <ListBox
                    collection=animals()
                    selection_mode=SelectionMode::Multiple
                    selection_behavior=SelectionBehavior::Replace
                    auto_focus=AutoFocus::First
                    on_selection_change=autofocus_first_change
                    aria_label="Autofocus first"
                >
                    <Items />
                </ListBox>
            </div>
            {autofocus_first_log}
        </Section>
        <Section name="autofocus-last">
            <div id="autofocus-last">
                <ListBox
                    collection=animals()
                    selection_mode=SelectionMode::Multiple
                    selection_behavior=SelectionBehavior::Replace
                    auto_focus=AutoFocus::Last
                    on_selection_change=autofocus_last_change
                    aria_label="Autofocus last"
                >
                    <Items />
                </ListBox>
            </div>
            {autofocus_last_log}
        </Section>
        <Section name="autofocus-selected-default">
            <div id="autofocus-selected-default">
                <ListBox
                    collection=animals()
                    selection_mode=SelectionMode::Multiple
                    selection_behavior=SelectionBehavior::Replace
                    default_selection=dog()
                    auto_focus=AutoFocus::First
                    on_selection_change=autofocus_default_change
                    aria_label="Autofocus with a selection"
                >
                    <Items />
                </ListBox>
            </div>
            {autofocus_default_log}
        </Section>
    }
}
