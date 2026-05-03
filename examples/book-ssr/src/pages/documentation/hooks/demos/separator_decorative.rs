use leptonic::hooks::*;
use leptos::prelude::*;

#[component]
pub fn SeparatorDecorativeDemo() -> impl IntoView {
    let div_sep = use_separator(UseSeparatorInput {
        orientation: SeparatorOrientation::Horizontal,
        element_type: SeparatorElementType::Div,
    });

    view! {
        <p>"This separator is rendered as a div with role=\"separator\""</p>
        <div
            {..div_sep.separator_props.into_attrs()}
            style="height: 2px; background: linear-gradient(90deg, transparent, #ccc, transparent); margin: 1em 0;"
        ></div>
        <p>"Content continues..."</p>
    }
}
