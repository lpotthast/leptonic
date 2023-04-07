use leptos::*;
use leptos_icons::*;

#[component]
pub fn Icon(cx: Scope, #[prop(into)] icon: Icon) -> impl IntoView {
    view! { cx,
        <LeptosIcon class="leptonic-icon" icon=icon />
    }
}
