use leptos::*;

use crate::Size;

#[component]
pub fn Skeleton(
    cx: Scope,
    #[prop(into, optional)] width: Option<Size>,
    #[prop(into, optional)] height: Option<Size>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let width = width.unwrap_or(Size::Percent(100.0));
    let height = height.unwrap_or(Size::Auto);

    view! { cx,
        <leptonic-skeleton style=format!("--height: {height}; --width: {width}")>
            {
                match children {
                    Some(children) => children(cx),
                    None => Fragment::new(vec![]),
                }
            }
        </leptonic-skeleton>
    }
}
