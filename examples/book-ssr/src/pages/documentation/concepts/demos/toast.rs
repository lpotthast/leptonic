use std::time::Duration;

use leptonic::{
    atoms::{
        button::Button,
        toast::{Toast, ToastCloseButton, ToastContent, ToastDescription, ToastRegion, ToastTitle},
    },
    hooks::toast::{ToastOptions, ToastQueue},
};
use leptos::prelude::*;

#[component]
pub fn ToastConceptDemo() -> impl IntoView {
    // The queue of the app's toasts; their content is a message. Create it once, e.g. next to your app's root,
    // and provide it as context to the code that adds toasts.
    let queue = ToastQueue::<&'static str>::new(Some(3));

    let save = move |_| {
        queue.add(
            "Your changes were saved.",
            ToastOptions {
                timeout: Some(Duration::from_secs(5)),
                ..ToastOptions::default()
            },
        );
    };

    view! {
        <Button on_press=save classes="demo-btn">"Save"</Button>
        <p class="demo-status">
            {move || match queue.visible_toasts.with(Vec::len) {
                1 => "1 toast shown.".to_owned(),
                count => format!("{count} toasts shown."),
            }}
        </p>

        // A landmark at the end of the page, rendered while there are toasts.
        <ToastRegion queue=queue classes="demo-toast-region" let:toast>
            <Toast toast=toast.clone() classes="demo-toast">
                <ToastContent classes="demo-toast-content">
                    <ToastTitle classes="demo-toast-title">"Saved"</ToastTitle>
                    <ToastDescription>{toast.content}</ToastDescription>
                </ToastContent>
                <ToastCloseButton classes="demo-toast-close">"\u{2715}"</ToastCloseButton>
            </Toast>
        </ToastRegion>
    }
}
