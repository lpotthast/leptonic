use leptos::*;

// TODO: Allow more styles / variants

#[component]
pub fn Separator(cx: Scope) -> impl IntoView {
    view! {cx,
        <hr class="leptonic-separator solid" />
    }
}
