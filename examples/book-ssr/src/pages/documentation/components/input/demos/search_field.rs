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
            />
        </div>
        <p class="demo-status">{results}</p>
        <p class="demo-caption">"Enter searches, Escape or the clear button empties the field."</p>
    }
}
