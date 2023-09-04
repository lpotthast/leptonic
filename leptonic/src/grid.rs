use leptos::*;

use crate::Size;

// TODO: Only allow rows as children.
#[component]
pub fn Grid(
    
    spacing: Size,
    #[prop(into, optional)] id: Option<AttributeValue>,
    #[prop(into, optional)] class: Option<AttributeValue>,
    #[prop(into, optional)] style: Option<AttributeValue>,
    children: Children,
) -> impl IntoView {
    view! {
        <leptonic-grid-container
            id=id
            class=class
            style=style
            style=("--leptonic-grid-spacing", format!("{spacing}"))
        >
            {children()}
        </leptonic-grid-container>
    }
}

// TODO: Only allow columns as children.
#[component]
pub fn Row(
    
    #[prop(into, optional)] spacing: Option<Size>,
    #[prop(into, optional)] id: Option<AttributeValue>,
    #[prop(into, optional)] class: Option<AttributeValue>,
    #[prop(into, optional)] style: Option<AttributeValue>,
    children: Children,
) -> impl IntoView {
    view! {
        <leptonic-grid-row
            id=id
            class=class
            style=style
            style=("--leptonic-grid-spacing", spacing.map(|spacing| format!("{spacing}")))
        >
            {children()}
        </leptonic-grid-row>
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
    #[prop(into, optional)] id: Option<AttributeValue>,
    #[prop(into, optional)] class: Option<AttributeValue>,
    #[prop(into, optional)] style: Option<AttributeValue>,
    children: Children,
) -> impl IntoView {
    view! {
        <leptonic-grid-col
            id=id
            class=class
            class:leptonic-grid-col-flex-start=h_align == ColAlign::Start
            class:leptonic-grid-col-flex-center=h_align == ColAlign::Center
            class:leptonic-grid-col-flex-end=h_align == ColAlign::End
            data-xs=xs.unwrap_or(12)
            data-sm=sm
            data-md=md
            data-lg=lg
            data-xl=xl
            style=style
        >
            {children()}
        </leptonic-grid-col>
    }
}
