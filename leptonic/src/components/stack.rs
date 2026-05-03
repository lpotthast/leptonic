use crate::utils::css::CssDimension;
use leptos::prelude::*;
use leptos_classes::Classes;
use leptos_styles::Styles;

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
    #[prop(into)] spacing: CssDimension,
    #[prop(optional)] orientation: StackOrientation,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: Children,
) -> impl IntoView {
    view! {
        <div
            class=classes.add("leptonic-stack")
            data-orientation=orientation.as_str()
            style=styles.add("--gap", spacing)
        >
            {children()}
        </div>
    }
}
