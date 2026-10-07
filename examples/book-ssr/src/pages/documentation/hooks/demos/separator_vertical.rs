use leptonic::hooks::*;
use leptos::prelude::*;

#[component]
pub fn SeparatorVerticalDemo() -> impl IntoView {
    let vertical_sep = use_separator(UseSeparatorInput {
        orientation: Orientation::Vertical.into(),
        element_type: SeparatorElementType::Div,
        ..UseSeparatorInput::default()
    });

    view! {
        <div class="demo-flex-center-row">
            <span>"Left content"</span>
            <div {..vertical_sep.props.into_attrs()} class="demo-separator-line"></div>
            <span>"Right content"</span>
        </div>
    }
}
