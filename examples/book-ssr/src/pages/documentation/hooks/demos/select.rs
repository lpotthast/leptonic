use leptos::prelude::*;

#[component]
pub fn SelectDemo() -> impl IntoView {
    let (is_open, set_is_open) = signal(false);
    let (selected, set_selected) = signal::<Option<String>>(None);

    let options: &[(&str, &str)] = &[
        ("apple", "Apple"),
        ("banana", "Banana"),
        ("cherry", "Cherry"),
        ("date", "Date"),
    ];

    view! {
        <div style="position: relative; display: inline-block; min-width: 200px;">
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
                )>{"\u{25bc}"}</span>
            </button>

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
                    {options.iter().map(|(_key, label)| {
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
    }
}
