use leptonic::hooks::*;
use leptos::prelude::*;

#[component]
pub fn SeparatorHorizontalDemo() -> impl IntoView {
    let horizontal_sep = use_separator(UseSeparatorInput {
        orientation: Orientation::Horizontal,
        element_type: SeparatorElementType::Hr,
    });

    view! {
        <p>"Content above the separator"</p>
        <hr {..horizontal_sep.separator_props.into_attrs()} class="demo-separator-line"/>
        <p>"Content below the separator"</p>
    }
}
