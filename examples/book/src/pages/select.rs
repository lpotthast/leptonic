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
        <Typography variant=TypographyVariant::H2>"Selects"</Typography>

        <Select options=vec![Foo::A, Foo::B, Foo::C] render_option=move |cx: Scope, val: &Foo| view! {cx, {format!("{val:?}")} } />
    }
}
