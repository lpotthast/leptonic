use leptonic::components::prelude::*;
use leptos::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Foo {
    A,
    B,
    C,
}

impl std::fmt::Display for Foo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::A => f.write_str("A"),
            Self::B => f.write_str("B"),
            Self::C => f.write_str("C"),
        }
    }
}

#[component]
pub fn SelectBasicDemo() -> impl IntoView {
    let (selected, set_selected) = signal(Foo::A);
    let (selected_multi, set_selected_multi) = signal(vec![Foo::A, Foo::B]);
    let (selected_multi2, set_selected_multi2) = signal(vec![Foo::A]);

    view! {
        <Select
            options=vec![Foo::A, Foo::B, Foo::C]
            search_text_provider=move |o| format!("{o}")
            render_option=move |o| format!("{o:?}")
            selected=selected
            set_selected=move |v| set_selected.set(v)
        />

        <Multiselect
            options=vec![Foo::A, Foo::B, Foo::C]
            search_text_provider=move |o| format!("{o}")
            render_option=move |o| format!("{o:?}")
            selected=selected_multi
            set_selected=move |v| set_selected_multi.set(v)
        />

        <p>"Using the "<Code inline=true>"max"</Code>" prop, a maximum number of selectable elements can be specified. Here: 2"</p>

        <Multiselect
            options=vec![Foo::A, Foo::B, Foo::C]
            max=2
            search_text_provider=move |o| format!("{o}")
            render_option=move |o| format!("{o:?}")
            selected=selected_multi2
            set_selected=set_selected_multi2
        />
    }
}
