use indoc::indoc;
use leptonic::{atoms::link::AnchorLink, components::prelude::*};
use leptos::prelude::*;

use crate::pages::documentation::{article::Article, toc::Toc};

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

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct User {
    name: String,
    value: ordered_float::OrderedFloat<f32>,
}

impl std::fmt::Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{} - {}", &self.name, &self.value))
    }
}

#[component]
#[allow(clippy::too_many_lines)]
pub fn PageSelect() -> impl IntoView {
    let (selected, set_selected) = signal(Foo::A);
    let (selected_opt, set_selected_opt) = signal(Option::<Foo>::None);
    let (selected_multi, set_selected_multi) = signal(vec![Foo::A, Foo::B]);
    let (selected_multi2, set_selected_multi2) = signal(vec![Foo::A]);

    let selectable_users = vec![
        User {
            name: "Tom".to_owned(),
            value: ordered_float::OrderedFloat(1.0),
        },
        User {
            name: "Bob".to_owned(),
            value: ordered_float::OrderedFloat(42.0),
        },
    ];

    let (selected_user, set_selected_user) = signal(selectable_users[0].clone());

    view! {
        <Article>
            <h1 id="select" class="anchor">
                "Select"
                <AnchorLink href="#select" description="Direct link to article header"/>
            </h1>

            <p>"Select inputs allow you to choose between different predefined values."</p>

            <p>"Lets assume this type definition, providing us with a set of values to choose from."</p>

            <Code>
                {indoc!(r"
                    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
                    enum Foo {
                        A,
                        B,
                        C,
                    }
                ")}
            </Code>

            <h2 id="variants" class="anchor">
                "Variants"
                <AnchorLink href="#variants" description="Direct link to section: Variants"/>
            </h2>

            <p>"There are three variants of the select component, accepting different inputs and changing only slightly in its behavior."</p>

            <h3 id="variant-select" class="anchor">
                "Select"
                <AnchorLink href="#variant-select" description="Direct link to section: Variant - Select"/>
            </h3>

            <p>"The simplest form, requiring a selected option to be present the whole time."</p>

            <Code>
                {indoc!(r#"
                    let (selected, set_selected) = signal(Foo::A);

                    view! {
                        <Select
                            options=vec![Foo::A, Foo::B, Foo::C]
                            search_text_provider=move |o| format!("{o}")
                            render_option=move |o| format!("{o:?}")
                            selected=selected
                            set_selected=move |v| set_selected.set(v)
                        />
                    }
                "#)}
            </Code>

            <Select
                options=vec![Foo::A, Foo::B, Foo::C]
                search_text_provider=move |o| format!("{o}")
                render_option=move |o| format!("{o:?}")
                selected=selected
                set_selected=move |v| set_selected.set(v)
            />

            <h3 id="variant-optional-select" class="anchor">
                "OptionalSelect"
                <AnchorLink href="#variant-optional-select" description="Direct link to section: Variant - OptionalSelect"/>
            </h3>

            <p>"As the name implies, this variant stores its chosen value in an " <Code inline=true>"Option"</Code> ", allowing the select to be initialized without a value and optionally allowing the user to deselect the current value."</p>

            <Code>
                {indoc!(r#"
                    let (selected_opt, set_selected_opt) = signal(Option::<Foo>::None);

                    view! {
                        <OptionalSelect
                            options=vec![Foo::A, Foo::B, Foo::C]
                            search_text_provider=move |o| format!("{o}")
                            render_option=move |o| format!("{o:?}")
                            selected=selected_opt
                            set_selected=set_selected_opt
                            allow_deselect=true
                        />
                    }
                "#)}
            </Code>

            <OptionalSelect
                options=vec![Foo::A, Foo::B, Foo::C]
                search_text_provider=move |o| format!("{o:?}")
                render_option=move |o| format!("{o:?}")
                selected=selected_opt
                set_selected=set_selected_opt
                allow_deselect=true
            />

            <h3 id="variant-multiselect" class="anchor">
                "Multiselect"
                <AnchorLink href="#variant-multiselect" description="Direct link to section: Variant - Multiselect"/>
            </h3>

            <p>"In its simplest form, the Select component can be created with a static list of options to choose from."</p>

            <Code>
                {indoc!(r#"
                    let (selected_multi, set_selected_multi) = signal(vec![Foo::A, Foo::B]);

                    view! {
                        <Multiselect
                            options=vec![Foo::A, Foo::B, Foo::C]
                            search_text_provider=move |o| format!("{o}")
                            render_option=move |o| format!("{o:?}")
                            selected=selected_multi
                            set_selected=move |v| set_selected_multi.set(v)
                        />
                    }
                "#)}
            </Code>

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

            <h2 id="keyboard-navigation" class="anchor">
                "Keyboard navigation"
                <AnchorLink href="#keyboard-navigation" description="Direct link to section: Keyboard navigation"/>
            </h2>

            <p>
                "The select component was designed with keyboard navigation in mind. "
                "Press "<Code inline=true>"Tab"</Code>" to jump to the next or "<Code inline=true>"Shift + Tab"</Code>" to jump to the previous select. "
                "Open the dropdown using "<Code inline=true>"Enter"</Code>". "
                "Preselect an available option using the "<Code inline=true>"ArrowDown"</Code>" and "<Code inline=true>"ArrowUp"</Code>" keys. "
                "When the dropdown is not open, starting to preselect an element using the arrow keys will open it. "
                "Choose an option by pressing "<Code inline=true>"Enter"</Code>". "
                "Close the dropdown by pressing "<Code inline=true>"Escape"</Code>"."
            </p>

            <p>
                "Select options can be searched. When opening the dropdown of available options, focus will automatically jump to the search input, allowing you to type instantly. "
                "When closing the dropdown, focus is automatically restored to the select, allowing you to "<Code inline=true>"Tab"</Code>" to the next element."
            </p>

            <h2 id="customization" class="anchor">
                "Customization"
                <AnchorLink href="#customization" description="Direct link to section: Customization"/>
            </h2>

            <p>"Let's define a select component which allows selection from a list of struct values."</p>

            <Code>
                {indoc!(r#"
                    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
                    struct User {
                        name: String,
                        value: ordered_float::OrderedFloat<f32>,
                    }

                    impl std::fmt::Display for User {
                        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                            f.write_fmt(format_args!("{} - {}", &self.name, &self.value))
                        }
                    }

                    let selectable_users = vec![
                        User {
                            name: "Tom".to_owned(),
                            value: ordered_float::OrderedFloat(1.0),
                        },
                        User {
                            name: "Bob".to_owned(),
                            value: ordered_float::OrderedFloat(42.0),
                        },
                    ];

                    let (selected_user, set_selected_user) = signal(selectable_users[0].clone());

                    view! {
                        <p>"Selected user is: " { move || selected_user.get().to_string() }</p>
                        <Select
                            options=selectable_users.clone()
                            search_text_provider=move |o| o.to_string()
                            render_option=move |o: User| o.name
                            selected=selected_user
                            set_selected=move |v| set_selected_user.set(v)
                        />
                    }
                "#)}
            </Code>

            <p>"Selected user is: " { move || selected_user.get().to_string() }</p>

            <Select
                options=selectable_users.clone()
                search_text_provider=move |o: User| o.to_string()
                render_option=move |o: User| o.name
                selected=selected_user
                set_selected=move |v| set_selected_user.set(v)
            />

            <h2 id="styling" class="anchor">
                "Styling"
                <AnchorLink href="#styling" description="Direct link to section: Styling"/>
            </h2>

            <p>"You may overwrite any of the following CSS variables to meet your styling needs."</p>

            <Code>
                {indoc!(r"
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
                    --select-focused-border-color
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
                ")}
            </Code>
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "Select", link: "#select" },
                Toc::Group { title: "Variants", link: "#variants", inner: vec![
                    Toc::Leaf { title: "Select", link: "#variant-select" },
                    Toc::Leaf { title: "OptionalSelect", link: "#variant-optional-select" },
                    Toc::Leaf { title: "Multiselect", link: "#variant-multiselect" },
                ]},
                Toc::Leaf { title: "Keyboard navigation", link: "#keyboard-navigation" },
                Toc::Leaf { title: "Customization", link: "#customization" },
                Toc::Leaf { title: "Styling", link: "#styling" },
            ]
        }/>
    }
}
