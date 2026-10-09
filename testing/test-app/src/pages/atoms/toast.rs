use std::time::Duration;

use leptonic::{
    atoms::{
        button::Button,
        toast::{Toast, ToastCloseButton, ToastContent, ToastDescription, ToastRegion, ToastTitle},
    },
    hooks::toast::{ToastOptions, ToastQueue},
};
use leptos::prelude::*;

/// Toasts of one queue (react-aria-components' `Toast.test.js` setup): "Toast" adds one without
/// a timeout, "Timed toast" one closing after half a second (short: the tests wait for it several
/// times), "Slow toast" after 3 seconds (for timing the time left after a pause), "Close
/// newest" closes the newest programmatically. "Add, then close oldest" adds a toast after 1.5
/// seconds and closes the oldest after 3 seconds (while the focus stays in a toast).
/// `#test-toast-closed` counts the `on_close` calls.
#[component]
pub fn PageAtomToast() -> impl IntoView {
    let queue = ToastQueue::<(String, String)>::new(None);
    let closed = RwSignal::new(0);
    let count = StoredValue::new(0);
    let add = move |timeout: Option<Duration>| {
        count.update_value(|count| *count += 1);
        queue.add(
            (
                format!("Toast {}", count.get_value()),
                "Description".to_owned(),
            ),
            ToastOptions {
                timeout,
                on_close: Some(Callback::new(move |()| {
                    closed.update(|closed| *closed += 1)
                })),
            },
        );
    };
    view! {
        <h1>"Toast"</h1>
        <ToastRegion queue=queue classes="test-toast-region" let:toast>
            <Toast toast=toast.clone()>
                <ToastContent>
                    <ToastTitle>{toast.content.0.clone()}</ToastTitle>
                    <ToastDescription>{toast.content.1.clone()}</ToastDescription>
                </ToastContent>
                <ToastCloseButton>"x"</ToastCloseButton>
            </Toast>
        </ToastRegion>
        <Button id="test-toast-add" on_press=move |_| add(None)>"Toast"</Button>
        <Button id="test-toast-add-timed" on_press=move |_| add(Some(Duration::from_millis(500)))>
            "Timed toast"
        </Button>
        <Button id="test-toast-add-slow" on_press=move |_| add(Some(Duration::from_secs(3)))>
            "Slow toast"
        </Button>
        <Button
            id="test-toast-close-newest"
            on_press=move |_| {
                if let Some(toast) = queue.visible_toasts.get_untracked().first() {
                    queue.close(toast.key);
                }
            }
        >
            "Close newest"
        </Button>
        <Button
            id="test-toast-add-then-close-oldest"
            on_press=move |_| {
                set_timeout(move || add(None), Duration::from_millis(1500));
                set_timeout(
                    move || {
                        if let Some(toast) = queue.visible_toasts.get_untracked().last() {
                            queue.close(toast.key);
                        }
                    },
                    Duration::from_secs(3),
                );
            }
        >
            "Add, then close oldest"
        </Button>
        <p>"Closed: " <span id="test-toast-closed">{move || closed.get()}</span></p>
    }
}
