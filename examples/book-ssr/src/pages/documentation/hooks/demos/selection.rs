use std::collections::HashSet;

use leptonic::{components::prelude::*, hooks::*, utils::css::em};
use leptos::prelude::*;

#[component]
pub fn SelectionDemo() -> impl IntoView {
    let (selection_mode, set_selection_mode) = signal(SelectionMode::Multiple);

    let items: Vec<&'static str> = vec!["Apple", "Banana", "Cherry", "Date", "Elderberry"];
    let items_clone: Vec<String> = items.iter().map(std::string::ToString::to_string).collect();

    let UseSelectionStateReturn {
        selected_keys,
        is_selected,
        toggle,
        clear_selection,
        select_all,
        selection_mode: current_mode,
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
        <Stack orientation=StackOrientation::Horizontal spacing=em(1.0) attr:style="margin-bottom: 1em;">
            <button
                on:click=move |_| set_selection_mode.set(SelectionMode::None)
                style=move || format!(
                    "padding: 0.5em 1em; border-radius: 4px; cursor: pointer; {}",
                    if selection_mode.get() == SelectionMode::None { "border: 2px solid var(--brand-color); background: var(--brand-color); color: white;" } else { "border: 2px solid #ccc;" }
                )
            >
                "None"
            </button>
            <button
                on:click=move |_| set_selection_mode.set(SelectionMode::Single)
                style=move || format!(
                    "padding: 0.5em 1em; border-radius: 4px; cursor: pointer; {}",
                    if selection_mode.get() == SelectionMode::Single { "border: 2px solid var(--brand-color); background: var(--brand-color); color: white;" } else { "border: 2px solid #ccc;" }
                )
            >
                "Single"
            </button>
            <button
                on:click=move |_| set_selection_mode.set(SelectionMode::Multiple)
                style=move || format!(
                    "padding: 0.5em 1em; border-radius: 4px; cursor: pointer; {}",
                    if selection_mode.get() == SelectionMode::Multiple { "border: 2px solid var(--brand-color); background: var(--brand-color); color: white;" } else { "border: 2px solid #ccc;" }
                )
            >
                "Multiple"
            </button>
        </Stack>

        <p style="font-size: 0.875em; color: #666;">"Current mode: " <strong>{ move || format!("{current_mode:?}") }</strong></p>

        <div style="border: 2px solid var(--brand-color); border-radius: 8px; padding: 0.5em; margin: 1em 0;">
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
        </div>

        <Stack orientation=StackOrientation::Horizontal spacing=em(0.5) attr:style="margin: 1em 0;">
            <button
                on:click={
                    let items_clone = items_clone.clone();
                    move |_| { select_all.run(items_clone.clone()); }
                }
                style="padding: 0.5em 1em; border-radius: 4px; cursor: pointer; border: 1px solid #ccc;"
            >
                "Select All"
            </button>
            <button
                on:click=move |_| { clear_selection.run(()); }
                style="padding: 0.5em 1em; border-radius: 4px; cursor: pointer; border: 1px solid #ccc;"
            >
                "Clear"
            </button>
        </Stack>

        <p>"Selected: " { move || format!("{:?}", selected_keys.get()) }</p>
    }
}
