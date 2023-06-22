use crate::prelude::*;
use leptos::*;

#[component]
pub fn Quicksearch(cx: Scope) -> impl IntoView {
    let (input, set_input) = create_signal(cx, "".to_owned());
    view! { cx,
        <Input
            get=input
            set=move |new_value| set_input.set(new_value)
            label="Search..."
            prepend=view! {cx, "Cmd+Shift+F"}.into_view(cx)
        />
    }
}
