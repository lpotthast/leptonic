use leptonic::{
    hooks::{
        IntoAttrs, Key, ListState, Node, Orientation, SelectionMode,
        collections::{
            CollectionOptions, LinkBehavior, ListLayout, Selection, SelectionOptions,
            UseListStateInput, UseSelectableItemInput, UseSelectableItemReturn,
            UseSelectableListInput, use_selectable_item, use_selectable_list,
        },
        use_collection, use_list_state,
    },
    utils::CapturedElement,
};
use leptos::prelude::*;

const FRUITS: [&str; 6] = ["Apple", "Banana", "Cherry", "Durian", "Elderberry", "Fig"];

#[component]
pub fn CollectionStateDemo() -> impl IntoView {
    // The items, built from data. Durian is sold out.
    let collection = use_collection(|b| {
        for fruit in FRUITS {
            b.item(fruit, fruit).disabled(fruit == "Durian");
        }
    });
    // App state: the selected fruits. The list shows it, and selecting writes it.
    let selected = RwSignal::new(Selection::keys([Key::from("Banana")]));
    let state = use_list_state(UseListStateInput {
        collection,
        selection: SelectionOptions {
            selection_mode: Signal::stored(SelectionMode::Multiple),
            selection: Some(selected.into()),
            ..SelectionOptions::default()
        },
    });
    // Keyboard navigation, type-ahead and the single tab stop of the list element.
    let list = use_selectable_list(UseSelectableListInput {
        state,
        element: CapturedElement::new(),
        orientation: Orientation::Vertical,
        layout: ListLayout::Stack,
        layout_delegate: None,
        keyboard_delegate: None,
        options: CollectionOptions::default(),
    })
    .props;
    let collection_id = list.collection_id.clone();
    let items = move || collection.with(|c| c.items().cloned().collect::<Vec<Node>>());

    let status = move || {
        let mut names: Vec<String> = state
            .selection
            .selected_keys()
            .iter()
            .map(ToString::to_string)
            .collect();
        names.sort();
        if names.is_empty() {
            "Nothing selected.".to_owned()
        } else {
            format!("Selected: {}.", names.join(", "))
        }
    };

    // The hooks bring behavior, not semantics: the roles and ARIA states are up to the concept you build.
    view! {
        <div
            {..list.into_attrs()}
            role="listbox"
            aria-label="Fruits"
            aria-multiselectable="true"
            class="demo-selectable-list"
        >
            <For each=items key=|node| node.key.clone() let:node>
                <FruitItem state collection_id=collection_id.clone() node/>
            </For>
        </div>
        <p class="demo-status">{status}</p>
    }
}

/// One item: selection on press, focus handling and the roving tab index.
#[component]
fn FruitItem(state: ListState, collection_id: String, node: Node) -> impl IntoView {
    let UseSelectableItemReturn {
        props,
        is_selected,
        is_disabled,
        ..
    } = use_selectable_item(UseSelectableItemInput {
        selection: state.selection,
        item_elements: state.item_elements,
        key: node.key.clone(),
        element: CapturedElement::new(),
        id: None,
        collection_id,
        is_disabled: Signal::stored(false),
        should_select_on_press_up: false,
        allows_different_press_origin: false,
        on_action: Signal::stored(None),
        on_context_menu: None,
        link_behavior: LinkBehavior::default(),
        focus: None,
        should_use_virtual_focus: false,
    });
    let (attrs, styles) = props.into_parts();

    view! {
        <div
            {..attrs}
            style=styles
            role="option"
            aria-selected=move || is_selected.get().to_string()
            aria-disabled=move || is_disabled.get().then_some("true")
            class="demo-selectable-item"
        >
            {node.text_value.to_string()}
        </div>
    }
}
