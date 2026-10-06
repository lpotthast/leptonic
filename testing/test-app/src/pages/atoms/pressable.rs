use leptonic::{
    atoms::prelude::{PressResponder, Pressable},
    hooks::PressEvent,
};
use leptos::prelude::*;

/// `Pressable` on its child (react-aria's `Pressable.test.js`): a button with its own click
/// handler, a `span` with a role made focusable, a disabled one, and one inside a
/// `PressResponder`. Every press and click is appended to `#test-pressable-log`.
#[component]
pub fn PageAtomPressable() -> impl IntoView {
    let log = RwSignal::new(Vec::<String>::new());
    let responders = RwSignal::new(false);
    let entry =
        move |name: &'static str| move |_: PressEvent| log.update(|l| l.push(name.to_owned()));

    view! {
        <h1>"Pressable"</h1>
        <Pressable on_press=entry("button press")>
            <button
                id="test-pressable-button"
                on:click=move |_| log.update(|l| l.push("button click".to_owned()))
            >
                "Button"
            </button>
        </Pressable>
        <Pressable on_press=entry("span press")>
            <span id="test-pressable-span" role="button">"Span"</span>
        </Pressable>
        <Pressable is_disabled=true on_press=entry("disabled press")>
            <span id="test-pressable-disabled" role="button">"Disabled"</span>
        </Pressable>
        <PressResponder on_press=entry("responder press")>
            <Pressable on_press=entry("inner press")>
                <span id="test-pressable-responder" role="button">"In a responder"</span>
            </Pressable>
        </PressResponder>
        <div>"Log: " <span id="test-pressable-log">{move || log.get().join(", ")}</span></div>
        // Mounted on demand, so a test can watch `console.warn` (PressResponder.test.js "should
        // warn if there is no pressable child", "should not warn if there is a pressable child").
        <button id="test-pressable-mount" on:click=move |_| responders.set(true)>
            "Mount responders"
        </button>
        <Show when=move || responders.get()>
            <PressResponder>
                <div>
                    <button id="test-responder-plain">"No pressable child"</button>
                </div>
            </PressResponder>
            <PressResponder>
                <div>
                    <Pressable>
                        <button id="test-responder-pressable">"Pressable child"</button>
                    </Pressable>
                </div>
            </PressResponder>
        </Show>
    }
}
