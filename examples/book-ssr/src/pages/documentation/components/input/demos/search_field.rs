use leptonic::components::prelude::*;
use leptos::prelude::*;

const RECIPES: &[&str] = &[
    "Apple pie",
    "Banana bread",
    "Carrot cake",
    "Lemon tart",
    "Pumpkin soup",
    "Tomato soup",
];

#[component]
pub fn SearchFieldDemo() -> impl IntoView {
    let (query, set_query) = signal(None::<String>);
    let disabled = RwSignal::new(false);
    let read_only = RwSignal::new(false);

    let results = move || {
        query.with(|query| match query {
            None => "Press Enter to search.".to_owned(),
            Some(query) => {
                let query = query.to_lowercase();
                let hits: Vec<&str> = RECIPES
                    .iter()
                    .copied()
                    .filter(|recipe| recipe.to_lowercase().contains(&query))
                    .collect();
                if hits.is_empty() {
                    "No recipes found.".to_owned()
                } else {
                    format!("Found: {}", hits.join(", "))
                }
            }
        })
    };

    view! {
        <div class="demo-form">
            <SearchField
                label="Recipes"
                placeholder="e.g. soup"
                on_submit=move |query: String| set_query.set(Some(query))
                on_clear=move |()| set_query.set(None)
                is_disabled=disabled
                is_read_only=read_only
            />
        </div>
        <p class="demo-status">{results}</p>

        <div class="demo-controls">
            <Checkbox is_selected=disabled set_selected=disabled>"Disabled"</Checkbox>
            <Checkbox is_selected=read_only set_selected=read_only>"Read-only"</Checkbox>
        </div>
    }
}
