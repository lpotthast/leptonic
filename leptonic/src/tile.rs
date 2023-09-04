use leptos::*;

#[component]
pub fn Tile( children: Children) -> impl IntoView {
    view! {
        <div class="leptonic-tile">
            { children() }
        </div>
    }
}
