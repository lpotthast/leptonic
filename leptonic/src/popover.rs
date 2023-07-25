use leptos::*;

#[component]
pub fn Popover(cx: Scope, children: Children) -> impl IntoView {
    view! {cx,
        <leptonic-popover>
            <leptonic-popover-content>
                { children(cx) }
            </leptonic-popover-content>
        </leptonic-popover>
    }
}
