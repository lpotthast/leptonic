use leptos::*;
use leptos_use::{use_element_size, UseElementSizeReturn};

use crate::Size;

#[component]
pub fn Skeleton(
    #[prop(into, optional)] width: Option<Size>,
    #[prop(into, optional)] height: Option<Size>,
    #[prop(into, optional, default = true)] animated: bool,
    #[prop(into, optional)] id: Option<AttributeValue>,
    #[prop(into, optional)] class: Option<AttributeValue>,
    #[prop(into, optional)] style: Option<AttributeValue>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let width = width.unwrap_or(Size::Percent(100.0));
    let height = height.unwrap_or(Size::Auto);

    let element: NodeRef<html::Custom> = create_node_ref();

    let UseElementSizeReturn {
        width: el_width,
        height: _,
    } = use_element_size(element);

    view! {
        <leptonic-skeleton
            node_ref=element
            id=id
            class=class
            data-animated=animated
            style=style
            style=("--height", format!("{height}"))
            style=("--width", format!("{width}"))
            style=("--el-width", move || format!("{}px", el_width.get()))
        >
            { match children {
                Some(children) => children(),
                None => Fragment::new(vec![]),
            } }
        </leptonic-skeleton>
    }
}
