//! `use_toast_state` alone (a queue showing up to [`MAX_VISIBLE`] toasts), for its performance
//! test (`leptonic/tests/perf_tests/hooks/use_toast_state.rs`): the number of toasts shown in
//! `#perf-visible`, mounted at first. Its controls ([`Control`]) mount and unmount it, and call
//! its methods: add a toast (`add`: without a timeout, `add-timed`: with an hour's), add
//! [`MANY`] toasts (`fill`), close the newest, pause and resume the timeouts, and clear it.

use std::time::Duration;

use leptonic::hooks::toast::{ToastOptions, ToastQueue, UseToastStateInput, use_toast_state};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

/// The toasts the queue shows at most.
const MAX_VISIBLE: usize = 5;
/// The toasts `fill` adds.
const MANY: usize = 100;

/// The queue, the number of its toasts shown. Hands the queue to the page's controls.
#[component]
fn Queue(queue: StoredValue<Option<ToastQueue<String>>>) -> impl IntoView {
    let toasts = use_toast_state::<String>(UseToastStateInput {
        max_visible_toasts: MAX_VISIBLE,
    });
    queue.set_value(Some(toasts));
    on_cleanup(move || queue.set_value(None));
    view! {
        <p>
            "Visible: " <span id="perf-visible">{move || toasts.visible_toasts.with(Vec::len)}</span>
        </p>
    }
}

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let queue = StoredValue::new(None::<ToastQueue<String>>);
    let added = StoredValue::new(0_u32);
    let add = move |timeout: Option<Duration>| {
        if let Some(queue) = queue.get_value() {
            added.update_value(|added| *added += 1);
            queue.add(
                format!("Toast {}", added.get_value()),
                ToastOptions {
                    timeout,
                    on_close: None,
                },
            );
        }
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
            if let Some(queue) = queue.get_value()
                && let Some(toast) = queue.visible_toasts.get_untracked().first()
            {
                queue.close(toast.key);
            }
        }),
        Control::action("pause-all", move || {
            if let Some(queue) = queue.get_value() {
                queue.pause_all();
            }
        }),
        Control::action("resume-all", move || {
            if let Some(queue) = queue.get_value() {
                queue.resume_all();
            }
        }),
        Control::action("clear", move || {
            if let Some(queue) = queue.get_value() {
                queue.clear();
            }
        }),
    ];
    view! {
        <h1>"use_toast_state performance"</h1>
        <PerfControls controls />
        <Show when=move || mounted.get()>
            <Queue queue />
        </Show>
    }
}
