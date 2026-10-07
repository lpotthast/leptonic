use std::time::Duration;

use leptonic::{
    atoms::{
        button::Button,
        toast::{Toast, ToastCloseButton, ToastContent, ToastDescription, ToastRegion, ToastTitle},
    },
    hooks::{ToastOptions, ToastQueue},
};
use leptos::prelude::*;

/// Toasts of one queue (react-aria-components' `Toast.test.js` setup): "Toast" adds one without
/// a timeout, "Timed toast" one closing after 1.5 seconds, "Slow toast" after 3 seconds, "Close
/// newest" closes the newest
/// programmatically. `#test-toast-closed` counts the `on_close` calls.
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
        <Button attr:id="test-toast-add" on_press=move |_| add(None)>"Toast"</Button>
        <Button attr:id="test-toast-add-timed" on_press=move |_| add(Some(Duration::from_millis(1500)))>
            "Timed toast"
        </Button>
        <Button attr:id="test-toast-add-slow" on_press=move |_| add(Some(Duration::from_secs(3)))>
            "Slow toast"
        </Button>
        <Button
            attr:id="test-toast-close-newest"
            on_press=move |_| {
                if let Some(toast) = queue.visible_toasts.get_untracked().first() {
                    queue.close(&toast.key);
                }
            }
        >
            "Close newest"
        </Button>
        <p>"Closed: " <span id="test-toast-closed">{move || closed.get()}</span></p>
    }
}
