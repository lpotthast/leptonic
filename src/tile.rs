use leptos::*;

#[component]
pub fn Tile(cx: Scope, children: Children) -> impl IntoView {
    view! { cx,
        <div class="leptonic-tile">
            { children(cx) }
        </div>
    }
}
