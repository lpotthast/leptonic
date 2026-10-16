//! `use_exit_animation` on a plain `<div>` (`#perf-exit`, its state as `data-state`: `open`,
//! `exiting`, `closed`), for its performance test
//! (`leptonic/tests/perf_tests/hooks/use_exit_animation.rs`): mounted and open at first. It has a
//! CSS animation from the start that runs only after a minute (so that no frame has animation work,
//! and no measured step starts an animation), which the hook awaits while exiting (`data-exiting`)
//! and the test finishes with a script; the control `unanimate` takes its class `perf-animated`
//! away. Its controls ([`Control`]) mount and unmount it, and close and open it.

use leptonic::{
    CapturedElement, flag,
    hooks::animation::{ExitState, UseExitAnimationInput, use_exit_animation},
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let open = RwSignal::new(true);
    let animated = RwSignal::new(true);
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("close", move || open.set(false)),
        Control::action("open", move || open.set(true)),
        Control::action("unanimate", move || animated.set(false)),
    ];
    view! {
        <h1>"use_exit_animation performance"</h1>
        <style>
            "@keyframes perf-exit { from { opacity: 1; } to { opacity: 0.5; } }
            .perf-animated { animation: perf-exit 1s 60s; }"
        </style>
        <PerfControls controls />
        <Show when=move || mounted.get()>
            <Exiting open animated />
        </Show>
    }
}

/// The element with `use_exit_animation`.
#[component]
fn Exiting(open: RwSignal<bool>, animated: RwSignal<bool>) -> impl IntoView {
    let element = CapturedElement::new();
    let exit = use_exit_animation(UseExitAnimationInput {
        element,
        is_open: open.into(),
        on_exit: None,
    });
    let state = move || match exit.exit_state.get() {
        ExitState::Open => "open",
        ExitState::Exiting => "exiting",
        ExitState::Closed => "closed",
    };
    view! {
        <div
            id="perf-exit"
            {..element.attr()}
            class:perf-animated=animated
            data-exiting=flag(exit.is_exiting)
            data-state=state
        >
            "Exiting"
        </div>
    }
}
