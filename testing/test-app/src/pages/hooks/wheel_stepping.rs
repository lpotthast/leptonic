use leptonic::hooks::interactions::{
    UseWheelSteppingInput, UseWheelSteppingReturn, use_wheel_stepping,
};
use leptos::prelude::*;

/// `use_wheel_stepping`: a field (`#test-ws-group` around the input `#test-ws-input` and a
/// button `#test-ws-inner`) stepping a count shown in `#test-ws-count`, a button before it
/// (`#test-ws-before`), buttons making it read-only (`#test-ws-read-only`) and disabled
/// (`#test-ws-disable`), and a second field whose wheel is disabled (`#test-ws-off-input`,
/// stepping `#test-ws-off-count`).
#[component]
pub fn PageHookWheelStepping() -> impl IntoView {
    let count = RwSignal::new(0_i32);
    let read_only = RwSignal::new(false);
    let disabled = RwSignal::new(false);
    let off_count = RwSignal::new(0_i32);
    view! {
        <div id="test-page-hook-wheel-stepping">
            <h1>"use_wheel_stepping"</h1>
            <button id="test-ws-before">"Before"</button>
            <Field
                id="test-ws"
                count
                is_read_only=read_only.into()
                is_disabled=disabled.into()
                is_wheel_disabled=false
            />
            <p>"Count: " <span id="test-ws-count">{move || count.get()}</span></p>
            <button id="test-ws-read-only" on:click=move |_| read_only.set(true)>
                "Read-only"
            </button>
            <button id="test-ws-disable" on:click=move |_| disabled.set(true)>
                "Disable"
            </button>
            <Field
                id="test-ws-off"
                count=off_count
                is_read_only=Signal::stored(false)
                is_disabled=Signal::stored(false)
                is_wheel_disabled=true
            />
            <p>"Count: " <span id="test-ws-off-count">{move || off_count.get()}</span></p>
        </div>
    }
}

/// A field stepping `count` with the wheel: a group (`#<id>-group`) around an input
/// (`#<id>-input`) and a button (`#<id>-inner`).
#[component]
fn Field(
    id: &'static str,
    count: RwSignal<i32>,
    is_read_only: Signal<bool>,
    is_disabled: Signal<bool>,
    is_wheel_disabled: bool,
) -> impl IntoView {
    let UseWheelSteppingReturn { group, input } = use_wheel_stepping(UseWheelSteppingInput {
        is_disabled,
        is_read_only,
        is_wheel_disabled,
        on_increment: Some(Callback::new(move |()| count.update(|c| *c += 1))),
        on_decrement: Some(Callback::new(move |()| count.update(|c| *c -= 1))),
    });
    view! {
        <div id=format!("{id}-group") {..group.attrs()}>
            <input
                id=format!("{id}-input")
                {..input.attrs()}
                prop:value=move || count.get().to_string()
            />
            <button id=format!("{id}-inner")>"Inner"</button>
        </div>
    }
}
