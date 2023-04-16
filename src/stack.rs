use std::borrow::Cow;

use leptos::*;

use crate::OptionDeref;

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
    #[prop(into, optional)] class: Option<Cow<'static, str>>,
    #[prop(optional)] orientation: StackOrientation,
    spacing: u32,
    #[prop(into, optional)] style: Option<Cow<'static, str>>,
    children: Children,
) -> impl IntoView {
    let gap = spacing as f32 / 10.0;
    let style = style.deref_or("");
    let class = class.map(|it| it.to_owned().to_string());
    view! { cx,
        <leptonic-stack
            class=class
            orientation=orientation.as_str()
            style=format!("--gap: {gap}em; {style}")
        >
            { children(cx) }
        </leptonic-stack>
    }
}
