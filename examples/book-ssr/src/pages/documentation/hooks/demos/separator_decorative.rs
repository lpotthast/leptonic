use leptonic::hooks::*;
use leptos::prelude::*;

#[component]
pub fn SeparatorDecorativeDemo() -> impl IntoView {
    let separator = use_separator(UseSeparatorInput {
        element_type: SeparatorElementType::Div,
        ..UseSeparatorInput::default()
    });

    view! {
        <p>"The separator below is a div with role=\"separator\", drawn as a fading line."</p>
        <div {..separator.props.into_attrs()} class="demo-separator-fade"></div>
        <p>"Content continues below it."</p>
    }
}
