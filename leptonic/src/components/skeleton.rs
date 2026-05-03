use leptos::{html, prelude::*};
use leptos_use::{UseElementSizeReturn, use_element_size};

use crate::utils::{
    classes::Classes,
    css::{CssDimension, CssValue, px},
    styles::Styles,
};

#[component]
pub fn Skeleton(
    #[prop(into, optional)] width: Option<CssDimension>,
    #[prop(into, optional)] height: Option<CssDimension>,
    #[prop(into, optional, default = true)] animated: bool,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let width = width.unwrap_or(CssDimension::Percent(100.0));
    let height = height.unwrap_or(CssDimension::Auto);

    let element: NodeRef<html::Div> = NodeRef::new();

    let UseElementSizeReturn {
        width: el_width,
        height: _,
    } = use_element_size(element);

    let styles = styles
        .add("--height", height)
        .add("--width", width)
        .add("--el-width", move || CssValue::from(px(el_width.get())));

    view! {
        <div
            class=classes.add("leptonic-skeleton")
            node_ref=element
            data-animated=animated
            style=styles
        >
            {match children {
                Some(children) => children().into_any(),
                None => ().into_any(),
            }}
        </div>
    }
}
