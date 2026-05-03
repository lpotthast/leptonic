use leptonic::hooks::*;
use leptos::prelude::*;

#[component]
pub fn SeparatorVerticalDemo() -> impl IntoView {
    let vertical_sep = use_separator(UseSeparatorInput {
        orientation: SeparatorOrientation::Vertical,
        element_type: SeparatorElementType::Div,
    });

    view! {
        <div style="display: flex; align-items: center; gap: 1em;">
            <span>"Left content"</span>
            <div
                {..vertical_sep.separator_props.into_attrs()}
                style="width: 1px; height: 24px; background: #ccc;"
            ></div>
            <span>"Right content"</span>
        </div>
    }
}
