use leptos::*;

use crate::Size;

// TODO: Only allow rows as children.
#[component]
pub fn Grid(
    cx: Scope,
    spacing: Size,
    #[prop(into, optional)] id: Option<AttributeValue>,
    #[prop(into, optional)] class: Option<AttributeValue>,
    #[prop(into, optional)] style: Option<AttributeValue>,
    children: Children,
) -> impl IntoView {
    view! { cx,
        <div
            id=id
            class=class
            class="leptonic-grid-container"
            style=style
            style=("--leptonic-grid-spacing", format!("{spacing}"))
        >
            { children(cx) }
        </div>
    }
}

// TODO: Only allow columns as children.
#[component]
pub fn Row(
    cx: Scope,
    #[prop(into, optional)] spacing: Option<Size>,
    #[prop(into, optional)] id: Option<AttributeValue>,
    #[prop(into, optional)] class: Option<AttributeValue>,
    #[prop(into, optional)] style: Option<AttributeValue>,
    children: Children,
) -> impl IntoView {
    view! { cx,
        <div
            id=id
            class=class
            class="leptonic-grid-row"
            style=style
            style=("--leptonic-grid-spacing", spacing.map(|spacing| format!("{spacing}")))
        >
            { children(cx) }
        </div>
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColAlign {
    #[default]
    Start,
    End,
}

#[component]
pub fn Col(
    cx: Scope,
    #[prop(optional)] xs: Option<u32>,
    #[prop(optional)] sm: Option<u32>,
    #[prop(optional)] md: Option<u32>,
    #[prop(optional)] lg: Option<u32>,
    #[prop(optional)] xl: Option<u32>,
    #[prop(optional, default = Default::default())] h_align: ColAlign,
    children: Children,
) -> impl IntoView {
    let mut classes = format!("leptonic-grid-col");
    if let Some(xs) = xs {
        classes.push_str(" leptonic-grid-col-");
        classes.push_str(&xs.to_string());
    }
    if let Some(sm) = sm {
        classes.push_str(" leptonic-grid-col-sm-");
        classes.push_str(&sm.to_string());
    }
    if let Some(md) = md {
        classes.push_str(" leptonic-grid-col-md-");
        classes.push_str(&md.to_string());
    }
    if let Some(lg) = lg {
        classes.push_str(" leptonic-grid-col-lg-");
        classes.push_str(&lg.to_string());
    }
    if let Some(xl) = xl {
        classes.push_str(" leptonic-grid-col-xl-");
        classes.push_str(&xl.to_string());
    }
    match h_align {
        ColAlign::Start => classes.push_str(" leptonic-grid-col-flex-start"),
        ColAlign::End => classes.push_str(" leptonic-grid-col-flex-end"),
    }

    view! { cx,
        <div class=classes>
            { children(cx) }
        </div>
    }
}
