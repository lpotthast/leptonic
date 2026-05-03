use std::collections::HashSet;

use leptonic::hooks::*;
use leptos::prelude::*;
use leptos_element_capture::CapturedElement;

/// A single option component that uses the `use_option` hook.
#[component]
fn ListboxOption(
    /// The key/label for this option.
    item_key: String,
    /// The display label for this option.
    label: &'static str,
    /// The selection state from the parent listbox.
    state: UseSelectionStateReturn<String>,
    /// The currently focused key from the parent listbox.
    focused_key: Signal<Option<String>>,
) -> impl IntoView {
    let UseOptionReturn {
        option_props,
        is_selected,
        is_focused: _,
        is_disabled,
        is_focus_visible,
        ..
    } = use_option(UseOptionInput {
        key: item_key.clone(),
        state,
        is_disabled: Signal::derive(|| false),
        should_select_on_press_up: false,
        should_use_virtual_focus: false,
        should_focus_on_hover: false,
        on_focus: None,
        on_press: None,
        text_value: Some(label.to_string()),
        focused_key,
    });
    let (option_props, option_styles) = option_props.into_parts();
    let option_styles = option_styles
        .add("padding", "0.75em 1em")
        .add("transition", "all 0.15s")
        .add("display", "flex")
        .add("align-items", "center")
        .add("gap", "0.5em");

    view! {
        <div
            {..option_props}
            style=option_styles
            style:cursor=move || if is_disabled.get() { "not-allowed" } else { "pointer" }
            style:background=move || if is_selected.get() { "var(--brand-color)" } else { "transparent" }
            style:color=move || if is_selected.get() { "white" } else { "" }
            style:outline=move || if is_focus_visible.get() { "2px solid #0066cc" } else { "none" }
            style:outline-offset=move || if is_focus_visible.get() { "-2px" } else { "" }
        >
            <span style=move || {
                format!(
                    "width: 16px; height: 16px; border: 2px solid {}; border-radius: 3px; display: flex; align-items: center; justify-content: center;",
                    if is_selected.get() { "white" } else { "currentColor" },
                )
            }>
                <Show when=move || is_selected.get()>"\u{2713}"</Show>
            </span>
            {label}
        </div>
    }
}

#[component]
pub fn ListboxDemo() -> impl IntoView {
    let items: Vec<(&'static str, &'static str)> = vec![
        ("apple", "Apple"),
        ("banana", "Banana"),
        ("cherry", "Cherry"),
        ("date", "Date"),
        ("elderberry", "Elderberry"),
    ];

    // Create a signal for the items (just the keys)
    let item_keys: Vec<String> = items.iter().map(|(k, _)| k.to_string()).collect();
    let items_signal = Signal::derive({
        let keys = item_keys.clone();
        move || keys.clone()
    });

    // Set up the listbox with selection
    let listbox = use_listbox(UseListBoxInput {
        selection_mode: SelectionMode::Multiple,
        selection_behavior: SelectionBehavior::Toggle,
        is_disabled: Signal::derive(|| false),
        selected_keys: None,
        default_selected_keys: None,
        on_selection_change: None,
        disabled_keys: Signal::derive(HashSet::new),
        disallow_empty_selection: false,
        items: items_signal,
        should_focus_wrap: true,
        auto_focus: Signal::derive(|| None),
        select_on_focus: false,
        aria_label: Some("Fruits"),
        aria_labelledby: None,
        get_text_value: Some(Callback::new(|k: String| k)),
        is_virtualized: false,
        orientation: ListBoxOrientation::Vertical,
        collection_ref: CapturedElement::default(),
        on_close: None,
        escape_key_behavior: EscapeKeyBehavior::default(),
    });

    // Get selection state for displaying and for options
    let selection_state = listbox.state.collection.selection_state;
    let selected_keys = selection_state.selected_keys;
    let focused_key = listbox.state.collection.selection_state.focused_key;

    view! {
        <div
            {..listbox.listbox_props.into_attrs()}
            style="
            border: 2px solid var(--brand-color);
            border-radius: 8px;
            max-width: 250px;
            margin: 1em 0;
            overflow: hidden;
            "
        >
            {items
                .into_iter()
                .map(|(key, label)| {
                    let state = selection_state;
                    view! { <ListboxOption item_key=key.to_string() label=label state=state focused_key=focused_key /> }
                })
                .collect_view()}
        </div>

        <p>
            "Selected: "
            {move || {
                match selected_keys.get() {
                    Selection::Keys(keys) => {
                        let mut keys_vec: Vec<_> = keys.into_iter().collect();
                        keys_vec.sort();
                        format!("{keys_vec:?}")
                    }
                    Selection::All => "All".to_string(),
                }
            }}
        </p>
    }
}
