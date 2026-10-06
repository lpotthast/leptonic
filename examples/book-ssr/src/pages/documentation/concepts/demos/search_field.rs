use leptonic::components::prelude::*;
use leptos::prelude::*;

#[component]
pub fn SearchFieldConceptDemo() -> impl IntoView {
    let (query, set_query) = signal(None::<String>);
    let disabled = RwSignal::new(false);

    view! {
        <div class="demo-form">
            <SearchField
                label="Search recipes"
                on_submit=move |query: String| set_query.set(Some(query))
                on_clear=move |()| set_query.set(None)
                is_disabled=disabled
            />
        </div>
        <p class="demo-status">
            {move || match query.get() {
                Some(query) => format!("Searching for \u{201c}{query}\u{201d}."),
                None => "No search yet.".to_owned(),
            }}
        </p>
        <div class="demo-controls">
            <Checkbox is_selected=disabled set_selected=disabled>"Disabled"</Checkbox>
        </div>
    }
}
