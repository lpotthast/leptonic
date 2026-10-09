use leptonic::{
    I18nProvider, Locale,
    atoms::grid_list::{
        GridList, GridListHeader, GridListItem, GridListItemDescription, GridListSection,
    },
    hooks::{
        collections::{
            ItemLink, Key, Selection, SelectionBehavior, SelectionMode, UseListCollectionInput,
            use_collection, use_list_collection,
        },
        gridlist::{FocusMode, KeyboardNavigationBehavior},
    },
};
use leptos::prelude::*;

use super::listbox::describe_selection;

const FRUITS: [&str; 3] = ["Apple", "Banana", "Cherry"];

/// Grid list features, as react-aria-components' `GridList` tests render them:
/// - `#glf-children`: arrow navigation; rows "Item 1" and "Item 2" whose first child takes focus
///   (buttons "Item n first", "Item n last").
/// - `#glf-children-rtl`: the same in ar-AE, right to left (one row: "RTL first", "RTL last").
/// - `#glf-tab`: Tab navigation; a row with the buttons "Tab first" and "Tab last".
/// - `#glf-child-arrows`: Tab navigation; rows whose child takes focus and that allow arrow
///   navigation (buttons "Arrow 1", "Arrow 2", "Arrow 3").
/// - `#glf-input`: Tab navigation, multiple selection; "Apple" with a text input, "Banana";
///   selection in `#glf-input-selection`.
/// - `#glf-action`: rows with an action and no selection (Apple, Banana, Cherry); actions in
///   `#glf-action-actions`.
/// - `#glf-replace`: multiple selection with replace behavior and an action; selection in
///   `#glf-replace-selection`, actions in `#glf-replace-actions`.
/// - `#glf-links`: link rows "One" (`#glf-one`) and "Two" (`#glf-two`).
/// - `#glf-sections`: sections "Fruit" (Apple, Banana with a description) and "Vegetables"
///   (Carrot).
#[component]
#[allow(clippy::too_many_lines)]
pub fn PageAtomGridListFeatures() -> impl IntoView {
    let fruits = || {
        use_list_collection(UseListCollectionInput {
            items: Signal::stored(FRUITS.to_vec()),
            key: |fruit| Key::from(*fruit),
            text_value: |fruit| (*fruit).to_owned(),
        })
    };
    let items = |prefix: &'static str, count: usize| {
        use_list_collection(UseListCollectionInput {
            items: Signal::stored((1..=count).collect::<Vec<_>>()),
            key: |i| Key::from(i.to_string()),
            text_value: move |i| format!("{prefix} {i}"),
        })
    };
    let input_selection = RwSignal::new(String::new());
    let actions = RwSignal::new(Vec::<String>::new());
    let replace_selection = RwSignal::new(String::new());
    let replace_actions = RwSignal::new(Vec::<String>::new());
    let links = use_collection(|b| {
        let _ = b.item("one", "One").link(ItemLink::new("#glf-one"));
        let _ = b.item("two", "Two").link(ItemLink::new("#glf-two"));
    });
    let sections = use_collection(|b| {
        b.section("fruit", |s| {
            s.header("fruit-header", "Fruit");
            s.item("apple", "Apple");
            s.item("banana", "Banana");
        });
        b.section("vegetables", |s| {
            s.header("vegetables-header", "Vegetables");
            s.item("carrot", "Carrot");
        });
    });

    view! {
        <h1>"GridList features"</h1>

        <div id="glf-children">
            <GridList collection=items("Item", 2) aria_label="Children">
                {(1..=2)
                    .map(|i| {
                        view! {
                            <GridListItem key=i.to_string() focus_mode=FocusMode::Child>
                                <button aria-label=format!("Item {i} first")>"First"</button>
                                <button aria-label=format!("Item {i} last")>"Last"</button>
                            </GridListItem>
                        }
                    })
                    .collect_view()}
            </GridList>
        </div>

        <div id="glf-children-rtl" dir="rtl">
            <I18nProvider locale={"ar-AE".parse::<Locale>().expect("a locale")}>
                <GridList collection=items("RTL", 1) aria_label="Children RTL">
                    <GridListItem key="1" focus_mode=FocusMode::Child>
                        <button aria-label="RTL first">"First"</button>
                        <button aria-label="RTL last">"Last"</button>
                    </GridListItem>
                </GridList>
            </I18nProvider>
        </div>

        <div id="glf-tab">
            <GridList
                collection=items("Tab", 1)
                aria_label="Tab"
                keyboard_navigation_behavior=KeyboardNavigationBehavior::Tab
            >
                <GridListItem key="1">
                    <button aria-label="Tab first">"Go"</button>
                    <button aria-label="Tab last">"Go 2"</button>
                </GridListItem>
            </GridList>
        </div>

        <div id="glf-child-arrows">
            <GridList
                collection=items("Arrow", 3)
                aria_label="Child arrows"
                keyboard_navigation_behavior=KeyboardNavigationBehavior::Tab
            >
                {(1..=3)
                    .map(|i| {
                        view! {
                            <GridListItem
                                key=i.to_string()
                                focus_mode=FocusMode::Child
                                allows_arrow_navigation=true
                            >
                                <button aria-label=format!("Arrow {i}")>"Go"</button>
                            </GridListItem>
                        }
                    })
                    .collect_view()}
            </GridList>
        </div>

        <div id="glf-input">
            <GridList
                collection=fruits()
                aria_label="Input"
                selection_mode=SelectionMode::Multiple
                keyboard_navigation_behavior=KeyboardNavigationBehavior::Tab
                on_selection_change={move |s: Selection| input_selection.set(describe_selection(&s))}
            >
                <GridListItem key="Apple">"Apple" <input aria-label="Apple input" /></GridListItem>
                <GridListItem key="Banana">"Banana"</GridListItem>
                <GridListItem key="Cherry">"Cherry"</GridListItem>
            </GridList>
        </div>
        <div>"Selection: " <span id="glf-input-selection">{input_selection}</span></div>

        <div id="glf-action">
            <GridList
                collection=fruits()
                aria_label="Actions"
                on_action={move |key: Key| actions.update(|a| a.push(key.to_string()))}
            >
                {FRUITS.map(|fruit| view! { <GridListItem key=fruit>{fruit}</GridListItem> }).collect_view()}
            </GridList>
        </div>
        <div>"Actions: " <span id="glf-action-actions">{move || actions.get().join(",")}</span></div>

        <div id="glf-replace">
            <GridList
                collection=fruits()
                aria_label="Replace"
                selection_mode=SelectionMode::Multiple
                selection_behavior=SelectionBehavior::Replace
                on_selection_change={move |s: Selection| replace_selection.set(describe_selection(&s))}
                on_action={move |key: Key| replace_actions.update(|a| a.push(key.to_string()))}
            >
                {FRUITS.map(|fruit| view! { <GridListItem key=fruit>{fruit}</GridListItem> }).collect_view()}
            </GridList>
        </div>
        <div>"Selection: " <span id="glf-replace-selection">{replace_selection}</span></div>
        <div>"Actions: " <span id="glf-replace-actions">{move || replace_actions.get().join(",")}</span></div>

        <div id="glf-links">
            <GridList collection=links aria_label="Links">
                <GridListItem key="one">"One"</GridListItem>
                <GridListItem key="two">"Two"</GridListItem>
            </GridList>
        </div>

        <div id="glf-sections">
            <GridList collection=sections aria_label="Groceries">
                <GridListSection key="fruit">
                    <GridListHeader />
                    <GridListItem key="apple">"Apple"</GridListItem>
                    <GridListItem key="banana">
                        "Banana"
                        <GridListItemDescription>"Yellow"</GridListItemDescription>
                    </GridListItem>
                </GridListSection>
                <GridListSection key="vegetables">
                    <GridListHeader />
                    <GridListItem key="carrot">"Carrot"</GridListItem>
                </GridListSection>
            </GridList>
        </div>
    }
}
