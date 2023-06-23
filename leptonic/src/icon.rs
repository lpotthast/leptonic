use leptos::*;
use leptos_icons::*;

use crate::Margin;

#[component]
pub fn Icon(
    cx: Scope,
    #[prop(into)] icon: Icon,
    #[prop(into, optional)] id: Option<AttributeValue>,
    #[prop(into, optional)] class: Option<AttributeValue>,
    #[prop(into, optional)] style: Option<AttributeValue>,
    #[prop(optional)] margin: Option<Margin>,
) -> impl IntoView {
    view! { cx,
        <leptonic-icon id=id class=class style=style style=("--margin", move || margin.map(|it| format!("{it}")))>
            <LeptosIcon icon=icon />
        </leptonic-icon>
    }
}
