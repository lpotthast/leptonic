use leptos::*;
use leptos_icons::*;

use crate::Margin;

#[component]
pub fn Icon(
    cx: Scope,
    #[prop(into)] icon: Icon,
    #[prop(optional)] margin: Option<Margin>,
) -> impl IntoView {
    let style = margin.map(|it| format!("--margin: {it}"));

    view! { cx,
        <leptonic-icon style=style>
            <LeptosIcon icon=icon />
        </leptonic-icon>
    }
}
