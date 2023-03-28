use leptos::*;

#[component]
pub fn Tile(cx: Scope, children: Children) -> impl IntoView {
    view! { cx,
        <div class="crud-tile">
            { children(cx) }
        </div>
    }
}
