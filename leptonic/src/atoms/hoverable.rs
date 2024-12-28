use leptos::prelude::*;

use crate::hooks::{use_hover, HoverEndEvent, HoverStartEvent, UseHoverInput, UseHoverReturn};

#[component]
pub fn Hoverable(
    #[prop(into, optional)] disabled: Option<Signal<bool>>,
    #[prop(into, optional)] on_hover_start: Option<Callback<HoverStartEvent>>,
    #[prop(into, optional)] on_hover_end: Option<Callback<HoverEndEvent>>,
    children: ChildrenFn,
) -> impl IntoView {
    let UseHoverReturn {
        attrs: (on_pointerenter, on_pointerleave),
        is_hovered: _,
    } = use_hover(UseHoverInput {
        disabled: disabled.unwrap_or(false.into()),
        on_hover_start,
        on_hover_end,
    });

    children().into_view()
        .add_any_attr(on_pointerenter)
        .add_any_attr(on_pointerleave)
}
