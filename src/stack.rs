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
    #[prop(optional)] orientation: StackOrientation,
    spacing: u32,
    children: Children,
) -> impl IntoView {
    view! { cx,
        <leptonic-stack
            orientation=orientation.as_str()
            style=format!("--gap: {gap}em", gap = spacing as f32 / 10.0)
        >
            { children(cx) }
        </leptonic-stack>
    }
}
