use leptonic::{atoms::prelude::*, hooks::PressEvent};
use leptos::prelude::*;

#[component]
pub fn PressResponderDemo() -> impl IntoView {
    let (log, set_log) = signal(Vec::<String>::new());

    let parent_on_press = Callback::new(move |_: PressEvent| {
        set_log.update(|l| l.push("Parent on_press (from PressResponder context)".into()));
    });

    let child_on_press = Callback::new(move |_: PressEvent| {
        set_log.update(|l| l.push("Child on_press (from Pressable)".into()));
    });

    view! {
        <PressResponder on_press=parent_on_press>
            <Pressable disabled=Signal::stored(false) on_press=child_on_press>
                <button style="padding: 0.5em 1em; cursor: pointer;">"Press me"</button>
            </Pressable>
        </PressResponder>

        <div style="margin-top: 1em; font-family: monospace; font-size: 0.85em;">
            <p><strong>"Event log:"</strong></p>
            <ul>
                {move || log.get().into_iter().map(|msg| {
                    view! { <li>{msg}</li> }
                }).collect_view()}
            </ul>
            <button style="margin-top: 0.5em; padding: 0.25em 0.5em; cursor: pointer;" on:click=move |_| set_log.set(vec![])>"Clear log"</button>
        </div>
    }
}
