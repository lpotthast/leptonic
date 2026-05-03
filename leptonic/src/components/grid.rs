use leptos::prelude::*;

use crate::utils::{classes::Classes, css::CssDimension, styles::Styles};

// TODO: Only allow rows as children.
#[component]
pub fn Grid(
    #[prop(into)] gap: CssDimension,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: Children,
) -> impl IntoView {
    view! {
        <div class=classes.add("leptonic-grid-container") style=styles.add("--leptonic-grid-gap", gap)>
            {children()}
        </div>
    }
}

// TODO: Only allow columns as children.
#[component]
pub fn Row(
    #[prop(into, optional)] gap: Option<CssDimension>,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: Children,
) -> impl IntoView {
    let styles = match gap {
        Some(g) => styles.add("--leptonic-grid-gap", g),
        None => styles,
    };
    view! {
        <div class=classes.add("leptonic-grid-row") style=styles>
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
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: Children,
) -> impl IntoView {
    view! {
        <div
            class=classes.add("leptonic-grid-col")
            style=styles
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
