use leptos::html;
use leptos::prelude::*;
use leptos_use::{use_element_size, UseElementSizeReturn};

use crate::Size;

#[component]
pub fn Skeleton(
    #[prop(into, optional)] width: Option<Size>,
    #[prop(into, optional)] height: Option<Size>,
    #[prop(into, optional, default = true)] animated: bool,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let width = width.unwrap_or(Size::Percent(100.0));
    let height = height.unwrap_or(Size::Auto);

    let element: NodeRef<html::Custom<&str>> = NodeRef::new();

    let UseElementSizeReturn {
        width: el_width,
        height: _,
    } = use_element_size(element);

    view! {
        <leptonic-skeleton
            node_ref=element
            data-animated=animated
            style=("--height", format!("{height}"))
            style=("--width", format!("{width}"))
            style=("--el-width", Signal::derive(move || format!("{}px", el_width.get())))
        >
            {match children {
                Some(children) => children().into_any(),
                None => ().into_any(),
            }}
        </leptonic-skeleton>
    }
}
