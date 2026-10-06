use leptonic::{
    atoms::focus_scope::FocusScope,
    components::prelude::*,
    hooks::{
        IntoAttrs, ListBoxData, UseHiddenSelectReturn, UseListBoxInput, UseListBoxReturn,
        UseOptionInput, UseOptionReturn, UseOverlayInput, UseSelectInput, UseSelectReturn,
        UseSelectStateInput,
        collections::{Key, use_list_collection},
        use_button, use_hidden_select, use_listbox, use_option, use_overlay, use_select,
        use_select_state,
    },
    utils::classes::Classes,
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
    let collection = use_list_collection(
        Signal::stored(FRUITS.to_vec()),
        |(key, _)| Key::from(*key),
        |(_, label)| (*label).to_owned(),
    );
    let state = use_select_state(UseSelectStateInput::new(collection));
    let disabled = RwSignal::new(false);

    let UseSelectReturn {
        label_props,
        trigger,
        trigger_props,
        value_props,
        listbox,
        hidden_select,
        ..
    } = use_select(UseSelectInput {
        has_label: true,
        is_disabled: disabled.into(),
        name: Some("fruit".to_owned()),
        ..UseSelectInput::new(state)
    });

    let button = use_button(trigger);
    let (button_attrs, button_styles) = button.props.into_parts();

    let selected_text = move || {
        state.selected_items().first().map_or_else(
            || "Select a fruit...".to_owned(),
            |n| n.text_value.to_string(),
        )
    };

    view! {
        <div class="demo-select">
            <span {..label_props.into_attrs()} class="demo-select-label">
                "Fruit"
            </span>
            <button
                {..button_attrs}
                {..trigger_props.into_attrs()}
                style=button_styles
                class="demo-select-trigger"
            >
                <span {..value_props.into_attrs()}>{selected_text}</span>
                <span aria-hidden="true">"\u{25bc}"</span>
            </button>

            <Show when=move || state.is_open()>
                <FruitPopover listbox=listbox.clone() close=Callback::new(move |()| state.close()) />
            </Show>

            <FruitHiddenSelect hidden_select=hidden_select.clone() />
        </div>

        <Checkbox state=disabled>"Disabled"</Checkbox>

        <p>
            "Selected: "
            <strong>{move || state.selected_key().map_or_else(|| "None".to_owned(), |k| k.to_string())}</strong>
        </p>
    }
}

/// The popover: dismissable with Escape or a click outside, returning focus to the trigger.
#[component]
fn FruitPopover(listbox: UseListBoxInput, close: Callback<()>) -> impl IntoView {
    let overlay = use_overlay(UseOverlayInput {
        is_open: Signal::stored(true),
        on_close: close,
        is_dismissable: true,
        should_close_on_blur: true,
        is_keyboard_dismiss_disabled: false,
        should_close_on_interact_outside: None,
    });
    let UseListBoxReturn { props, data } = use_listbox(listbox);

    view! {
        <FocusScope restore_focus=true>
            <div {..overlay.props.into_attrs()} class="demo-select-popover">
                <div {..props.into_attrs()}>
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

#[component]
fn FruitOption(list: ListBoxData, key: &'static str, label: &'static str) -> impl IntoView {
    let UseOptionReturn {
        props,
        is_selected,
        is_focused,
        ..
    } = use_option(UseOptionInput {
        list,
        key: Key::from(key),
    });
    let (attrs, styles) = props.into_parts();

    view! {
        <div
            {..attrs}
            style=styles
            class=Classes::from("demo-select-option")
                .add_reactive("selected", is_selected)
                .add_reactive("focused", is_focused)
        >
            {label}
        </div>
    }
}

/// A visually hidden native `<select>` that takes part in forms and autofill.
#[component]
fn FruitHiddenSelect(hidden_select: leptonic::hooks::UseHiddenSelectInput) -> impl IntoView {
    let UseHiddenSelectReturn {
        container_props,
        select_props,
        options,
        ..
    } = use_hidden_select(hidden_select);

    view! {
        <div {..container_props.into_attrs()}>
            <select {..select_props.into_attrs()}>
                <For
                    each=move || options.get()
                    key=|option| (option.value.clone(), option.is_selected)
                    let:option
                >
                    <option value=option.value selected=option.is_selected>
                        {option.text}
                    </option>
                </For>
            </select>
        </div>
    }
}
