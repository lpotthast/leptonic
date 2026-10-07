use leptonic::atoms::{
    checkbox::Checkbox,
    field::Label,
    input::Input,
    search_field::{SearchField, SearchFieldClearButton},
};
use leptos::prelude::*;

#[component]
pub fn SearchFieldConceptDemo() -> impl IntoView {
    let (query, set_query) = signal(None::<String>);
    let disabled = RwSignal::new(false);

    view! {
        // `data-empty` on the field hides the clear button while there is nothing to clear.
        <SearchField
            on_submit=move |query: String| set_query.set(Some(query))
            on_clear=move |()| set_query.set(None)
            is_disabled=disabled
            classes=["demo-field", "demo-search-field"]
        >
            <Label classes="demo-field-label">"Search recipes"</Label>
            <div class="demo-input-row">
                <Input classes="demo-atom-input"/>
                <SearchFieldClearButton classes=["demo-btn", "demo-search-field-clear"]>
                    <span aria-hidden="true">"\u{2715}"</span>
                </SearchFieldClearButton>
            </div>
        </SearchField>
        <p class="demo-status">
            {move || match query.get() {
                Some(query) => format!("Searching for \u{201c}{query}\u{201d}."),
                None => "No search yet.".to_owned(),
            }}
        </p>
        <div class="demo-controls">
            <Checkbox is_selected=disabled set_selected=disabled classes="demo-check">
                <span class="demo-check-box" aria-hidden="true"></span>
                "Disabled"
            </Checkbox>
        </div>
    }
}
