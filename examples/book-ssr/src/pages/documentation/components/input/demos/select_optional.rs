use leptonic::components::prelude::*;
use leptos::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq)]
struct User {
    id: u32,
    name: String,
}

// The `Display` text identifies an option, so it must be unique among the options.
impl std::fmt::Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} (#{})", self.name, self.id)
    }
}

#[component]
pub fn SelectOptionalDemo() -> impl IntoView {
    let users = vec![
        User {
            id: 1,
            name: "Tom".to_owned(),
        },
        User {
            id: 2,
            name: "Bob".to_owned(),
        },
        User {
            id: 3,
            name: "Alice".to_owned(),
        },
    ];
    let assignee = RwSignal::new(Option::<User>::None);

    view! {
        <OptionalSelect
            label="Assignee"
            options=users
            search_text_provider=move |u: User| u.name
            render_option=move |u: User| u.name
            selected=assignee
            set_selected=assignee
            allow_deselect=true
        />
        <p class="demo-status">
            {move || assignee.get().map_or_else(|| "Unassigned.".to_owned(), |user| format!("Assigned to {}.", user.name))}
        </p>
    }
}
