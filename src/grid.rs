use std::borrow::Cow;

use leptos::*;

// TODO: Only allow rows as children.
#[component]
pub fn Grid(
    cx: Scope,
    spacing: u32,
    #[prop(into, optional)] class: Cow<'static, str>,
    children: Children
) -> impl IntoView {
    assert!(spacing <= 10);
    let classes = format!("leptonic-grid-container spacing-{spacing} {class}");
    view! { cx,
        <div class=classes>
            { children(cx) }
        </div>
    }
}

// TODO: Only allow columns as children.
#[component]
pub fn Row(cx: Scope,  children: Children) -> impl IntoView {
    let classes = format!("leptonic-grid-row");
    view! { cx,
        <div class=classes>
            { children(cx) }
        </div>
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColAlign {
    #[default]
    Start,
    End
}

#[component]
pub fn Col(
    cx: Scope,
    #[prop(optional)]
    xs: Option<u32>,
    #[prop(optional)]
    sm: Option<u32>,
    #[prop(optional)]
    md: Option<u32>,
    #[prop(optional)]
    lg: Option<u32>,
    #[prop(optional)]
    xl: Option<u32>,
    #[prop(optional, default = Default::default())]
    h_align: ColAlign,
    children: Children
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
