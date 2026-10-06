use std::time::Duration;

use leptonic::atoms::prelude::{Focusable, Pressable, Tooltip, TooltipTrigger};
use leptos::prelude::*;

/// `Focusable` on its child (react-aria's `Focusable.test.js`), and `Focusable`/`Pressable` as
/// custom tooltip triggers (react-aria-components' `Tooltip.test.js`). Focus events are appended
/// to `#test-focusable-log`.
#[component]
pub fn PageAtomFocusable() -> impl IntoView {
    let log = RwSignal::new(Vec::<String>::new());
    let auto = RwSignal::new(false);
    let entry = move |name: &'static str| move |_| log.update(|l| l.push(name.to_owned()));

    view! {
        <h1>"Focusable"</h1>
        <button id="test-focusable-before">"Before"</button>
        <Focusable on_focus=entry("focus") on_blur=entry("blur")>
            <span id="test-focusable" role="button">"Focusable"</span>
        </Focusable>
        <Focusable is_disabled=true>
            <span id="test-focusable-disabled" role="button">"Disabled"</span>
        </Focusable>
        <Focusable exclude_from_tab_order=true>
            <span id="test-focusable-excluded" role="button">"Excluded"</span>
        </Focusable>
        <Focusable on_focus=entry("merged focus")>
            <span
                id="test-focusable-merged"
                role="button"
                tabindex="-1"
                on:focus=move |_| log.update(|l| l.push("own focus".to_owned()))
            >
                "Merged"
            </span>
        </Focusable>
        <TooltipTrigger delay=Duration::from_millis(100) close_delay=Duration::from_millis(0)>
            <Focusable>
                <span id="test-focusable-trigger" role="button">"Focusable trigger"</span>
            </Focusable>
            <Tooltip>"Focusable tooltip"</Tooltip>
        </TooltipTrigger>
        <TooltipTrigger delay=Duration::from_millis(100) close_delay=Duration::from_millis(0)>
            <Pressable>
                <span id="test-pressable-trigger" role="button">"Pressable trigger"</span>
            </Pressable>
            <Tooltip>"Pressable tooltip"</Tooltip>
        </TooltipTrigger>
        // "supports autoFocus", on a child that only becomes focusable through the atom.
        <button id="test-focusable-mount-auto" on:click=move |_| auto.set(true)>
            "Mount auto-focused"
        </button>
        <Show when=move || auto.get()>
            <Focusable auto_focus=true>
                <span id="test-focusable-auto" role="img" aria-label="Info">
                    "i"
                </span>
            </Focusable>
        </Show>
        <div>"Log: " <span id="test-focusable-log">{move || log.get().join(", ")}</span></div>
    }
}
