use leptos::*;

use crate::Margin;

#[component]
pub fn Drawer(
    cx: Scope,
    #[prop(into, optional)] id: Option<String>,
    #[prop(optional)] margin: Option<Margin>,
    children: Children,
) -> impl IntoView {
    let style = margin.map(|it| format!("--margin: {it}"));

    view! { cx,
        <leptonic-drawer id=id style=style>
            { children(cx) }
        </leptonic-drawer>
    }
}
