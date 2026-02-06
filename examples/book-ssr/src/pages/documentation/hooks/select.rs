use crate::pages::documentation::article::Article;
use crate::pages::documentation::toc::Toc;
use indoc::indoc;
use leptonic::atoms::link::AnchorLink;
use leptonic::components::prelude::*;
use leptos::prelude::*;

#[component]
pub fn PageUseSelectHook() -> impl IntoView {
    let (is_open, set_is_open) = signal(false);
    let (selected, set_selected) = signal::<Option<String>>(None);

    let options: &[(&str, &str)] = &[
        ("apple", "Apple"),
        ("banana", "Banana"),
        ("cherry", "Cherry"),
        ("date", "Date"),
    ];

    view! {
        <Article>
            <h1 id="select" class="anchor">
                "Select Hooks"
                <AnchorLink href="#select" description="Direct link to article header"/>
            </h1>

            <p>"Hooks for creating accessible dropdown select menus with keyboard navigation and hidden native select for form submission."</p>

            <h2 id="demo" class="anchor">
                "Interactive Demo"
                <AnchorLink href="#demo" description="Direct link to demo"/>
            </h2>

            <div style="position: relative; display: inline-block; min-width: 200px; margin: 1em 0;">
                // Trigger button
                <button
                    on:click=move |_| set_is_open.update(|v| *v = !*v)
                    aria-haspopup="listbox"
                    aria-expanded=move || is_open.get()
                    style="
                        width: 100%;
                        padding: 0.75em 1em;
                        border: 2px solid var(--brand-color);
                        border-radius: 8px;
                        background: white;
                        cursor: pointer;
                        display: flex;
                        justify-content: space-between;
                        align-items: center;
                        font-size: 1em;
                    "
                >
                    <span>{ move || selected.get().unwrap_or_else(|| "Select a fruit...".to_string()) }</span>
                    <span style=move || format!(
                        "transition: transform 0.2s; {}",
                        if is_open.get() { "transform: rotate(180deg);" } else { "" }
                    )>"▼"</span>
                </button>

                // Dropdown listbox
                <Show when=move || is_open.get()>
                    <div
                        role="listbox"
                        style="
                            position: absolute;
                            top: 100%;
                            left: 0;
                            right: 0;
                            margin-top: 4px;
                            background: white;
                            border: 1px solid #ddd;
                            border-radius: 8px;
                            box-shadow: 0 4px 12px rgba(0,0,0,0.15);
                            z-index: 100;
                            overflow: hidden;
                        "
                    >
                        {options.iter().map(|(key, label)| {
                            let label_owned = label.to_string();
                            let label_for_click = label_owned.clone();
                            let label_for_check = label_owned.clone();
                            let label_for_style = label_owned.clone();
                            view! {
                                <div
                                    role="option"
                                    aria-selected=move || selected.get().as_ref() == Some(&label_for_check)
                                    on:click=move |_| {
                                        set_selected.set(Some(label_for_click.clone()));
                                        set_is_open.set(false);
                                    }
                                    style=move || format!(
                                        "padding: 0.75em 1em; cursor: pointer; transition: background 0.15s; {}",
                                        if selected.get().as_ref() == Some(&label_for_style) {
                                            "background: var(--brand-color); color: white;"
                                        } else {
                                            "background: transparent;"
                                        }
                                    )
                                >
                                    { *label }
                                </div>
                            }
                        }).collect::<Vec<_>>()}
                    </div>
                </Show>

                // Hidden native select for forms
                <select
                    style="position: absolute; width: 1px; height: 1px; opacity: 0; overflow: hidden;"
                    aria-hidden="true"
                    tabindex="-1"
                >
                    <option value="">"Select a fruit..."</option>
                    {options.iter().map(|(key, label)| {
                        view! {
                            <option value=*key>{ *label }</option>
                        }
                    }).collect::<Vec<_>>()}
                </select>
            </div>

            <p>"Selected: " <strong>{ move || selected.get().unwrap_or_else(|| "None".to_string()) }</strong></p>

            <h2 id="use_select" class="anchor">
                "use_select"
                <AnchorLink href="#use_select" description="Direct link to use_select"/>
            </h2>

            <Code>
                {indoc!(r#"
                    let UseSelectReturn {
                        trigger_attrs,
                        value_props,
                        listbox_props,
                        hidden_props,
                        state,
                    } = use_select(UseSelectInput {
                        label: Some("Fruit".into()),
                        selected_key: selected.into(),
                        is_disabled: false.into(),
                        is_required: false.into(),
                        name: Some("fruit".into()),
                        on_selection_change: Some(Callback::new(move |key| set_selected.set(Some(key)))),
                        on_open_change: Some(Callback::new(move |open| set_is_open.set(open))),
                    });
                "#)}
            </Code>

            <p>"Returns props for:"</p>
            <ul>
                <li><code>"trigger_attrs"</code> " - The button that opens the dropdown"</li>
                <li><code>"value_props"</code> " - The displayed selected value"</li>
                <li><code>"listbox_props"</code> " - The dropdown listbox"</li>
                <li><code>"hidden_props"</code> " - Props for hidden native select"</li>
            </ul>

            <h2 id="use_hidden_select" class="anchor">
                "use_hidden_select"
                <AnchorLink href="#use_hidden_select" description="Direct link to use_hidden_select"/>
            </h2>

            <p>"Creates a hidden native select element for form submission:"</p>

            <Code>
                {indoc!(r#"
                    let UseHiddenSelectReturn { container_attrs, select_attrs, input_attrs } =
                        use_hidden_select(UseHiddenSelectInput {
                            name: "fruit".to_string(),
                            trigger_ref: trigger_ref.into(),
                            label: Some("Fruit".into()),
                            is_disabled: false.into(),
                            selected_key: selected.into(),
                        });
                "#)}
            </Code>

            <p>"The hidden select ensures:"</p>
            <ul>
                <li>"Form submission works correctly"</li>
                <li>"Autofill works for returning users"</li>
                <li>"Compatibility with form libraries"</li>
            </ul>

            <h2 id="keyboard" class="anchor">
                "Keyboard Navigation"
                <AnchorLink href="#keyboard" description="Direct link to keyboard"/>
            </h2>

            <ul>
                <li><strong>"Space/Enter"</strong> " - Open dropdown / select option"</li>
                <li><strong>"Arrow Down/Up"</strong> " - Open dropdown / navigate options"</li>
                <li><strong>"Escape"</strong> " - Close dropdown"</li>
                <li><strong>"Home/End"</strong> " - Jump to first/last option"</li>
                <li><strong>"Type characters"</strong> " - Type-ahead to jump to matching options"</li>
            </ul>

            <h2 id="features" class="anchor">
                "Features"
                <AnchorLink href="#features" description="Direct link to features"/>
            </h2>

            <ul>
                <li>"Accessible dropdown with ARIA attributes"</li>
                <li>"Keyboard navigation"</li>
                <li>"Type-ahead selection"</li>
                <li>"Hidden native select for forms"</li>
                <li>"Disabled and required states"</li>
                <li>"Custom trigger and option rendering"</li>
            </ul>
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "Select Hooks", link: "#select" },
                Toc::Leaf { title: "Demo", link: "#demo" },
                Toc::Leaf { title: "use_select", link: "#use_select" },
                Toc::Leaf { title: "use_hidden_select", link: "#use_hidden_select" },
                Toc::Leaf { title: "Keyboard Navigation", link: "#keyboard" },
                Toc::Leaf { title: "Features", link: "#features" },
            ]
        }/>
    }
}
