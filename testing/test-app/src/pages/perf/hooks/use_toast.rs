//! Toasts of `use_toast` on plain elements (`.perf-toast`: a `<div>` with its content, a title
//! "Toast <n>" and a description, and a close button of `use_button` with the hook's
//! `close_button`), without a region, for its performance test
//! (`leptonic/tests/perf_tests/hooks/use_toast.rs`): no toast at first. Its controls
//! ([`Control`]) add a toast (`add`: without a timeout, `add-timed`: with an hour's,
//! `add-expiring`: with [`EXPIRING`]), close the newest, set and unset the toasts' label
//! (`aria_label`), and read how many toasts closed (`closes`).

use std::time::Duration;

use leptonic::{
    IntoAttrs,
    hooks::{
        button::use_button,
        toast::{QueuedToast, ToastOptions, ToastQueue, UseToastInput, UseToastReturn, use_toast},
    },
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

/// The timeout of `add-expiring`: long enough for the measurement of adding the toast to end
/// before it runs out.
const EXPIRING: Duration = Duration::from_secs(2);

/// A toast: `use_toast` on plain elements.
#[component]
fn HookToast(
    toast: QueuedToast<String>,
    queue: ToastQueue<String>,
    label: RwSignal<Option<String>>,
) -> impl IntoView {
    let title = toast.content.clone();
    let UseToastReturn {
        toast_props,
        content_props,
        title_id,
        description_props,
        close_button,
    } = use_toast(UseToastInput {
        queue,
        toast,
        aria_label: label.into(),
        aria_labelledby: Signal::default(),
        aria_describedby: Signal::default(),
    });
    let (close_attrs, close_styles) = use_button(close_button).props.into_parts();
    view! {
        <div {..toast_props.into_attrs()} class="perf-toast">
            <div {..content_props.into_attrs()} class="perf-toast-content">
                <div id=title_id>{title}</div>
                <div {..description_props.into_attrs()}>"Description"</div>
            </div>
            <button {..close_attrs} style=close_styles class="perf-toast-close">
                "Close"
            </button>
        </div>
    }
}

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
        <h1>"use_toast performance"</h1>
        <PerfControls controls />
        <div id="perf-toasts">
            <For
                each=move || queue.visible_toasts.get()
                key=|toast| toast.key
                children=move |toast| view! { <HookToast toast queue label /> }
            />
        </div>
    }
}
