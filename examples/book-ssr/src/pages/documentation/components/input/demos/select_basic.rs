use std::fmt;

use leptonic::components::prelude::*;
use leptos::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Fruit {
    Apple,
    Banana,
    Cherry,
}

impl fmt::Display for Fruit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Apple => "Apple",
            Self::Banana => "Banana",
            Self::Cherry => "Cherry",
        })
    }
}

#[component]
pub fn SelectBasicDemo() -> impl IntoView {
    let fruit = RwSignal::new(Fruit::Apple);
    let disabled = RwSignal::new(false);

    view! {
        <Select
            label="Fruit"
            options=vec![Fruit::Apple, Fruit::Banana, Fruit::Cherry]
            selected=fruit
            set_selected=fruit
            search_text_provider=|fruit: Fruit| fruit.to_string()
            render_option=|fruit: Fruit| fruit.to_string()
            is_disabled=disabled
        />
        <p class="demo-status">"Selected: "{move || fruit.get().to_string()}</p>
        <div class="demo-controls">
            <Checkbox is_selected=disabled set_selected=disabled>"Disabled"</Checkbox>
        </div>
    }
}
