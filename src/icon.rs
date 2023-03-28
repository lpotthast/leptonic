use leptos::*;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Bi {
    Unknown,
}

#[component]
pub fn Icon(cx: Scope, variant: Bi) -> impl IntoView {
    view! { cx,
        "BI"
    }
}
