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
pub fn SelectOptionalDemo() -> impl IntoView {
    let (selected_opt, set_selected_opt) = signal(Option::<Foo>::None);

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
        <OptionalSelect
            options=vec![Foo::A, Foo::B, Foo::C]
            search_text_provider=move |o| format!("{o:?}")
            render_option=move |o| format!("{o:?}")
            selected=selected_opt
            set_selected=set_selected_opt
            allow_deselect=true
        />

        <p>"Selected user is: " { move || selected_user.get().to_string() }</p>

        <Select
            options=selectable_users.clone()
            search_text_provider=move |o: User| o.to_string()
            render_option=move |o: User| o.name
            selected=selected_user
            set_selected=move |v| set_selected_user.set(v)
        />
    }
}
