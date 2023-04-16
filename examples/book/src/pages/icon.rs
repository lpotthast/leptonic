use leptonic::prelude::*;
use leptos::*;
use leptos_icons::*;

#[component]
pub fn PageIcon(cx: Scope) -> impl IntoView {
    view! { cx,
        <h2>"Icons"</h2>

        <Icon icon=BsIcon::BsFolderFill/>
        <Icon icon=BsIcon::BsFolder/>
    }
}
