use leptonic::hooks::*;
use leptos::prelude::*;

#[component]
pub fn SeparatorHorizontalDemo() -> impl IntoView {
    let horizontal_sep = use_separator(UseSeparatorInput {
        orientation: SeparatorOrientation::Horizontal,
        element_type: SeparatorElementType::Hr,
    });

    view! {
        <p>"Content above the separator"</p>
        <hr {..horizontal_sep.separator_props.into_attrs()} style="border: none; border-top: 1px solid #ccc; margin: 1em 0;"/>
        <p>"Content below the separator"</p>
    }
}
