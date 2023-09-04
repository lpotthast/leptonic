use leptos::*;

use crate::Margin;

#[component]
pub fn Icon(
    #[prop(into)] icon: leptos_icons::Icon,
    #[prop(into, optional)] id: Option<AttributeValue>,
    #[prop(into, optional)] class: Option<AttributeValue>,
    #[prop(into, optional)] style: Option<AttributeValue>,
    #[prop(into, optional)] aria_label: Option<AttributeValue>,
    #[prop(optional)] margin: Option<Margin>,
) -> impl IntoView {
    let ico = leptos_icons::Icon(leptos_icons::IconProps {
        icon: icon.into(),
        width: None,
        height: None,
        class: None,
        style: None,
    })
    .into_view();

    view! {
        <leptonic-icon id=id class=class aria_label=aria_label style=style style=("--margin", move || margin.map(|it| format!("{it}")))>
            { ico }
        </leptonic-icon>
    }
}
