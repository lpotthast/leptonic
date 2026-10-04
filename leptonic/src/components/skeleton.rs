use leptos::{html, prelude::*};
use leptos_use::{UseElementSizeReturn, use_element_size};

use crate::utils::{
    classes::Classes,
    css::{CssDimension, computed_px, css_custom_property, pct},
    styles::Styles,
};

css_custom_property!(SKELETON_WIDTH: CssDimension = "--width");
css_custom_property!(SKELETON_HEIGHT: CssDimension = "--height");
css_custom_property!(SKELETON_EL_WIDTH: CssDimension = "--el-width");

#[component]
pub fn Skeleton(
    #[prop(into, optional)] width: Option<CssDimension>,
    #[prop(into, optional)] height: Option<CssDimension>,
    #[prop(into, optional, default = true)] animated: bool,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let width = width.unwrap_or_else(|| pct(100.0));

    let element: NodeRef<html::Div> = NodeRef::new();

    let UseElementSizeReturn {
        width: el_width,
        height: _,
    } = use_element_size(element);

    let styles = styles
        // Without an explicit height, `var(--height)` is unset and the element sizes to its content.
        .add_optional(height.map(|height| SKELETON_HEIGHT.declare(height)))
        .add(SKELETON_WIDTH.declare(width))
        .add_reactive(move || SKELETON_EL_WIDTH.declare(computed_px(el_width.get())));

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
