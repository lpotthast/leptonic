use std::collections::HashSet;

use leptonic::{
    hooks::{
        IntoAttrs, ListBoxData, SelectionMode, UseListBoxInput, UseListBoxReturn, UseOptionInput,
        UseOptionReturn,
        collections::{
            CollectionOptions, Key, SelectionOptions, UseListStateInput, use_list_collection,
            use_list_state,
        },
        use_listbox, use_option,
    },
    utils::{CapturedElement, classes::Classes},
};
use leptos::prelude::*;

const FRUITS: [(&str, &str); 5] = [
    ("apple", "Apple"),
    ("banana", "Banana"),
    ("cherry", "Cherry"),
    ("date", "Date"),
    ("elderberry", "Elderberry"),
];

/// One option, rendered with `use_option`.
#[component]
fn ListboxOption(list: ListBoxData, key: &'static str, label: &'static str) -> impl IntoView {
    let UseOptionReturn {
        props,
        is_selected,
        is_disabled,
        is_focus_visible,
        ..
    } = use_option(UseOptionInput {
        list,
        key: Key::from(key),
    });
    let (attrs, styles) = props.into_parts();

    view! {
        <div
            {..attrs}
            class=Classes::from("demo-listbox-option")
                .add_reactive("disabled", is_disabled)
                .add_reactive("selected", is_selected)
                .add_reactive("focus-visible", is_focus_visible)
            style=styles
        >
            <span class="demo-listbox-checkbox">
                <Show when=move || is_selected.get()>"\u{2713}"</Show>
            </span>
            {label}
        </div>
    }
}

#[component]
pub fn ListboxDemo() -> impl IntoView {
    // The options, as a collection: keys identify them, texts are used for type-ahead.
    let collection = use_list_collection(
        Signal::stored(FRUITS.to_vec()),
        |(key, _)| Key::from(*key),
        |(_, label)| (*label).to_owned(),
    );
    let state = use_list_state(UseListStateInput {
        collection,
        selection: SelectionOptions {
            selection_mode: Signal::stored(SelectionMode::Multiple),
            // Disabled options can't be focused or selected.
            disabled_keys: Signal::stored(HashSet::from([Key::from("date")])),
            ..SelectionOptions::default()
        },
    });

    let UseListBoxReturn { props, data } = use_listbox(UseListBoxInput {
        aria_label: "Fruits".into(),
        options: CollectionOptions {
            should_focus_wrap: true,
            ..CollectionOptions::default()
        },
        ..UseListBoxInput::new(state, CapturedElement::new())
    });

    let selected = move || {
        let mut keys: Vec<String> = state
            .selection
            .selected_keys()
            .iter()
            .map(ToString::to_string)
            .collect();
        keys.sort();
        keys.join(", ")
    };

    view! {
        <div {..props.into_attrs()} class="demo-listbox">
            {FRUITS
                .map(|(key, label)| {
                    view! { <ListboxOption list=data.clone() key=key label=label /> }
                })
                .collect_view()}
        </div>

        <p>"Selected: " {selected}</p>
    }
}
