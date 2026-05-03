use leptos::{attr::custom::custom_attribute, ev, prelude::*};
use web_sys::FocusEvent;

use crate::hooks::*;

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
        is_text_input: false,
        on_focus,
        on_blur,
        on_focus_change,
    });

    // Extract the data_focus_visible signal before consuming the props,
    // then wrap it in a plain closure using `try_get()` for the DOM binding.
    //
    // Why: `Signal::derive` stores its value in an `ArenaItem` tied to the
    // component's reactive owner. When `FocusRing` is used inside reactive
    // iteration (`{move || items.get().map(...)}`), old components are dropped
    // on re-run, disposing the `ArenaItem`. The Leptos rendering engine then
    // calls `.get()` on the disposed signal during DOM reconciliation, which
    // panics. A plain closure is NOT arena-allocated and survives disposal;
    // `try_get()` returns `None` for disposed inner signals instead of panicking.
    let data_focus_visible_signal = focus_ring_props.data_focus_visible;
    let on_focus_attr = focus_ring_props.on_focus.into_on(ev::focus);
    let on_blur_attr = focus_ring_props.on_blur.into_on(ev::blur);
    let on_focusin_attr = focus_ring_props.on_focusin.into_on(ev::focusin);
    let on_focusout_attr = focus_ring_props.on_focusout.into_on(ev::focusout);

    provide_context(FocusRingContext {
        is_focused,
        is_focus_visible,
    });

    children()
        .into_view()
        .add_any_attr(on_focus_attr)
        .add_any_attr(on_blur_attr)
        .add_any_attr(on_focusin_attr)
        .add_any_attr(on_focusout_attr)
        .add_any_attr(custom_attribute("data-focus-visible", move || {
            data_focus_visible_signal.try_get().flatten()
        }))
}
