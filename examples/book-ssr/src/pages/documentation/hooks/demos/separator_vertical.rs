use leptonic::hooks::*;
use leptos::prelude::*;

#[component]
pub fn SeparatorVerticalDemo() -> impl IntoView {
    let vertical_sep = use_separator(UseSeparatorInput {
        orientation: Orientation::Vertical,
        element_type: SeparatorElementType::Div,
    });

    view! {
        <div class="demo-flex-center-row">
            <span>"Left content"</span>
            <div {..vertical_sep.separator_props.into_attrs()} class="demo-separator-vertical"></div>
            <span>"Right content"</span>
        </div>
    }
}
