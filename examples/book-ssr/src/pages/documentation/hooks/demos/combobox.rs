use leptonic::{
    components::prelude::*,
    hooks::{
        ComboBoxState, IntoAttrs, ListBoxData, UseComboBoxInput, UseComboBoxReturn,
        UseComboBoxStateInput, UseListBoxInput, UseListBoxReturn, UseOptionInput, UseOptionReturn,
        UseTextFieldReturn,
        collections::{Node, use_collection},
        use_button, use_combobox, use_combobox_state, use_contains_filter, use_listbox, use_option,
        use_text_field,
    },
    utils::CapturedElement,
};
use leptos::prelude::*;

const FRUITS: [(&str, &str); 8] = [
    ("apple", "Apple"),
    ("apricot", "Apricot"),
    ("banana", "Banana"),
    ("blueberry", "Blueberry"),
    ("cherry", "Cherry"),
    ("durian", "Durian"),
    ("elderberry", "Elderberry"),
    ("grape", "Grape"),
];

#[component]
pub fn ComboboxDemo() -> impl IntoView {
    // All options. Keys identify them, texts are matched against the input. Durian is sold out.
    let collection = use_collection(|b| {
        for (key, name) in FRUITS {
            b.item(key, name).disabled(key == "durian");
        }
    });
    // Selection, input text, filtering and the open state.
    let state = use_combobox_state(UseComboBoxStateInput {
        filter: Some(use_contains_filter()),
        ..UseComboBoxStateInput::new(collection)
    });
    let disabled = RwSignal::new(false);

    // The popover element: focus moving into it doesn't count as leaving the combobox.
    let popover = CapturedElement::new();
    let UseComboBoxReturn {
        label_on_click,
        input,
        input_props,
        button,
        listbox,
    } = use_combobox(UseComboBoxInput {
        has_label: true.into(),
        placeholder: Some("Search fruits\u{2026}".to_owned()),
        is_disabled: disabled.into(),
        popover,
        ..UseComboBoxInput::new(state)
    });

    // The input is a text field with the combobox's configuration.
    let UseTextFieldReturn {
        label_props,
        input_props: field_props,
        ..
    } = use_text_field(input);
    let (button_attrs, button_styles) = use_button(button).props.into_parts();
    let label_attrs = (label_on_click.into_on(leptos::ev::click),);
    let listbox = StoredValue::new(listbox);

    view! {
        <div class="demo-combo">
            <label {..label_props.into_attrs()} {..label_attrs} class="demo-combo-label">
                "Fruit"
            </label>
            <div class="demo-combo-field">
                <input {..field_props.into_attrs()} {..input_props.into_attrs()} class="demo-combo-input"/>
                <button {..button_attrs} style=button_styles class="demo-combo-button">
                    <span aria-hidden="true">"\u{25bc}"</span>
                </button>
            </div>

            <Show when=move || state.is_open()>
                <div {..popover.attr()} class="demo-combo-popover demo-combo-below">
                    <FruitListBox listbox=listbox.get_value()/>
                </div>
            </Show>
        </div>

        <DemoState state/>
        <div class="demo-controls">
            <Checkbox is_selected=disabled set_selected=disabled>"Disabled"</Checkbox>
        </div>
    }
}

/// The options of the popover: the items of the listbox's collection, which the combobox filters by the input text.
#[component]
fn FruitListBox(listbox: UseListBoxInput) -> impl IntoView {
    let UseListBoxReturn { props, data } = use_listbox(listbox);
    let collection = data.state.collection;
    let items = move || collection.with(|c| c.items().cloned().collect::<Vec<Node>>());

    view! {
        <div {..props.into_attrs()} class="demo-combo-listbox">
            <For each=items key=|node| node.key.clone() let:node>
                <FruitOption list=data.clone() node/>
            </For>
        </div>
    }
}

/// One option. It is focused virtually: DOM focus stays in the input, which points at the option with
/// `aria-activedescendant`. The hook sets `aria-selected` and `aria-disabled`; the focus is shown through
/// `data-focused`, rendered from `is_focused`.
#[component]
fn FruitOption(list: ListBoxData, node: Node) -> impl IntoView {
    let UseOptionReturn {
        props,
        is_selected,
        is_focused,
        ..
    } = use_option(UseOptionInput {
        list,
        key: node.key.clone(),
    });
    let (attrs, styles) = props.into_parts();

    view! {
        <div {..attrs} style=styles data-focused=move || is_focused.get().then_some("") class="demo-combo-option">
            <span class="demo-combo-check" aria-hidden="true">
                <Show when=move || is_selected.get()>"\u{2713}"</Show>
            </span>
            {node.text_value.to_string()}
        </div>
    }
}

#[component]
fn DemoState(state: ComboBoxState) -> impl IntoView {
    let value = move || {
        state
            .selected_key()
            .map_or_else(|| "none".to_owned(), |key| key.to_string())
    };
    let input_value = move || state.input_value();
    let open = move || if state.is_open() { "open" } else { "closed" };

    view! {
        <p class="demo-status">
            "Value: "{value}". Input: \u{201c}"{input_value}"\u{201d}. Popover: "{open}"."
        </p>
    }

}
