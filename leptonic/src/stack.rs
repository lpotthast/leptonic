use leptos::*;

#[derive(Debug, Default)]
pub enum StackOrientation {
    #[default]
    Vertical,
    Horizontal,
}

impl StackOrientation {
    pub fn as_str(&self) -> &'static str {
        match self {
            StackOrientation::Vertical => "vertical",
            StackOrientation::Horizontal => "horizontal",
        }
    }
}

#[component]
pub fn Stack(
    cx: Scope,
    spacing: u32,
    #[prop(optional)] orientation: StackOrientation,
    #[prop(into, optional)] id: Option<AttributeValue>,
    #[prop(into, optional)] class: Option<AttributeValue>,
    #[prop(into, optional)] style: Option<AttributeValue>,
    children: Children,
) -> impl IntoView {
    let gap = spacing as f32 / 10.0;
    view! { cx,
        <leptonic-stack
            id=id
            class=class
            orientation=orientation.as_str()
            style=style
            style=("--gap", format!("{gap}em"))
        >
            { children(cx) }
        </leptonic-stack>
    }
}
