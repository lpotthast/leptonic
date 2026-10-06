use leptonic::{
    components::prelude::{NumberField, SearchField, TextField},
    hooks::InputType,
};
use leptos::prelude::*;

/// The themed text, search and number field components. Their values are mirrored in
/// `#cmp-name-value`, `#cmp-query-value` and `#cmp-count-value`.
#[component]
pub fn PageComponentTextField() -> impl IntoView {
    let name = RwSignal::new(String::new());
    let query = RwSignal::new("leptos".to_owned());
    let count = RwSignal::new(Some(3_u8));
    let show_password = RwSignal::new(false);

    view! {
        <div id="test-page-component-text-field">
            <h1>"Text field components"</h1>
            <div id="cmp-name">
                <TextField
                    label="Name"
                    description="Your full name."
                    state=name
                    is_required=true
                />
            </div>
            <div>"Name: " <span id="cmp-name-value">{name}</span></div>

            // Reactive input type and description: a "show password" toggle.
            <div id="cmp-password">
                <TextField
                    label="Password"
                    description=Signal::derive(move || {
                        if show_password.get() { "Visible." } else { "Hidden." }.to_owned()
                    })
                    input_type=move || {
                        if show_password.get() { InputType::Text } else { InputType::Password }
                    }
                />
            </div>
            <button id="cmp-password-toggle" on:click=move |_| show_password.update(|s| *s = !*s)>
                "Show password"
            </button>

            <div id="cmp-query">
                <SearchField label="Search" state=query />
            </div>
            <div>"Query: " <span id="cmp-query-value">{query}</span></div>

            <div id="cmp-count">
                <NumberField label="Count" state=count max_value=4 />
            </div>
            <div>
                "Count: "
                <span id="cmp-count-value">
                    {move || count.get().map_or_else(|| "empty".to_owned(), |c| c.to_string())}
                </span>
            </div>
        </div>
    }
}
