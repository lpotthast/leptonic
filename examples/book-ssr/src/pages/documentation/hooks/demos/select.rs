use std::collections::HashSet;

use leptonic::{
    atoms::{
        checkbox::{CheckboxButton, CheckboxField},
        focus_scope::FocusScope,
    },
    hooks::{
        button::use_button,
        collections::{CloseOnSelect, Key, UseListCollectionInput, use_list_collection},
        form::ValidationBehavior,
        listbox::{
            ListBoxData, UseListBoxInput, UseListBoxReturn, UseOptionInput, UseOptionReturn,
            use_listbox, use_option,
        },
        overlay::{UseOverlayInput, use_overlay},
        select::{
            SelectMode, UseHiddenSelectReturn, UseSelectInput, UseSelectReturn,
            UseSelectStateInput, use_hidden_select, use_select, use_select_state,
        },
    },
};
use leptos::prelude::*;

const FRUITS: [(&str, &str); 4] = [
    ("apple", "Apple"),
    ("banana", "Banana"),
    ("cherry", "Cherry"),
    ("date", "Date"),
];

#[component]
pub fn SelectDemo() -> impl IntoView {
    let collection = use_list_collection(UseListCollectionInput {
        items: Signal::stored(FRUITS.to_vec()),
        key: |(key, _)| Key::from(*key),
        text_value: |(_, label)| (*label).to_owned(),
    });
    let state = use_select_state(UseSelectStateInput {
        collection,
        selection_mode: SelectMode::Single,
        default_value: Vec::new(),
        value: None,
        on_change: None,
        disabled_keys: Signal::stored(HashSet::new()),
        should_close_on_select: CloseOnSelect::Auto,
        allows_empty_collection: false,
        default_open: false,
        is_open: None,
        on_open_change: None,
        is_invalid: Signal::stored(false),
        validate: None,
        validation_behavior: ValidationBehavior::default(),
        name: Some("fruit".to_owned()),
    });
    let disabled = RwSignal::new(false);

    let UseSelectReturn {
        label,
        trigger_button,
        trigger,
        value,
        listbox,
        hidden_select,
        ..
    } = use_select(UseSelectInput {
        has_label: true.into(),
        is_disabled: disabled.into(),
        state,
        id: None,
        is_required: Signal::stored(false),
        aria_label: MaybeProp::default(),
        aria_labelledby: Signal::default(),
        aria_describedby: Signal::default(),
        keyboard_delegate: None,
        on_focus: None,
        on_blur: None,
        on_focus_change: None,
        form: None,
    });

    let (button_attrs, button_styles) = use_button(trigger_button).button.attrs();

    let selected_text = move || {
        state.selected_items().first().map_or_else(
            || "Select a fruit\u{2026}".to_owned(),
            |n| n.text_value.to_string(),
        )
    };

    view! {
        <div class="demo-select">
            <span {..label.attrs()} class="demo-select-label">
                "Fruit"
            </span>
            <button
                {..button_attrs}
                {..trigger.attrs()}
                style=button_styles
                class="demo-select-trigger"
            >
                <span {..value.attrs()}>{selected_text}</span>
                <span aria-hidden="true">"\u{25bc}"</span>
            </button>

            <Show when=move || state.is_open()>
                <FruitPopover listbox=listbox.clone() close=Callback::new(move |()| state.close()) />
            </Show>

            <FruitHiddenSelect hidden_select=hidden_select.clone() />
        </div>

        <p class="demo-status">
            "Selected: "{move || state.selected_key().map_or_else(|| "none".to_owned(), |key| key.to_string())}
        </p>
        <div class="demo-controls">
            <CheckboxField is_selected=disabled set_selected=disabled>
                <CheckboxButton classes="demo-check">
                    <span class="demo-check-box" aria-hidden="true"></span>
                    "Disabled"
                </CheckboxButton>
            </CheckboxField>
        </div>
    }
}

/// The popover: dismissable with Escape or a click outside, returning focus to the trigger.
#[component]
fn FruitPopover(listbox: UseListBoxInput, close: Callback<()>) -> impl IntoView {
    let overlay = use_overlay(UseOverlayInput {
        is_dismissable: true.into(),
        should_close_on_blur: true.into(),
        is_open: Signal::stored(true),
        on_close: close,
        is_keyboard_dismiss_disabled: Signal::stored(false),
        should_close_on_interact_outside: None,
        group: None,
    });
    let UseListBoxReturn { listbox, data } = use_listbox(listbox);

    view! {
        <FocusScope restore_focus=true>
            <div {..overlay.overlay.attrs()} class="demo-select-popover">
                <div {..listbox.attrs()}>
                    {FRUITS
                        .map(|(key, label)| {
                            view! { <FruitOption list=data.clone() key=key label=label /> }
                        })
                        .collect_view()}
                </div>
            </div>
        </FocusScope>
    }
}

/// An option. The hook sets `aria-selected`; the focus is shown through `data-focused`, rendered from `is_focused`.
#[component]
fn FruitOption(list: ListBoxData, key: &'static str, label: &'static str) -> impl IntoView {
    let UseOptionReturn {
        option, is_focused, ..
    } = use_option(UseOptionInput {
        list,
        key: Key::from(key),
        on_context_menu: None,
    });
    let (attrs, styles) = option.attrs();

    view! {
        <div {..attrs} style=styles data-focused=move || is_focused.get().then_some("") class="demo-select-option">
            {label}
        </div>

    }
}

/// A visually hidden native `<select>` that takes part in forms and autofill.
#[component]
fn FruitHiddenSelect(
    hidden_select: leptonic::hooks::select::UseHiddenSelectInput,
) -> impl IntoView {
    let state = hidden_select.state;
    let UseHiddenSelectReturn {
        container,
        select,
        options,
        ..
    } = use_hidden_select(hidden_select);

    view! {
        <div {..container.attrs()}>
            <select {..select.attrs()}>
                <For
                    each=move || options.get()
                    key=|option| option.key.clone()
                    let:option
                >
                    {
                        let value = option.value.clone();
                        let text = option.text.clone();
                        let selected = move || option.is_selected(&state);
                        view! {
                            <option value=value selected=selected.clone() prop:selected=selected>
                                {text}
                            </option>
                        }
                    }
                </For>
            </select>
        </div>
    }
}
