use indoc::indoc;
use leptonic::prelude::*;
use leptos::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Foo {
    A,
    B,
    C,
}

impl std::fmt::Display for Foo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Foo::A => f.write_str("A"),
            Foo::B => f.write_str("B"),
            Foo::C => f.write_str("C"),
        }
    }
}

#[component]
pub fn PageSelect(cx: Scope) -> impl IntoView {
    let (options, set_options) = create_signal::<Vec<Foo>>(cx, vec![]);

    let add_option = move || {
        set_options.update(|options| options.push(Foo::A));
    };

    let (selected, set_selected) = create_signal(cx, Foo::A);
    let (selected_opt, set_selected_opt) = create_signal(cx, Option::<Foo>::None);
    let (selected_multi, set_selected_multi) = create_signal(cx, vec![Foo::A, Foo::B]);

    view! { cx,
        <H2>"Selects"</H2>

        <P>"Select input provide the ability to let your users choose between different predefined values."</P>

        <P>"Lets assume this enum definition."</P>

        <Code>
            {indoc!(r#"
                #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
                enum Foo {
                    A,
                    B,
                    C,
                }
            "#)}
        </Code>

        <P>"In its simplest form, the Select component can be created with a static list of options to choose from."</P>

        <Code>
            {indoc!(r#"
                <Select
                    options=vec![Foo::A, Foo::B, Foo::C]
                    render_option=move |cx: Scope, val: &Foo| view! {cx, {format!("{val:?}")} }
                />
            "#)}
        </Code>

        <Select
            options=vec![Foo::A, Foo::B, Foo::C]
            render_option=Callback::new(cx, move |(cx, o)| format!("{o:?}").into_view(cx))
            selected=selected
            set_selected=Callback::new(cx, move |v| set_selected.set(v))
        />

        <P>"..."</P>

        <OptionalSelect
            options=vec![Foo::A, Foo::B, Foo::C]
            render_option=Callback::new(cx, move |(cx, o)| format!("{o:?}").into_view(cx))
            selected=selected_opt
            set_selected=Callback::new(cx, move |v| set_selected_opt.set(v))
            allow_deselect=true
        />

        <P>"..."</P>

        <Multiselect
            options=vec![Foo::A, Foo::B, Foo::C]
            render_option=Callback::new(cx, move |(cx, o)| format!("{o:?}").into_view(cx))
            selected=selected_multi
            set_selected=Callback::new(cx, move |v| set_selected_multi.set(v))
        />

        <P>"..."</P>

        <Button on_click=move |_| add_option()>
            "Add option"
        </Button>

        <div style="margin-bottom: 10em;"></div>
    }
}
