//! `use_modal_backdrop` alone, for its performance test
//! (`leptonic/tests/perf_tests/hooks/use_modal_backdrop.rs`): its modal props on a plain element
//! (`.perf-modal`, rendered while open, 300px wide, 100px from the top, with a button
//! `#perf-modal-button` it focuses when it opens), with a `use_overlay_trigger_state` bound to the
//! app's open state, next to a plain button (`#perf-opener`) to focus before opening it, mounted at
//! first, closed, dismissable. Its controls ([`Control`]) mount and unmount it, open and close it
//! (the app's state), make it dismissable or not, and disable and enable dismissing it with
//! Escape.

use leptonic::hooks::{
    modal::{UseModalBackdropInput, use_modal_backdrop},
    overlay::{UseOverlayTriggerStateInput, use_overlay_trigger_state},
};
use leptos::{html, prelude::*};

use crate::pages::perf::{Control, PerfControls};

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let open = RwSignal::new(false);
    let dismissable = RwSignal::new(true);
    let keyboard_dismiss_disabled = RwSignal::new(false);
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("open", move || open.set(true)),
        Control::action("close", move || open.set(false)),
        Control::action("dismissable", move || dismissable.set(true)),
        Control::action("not-dismissable", move || dismissable.set(false)),
        Control::action("disable-keyboard-dismiss", move || {
            keyboard_dismiss_disabled.set(true);
        }),
        Control::action("enable-keyboard-dismiss", move || {
            keyboard_dismiss_disabled.set(false);
        }),
    ];
    view! {
        <h1>"use_modal_backdrop performance"</h1>
        <PerfControls controls />
        <button id="perf-opener">"Opener"</button>
        <Show when=move || mounted.get()>
            <Subject
                open
                dismissable=dismissable.into()
                keyboard_dismiss_disabled=keyboard_dismiss_disabled.into()
            />
        </Show>
    }
}

/// Calls the hook, with a plain modal element while open.
#[component]
fn Subject(
    open: RwSignal<bool>,
    dismissable: Signal<bool>,
    keyboard_dismiss_disabled: Signal<bool>,
) -> impl IntoView {
    let state = use_overlay_trigger_state(UseOverlayTriggerStateInput {
        value: Some(open.into()),
        ..UseOverlayTriggerStateInput::default()
    });
    let backdrop = use_modal_backdrop(UseModalBackdropInput {
        state,
        is_dismissable: dismissable,
        is_keyboard_dismiss_disabled: keyboard_dismiss_disabled,
        should_close_on_interact_outside: None,
        is_entering: Signal::stored(false),
    });
    let modal = backdrop.modal;
    let is_open = state.is_open;
    view! {
        <Show when=move || is_open.get()>
            {
                let button = NodeRef::<html::Button>::new();
                // Focuses the modal's button when it opens, as a dialog in it would.
                Effect::new(move || {
                    if let Some(button) = button.get() {
                        let _ = button.focus();
                    }
                });
                view! {
                    <div
                        class="perf-modal"
                        style="position: fixed; top: 100px; left: 100px; width: 300px; background: white;"
                        {..modal.attrs()}
                    >
                        <button id="perf-modal-button" node_ref=button>
                            "Apply"
                        </button>
                    </div>
                }
            }
        </Show>
    }
}
