use std::collections::HashSet;

use leptonic::hooks::*;
use leptos::prelude::*;
use leptos_classes::Classes;

#[component]
pub fn SelectionDomainDemo() -> impl IntoView {
    let items: Vec<&'static str> = vec!["Apple", "Banana", "Cherry", "Date", "Elderberry"];

    let UseSelectionStateReturn {
        selected_keys,
        is_selected,
        toggle,
        ..
    } = use_selection_state(UseSelectionStateInput {
        selection_mode: SelectionMode::Multiple,
        selection_behavior: SelectionBehavior::Toggle,
        disabled: Signal::derive(|| false),
        selected_keys: None,
        default_selected_keys: None,
        disabled_keys: Signal::derive(HashSet::new),
        on_selection_change: None,
        disallow_empty_selection: false,
        disabled_behavior: DisabledBehavior::default(),
    });

    view! {
        {items.into_iter().map(|item| {
            let key = item.to_string();
            let key_for_click = key.clone();
            let key_for_check = key.clone();
            view! {
                <div
                    on:click=move |_| { toggle.run(key_for_click.clone()); }
                    style=move || format!(
                        "padding: 0.75em 1em; cursor: pointer; border-radius: 4px; margin: 0.25em; transition: all 0.2s; {}",
                        if is_selected.run(key_for_check.clone()) {
                            "background: var(--brand-color); color: white;"
                        } else {
                            "background: transparent;"
                        }
                    )
                >
                    { item }
                </div>
            }
        }).collect::<Vec<_>>()}
        <hr/>
        <p class=Classes::from("demo-mt-1")>
            "Selected: "<strong>{ move || format!("{:?}", selected_keys.get()) }</strong>
        </p>
    }
}
