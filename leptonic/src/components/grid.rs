use leptos::prelude::*;

use crate::Size;

// TODO: Only allow rows as children.
#[component]
pub fn Grid(gap: Size, children: Children) -> impl IntoView {
    view! {
        <div class="leptonic-grid-container" style=("--leptonic-grid-gap", format!("{gap}"))>
            {children()}
        </div>
    }
}

// TODO: Only allow columns as children.
#[component]
pub fn Row(#[prop(into, optional)] gap: Option<Size>, children: Children) -> impl IntoView {
    view! {
        <div
            class="leptonic-grid-row"
            style=gap.map(|gap| ("--leptonic-grid-gap", format!("{gap}")))
        >
            {children()}
        </div>
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColAlign {
    #[default]
    Start,
    Center,
    End,
}

#[component]
pub fn Col(
    #[prop(optional)] xs: Option<u32>,
    #[prop(optional)] sm: Option<u32>,
    #[prop(optional)] md: Option<u32>,
    #[prop(optional)] lg: Option<u32>,
    #[prop(optional)] xl: Option<u32>,
    #[prop(optional, default = Default::default())] h_align: ColAlign,
    children: Children,
) -> impl IntoView {
    view! {
        <div
            class="leptonic-grid-col"
            class:leptonic-grid-col-flex-start=h_align == ColAlign::Start
            class:leptonic-grid-col-flex-center=h_align == ColAlign::Center
            class:leptonic-grid-col-flex-end=h_align == ColAlign::End
            data-xs=xs.unwrap_or(12)
            data-sm=sm
            data-md=md
            data-lg=lg
            data-xl=xl
        >
            {children()}
        </div>
    }
}
