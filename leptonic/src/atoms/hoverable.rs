use leptos::prelude::*;

use crate::hooks::*;

#[component]
#[allow(clippy::needless_pass_by_value)]
pub fn Hoverable(
    #[prop(into, optional)] disabled: Option<Signal<bool>>,
    #[prop(into, optional)] on_hover_start: Option<Callback<HoverStartEvent>>,
    #[prop(into, optional)] on_hover_end: Option<Callback<HoverEndEvent>>,
    children: ChildrenFn,
) -> impl IntoView {
    let UseHoverReturn {
        props: hover_props,
        is_hovered: _,
    } = use_hover(UseHoverInput {
        disabled: disabled.unwrap_or(false.into()),
        on_hover_start,
        on_hover_end,
        on_hover_change: None,
    });
    let (on_pointerenter, on_pointerleave) = hover_props.into_attrs();

    children()
        .into_view()
        .add_any_attr(on_pointerenter)
        .add_any_attr(on_pointerleave)
}
