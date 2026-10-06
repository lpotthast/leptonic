use std::fmt;

use leptonic::components::prelude::*;
use leptos::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq)]
enum CoffeeSize {
    Small,
    Medium,
    Large,
}

impl fmt::Display for CoffeeSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Small => "Small",
            Self::Medium => "Medium",
            Self::Large => "Large",
        })
    }
}

#[component]
pub fn SelectConceptDemo() -> impl IntoView {
    let size = RwSignal::new(CoffeeSize::Medium);
    let disabled = RwSignal::new(false);

    view! {
        <Select
            label="Coffee size"
            options=vec![CoffeeSize::Small, CoffeeSize::Medium, CoffeeSize::Large]
            selected=size
            set_selected=size
            search_text_provider=|size: CoffeeSize| size.to_string()
            render_option=|size: CoffeeSize| size.to_string()
            is_disabled=disabled
        />
        <p class="demo-status">"Selected: "{move || size.get().to_string()}</p>
        <div class="demo-controls">
            <Checkbox is_selected=disabled set_selected=disabled>"Disabled"</Checkbox>
        </div>
    }
}
