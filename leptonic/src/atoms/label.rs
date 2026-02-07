use crate::hooks::*;
use crate::utils::classes::Classes;
use crate::utils::styles::Styles;
use leptos::prelude::*;

#[component]
pub fn Label(
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: Children,
) -> impl IntoView {
    let UseLabelReturn {
        label_props,
        field_props: _,
    } = use_label(UseLabelInput {
        id: Some(uuid::Uuid::new_v4().to_string()),
        label_element_type: Some(LabelElementType::Label),
    });

    view! {
        <label {..label_props.into_attrs()} class=classes style=styles>
            {children()}
        </label>
    }
}
