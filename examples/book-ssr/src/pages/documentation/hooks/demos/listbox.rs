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
    utils::CapturedElement,
};
use leptos::prelude::*;

const FRUITS: [(&str, &str); 5] = [
    ("apple", "Apple"),
    ("banana", "Banana"),
    ("cherry", "Cherry"),
    ("date", "Date"),
    ("elderberry", "Elderberry"),
];

/// One option, rendered with `use_option`. The hook sets `aria-selected` and `aria-disabled`; keyboard focus is shown
/// through `data-focus-visible`, rendered from the hook's `is_focus_visible`.
#[component]
fn ListboxOption(list: ListBoxData, key: &'static str, label: &'static str) -> impl IntoView {
    let UseOptionReturn {
        props,
        is_selected,
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
            data-focus-visible=move || is_focus_visible.get().then_some("")
            class="demo-listbox-option"
            style=styles
        >
            <span class="demo-listbox-checkbox" aria-hidden="true">
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
        if keys.is_empty() { "Nothing selected.".to_owned() } else { format!("Selected: {}.", keys.join(", ")) }
    };

    view! {
        <div {..props.into_attrs()} class="demo-listbox">
            {FRUITS
                .map(|(key, label)| {
                    view! { <ListboxOption list=data.clone() key=key label=label /> }
                })
                .collect_view()}
        </div>

        <p class="demo-status">{selected}</p>

    }
}
