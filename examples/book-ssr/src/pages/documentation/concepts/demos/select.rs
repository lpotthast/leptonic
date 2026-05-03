use leptonic::components::prelude::*;
use leptos::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum CoffeeSize {
    Small,
    Medium,
    Large,
}

impl std::fmt::Display for CoffeeSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Small => write!(f, "Small"),
            Self::Medium => write!(f, "Medium"),
            Self::Large => write!(f, "Large"),
        }
    }
}

#[component]
pub fn SelectConceptDemo() -> impl IntoView {
    let (selected, set_selected) = signal(CoffeeSize::Medium);

    view! {
        <Select
            options=Signal::from(vec![CoffeeSize::Small, CoffeeSize::Medium, CoffeeSize::Large])
            selected=selected
            set_selected=set_selected
            search_text_provider=move |o| format!("{o:?}")
            render_option=move |o| format!("{o:?}")
        />
    }
}
