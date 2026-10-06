use leptonic::{
    atoms::{
        field::Label,
        input::Input,
        search_field::{SearchField, SearchFieldClearButton},
    },
    components::prelude::Checkbox,
};
use leptos::prelude::*;

#[component]
pub fn SearchFieldAtomDemo() -> impl IntoView {
    let (submitted, set_submitted) = signal(None::<String>);
    let (cleared, set_cleared) = signal(0_u32);
    let disabled = RwSignal::new(false);

    view! {
        // `data-empty` on the field hides the clear button while there is nothing to clear.
        <SearchField
            on_submit=move |query| set_submitted.set(Some(query))
            on_clear=move |()| set_cleared.update(|n| *n += 1)
            placeholder="Search recipes\u{2026}"
            is_disabled=disabled
            classes=["demo-field", "demo-search-field"]
        >
            <Label classes="demo-field-label">"Search"</Label>
            <div class="demo-search-field-row">
                <Input classes=["demo-input", "demo-text-input", "demo-atom-input"]/>
                <SearchFieldClearButton classes=["demo-btn", "demo-search-field-clear"]>"\u{2715}"</SearchFieldClearButton>
            </div>
        </SearchField>

        <p class="demo-status">
            {move || match submitted.get() {
                Some(query) => format!("Submitted: {query}"),
                None => "Nothing submitted yet.".to_owned(),
            }}
            {move || format!(" Cleared {} times.", cleared.get())}
        </p>
        <p class="demo-caption">"Press Enter to submit, Escape to clear."</p>

        <div class="demo-toggle-settings">
            <Checkbox state=disabled>"Disabled"</Checkbox>
        </div>
    }
}
