use std::fmt;

use leptonic::components::prelude::*;
use leptos::prelude::*;

// `Multiselect` keeps its selection sorted, so options implement `Ord`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Topping {
    Cheese,
    Mushrooms,
    Olives,
    Peppers,
}

impl fmt::Display for Topping {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Cheese => "Cheese",
            Self::Mushrooms => "Mushrooms",
            Self::Olives => "Olives",
            Self::Peppers => "Peppers",
        })
    }
}

#[component]
pub fn SelectMultipleDemo() -> impl IntoView {
    let toppings = RwSignal::new(vec![Topping::Cheese]);

    view! {
        <Multiselect
            label="Toppings (at most two)"
            options=vec![Topping::Cheese, Topping::Mushrooms, Topping::Olives, Topping::Peppers]
            max=2
            selected=toppings
            set_selected=toppings
            search_text_provider=|topping: Topping| topping.to_string()
            render_option=|topping: Topping| topping.to_string()
        />
        <p class="demo-status">
            {move || {
                let names: Vec<String> = toppings.get().iter().map(ToString::to_string).collect();
                if names.is_empty() { "No toppings selected.".to_owned() } else { format!("Selected: {}.", names.join(", ")) }
            }}
        </p>
    }
}
