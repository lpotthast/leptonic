//! `use_clipboard` on a focusable `<div role="button">` (`#perf-clipboard`, its item the text
//! "hello world"), for its performance test
//! (`leptonic/tests/perf_tests/hooks/use_clipboard.rs`): after a plain button (`#perf-before`)
//! to tab from, mounted at first, handling copy, cut and paste. Its controls ([`Control`])
//! mount and unmount it, disable and enable it, and read the counts (`copies`, `cuts`, `pastes`).

use leptonic::hooks::{
    clipboard::{UseClipboardInput, use_clipboard},
    dnd::DragItem,
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

/// The clipboard handling's input and counters.
#[derive(Clone, Copy)]
struct Inputs {
    disabled: RwSignal<bool>,
    copy_count: StoredValue<u32>,
    cut_count: StoredValue<u32>,
    paste_count: StoredValue<u32>,
}

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let inputs = Inputs {
        disabled: RwSignal::new(false),
        copy_count: StoredValue::new(0),
        cut_count: StoredValue::new(0),
        paste_count: StoredValue::new(0),
    };
    let Inputs {
        disabled,
        copy_count,
        cut_count,
        paste_count,
    } = inputs;
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("disable", move || disabled.set(true)),
        Control::action("enable", move || disabled.set(false)),
        Control::read("copies", move || f64::from(copy_count.get_value())),
        Control::read("cuts", move || f64::from(cut_count.get_value())),
        Control::read("pastes", move || f64::from(paste_count.get_value())),
    ];
    view! {
        <h1>"use_clipboard performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <Show when=move || mounted.get()>
            <Clipboard inputs />
        </Show>
    }
}

/// The element with `use_clipboard`.
#[component]
fn Clipboard(inputs: Inputs) -> impl IntoView {
    let clipboard = use_clipboard(UseClipboardInput {
        get_items: Some(Callback::new(|_| vec![DragItem::text("hello world")])),
        on_copy: Some(Callback::new(move |()| {
            inputs.copy_count.update_value(|count| *count += 1);
        })),
        on_cut: Some(Callback::new(move |()| {
            inputs.cut_count.update_value(|count| *count += 1);
        })),
        on_paste: Some(Callback::new(move |_| {
            inputs.paste_count.update_value(|count| *count += 1);
        })),
        is_disabled: inputs.disabled.into(),
    });
    view! {
        <div
            id="perf-clipboard"
            role="button"
            tabindex="0"
            {..clipboard.target.attrs()}
        >
            "Copy"
        </div>
    }
}
