//! `use_enter_animation` on a plain `<div>` (`#perf-enter`), for its performance test
//! (`leptonic/tests/perf_tests/hooks/use_enter_animation.rs`): mounted at first, not ready (so
//! hidden). It has a CSS animation from the start that runs only after a minute (so that no frame
//! has animation work, and no measured step starts an animation), which the hook awaits while
//! entering (`data-entering`) and the test finishes with a script; the control `unanimate` takes
//! its class `perf-animated` away. Its controls ([`Control`]) mount and unmount it, and make it
//! ready (`ready`).

use leptonic::{
    CapturedElement, flag,
    hooks::animation::{UseEnterAnimationInput, use_enter_animation},
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let ready = RwSignal::new(false);
    let animated = RwSignal::new(true);
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("ready", move || ready.set(true)),
        Control::action("unanimate", move || animated.set(false)),
    ];
    view! {
        <h1>"use_enter_animation performance"</h1>
        <style>
            "@keyframes perf-enter { from { opacity: 0.5; } to { opacity: 1; } }
            .perf-animated { animation: perf-enter 1s 60s; }"
        </style>
        <PerfControls controls />
        <Show when=move || mounted.get()>
            <Entering ready animated />
        </Show>
    }
}

/// The element with `use_enter_animation`.
#[component]
fn Entering(ready: RwSignal<bool>, animated: RwSignal<bool>) -> impl IntoView {
    let element = CapturedElement::new();
    let enter = use_enter_animation(UseEnterAnimationInput {
        element,
        is_ready: ready.into(),
        on_enter: None,
    });
    view! {
        <div
            id="perf-enter"
            {..element.attr()}
            class:perf-animated=animated
            style=enter.styles
            data-entering=flag(enter.is_entering)
        >
            "Entering"
        </div>
    }
}
