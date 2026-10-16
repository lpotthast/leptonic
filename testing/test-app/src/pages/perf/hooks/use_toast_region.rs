//! `use_toast_region` on a plain `<div class="perf-region">` showing the toasts of a queue (plain
//! `<div role="alertdialog" tabindex="0">`s with a title "Toast <n>" and a close button
//! `.perf-close`), for its performance test
//! (`leptonic/tests/perf_tests/hooks/use_toast_region.rs`): no toast at first, between two plain
//! buttons (`#perf-before`, `#perf-after`) to tab from and to, mounted at first. Its controls
//! ([`Control`]) mount and unmount it, add a toast (`add`: without a timeout, `add-timed`: with an
//! hour's), add [`MANY`] toasts (`fill`), close the newest, and set and unset its label
//! (`aria_label`).

use std::time::Duration;

use leptonic::{
    CapturedElement, IntoAttrs,
    hooks::toast::{
        ToastOptions, ToastQueue, UseToastRegionInput, UseToastRegionReturn, use_toast_region,
    },
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

/// The toasts `fill` adds.
const MANY: usize = 50;

/// The region: `use_toast_region` on a plain `<div>`, with the queue's toasts.
#[component]
fn HookRegion(queue: ToastQueue<String>, label: RwSignal<Option<String>>) -> impl IntoView {
    let element = CapturedElement::new();
    let UseToastRegionReturn { region_props } = use_toast_region(UseToastRegionInput {
        queue,
        element,
        aria_label: label.into(),
        aria_labelledby: Signal::default(),
    });
    view! {
        <div {..region_props.into_attrs()} {..element.attr()} class="perf-region">
            <For
                each=move || queue.visible_toasts.get()
                key=|toast| toast.key
                children=move |toast| {
                    let key = toast.key;
                    view! {
                        <div role="alertdialog" tabindex="0" class="perf-toast">
                            {toast.content}
                            <button class="perf-close" on:click=move |_| queue.close(key)>
                                "Close"
                            </button>
                        </div>
                    }
                }
            />
        </div>
    }
}

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let label = RwSignal::new(None::<String>);
    let queue = ToastQueue::<String>::new(None);
    let added = StoredValue::new(0_u32);
    let add = move |timeout: Option<Duration>| {
        added.update_value(|added| *added += 1);
        queue.add(
            format!("Toast {}", added.get_value()),
            ToastOptions {
                timeout,
                on_close: None,
            },
        );
    };
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("add", move || add(None)),
        Control::action("add-timed", move || add(Some(Duration::from_secs(3600)))),
        Control::action("fill", move || {
            for _ in 0..MANY {
                add(None);
            }
        }),
        Control::action("close-newest", move || {
            if let Some(toast) = queue.visible_toasts.get_untracked().first() {
                queue.close(toast.key);
            }
        }),
        Control::action("set-label", move || label.set(Some("Alerts".to_owned()))),
        Control::action("unset-label", move || label.set(None)),
    ];
    view! {
        <h1>"use_toast_region performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <Show when=move || mounted.get()>
            <HookRegion queue label />
        </Show>
        <button id="perf-after">"After"</button>
    }
}
