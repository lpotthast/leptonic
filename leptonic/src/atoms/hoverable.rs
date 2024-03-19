use leptos::{
    ev::{pointerenter, pointerleave},
    *,
};

use crate::hooks::{use_hover, HoverEndEvent, HoverStartEvent, UseHoverInput, UseHoverReturn};

#[component]
pub fn Hoverable(
    #[prop(into, optional)] disabled: Option<MaybeSignal<bool>>,
    #[prop(into, optional)] on_hover_start: Option<Callback<HoverStartEvent>>,
    #[prop(into, optional)] on_hover_end: Option<Callback<HoverEndEvent>>,
    children: ChildrenFn,
) -> impl IntoView {
    let UseHoverReturn {
        props,
        is_hovered: _,
    } = use_hover(UseHoverInput {
        disabled: disabled.unwrap_or(false.into()),
        on_hover_start,
        on_hover_end,
    });

    children()
        .into_view()
        .on(pointerenter, props.on_pointer_enter)
        .on(pointerleave, props.on_pointer_leave)
}
