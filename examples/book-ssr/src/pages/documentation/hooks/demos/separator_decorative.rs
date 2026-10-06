use leptonic::hooks::*;
use leptos::prelude::*;

#[component]
pub fn SeparatorDecorativeDemo() -> impl IntoView {
    let div_sep = use_separator(UseSeparatorInput {
        orientation: Orientation::Horizontal,
        element_type: SeparatorElementType::Div,
    });

    view! {
        <p>"This separator is a div with role=\"separator\", styled as a fading line."</p>
        <div {..div_sep.separator_props.into_attrs()} class="demo-separator-fade"></div>
        <p>"Content continues\u{2026}"</p>
    }
}
