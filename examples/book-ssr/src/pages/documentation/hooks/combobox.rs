use crate::pages::documentation::article::Article;
use crate::pages::documentation::toc::Toc;
use indoc::indoc;

use leptonic::components::prelude::*;
use leptonic::hooks::{use_combobox, MenuTriggerAction, UseComboBoxInput};
use leptos::prelude::*;

#[component]
pub fn PageUseCombobox() -> impl IntoView {
    view! {
        <Article>
            <h1 id="combobox" class="anchor">
                "use_combobox"
                <AnchorLink href="#combobox" description="Direct link to article header"/>
            </h1>

            <p>"Hook for creating accessible comboboxes - text inputs combined with listboxes for autocomplete functionality."</p>

            <h2 id="demo" class="anchor">
                "Interactive Demo"
                <AnchorLink href="#demo" description="Direct link to demo"/>
            </h2>

            <p>"Type in the input to filter the list of fruits, or click the dropdown button to see all options."</p>

            <ComboboxDemo />

            <h2 id="use_combobox" class="anchor">
                "use_combobox"
                <AnchorLink href="#use_combobox" description="Direct link to use_combobox"/>
            </h2>

            <p>"The "<code>"use_combobox"</code>" hook manages the combobox state, filtering, and provides props for the input, button, and listbox elements."</p>

            <Code>
                {indoc!(r#"
                    use leptonic::hooks::{use_combobox, UseComboBoxInput, MenuTriggerAction};

                    let items = Signal::derive(|| vec![
                        "Apple".to_string(),
                        "Banana".to_string(),
                        "Cherry".to_string(),
                    ]);

                    let combobox = use_combobox(UseComboBoxInput {
                        items,
                        aria_label: Some("Select a fruit"),
                        placeholder: Some("Search fruits..."),
                        get_text_value: Some(Callback::new(|s: String| s)),
                        menu_trigger: MenuTriggerAction::Focus,
                        ..Default::default()
                    });

                    view! {
                        <input {..combobox.input_props.into_attrs()} />
                        <button {..combobox.button_props.into_attrs()}>"▼"</button>
                        <Show when=move || combobox.is_open.get()>
                            <ul
                                id=combobox.listbox_props.id.clone()
                                role=combobox.listbox_props.role
                            >
                                <For
                                    each=move || combobox.filtered_items.get()
                                    key=|item| item.clone()
                                    children={
                                        let select = combobox.select;
                                        move |item| {
                                            let item_clone = item.clone();
                                            view! {
                                                <li
                                                    role="option"
                                                    on:click=move |_| select.run(item_clone.clone())
                                                >
                                                    {item}
                                                </li>
                                            }
                                        }
                                    }
                                />
                            </ul>
                        </Show>
                    }
                "#)}
            </Code>

            <p>"The hook returns:"</p>
            <ul>
                <li><code>"input_props"</code>" - Props to spread onto the input element (handles events and ARIA)"</li>
                <li><code>"button_props"</code>" - Props for the dropdown toggle button"</li>
                <li><code>"listbox_props"</code>" - Props for the listbox element (id, role, aria-labelledby)"</li>
                <li><code>"is_open"</code>" - Signal indicating if the listbox is visible"</li>
                <li><code>"filtered_items"</code>" - Signal with items filtered by input value"</li>
                <li><code>"selected_key"</code>" - Signal with the currently selected key"</li>
                <li><code>"focused_key"</code>" - Signal with the currently focused key in the listbox"</li>
                <li><code>"select"</code>" - Callback to select an item"</li>
                <li><code>"open"</code>" / "<code>"close"</code>" / "<code>"toggle"</code>" - Callbacks to control visibility"</li>
                <li><code>"clear"</code>" - Callback to clear input and selection"</li>
            </ul>

            <h2 id="trigger-actions" class="anchor">
                "Menu Trigger Actions"
                <AnchorLink href="#trigger-actions" description="Direct link to trigger actions"/>
            </h2>

            <p>"The "<code>"menu_trigger"</code>" option controls when the listbox appears:"</p>

            <ul>
                <li><code>"MenuTriggerAction::Input"</code>" - Open when user types (default)"</li>
                <li><code>"MenuTriggerAction::Focus"</code>" - Open when input is focused"</li>
                <li><code>"MenuTriggerAction::Manual"</code>" - Only open via button click"</li>
            </ul>

            <h2 id="keyboard" class="anchor">
                "Keyboard Navigation"
                <AnchorLink href="#keyboard" description="Direct link to keyboard"/>
            </h2>

            <ul>
                <li><strong>"Arrow Down"</strong> " - Open listbox / move to next option"</li>
                <li><strong>"Arrow Up"</strong> " - Move to previous option"</li>
                <li><strong>"Enter"</strong> " - Select highlighted option"</li>
                <li><strong>"Escape"</strong> " - Close listbox"</li>
                <li><strong>"Home/End"</strong> " - Jump to first/last option"</li>
            </ul>

            <h2 id="filtering" class="anchor">
                "Filtering"
                <AnchorLink href="#filtering" description="Direct link to filtering"/>
            </h2>

            <p>"Items are filtered automatically based on the input value. The default filter performs a case-insensitive substring match using the "<code>"get_text_value"</code>" callback."</p>

            <p>"You can provide a custom filter function via the "<code>"filter"</code>" option:"</p>

            <Code>
                {indoc!(r"
                    let combobox = use_combobox(UseComboBoxInput {
                        items,
                        filter: Some(Callback::new(|(query, items): (String, Vec<String>)| {
                            // Custom filtering logic
                            items.into_iter()
                                .filter(|item| item.starts_with(&query))
                                .collect()
                        })),
                        ..Default::default()
                    });
                ")}
            </Code>

            <h2 id="accessibility" class="anchor">
                "Accessibility"
                <AnchorLink href="#accessibility" description="Direct link to accessibility"/>
            </h2>

            <ul>
                <li>"Full keyboard navigation support"</li>
                <li>"ARIA combobox pattern with proper roles and attributes"</li>
                <li><code>"aria-expanded"</code>", "<code>"aria-controls"</code>", "<code>"aria-activedescendant"</code>" managed automatically"</li>
                <li>"Input associated with listbox via "<code>"aria-controls"</code></li>
                <li>"Screen reader announcements for filtered results"</li>
            </ul>

            <h2 id="features" class="anchor">
                "Features"
                <AnchorLink href="#features" description="Direct link to features"/>
            </h2>

            <ul>
                <li>"Text input with dropdown suggestions"</li>
                <li>"Automatic filtering based on input value"</li>
                <li>"Multiple trigger modes (Input, Focus, Manual)"</li>
                <li>"Full keyboard navigation"</li>
                <li>"Controlled and uncontrolled usage"</li>
                <li>"Custom value support (allows_custom_value option)"</li>
                <li>"Disabled keys support"</li>
                <li>"Integration with form submission (name attribute)"</li>
            </ul>
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "use_combobox", link: "#combobox" },
                Toc::Leaf { title: "Demo", link: "#demo" },
                Toc::Leaf { title: "use_combobox API", link: "#use_combobox" },
                Toc::Leaf { title: "Trigger Actions", link: "#trigger-actions" },
                Toc::Leaf { title: "Keyboard Navigation", link: "#keyboard" },
                Toc::Leaf { title: "Filtering", link: "#filtering" },
                Toc::Leaf { title: "Accessibility", link: "#accessibility" },
                Toc::Leaf { title: "Features", link: "#features" },
            ]
        }/>
    }
}

/// Interactive demo of the use_combobox hook
#[component]
fn ComboboxDemo() -> impl IntoView {
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
    let listbox_id = combobox.listbox_props.id.clone();

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
                            let item_for_click = item.clone();
                            let item_for_display = item.clone();
                            let item_for_focus_check = item.clone();
                            let is_focused = Memo::new(move |_| {
                                focused_key.get().as_ref() == Some(&item_for_focus_check)
                            });
                            view! {
                                <li
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
            </Show>
        </div>

        <p>"Selected: " <strong>{ move || selected_key.get().unwrap_or_else(|| "None".to_string()) }</strong></p>
    }
}
