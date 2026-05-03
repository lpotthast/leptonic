use leptonic::hooks::*;
use leptos::prelude::*;

/// Interactive demo of the use_combobox hook
#[component]
pub fn ComboboxDemo() -> impl IntoView {
    let items: Signal<Vec<String>> = Signal::derive(|| {
        vec![
            "Apple".to_string(),
            "Apricot".to_string(),
            "Banana".to_string(),
            "Blueberry".to_string(),
            "Cherry".to_string(),
            "Date".to_string(),
            "Elderberry".to_string(),
            "Fig".to_string(),
            "Grape".to_string(),
        ]
    });

    let combobox = use_combobox(UseComboBoxInput {
        items,
        aria_label: Some("Select a fruit"),
        placeholder: Some("Search fruits..."),
        get_text_value: Some(Callback::new(|s: String| s)),
        menu_trigger: MenuTriggerAction::Focus,
        ..Default::default()
    });

    let is_open = combobox.is_open;
    let filtered_items = combobox.filtered_items;
    let select = combobox.select;
    let selected_key = combobox.selected_key;
    let focused_key = combobox.focused_key;
    let get_option_id = combobox.get_option_id;
    let listbox_id = combobox.listbox_props.id.clone();
    let popover_attrs = combobox.popover_props.into_attrs();

    view! {
        <div style="position: relative; display: inline-block; min-width: 250px; margin: 1em 0;">
            <div style="display: flex; gap: 4px;">
                <input
                    style="
                        flex: 1;
                        padding: 0.75em 1em;
                        border: 2px solid var(--brand-color);
                        border-radius: 8px;
                        font-size: 1em;
                    "
                    {..combobox.input_props.into_attrs()}
                />
                <button
                    style="
                        padding: 0.75em 1em;
                        border: 2px solid var(--brand-color);
                        border-radius: 8px;
                        background: var(--brand-color);
                        color: white;
                        cursor: pointer;
                    "
                    {..combobox.button_props.into_attrs()}
                >
                    "▼"
                </button>
            </div>

            <Show when=move || is_open.get() && !filtered_items.get().is_empty()>
                <div {..popover_attrs.clone()}>
                    <ul
                        id=listbox_id.clone()
                        role="listbox"
                        style="
                            position: absolute;
                            top: 100%;
                            left: 0;
                            right: 0;
                            margin: 4px 0 0 0;
                            padding: 0;
                            list-style: none;
                            max-height: 200px;
                            overflow-y: auto;
                            background: white;
                            border: 1px solid #ddd;
                            border-radius: 8px;
                            box-shadow: 0 4px 12px rgba(0,0,0,0.15);
                            z-index: 100;
                        "
                    >
                        <For
                            each=move || filtered_items.get()
                            key=|item| item.clone()
                            children=move |item| {
                                let option_id = get_option_id.run(item.clone());
                                let item_for_click = item.clone();
                                let item_for_display = item.clone();
                                let item_for_focus_check = item.clone();
                                let is_focused = Memo::new(move |_| {
                                    focused_key.get().as_ref() == Some(&item_for_focus_check)
                                });
                                view! {
                                    <li
                                        id=option_id
                                        role="option"
                                        aria-selected=move || if is_focused.get() { "true" } else { "false" }
                                        on:click=move |_| select.run(item_for_click.clone())
                                        style=move || format!(
                                            "padding: 0.75em 1em; cursor: pointer; transition: background 0.15s; {}",
                                            if is_focused.get() { "background: #e0e0ff;" } else { "" }
                                        )
                                    >
                                        { item_for_display }
                                    </li>
                                }
                            }
                        />
                    </ul>
                </div>
            </Show>
        </div>

        <p>"Selected: " <strong>{ move || selected_key.get().unwrap_or_else(|| "None".to_string()) }</strong></p>
    }
}
