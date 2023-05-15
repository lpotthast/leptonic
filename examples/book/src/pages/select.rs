use leptonic::prelude::*;
use leptos::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Foo {
    A,
    B,
    C,
}

#[component]
pub fn PageSelect(cx: Scope) -> impl IntoView {
    view! { cx,
        <H2>"Selects"</H2>

        <Select options=vec![Foo::A, Foo::B, Foo::C] render_option=move |cx: Scope, val: &Foo| view! {cx, {format!("{val:?}")} } />
    }
}
