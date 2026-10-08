use std::collections::HashSet;

use leptonic::{
    atoms::{
        listbox::{ListBox, ListBoxItem, ListBoxItems, ListBoxSection, ListBoxSectionHeading},
        separator::Separator,
    },
    hooks::{
        DisabledBehavior, Orientation, SelectionBehavior, SelectionMode,
        collections::{
            CollectionMemo, ItemLink, Key, ListLayout, Selection, use_collection,
            use_list_collection,
        },
    },
    utils::i18n::{I18nProvider, Locale},
};
use leptos::prelude::*;

use super::listbox::describe_selection;

const ANIMALS: [&str; 3] = ["Cat", "Dog", "Kangaroo"];

fn animals() -> CollectionMemo {
    use_list_collection(
        Signal::stored(ANIMALS.to_vec()),
        |animal| Key::from(*animal),
        |animal| (*animal).to_owned(),
    )
}

/// The options of the surrounding listbox's collection.
#[component]
fn Items() -> impl IntoView {
    view! { <ListBoxItems let:node>{node.text_value.to_string()}</ListBoxItems> }
}

/// Listbox features, as react-aria-components' `ListBox` tests render them:
/// - `#lbf-sections`: multiple selection; a section with the header "Veggies" (Lettuce, Tomato,
///   Onion), a separator, a section labelled "Protein" (Ham, Tuna, Tofu).
/// - `#lbf-replace`: multiple selection with replace behavior and an action (Cat, Dog,
///   Kangaroo); selection in `#lbf-replace-selection`, actions in `#lbf-replace-actions`.
/// - `#lbf-action`: no selection, an action; actions in `#lbf-action-actions`.
/// - `#lbf-links`: link items "One" (`#lbf-one`) and "Two" (`#lbf-two`) without selection;
///   `#lbf-links-single`: the same with single selection.
/// - `#lbf-horizontal`: horizontal; `#lbf-rtl`: horizontal, right-to-left.
/// - `#lbf-grid`: grid layout, two columns.
/// - `#lbf-page`: 30 options ("Option 1", ...) in a scrolling listbox.
/// - `#lbf-wrap`: `should_focus_wrap`.
/// - `#lbf-disabled-selection`: Dog disabled with `DisabledBehavior::Selection`;
///   `#lbf-item-disabled-behavior`: Dog disabled with its own `DisabledBehavior::Selection`,
///   Kangaroo disabled.
/// - `#lbf-empty`: no options, the empty state "No results".
/// - `#lbf-removal`: Cat, Dog, Kangaroo; `#lbf-remove-dog` removes Dog.
/// - `#lbf-relabel`: Cat labelled "Cat" (a section "Pets" labelled "Pets"); `#lbf-relabel-cat`
///   relabels them "Kitten" and "Animals".
#[component]
#[allow(clippy::too_many_lines)]
pub fn PageAtomListBoxFeatures() -> impl IntoView {
    let sections = use_collection(|b| {
        b.section("veggies", |s| {
            s.header("veggies-header", "Veggies");
            s.item("lettuce", "Lettuce");
            s.item("tomato", "Tomato");
            s.item("onion", "Onion");
        });
        b.separator("separator");
        let _ = b
            .section("protein", |s| {
                s.item("ham", "Ham");
                s.item("tuna", "Tuna");
                s.item("tofu", "Tofu");
            })
            .aria_label("Protein");
    });
    let replace_selection = RwSignal::new(String::new());
    let replace_actions = RwSignal::new(Vec::<String>::new());
    let actions = RwSignal::new(Vec::<String>::new());
    let links = use_collection(|b| {
        let _ = b.item("one", "One").link(ItemLink::new("#lbf-one"));
        let _ = b.item("two", "Two").link(ItemLink::new("#lbf-two"));
    });
    let options: Vec<String> = (1..=30).map(|i| format!("Option {i}")).collect();
    let many = use_list_collection(
        Signal::stored(options),
        |option: &String| Key::from(option.as_str()),
        |option: &String| option.clone(),
    );
    let item_disabled_behavior = use_collection(|b| {
        b.item("Cat", "Cat");
        let _ = b
            .item("Dog", "Dog")
            .disabled(true)
            .disabled_behavior(DisabledBehavior::Selection);
        let _ = b.item("Kangaroo", "Kangaroo").disabled(true);
    });
    let empty = use_list_collection(
        Signal::stored(Vec::<&str>::new()),
        |key| Key::from(*key),
        |key| (*key).to_owned(),
    );
    let remaining = RwSignal::new(ANIMALS.to_vec());
    let removal = use_list_collection(
        remaining.into(),
        |animal| Key::from(*animal),
        |animal| (*animal).to_owned(),
    );
    let relabeled = RwSignal::new(false);
    let relabel = use_collection(move |b| {
        let relabeled = relabeled.get();
        let _ = b
            .section("pets", |s| {
                let _ = s
                    .item("cat", "Cat")
                    .aria_label(if relabeled { "Kitten" } else { "Cat" });
                s.item("dog", "Dog");
            })
            .aria_label(if relabeled { "Animals" } else { "Pets" });
    });
    let rtl = "ar-AE".parse::<Locale>().expect("a locale");

    view! {
        <h1>"ListBox features"</h1>
        <style>
            "#lbf-grid [role=listbox] { display: grid; grid-template-columns: 100px 100px; }
            #lbf-page [role=listbox] { height: 100px; overflow: auto; }"
        </style>

        <div id="lbf-sections">
            <ListBox collection=sections selection_mode=SelectionMode::Multiple aria_label="Sandwich contents">
                <ListBoxSection key="veggies">
                    <ListBoxSectionHeading />
                    <ListBoxItem key="lettuce">"Lettuce"</ListBoxItem>
                    <ListBoxItem key="tomato">"Tomato"</ListBoxItem>
                    <ListBoxItem key="onion">"Onion"</ListBoxItem>
                </ListBoxSection>
                <Separator />
                <ListBoxSection key="protein">
                    <ListBoxItem key="ham">"Ham"</ListBoxItem>
                    <ListBoxItem key="tuna">"Tuna"</ListBoxItem>
                    <ListBoxItem key="tofu">"Tofu"</ListBoxItem>
                </ListBoxSection>
            </ListBox>
        </div>

        <div id="lbf-replace">
            <ListBox
                collection=animals()
                selection_mode=SelectionMode::Multiple
                selection_behavior=SelectionBehavior::Replace
                aria_label="Replace"
                on_selection_change={move |s: Selection| replace_selection.set(describe_selection(&s))}
                on_action={move |key: Key| replace_actions.update(|a| a.push(key.to_string()))}
            >
                <Items />
            </ListBox>
        </div>
        <div>"Selection: " <span id="lbf-replace-selection">{replace_selection}</span></div>
        <div>"Actions: " <span id="lbf-replace-actions">{move || replace_actions.get().join(",")}</span></div>

        <div id="lbf-action">
            <ListBox
                collection=animals()
                aria_label="Actions"
                on_action={move |key: Key| actions.update(|a| a.push(key.to_string()))}
            >
                <Items />
            </ListBox>
        </div>
        <div>"Actions: " <span id="lbf-action-actions">{move || actions.get().join(",")}</span></div>

        <div id="lbf-links">
            <ListBox collection=links aria_label="Links">
                <Items />
            </ListBox>
        </div>
        <div id="lbf-links-single">
            <ListBox collection=links selection_mode=SelectionMode::Single aria_label="Single links">
                <Items />
            </ListBox>
        </div>

        <div id="lbf-horizontal">
            <ListBox collection=animals() orientation=Orientation::Horizontal aria_label="Horizontal">
                <Items />
            </ListBox>
        </div>
        <div id="lbf-rtl">
            <I18nProvider locale=rtl>
                <ListBox collection=animals() orientation=Orientation::Horizontal aria_label="Right to left">
                    <Items />
                </ListBox>
            </I18nProvider>
        </div>

        <div id="lbf-grid">
            <ListBox
                collection=animals()
                layout=ListLayout::Grid
                aria_label="Grid"
            >
                <Items />
            </ListBox>
        </div>

        <div id="lbf-page">
            <ListBox
                collection=many
                aria_label="Many"
            >
                <ListBoxItems let:node>
                    <div style="height: 20px">{node.text_value.to_string()}</div>
                </ListBoxItems>
            </ListBox>
        </div>

        <div id="lbf-wrap">
            <ListBox collection=animals() should_focus_wrap=true aria_label="Wrapping">
                <Items />
            </ListBox>
        </div>

        <div id="lbf-disabled-selection">
            <ListBox
                collection=animals()
                selection_mode=SelectionMode::Multiple
                disabled_keys=Signal::stored(HashSet::from([Key::from("Dog")]))
                disabled_behavior=DisabledBehavior::Selection
                aria_label="Disabled selection"
            >
                <Items />
            </ListBox>
        </div>
        <div id="lbf-item-disabled-behavior">
            <ListBox
                collection=item_disabled_behavior
                selection_mode=SelectionMode::Multiple
                aria_label="Item disabled behavior"
            >
                <Items />
            </ListBox>
        </div>

        <div id="lbf-empty">
            <ListBox collection=empty aria_label="Empty" empty_state=|| "No results">
                <Items />
            </ListBox>
        </div>

        <div id="lbf-removal">
            <button id="lbf-remove-dog" on:click=move |_| remaining.update(|r| r.retain(|a| *a != "Dog"))>
                "Remove Dog"
            </button>
            <ListBox collection=removal aria_label="Removal">
                <Items />
            </ListBox>
        </div>

        <div id="lbf-relabel">
            <button id="lbf-relabel-cat" on:click=move |_| relabeled.set(true)>"Relabel"</button>
            <ListBox collection=relabel aria_label="Relabel">
                <ListBoxSection key="pets">
                    <ListBoxItem key="cat">"Cat"</ListBoxItem>
                    <ListBoxItem key="dog">"Dog"</ListBoxItem>
                </ListBoxSection>
            </ListBox>
        </div>
    }
}
