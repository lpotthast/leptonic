// No upstream: an atom applying `use_hover` to its child (react-aria-components has no `Hoverable`).
use leptos::{attr::custom::custom_attribute, prelude::*};

use crate::{
    IntoAttrs,
    hooks::interactions::{
        HoverEndEvent, HoverStartEvent, UseHoverInput, UseHoverReturn, use_hover,
    },
};

/// Applies hover handling to its child, which gets `data-hovered` while hovered.
#[component]
pub fn Hoverable<V>(
    #[prop(into, optional)] is_disabled: Signal<bool>,
    #[prop(into, optional)] on_hover_start: Option<Callback<HoverStartEvent>>,
    #[prop(into, optional)] on_hover_end: Option<Callback<HoverEndEvent>>,
    /// Called when the hover state changes.
    #[prop(into, optional)]
    on_hover_change: Option<Callback<bool>>,
    /// The hoverable element.
    children: TypedChildren<V>,
) -> impl IntoView
where
    V: IntoView + 'static,
{
    let UseHoverReturn {
        props: hover_props,
        is_hovered,
    } = use_hover(UseHoverInput {
        is_disabled,
        on_hover_start,
        on_hover_end,
        on_hover_change,
    });

    children.into_inner()().add_any_attr((
        hover_props.into_attrs(),
        custom_attribute("data-hovered", move || {
            is_hovered.try_get().unwrap_or_default().then_some("true")
        }),
    ))
}
