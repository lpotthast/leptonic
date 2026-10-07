use std::time::Duration;

use leptonic::{
    atoms::toast::{
        Toast, ToastCloseButton, ToastContent, ToastDescription, ToastRegion, ToastTitle,
    },
    components::prelude::{Button, Icon},
    hooks::{ToastOptions, ToastQueue},
    prelude::icondata,
};
use leptos::prelude::*;

#[component]
pub fn ToastAtomDemo() -> impl IntoView {
    // The toasts' content is the name of an uploaded file. Up to three show at once.
    let queue = ToastQueue::<String>::new(Some(3));
    let uploads = StoredValue::new(0);

    let upload = move |_| {
        uploads.update_value(|uploads| *uploads += 1);
        queue.add(
            format!("report-{}.pdf", uploads.get_value()),
            ToastOptions {
                timeout: Some(Duration::from_secs(8)),
                ..ToastOptions::default()
            },
        );
    };

    view! {
        <Button on_press=upload>"Upload report"</Button>

        // Rendered at the end of the page while there are toasts.
        <ToastRegion queue=queue classes="demo-toast-region" let:toast>
            <Toast toast=toast.clone() classes="demo-toast">
                <ToastContent classes="demo-toast-content">
                    <ToastTitle classes="demo-toast-title">"Upload complete"</ToastTitle>
                    <ToastDescription>{format!("{} was uploaded.", toast.content)}</ToastDescription>
                </ToastContent>
                <ToastCloseButton classes="demo-toast-close">
                    <Icon icon=icondata::BsX/>
                </ToastCloseButton>
            </Toast>
        </ToastRegion>

        <p class="demo-status">
            {move || match queue.visible_toasts.with(Vec::len) {
                1 => "1 toast shown.".to_owned(),
                count => format!("{count} toasts shown."),
            }}
        </p>
    }
}
