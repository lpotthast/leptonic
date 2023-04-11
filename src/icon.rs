use leptos::*;
use leptos_icons::*;

#[component]
pub fn Icon(cx: Scope, #[prop(into)] icon: Icon) -> impl IntoView {
    view! { cx,
        <leptonic-icon>
            <LeptosIcon icon=icon />
        </leptonic-icon>
    }
}
