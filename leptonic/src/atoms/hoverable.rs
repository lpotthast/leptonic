use leptos::{attr::custom::custom_attribute, prelude::*};

use crate::hooks::*;

/// Applies hover handling to its child, which gets `data-hovered` while hovered.
#[component]
#[allow(clippy::needless_pass_by_value)]
pub fn Hoverable(
    #[prop(into, optional)] is_disabled: Signal<bool>,
    #[prop(into, optional)] on_hover_start: Option<Callback<HoverStartEvent>>,
    #[prop(into, optional)] on_hover_end: Option<Callback<HoverEndEvent>>,
    children: ChildrenFn,
) -> impl IntoView {
    let UseHoverReturn {
        props: hover_props,
        is_hovered,
    } = use_hover(UseHoverInput {
        is_disabled,
        on_hover_start,
        on_hover_end,
        on_hover_change: None,
    });
    let (on_pointerenter, on_pointerleave) = hover_props.into_attrs();

    children()
        .into_view()
        .add_any_attr(on_pointerenter)
        .add_any_attr(on_pointerleave)
        .add_any_attr(custom_attribute("data-hovered", move || {
            is_hovered.try_get().unwrap_or_default().then_some("true")
        }))
}
