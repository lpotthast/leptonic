use leptos::*;

use crate::Margin;

#[component]
pub fn Drawer(
    cx: Scope,
    #[prop(optional)] margin: Option<Margin>,
    children: Children,
) -> impl IntoView {
    let style = margin.map(|it| format!("--margin: {it}"));

    view! { cx,
        <leptonic-drawer style=style>
            { children(cx) }
        </leptonic-drawer>
    }
}
