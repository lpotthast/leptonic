use leptonic::{
    I18nProvider, Locale, Orientation,
    atoms::grid_list::{GridList, GridListHeader, GridListItem, GridListSection},
    hooks::{
        collections::{
            AutoFocus, CollectionMemo, EscapeKeyBehavior, Key, ListLayout, Selection,
            SelectionBehavior, SelectionMode, UseListCollectionInput, use_collection,
            use_list_collection,
        },
        gridlist::KeyboardNavigationBehavior,
    },
};
use leptos::prelude::*;

use super::listbox::describe_selection;
use crate::pages::Section;

const ANIMALS: [&str; 3] = ["Cat", "Dog", "Kangaroo"];

fn animals() -> CollectionMemo {
    use_list_collection(UseListCollectionInput {
        items: Signal::stored(ANIMALS.to_vec()),
        key: |animal| Key::from(*animal),
        text_value: |animal| (*animal).to_owned(),
    })
}

const MORE_ANIMALS: [&str; 6] = ["Cat", "Dog", "Kangaroo", "Koala", "Panda", "Snake"];

/// A horizontal grid layout of six animals in two rows, flowing into three columns (Cat, Dog;
/// Kangaroo, Koala; Panda, Snake), labelled `label`.
#[component]
fn HorizontalGrid(label: &'static str) -> impl IntoView {
    let collection = use_list_collection(UseListCollectionInput {
        items: Signal::stored(MORE_ANIMALS.to_vec()),
        key: |animal| Key::from(*animal),
        text_value: |animal| (*animal).to_owned(),
    });
    view! {
        <style>
            ".test-glc-hgrid { display: grid; grid-auto-flow: column; grid-template-rows: auto auto; \
            grid-auto-columns: 100px; gap: 8px; }"
        </style>
        <GridList
            collection=collection
            aria_label=label
            layout=ListLayout::Grid
            orientation=Orientation::Horizontal
            classes="test-glc-hgrid"
        >
            {MORE_ANIMALS
                .map(|animal| view! { <GridListItem key=animal>{animal}</GridListItem> })
                .collect_view()}
        </GridList>
    }
}

fn animal_rows() -> impl IntoView {
    ANIMALS
        .map(|animal| view! { <GridListItem key=animal>{animal}</GridListItem> })
        .collect_view()
}

/// A grid list of the animals (Cat, Dog, Kangaroo) labelled `label`, reporting its selection in
/// `#test-glc-{id}-selection`.
#[component]
fn Animals(
    label: &'static str,
    id: &'static str,
    #[prop(optional)] selection_mode: SelectionMode,
    #[prop(optional)] selection_behavior: SelectionBehavior,
    #[prop(optional)] default_selection: Selection,
    #[prop(optional)] auto_focus: Option<AutoFocus>,
    #[prop(optional)] escape_key_behavior: EscapeKeyBehavior,
    #[prop(optional)] should_select_on_press_up: bool,
) -> impl IntoView {
    let selection = RwSignal::new(describe_selection(&default_selection));
    let on_selection_change =
        Callback::new(move |s: Selection| selection.set(describe_selection(&s)));
    // `auto_focus` is an optional prop: set only where given.
    let list = match auto_focus {
        Some(auto_focus) => view! {
            <GridList
                collection=animals()
                aria_label=label
                selection_mode=selection_mode
                selection_behavior=selection_behavior
                default_selection=default_selection
                auto_focus=auto_focus
                on_selection_change=on_selection_change
            >
                {animal_rows()}
            </GridList>
        }
        .into_any(),
        None => view! {
            <GridList
                collection=animals()
                aria_label=label
                selection_mode=selection_mode
                selection_behavior=selection_behavior
                default_selection=default_selection
                escape_key_behavior=escape_key_behavior
                should_select_on_press_up=should_select_on_press_up
                on_selection_change=on_selection_change
            >
                {animal_rows()}
            </GridList>
        }
        .into_any(),
    };
    view! {
        <button id=format!("test-glc-{id}-before")>"Before"</button>
        {list}
        <div>"Selection: " <span id=format!("test-glc-{id}-selection")>{selection}</span></div>
    }
}

/// Cases of react-aria-components' `GridList.test.js`, one `Section` each (load one with
/// `goto_sections`): auto focus (`autofocus*`), focus ring and press state (`interactive`,
/// `static`), Escape with `EscapeKeyBehavior::None` (`escape`), an empty list (`empty`), a grid
/// layout (`grid-layout`), a horizontal grid layout (`horizontal-grid-layout`, and right to left in
/// `horizontal-grid-layout-rtl`), sections labelled by a header and/or an `aria-label` (`sections`),
/// selecting on press up (`press-up-*`) and a text input in a row with arrow navigation
/// (`input-arrow`). Every grid list follows a "Before" button (`#test-glc-{id}-before`).
#[component]
#[allow(clippy::too_many_lines)]
pub fn PageAtomGridListCases() -> impl IntoView {
    let empty = use_list_collection(UseListCollectionInput {
        items: Signal::stored(Vec::<&'static str>::new()),
        key: |animal| Key::from(*animal),
        text_value: |animal| (*animal).to_owned(),
    });
    let sections = use_collection(|b| {
        b.section("animals", |s| {
            s.header("animals-header", "Favorite Animal");
            s.item("cat", "Cat");
            s.item("dog", "Dog");
        });
        let _ = b
            .section("ice-cream", |s| {
                s.item("vanilla", "Vanilla");
                s.item("chocolate", "Chocolate");
            })
            .aria_label("Favorite Ice Cream");
        let _ = b
            .section("fruit", |s| {
                s.header("fruit-header", "Fruit");
                s.item("apple", "Apple");
            })
            .aria_label("Favorite");
    });
    let input_selection = RwSignal::new(String::new());
    let input_fruits = use_list_collection(UseListCollectionInput {
        items: Signal::stored(vec!["Apple", "Banana"]),
        key: |fruit| Key::from(*fruit),
        text_value: |fruit| (*fruit).to_owned(),
    });

    view! {
        <h1>"GridList cases"</h1>
        <Section name="autofocus">
            <Animals label="Autofocus" id="autofocus" auto_focus=AutoFocus::Selected />
        </Section>
        <Section name="autofocus-first">
            <Animals
                label="Autofocus first"
                id="autofocus-first"
                selection_mode=SelectionMode::Single
                selection_behavior=SelectionBehavior::Replace
                auto_focus=AutoFocus::First
            />
        </Section>
        <Section name="autofocus-last">
            <Animals
                label="Autofocus last"
                id="autofocus-last"
                selection_mode=SelectionMode::Single
                selection_behavior=SelectionBehavior::Replace
                auto_focus=AutoFocus::Last
            />
        </Section>
        <Section name="autofocus-none">
            <Animals
                label="Autofocus without selection"
                id="autofocus-none"
                selection_mode=SelectionMode::None
                selection_behavior=SelectionBehavior::Replace
                auto_focus=AutoFocus::First
            />
        </Section>
        <Section name="autofocus-all">
            <Animals
                label="Autofocus all selected"
                id="autofocus-all"
                selection_mode=SelectionMode::Multiple
                selection_behavior=SelectionBehavior::Replace
                default_selection=Selection::All
                auto_focus=AutoFocus::First
            />
        </Section>
        <Section name="interactive">
            <Animals label="Interactive" id="interactive" selection_mode=SelectionMode::Multiple />
        </Section>
        <Section name="static">
            <Animals label="Static" id="static" />
        </Section>
        <Section name="escape">
            <Animals
                label="Escape"
                id="escape"
                selection_mode=SelectionMode::Multiple
                escape_key_behavior=EscapeKeyBehavior::None
            />
        </Section>
        <Section name="empty">
            <button id="test-glc-empty-before">"Before"</button>
            <GridList collection=empty aria_label="Empty" children=Box::new(|| ().into_any()) />
            <button id="test-glc-empty-after">"After"</button>
        </Section>
        <Section name="grid-layout">
            // The rows side by side: ArrowLeft/ArrowRight move between them.
            <style>".test-glc-grid { display: flex; gap: 8px; }"</style>
            <button id="test-glc-grid-before">"Before"</button>
            <GridList
                collection=animals()
                aria_label="Grid layout"
                layout=ListLayout::Grid
                classes="test-glc-grid"
            >
                <GridListItem key="Cat">"Cat"</GridListItem>
                <GridListItem key="Dog">"Dog " <button aria-label="Info">"ⓘ"</button></GridListItem>
                <GridListItem key="Kangaroo">"Kangaroo"</GridListItem>
            </GridList>
            <button id="test-glc-grid-after">"After"</button>
        </Section>
        <Section name="horizontal-grid-layout">
            <button id="test-glc-hgrid-before">"Before"</button>
            <HorizontalGrid label="Horizontal grid" />
        </Section>
        <Section name="horizontal-grid-layout-rtl">
            <div dir="rtl">
                <I18nProvider locale={"ar-AE".parse::<Locale>().expect("a locale")}>
                    <button id="test-glc-hgrid-rtl-before">"Before"</button>
                    <HorizontalGrid label="Horizontal grid RTL" />
                </I18nProvider>
            </div>
        </Section>
        <Section name="sections">
            <GridList collection=sections aria_label="Sections">
                <GridListSection key="animals">
                    <GridListHeader />
                    <GridListItem key="cat">"Cat"</GridListItem>
                    <GridListItem key="dog">"Dog"</GridListItem>
                </GridListSection>
                <GridListSection key="ice-cream">
                    <GridListItem key="vanilla">"Vanilla"</GridListItem>
                    <GridListItem key="chocolate">"Chocolate"</GridListItem>
                </GridListSection>
                <GridListSection key="fruit">
                    <GridListHeader />
                    <GridListItem key="apple">"Apple"</GridListItem>
                </GridListSection>
            </GridList>
        </Section>
        <Section name="press-up-default">
            <Animals label="Press default" id="press-up-default" selection_mode=SelectionMode::Single />
        </Section>
        <Section name="press-up-true">
            <Animals
                label="Press up"
                id="press-up-true"
                selection_mode=SelectionMode::Single
                should_select_on_press_up=true
            />
        </Section>
        <Section name="input-arrow">
            <GridList
                collection=input_fruits
                aria_label="Input arrow"
                selection_mode=SelectionMode::Multiple
                keyboard_navigation_behavior=KeyboardNavigationBehavior::Arrow
                on_selection_change={move |s: Selection| input_selection.set(describe_selection(&s))}
            >
                <GridListItem key="Apple">"Apple " <input aria-label="input 1" /></GridListItem>
                <GridListItem key="Banana">"Banana"</GridListItem>
            </GridList>
            <div>"Selection: " <span id="test-glc-input-arrow-selection">{input_selection}</span></div>
        </Section>
    }
}
