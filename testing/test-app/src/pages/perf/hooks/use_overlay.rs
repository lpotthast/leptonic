//! `use_overlay` alone, for its performance test
//! (`leptonic/tests/perf_tests/hooks/use_overlay.rs`): its props on a plain overlay element
//! (`.perf-overlay`, rendered while open, with a button `#perf-overlay-button` it focuses when it
//! opens), next to plain buttons (`#perf-opener` to focus before opening it, `#perf-after` to tab
//! to), mounted at first, closed, dismissable, closing on blur. The open state is the app's (its
//! `on_close` closes it). Its controls ([`Control`]) mount and unmount it, open and close it (the
//! app's state), make it dismissable or not, disable and enable dismissing it with Escape, and stop
//! and start closing it on blur.

use leptonic::hooks::overlay::{UseOverlayInput, use_overlay};
use leptos::{html, prelude::*};

use crate::pages::perf::{Control, PerfControls};

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let open = RwSignal::new(false);
    let dismissable = RwSignal::new(true);
    let keyboard_dismiss_disabled = RwSignal::new(false);
    let close_on_blur = RwSignal::new(true);
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
        Control::action("close-on-blur", move || close_on_blur.set(true)),
        Control::action("stay-open-on-blur", move || close_on_blur.set(false)),
    ];
    view! {
        <h1>"use_overlay performance"</h1>
        <PerfControls controls />
        <button id="perf-opener">"Opener"</button>
        <Show when=move || mounted.get()>
            <Subject
                open
                dismissable=dismissable.into()
                keyboard_dismiss_disabled=keyboard_dismiss_disabled.into()
                close_on_blur=close_on_blur.into()
            />
        </Show>
        <button id="perf-after">"After"</button>
    }
}

/// Calls the hook, with a plain overlay element while open.
#[component]
fn Subject(
    open: RwSignal<bool>,
    dismissable: Signal<bool>,
    keyboard_dismiss_disabled: Signal<bool>,
    close_on_blur: Signal<bool>,
) -> impl IntoView {
    let overlay = use_overlay(UseOverlayInput {
        is_open: open.into(),
        on_close: Callback::new(move |()| open.set(false)),
        is_dismissable: dismissable,
        should_close_on_blur: close_on_blur,
        is_keyboard_dismiss_disabled: keyboard_dismiss_disabled,
        should_close_on_interact_outside: None,
        group: None,
    });
    let attrs = StoredValue::new(overlay.overlay.attrs());
    view! {
        <Show when=move || open.get()>
            {
                let button = NodeRef::<html::Button>::new();
                // Focuses the overlay's button when it opens.
                Effect::new(move || {
                    if let Some(button) = button.get() {
                        let _ = button.focus();
                    }
                });
                view! {
                    <div
                        class="perf-overlay"
                        style="position: fixed; top: 100px; left: 100px; width: 300px; background: white;"
                        {..attrs.get_value()}
                    >
                        <button id="perf-overlay-button" node_ref=button>
                            "Apply"
                        </button>
                    </div>
                }
            }
        </Show>
    }
}
