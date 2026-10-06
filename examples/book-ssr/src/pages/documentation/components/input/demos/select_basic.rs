use leptonic::components::prelude::*;
use leptos::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Fruit {
    Apple,
    Banana,
    Cherry,
}

impl std::fmt::Display for Fruit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Apple => "Apple",
            Self::Banana => "Banana",
            Self::Cherry => "Cherry",
        })
    }
}

#[component]
pub fn SelectBasicDemo() -> impl IntoView {
    let fruits = vec![Fruit::Apple, Fruit::Banana, Fruit::Cherry];
    let (selected, set_selected) = signal(Fruit::Apple);
    let (selected_multi, set_selected_multi) = signal(vec![Fruit::Apple]);

    view! {
        <div class="demo-control-stack">
            <div>
                <Select
                    options=fruits.clone()
                    search_text_provider=move |o: Fruit| o.to_string()
                    render_option=move |o: Fruit| o.to_string()
                    selected=selected
                    set_selected=set_selected
                />
                <p class="demo-status">"Selected: "{move || selected.get().to_string()}</p>
            </div>
            <div>
                <Multiselect
                    options=fruits
                    max=2
                    search_text_provider=move |o: Fruit| o.to_string()
                    render_option=move |o: Fruit| o.to_string()
                    selected=selected_multi
                    set_selected=set_selected_multi
                />
                <p class="demo-status">
                    {move || {
                        let names: Vec<String> = selected_multi.get().iter().map(ToString::to_string).collect();
                        format!("Selected (at most 2): {}", names.join(", "))
                    }}
                </p>
            </div>
        </div>
    }
}
