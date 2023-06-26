use indoc::indoc;
use leptonic::prelude::*;
use leptos::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
    let (selected, set_selected) = create_signal(cx, Foo::A);
    let (selected_opt, set_selected_opt) = create_signal(cx, Option::<Foo>::None);
    let (selected_multi, set_selected_multi) = create_signal(cx, vec![Foo::A, Foo::B]);
    let (selected_multi2, set_selected_multi2) = create_signal(cx, vec![Foo::A]);

    view! { cx,
        <H1>"Selects"</H1>

        <P>"Select inputs allow you to choose between different predefined values."</P>

        <P>"Lets assume this type definition, providing us with a set of values to choose from."</P>

        <Code>
            {indoc!(r#"
                #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
                enum Foo {
                    A,
                    B,
                    C,
                }
            "#)}
        </Code>

        <H2>"Variants"</H2>

        <P>"There are three variants of the select component, accepting different inputs and changing only slightly in its behavior."</P>

        <H3>"Select"</H3>

        <P>"The simplest form, requiring a selected option to be present the whole time."</P>

        <Code>
            {indoc!(r#"
                let (selected, set_selected) = create_signal(cx, Foo::A);

                view! {cx,
                    <Select
                        options=vec![Foo::A, Foo::B, Foo::C]
                        render_option=create_callback(cx, move |(_cx, o)| format!("{o:?}"))
                        selected=selected
                        set_selected=create_callback(cx, move |v| set_selected.set(v))
                    />
                }
            "#)}
        </Code>

        <Select
            options=vec![Foo::A, Foo::B, Foo::C]
            render_option=create_callback(cx, move |(_cx, o)| format!("{o:?}"))
            selected=selected
            set_selected=create_callback(cx, move |v| set_selected.set(v))
        />

        <H3>"OptionalSelect"</H3>

        <P>"As the name implies, this variant stores its chosen value in an " <Code inline=true>"Option"</Code> ", allowing the select to be initialized without a value and optionally allowing the user to deselect the current value."</P>

        <Code>
            {indoc!(r#"
                let (selected_opt, set_selected_opt) = create_signal(cx, Option::<Foo>::None);

                view! {cx,
                    <OptionalSelect
                        options=vec![Foo::A, Foo::B, Foo::C]
                        render_option=create_callback(cx, move |(_cx, o)| format!("{o:?}"))
                        selected=selected_opt
                        set_selected=create_callback(cx, move |v| set_selected_opt.set(v))
                        allow_deselect=true
                    />
                }
            "#)}
        </Code>

        <OptionalSelect
            options=vec![Foo::A, Foo::B, Foo::C]
            render_option=create_callback(cx, move |(_cx, o)| format!("{o:?}"))
            selected=selected_opt
            set_selected=create_callback(cx, move |v| set_selected_opt.set(v))
            allow_deselect=true
        />

        <H3>"Multiselect"</H3>

        <P>"In its simplest form, the Select component can be created with a static list of options to choose from."</P>

        <Code>
            {indoc!(r#"
                let (selected_multi, set_selected_multi) = create_signal(cx, vec![Foo::A, Foo::B]);

                view! {cx,
                    <Multiselect
                        options=vec![Foo::A, Foo::B, Foo::C]
                        render_option=create_callback(cx, move |(_cx, o)| format!("{o:?}"))
                        selected=selected_multi
                        set_selected=create_callback(cx, move |v| set_selected_multi.set(v))
                    />
                }
            "#)}
        </Code>

        <Multiselect
            options=vec![Foo::A, Foo::B, Foo::C]
            render_option=create_callback(cx, move |(_cx, o)| format!("{o:?}"))
            selected=selected_multi
            set_selected=create_callback(cx, move |v| set_selected_multi.set(v))
        />

        <P>"Using the "<Code inline=true>"max"</Code>" prop, a maximum number of selectable elements can be specified. Here: 2"</P>

        <Multiselect
            options=vec![Foo::A, Foo::B, Foo::C]
            max=2
            render_option=create_callback(cx, move |(_cx, o)| format!("{o:?}"))
            selected=selected_multi2
            set_selected=create_callback(cx, move |v| set_selected_multi2.set(v))
        />

        <H2>"Keyboard navigation"</H2>

        <P>
            "The select component was designed with keyboard navigation in mind. "
            "Press "<Code inline=true>"Tab"</Code>" to jump to the next or "<Code inline=true>"Shift + Tab"</Code>" to jump to the previous select. "
            "Open the dropdown using "<Code inline=true>"Enter"</Code>". "
            "Preselect an available option using the "<Code inline=true>"ArrowDown"</Code>" and "<Code inline=true>"ArrowUp"</Code>" keys. "
            "When the dropdown is not open, starting to preselect an element using the arrow keys will open it. "
            "Choose an option by pressing "<Code inline=true>"Enter"</Code>". "
            "Close the dropdown by pressing "<Code inline=true>"Escape"</Code>"."
        </P>

        <P>
            "Select options can be searched. When opening the dropdown of available options, focus will automatically jump to the search input, allowing you to type instantly. "
            "When closing the dropdown, focus is automatically restored to the select, allowing you to "<Code inline=true>"Tab"</Code>" to the next element."
        </P>

        <H2>"Styling"</H2>

        <P>"You may overwrite any of the following CSS variables to meet your styling needs."</P>

        <Code>
            {indoc!(r#"
                --select-padding
                --select-min-height
                --select-selected-color
                --select-selected-background-color
                --select-selected-border
                --select-selected-border-bottom
                --select-selected-border-radius
                --select-selected-badge-color
                --select-selected-badge-background-color
                --select-selected-placeholder-color
                --select-focus-highlight-background-color
                --select-dropdown-background-color
                --select-dropdown-shadow
                --select-search-color
                --select-search-background-color
                --select-no-items-color
                --select-no-items-background-color
                --select-item-color
                --select-item-background-color
                --select-item-padding
                --select-item-disabled-background-color
                --select-item-disabled-color
                --select-item-preselected-background-color
                --select-item-hover-background-color
                --select-item-selected-background-color
            "#)}
        </Code>
    }
}
