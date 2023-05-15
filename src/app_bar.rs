use leptos::*;

use crate::Height;

#[component]
pub fn AppBar(
    cx: Scope,
    #[prop(optional)] height: Option<Height>,
    children: Children,
) -> impl IntoView {
    let style = height.map(|it| format!("--app-bar-height: {it}"));
    view! { cx,
        <leptonic-app-bar style=style>
            { children(cx) }
        </leptonic-app-bar>
    }
}
