//! `use_keyboard` on a focusable `<div>` (`#perf-keyboard`), for its performance test
//! (`leptonic/tests/perf_tests/hooks/use_keyboard.rs`): after a plain button (`#perf-before`) to
//! tab from, mounted at first, counting key downs and key ups and handling the shortcut `A`.
//! Its controls ([`Control`]) mount and unmount it, disable and enable it, and read the counts
//! (`key-downs`, `key-ups`, `shortcuts`).

use leptonic::{
    IntoAttrs, KeyboardKey, KeyboardShortcuts, Shortcut,
    hooks::interactions::{UseKeyboardInput, use_keyboard},
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

/// The key handling's input and counters.
#[derive(Clone, Copy)]
struct Inputs {
    disabled: RwSignal<bool>,
    key_down_count: StoredValue<u32>,
    key_up_count: StoredValue<u32>,
    shortcut_count: StoredValue<u32>,
}

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let inputs = Inputs {
        disabled: RwSignal::new(false),
        key_down_count: StoredValue::new(0),
        key_up_count: StoredValue::new(0),
        shortcut_count: StoredValue::new(0),
    };
    let Inputs {
        disabled,
        key_down_count,
        key_up_count,
        shortcut_count,
    } = inputs;
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("disable", move || disabled.set(true)),
        Control::action("enable", move || disabled.set(false)),
        Control::read("key-downs", move || f64::from(key_down_count.get_value())),
        Control::read("key-ups", move || f64::from(key_up_count.get_value())),
        Control::read("shortcuts", move || f64::from(shortcut_count.get_value())),
    ];
    view! {
        <h1>"use_keyboard performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <Show when=move || mounted.get()>
            <Keyed inputs />
        </Show>
    }
}

/// The element with `use_keyboard`.
#[component]
fn Keyed(inputs: Inputs) -> impl IntoView {
    let keyboard = use_keyboard(UseKeyboardInput {
        is_disabled: inputs.disabled.into(),
        on_key_down: Some(Callback::new(move |_| {
            inputs.key_down_count.update_value(|count| *count += 1);
        })),
        on_key_up: Some(Callback::new(move |_| {
            inputs.key_up_count.update_value(|count| *count += 1);
        })),
        shortcuts: Some(
            KeyboardShortcuts::new().on(Shortcut::new(KeyboardKey::A), move |_| {
                inputs.shortcut_count.update_value(|count| *count += 1);
            }),
        ),
        ..UseKeyboardInput::default()
    });
    view! {
        <div id="perf-keyboard" tabindex="0" {..keyboard.props.into_attrs()}>
            "Type here"
        </div>
    }
}
