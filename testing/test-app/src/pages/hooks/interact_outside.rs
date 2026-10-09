use leptonic::{
    IntoAttrs,
    hooks::interactions::{InteractOutsideEvent, UseInteractOutsideInput, use_interact_outside},
};
use leptos::prelude::*;

/// `use_interact_outside` (react-aria's `useInteractOutside.test.js`): a target and a toggle
/// disabling the hook. Callbacks are appended to `#test-interact-outside-log`.
#[component]
pub fn PageHookInteractOutside() -> impl IntoView {
    let log = RwSignal::new(Vec::<String>::new());
    let disabled = RwSignal::new(false);
    let outside = use_interact_outside(UseInteractOutsideInput {
        is_disabled: disabled.into(),
        on_interact_outside_start: Some(Callback::new(move |_: InteractOutsideEvent| {
            log.update(|l| l.push("start".to_owned()));
        })),
        on_interact_outside: Some(Callback::new(move |_: InteractOutsideEvent| {
            log.update(|l| l.push("outside".to_owned()));
        })),
        element: None,
    });

    view! {
        <div id="test-page-hook-interact-outside">
            <h1>"use_interact_outside"</h1>
            <div
                id="test-interact-outside-target"
                style="display: inline-block; padding: 16px; border: 1px solid"
                {..outside.props.into_attrs()}
            >
                "Target"
            </div>
            <p id="test-interact-outside-away">"Outside of the target"</p>
            <button
                id="test-interact-outside-disable"
                on:click=move |_| disabled.update(|d| *d = !*d)
            >
                "Toggle disabled"
            </button>
            <button id="test-interact-outside-reset" on:click=move |_| log.set(Vec::new())>
                "Reset log"
            </button>
            <div>
                "Log: " <span id="test-interact-outside-log">{move || log.get().join(",")}</span>
            </div>
        </div>
    }
}
