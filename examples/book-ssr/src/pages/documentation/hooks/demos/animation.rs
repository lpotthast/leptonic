use leptonic::{
    CapturedElement,
    atoms::button::Button,
    hooks::animation::{
        UseEnterAnimationInput, UseEnterAnimationReturn, UseExitAnimationInput,
        UseExitAnimationReturn, use_enter_animation, use_exit_animation,
    },
};
use leptos::prelude::*;

#[component]
pub fn AnimationDemo() -> impl IntoView {
    let is_open = RwSignal::new(false);

    // The exit animation decides how long the panel stays mounted, so it lives outside the `Show`.
    let element = CapturedElement::new();
    let UseExitAnimationReturn {
        is_exiting,
        exit_state,
    } = use_exit_animation(UseExitAnimationInput {
        element,
        is_open: is_open.into(),
        on_exit: None,
    });

    view! {
        <Button on_press=move |_| is_open.update(|open| *open = !*open) classes="demo-btn">
            {move || if is_open.get() { "Hide panel" } else { "Show panel" }}
        </Button>
        <Show when=move || is_open.get() || is_exiting.get()>
            <AnimatedPanel element=element is_exiting=is_exiting/>
        </Show>
        <p class="demo-status">{move || format!("Exit state: {:?}", exit_state.get())}</p>
    }
}

/// The panel, mounted for each opening. The CSS animations run on `[data-entering]` and `[data-exiting]`.
#[component]
fn AnimatedPanel(element: CapturedElement, is_exiting: Signal<bool>) -> impl IntoView {
    // Created with each mount: an enter animation is tracked once per hook.
    let UseEnterAnimationReturn {
        is_entering,
        styles,
    } = use_enter_animation(UseEnterAnimationInput {
        element,
        is_ready: Signal::stored(true),
        on_enter: None,
    });

    view! {
        <div
            {..element.attr()}
            class="demo-animated-panel"
            style=styles
            data-entering=move || is_entering.get().then_some("")
            data-exiting=move || is_exiting.get().then_some("")
        >
            "I fade in when shown and fade out before I am removed."
        </div>
    }
}
