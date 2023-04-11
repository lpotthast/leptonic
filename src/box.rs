use std::borrow::Cow;

use leptos::*;

#[component]
pub fn Box(
    cx: Scope,
    #[prop(into, optional)] style: Cow<'static, str>,
    children: Children,
) -> impl IntoView {
    view! { cx,
        <div class="leptonic-box" style=style.as_ref()>
            { children(cx) }
        </div>
    }
}
