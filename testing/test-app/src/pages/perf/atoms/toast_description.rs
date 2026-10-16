//! A `ToastDescription` in the toasts (`[role=alertdialog]`, each a `Toast` with a `ToastContent`
//! holding a `ToastTitle` "Toast <n>" and a `ToastDescription`, and a `ToastCloseButton`) of a
//! `ToastRegion`, for its performance test
//! (`leptonic/tests/perf_tests/atoms/toast_description.rs`): no toast at first, between two plain
//! buttons (`#perf-before`, `#perf-after`) to tab from and to. Its controls ([`Control`]) add a
//! toast (without a timeout) and close the newest, mount and unmount the `ToastDescription` of the
//! toasts shown, and read how many toasts closed (`closes`).

use leptonic::{
    atoms::toast::{
        Toast, ToastCloseButton, ToastContent, ToastDescription, ToastRegion, ToastTitle,
    },
    hooks::toast::{ToastOptions, ToastQueue},
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

#[component]
pub fn PagePerf() -> impl IntoView {
    // Whether the toasts show their `ToastDescription`.
    let part = RwSignal::new(true);
    let queue = ToastQueue::<String>::new(None);
    let added = StoredValue::new(0_u32);
    let close_count = StoredValue::new(0_u32);
    let controls = vec![
        Control::action("add", move || {
            added.update_value(|added| *added += 1);
            queue.add(
                format!("Toast {}", added.get_value()),
                ToastOptions {
                    timeout: None,
                    on_close: Some(Callback::new(move |()| {
                        close_count.update_value(|count| *count += 1);
                    })),
                },
            );
        }),
        Control::action("close-newest", move || {
            if let Some(toast) = queue.visible_toasts.get_untracked().first() {
                queue.close(toast.key);
            }
        }),
        Control::action("mount", move || part.set(true)),
        Control::action("unmount", move || part.set(false)),
        Control::read("closes", move || f64::from(close_count.get_value())),
    ];
    view! {
        <h1>"ToastDescription performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <ToastRegion queue=queue let:toast>
            <Toast toast=toast.clone()>
                <ToastContent>
                    <ToastTitle>{toast.content.clone()}</ToastTitle>
                    <Show when=move || part.get()>
                        <ToastDescription>"Description"</ToastDescription>
                    </Show>
                </ToastContent>
                <ToastCloseButton>"Close"</ToastCloseButton>
            </Toast>
        </ToastRegion>
        <button id="perf-after">"After"</button>
    }
}
