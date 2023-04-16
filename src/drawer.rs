use leptos::*;

#[component]
pub fn Drawer(cx: Scope, children: Children) -> impl IntoView {
    view! { cx,
        <leptonic-drawer>
            { children(cx) }
        </leptonic-drawer>
    }
}
