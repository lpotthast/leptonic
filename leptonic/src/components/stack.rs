use leptos::*;

use crate::Size;

#[derive(Debug, Clone, Copy, Default)]
pub enum StackOrientation {
    #[default]
    Vertical,
    Horizontal,
}

impl StackOrientation {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Vertical => "vertical",
            Self::Horizontal => "horizontal",
        }
    }
}

#[component]
pub fn Stack(
    spacing: Size,
    #[prop(optional)] orientation: StackOrientation,
    #[prop(into, optional)] id: Option<AttributeValue>,
    #[prop(into, optional)] class: Option<AttributeValue>,
    #[prop(into, optional)] style: Option<AttributeValue>,
    children: Children,
) -> impl IntoView {
    view! {
        <leptonic-stack
            id=id
            class=class
            data-orientation=orientation.as_str()
            style=style
            style=("--gap", format!("{spacing}"))
        >
            { children() }
        </leptonic-stack>
    }
}
