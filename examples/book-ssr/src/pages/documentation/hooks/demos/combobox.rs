use std::collections::HashSet;

use leptonic::{
    atoms::checkbox::Checkbox,
    hooks::{
        ComboBoxFormValue, ComboBoxMenuTrigger, ComboBoxState, IntoAttrs, ListBoxData, SelectMode, UseComboBoxInput,
        UseComboBoxReturn, UseComboBoxStateInput, UseListBoxInput, UseListBoxReturn,
        UseOptionInput, UseOptionReturn, UseTextFieldReturn, ValidationBehavior,
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
        collection,
        selection_mode: SelectMode::Single,
        default_value: Vec::new(),
        value: None,
        on_change: None,
        default_input_value: None,
        input_value: None,
        on_input_change: None,
        disabled_keys: Signal::stored(HashSet::new()),
        menu_trigger: ComboBoxMenuTrigger::Input,
        allows_empty_collection: false,
        allows_custom_value: false,
        should_close_on_blur: true,
        is_read_only: Signal::stored(false),
        on_open_change: None,
        is_invalid: Signal::stored(false),
        validate: None,
        validation_behavior: ValidationBehavior::default(),
        name: None,
    });
    let disabled = RwSignal::new(false);

    // The popover element: focus moving into it doesn't count as leaving the combobox.
    let popover = CapturedElement::new();
    let UseComboBoxReturn {
        input,
        input_props,
        button,
        listbox,
        ..
    } = use_combobox(UseComboBoxInput {
        has_label: true.into(),
        placeholder: "Search fruits\u{2026}".into(),
        is_disabled: disabled.into(),
        popover,
        state,
        id: None,
        is_required: Signal::stored(false),
        aria_label: MaybeProp::default(),
        aria_labelledby: None,
        aria_describedby: None,
        form_value: ComboBoxFormValue::Key,
        form: None,
        should_focus_wrap: false,
        keyboard_delegate: None,
        on_focus: None,
        on_blur: None,
    });

    // The input is a text field with the combobox's configuration.
    let UseTextFieldReturn {
        label_props,
        input_props: field_props,
        ..
    } = use_text_field(input);
    let (button_attrs, button_styles) = use_button(button).props.into_parts();
    let listbox = StoredValue::new(listbox);

    view! {
        <div class="demo-combo">
            <label {..label_props.into_attrs()} class="demo-combo-label">
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
            <Checkbox is_selected=disabled set_selected=disabled classes="demo-check">
                <span class="demo-check-box" aria-hidden="true"></span>
                "Disabled"
            </Checkbox>
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
        on_context_menu: None,
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
