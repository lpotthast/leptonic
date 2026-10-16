//! `use_focusable` on a plain `<div role="button">` (`#perf-focusable`), for its performance
//! test (`leptonic/tests/perf_tests/hooks/use_focusable.rs`): between two plain buttons
//! (`#perf-before`, `#perf-after`) to tab from and to, mounted at first, its focus (from
//! `on_focus_change`) as `data-focused`, with a key handler (counting key downs) and the shortcut
//! `A` (counting its uses). Its controls ([`Control`]) mount and unmount it, change its reactive
//! inputs, focus it (`focus`, its focus handle), and read the counts (`key-downs`, `shortcuts`).

use leptonic::{
    IntoAttrs, KeyboardKey, KeyboardShortcuts, Shortcut,
    hooks::focus::{FocusHandle, UseFocusableInput, UseFocusableReturn, use_focusable},
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

/// The element's reactive inputs, counters and focus handle.
#[derive(Clone, Copy)]
struct Inputs {
    disabled: RwSignal<bool>,
    excluded: RwSignal<bool>,
    key_down_count: StoredValue<u32>,
    shortcut_count: StoredValue<u32>,
    focus_handle: StoredValue<Option<FocusHandle>>,
}

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let inputs = Inputs {
        disabled: RwSignal::new(false),
        excluded: RwSignal::new(false),
        key_down_count: StoredValue::new(0),
        shortcut_count: StoredValue::new(0),
        focus_handle: StoredValue::new(None),
    };
    let Inputs {
        disabled,
        excluded,
        key_down_count,
        shortcut_count,
        focus_handle,
    } = inputs;
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("disable", move || disabled.set(true)),
        Control::action("enable", move || disabled.set(false)),
        Control::action("exclude-from-tab-order", move || excluded.set(true)),
        Control::action("include-in-tab-order", move || excluded.set(false)),
        Control::action("focus", move || {
            if let Some(handle) = focus_handle.get_value() {
                handle.focus();
            }
        }),
        Control::read("key-downs", move || f64::from(key_down_count.get_value())),
        Control::read("shortcuts", move || f64::from(shortcut_count.get_value())),
    ];
    view! {
        <h1>"use_focusable performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <Show when=move || mounted.get()>
            <Focusable inputs />
        </Show>
        <button id="perf-after">"After"</button>
    }
}

/// The element with `use_focusable`.
#[component]
fn Focusable(inputs: Inputs) -> impl IntoView {
    let focused = RwSignal::new(false);
    let UseFocusableReturn {
        props,
        focus_handle,
    } = use_focusable(UseFocusableInput {
        is_disabled: inputs.disabled.into(),
        exclude_from_tab_order: inputs.excluded.into(),
        on_focus_change: Some(Callback::new(move |is_focused| focused.set(is_focused))),
        on_key_down: Some(Callback::new(move |_| {
            inputs.key_down_count.update_value(|count| *count += 1);
        })),
        shortcuts: Some(
            KeyboardShortcuts::new().on(Shortcut::new(KeyboardKey::A), move |_| {
                inputs.shortcut_count.update_value(|count| *count += 1);
            }),
        ),
        ..UseFocusableInput::default()
    });
    inputs.focus_handle.set_value(Some(focus_handle));
    view! {
        <div
            id="perf-focusable"
            role="button"
            {..props.into_attrs()}
            data-focused=move || focused.get().then_some("true")
        >
            "Focus me"
        </div>
    }
}
