//! `Toast`s (`[role=alertdialog]`, each with a `ToastContent` holding a `ToastTitle` "Toast <n>"
//! and a `ToastDescription`, and a `ToastCloseButton`) in their `ToastRegion`, for their
//! performance test (`leptonic/tests/perf_tests/atoms/toast.rs`): no toast at first, a plain
//! button (`#perf-before`) to tab from. Its controls ([`Control`]) add a toast (`add`: without a
//! timeout, `add-timed`: with an hour's, `add-expiring`: with [`EXPIRING`]), close the newest,
//! set and unset the toasts' label (`aria_label`), and read how many toasts closed (`closes`).

use std::time::Duration;

use leptonic::{
    atoms::toast::{
        Toast, ToastCloseButton, ToastContent, ToastDescription, ToastRegion, ToastTitle,
    },
    hooks::toast::{ToastOptions, ToastQueue},
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

/// The timeout of `add-expiring`: long enough for the measurement of adding the toast to end
/// before it runs out.
const EXPIRING: Duration = Duration::from_secs(2);

#[component]
pub fn PagePerf() -> impl IntoView {
    let label = RwSignal::new(None::<String>);
    let queue = ToastQueue::<String>::new(None);
    let added = StoredValue::new(0_u32);
    let close_count = StoredValue::new(0_u32);
    let add = move |timeout: Option<Duration>| {
        added.update_value(|added| *added += 1);
        queue.add(
            format!("Toast {}", added.get_value()),
            ToastOptions {
                timeout,
                on_close: Some(Callback::new(move |()| {
                    close_count.update_value(|count| *count += 1);
                })),
            },
        );
    };
    let controls = vec![
        Control::action("add", move || add(None)),
        Control::action("add-timed", move || add(Some(Duration::from_secs(3600)))),
        Control::action("add-expiring", move || add(Some(EXPIRING))),
        Control::action("close-newest", move || {
            if let Some(toast) = queue.visible_toasts.get_untracked().first() {
                queue.close(toast.key);
            }
        }),
        Control::action("set-label", move || label.set(Some("Notice".to_owned()))),
        Control::action("unset-label", move || label.set(None)),
        Control::read("closes", move || f64::from(close_count.get_value())),
    ];
    view! {
        <h1>"Toast performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <ToastRegion queue=queue let:toast>
            <Toast toast=toast.clone() aria_label=label>
                <ToastContent>
                    <ToastTitle>{toast.content.clone()}</ToastTitle>
                    <ToastDescription>"Description"</ToastDescription>
                </ToastContent>
                <ToastCloseButton>"Close"</ToastCloseButton>
            </Toast>
        </ToastRegion>
    }
}
