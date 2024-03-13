use leptos::*;

#[component]
pub fn Article(children: Children) -> impl IntoView {
    view! {
        <article style="width: 100%; height: 100%;">
            { children() }
        </article>
    }
}
