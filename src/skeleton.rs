use leptos::*;

#[component]
pub fn Skeleton(
    cx: Scope,
    #[prop(into, optional)] width: Option<String>,
    #[prop(into, optional)] height: Option<String>,
    children: Children,
) -> impl IntoView {
    let width = width.unwrap_or("100%".to_owned());
    let height = height.unwrap_or("auto".to_owned());

    view! { cx,
        <leptonic-skeleton style=format!("--height: {height}; --width: {width}")>
            { children(cx) }
        </leptonic-skeleton>
    }
}
