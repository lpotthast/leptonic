use leptos::*;

#[component]
pub fn Stack(cx: Scope, spacing: u32, children: Children) -> impl IntoView {
    assert!(spacing <= 10);
    let classes = format!("leptonic-stack spacing-{spacing}");
    view! { cx,
        <div class=classes>
            { children(cx) }
        </div>
    }
}
