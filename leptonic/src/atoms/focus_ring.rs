use leptos::prelude::*;
use web_sys::FocusEvent;

use crate::hooks::{use_focus_ring, UseFocusRingInput, UseFocusRingReturn};

/// Context for accessing focus ring state from child components.
#[derive(Clone, Copy)]
pub struct FocusRingContext {
    pub is_focused: Signal<bool>,
    pub is_focus_visible: Signal<bool>,
}

#[component(transparent)]
pub fn FocusRing(
    #[prop(into, optional)] disabled: Option<Signal<bool>>,
    #[prop(optional)] within: bool,
    #[prop(optional)] auto_focus: bool,
    #[prop(into, optional)] on_focus: Option<Callback<FocusEvent>>,
    #[prop(into, optional)] on_blur: Option<Callback<FocusEvent>>,
    #[prop(into, optional)] on_focus_change: Option<Callback<bool>>,
    children: Children,
) -> impl IntoView {
    let UseFocusRingReturn {
        is_focus_visible,
        is_focused,
        props: focus_ring_props,
    } = use_focus_ring(UseFocusRingInput {
        disabled: disabled.unwrap_or(false.into()),
        within,
        auto_focus,
        on_focus,
        on_blur,
        on_focus_change,
    });
    let (on_focus, on_blur, on_focusin, on_focusout, data_focus_visible) =
        focus_ring_props.into_attrs();

    provide_context(FocusRingContext {
        is_focused,
        is_focus_visible,
    });

    children()
        .into_view()
        .add_any_attr(on_focus)
        .add_any_attr(on_blur)
        .add_any_attr(on_focusin)
        .add_any_attr(on_focusout)
        .add_any_attr(data_focus_visible)
}
