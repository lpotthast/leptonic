use leptonic::{
    components::prelude::*,
    hooks::{
        UseEnterAnimationInput, UseEnterAnimationReturn, UseExitAnimationInput,
        UseExitAnimationReturn, use_enter_animation, use_exit_animation,
    },
    utils::CapturedElement,
};
use leptos::prelude::*;

#[component]
pub fn AnimationDemo() -> impl IntoView {
    let is_open = RwSignal::new(false);

    view! {
        <Button on_press=move |_| is_open.update(|open| *open = !*open)>
            {move || if is_open.get() { "Hide panel" } else { "Show panel" }}
        </Button>
        <AnimatedPanel is_open=is_open.into()/>
    }
}

/// A panel that fades in and out. The CSS animations run on `[data-entering]` and `[data-exiting]`; the panel stays
/// in the DOM until its exit animation finished.
#[component]
fn AnimatedPanel(is_open: Signal<bool>) -> impl IntoView {
    let element = CapturedElement::new();
    let UseEnterAnimationReturn { is_entering } = use_enter_animation(UseEnterAnimationInput {
        element,
        is_ready: is_open,
    });
    let UseExitAnimationReturn {
        is_exiting,
        exit_state,
    } = use_exit_animation(UseExitAnimationInput { element, is_open });

    view! {
        <Show when=move || is_open.get() || is_exiting.get()>
            <div
                {..element.attr()}
                class="demo-animated-panel"
                data-entering=move || is_entering.get().then_some("")
                data-exiting=move || is_exiting.get().then_some("")
            >
                "I fade in when shown and fade out before I am removed."
            </div>
        </Show>
        <p class="demo-caption">{move || format!("Exit state: {:?}", exit_state.get())}</p>
    }
}
