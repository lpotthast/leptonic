use leptos::prelude::*;

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
    children: Children,
) -> impl IntoView {
    view! {
        <leptonic-stack data-orientation=orientation.as_str() style=("--gap", format!("{spacing}"))>
            {children()}
        </leptonic-stack>
    }
}
