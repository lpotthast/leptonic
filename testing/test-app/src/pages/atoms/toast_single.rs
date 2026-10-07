use leptonic::{
    atoms::{
        button::Button,
        toast::{Toast, ToastCloseButton, ToastContent, ToastRegion, ToastTitle},
    },
    hooks::{ToastOptions, use_toast_state},
};
use leptos::prelude::*;

/// One toast at a time (`use_toast_state`'s default) in a region named "Alerts": "Alert" adds
/// one ("Alert 1", "Alert 2", ...) without a timeout.
#[component]
pub fn PageAtomToastSingle() -> impl IntoView {
    let queue = use_toast_state::<String>(None);
    let count = StoredValue::new(0);
    view! {
        <h1>"One toast at a time"</h1>
        <ToastRegion queue=queue aria_label="Alerts" let:toast>
            <Toast toast=toast.clone()>
                <ToastContent>
                    <ToastTitle>{toast.content.clone()}</ToastTitle>
                </ToastContent>
                <ToastCloseButton>"x"</ToastCloseButton>
            </Toast>
        </ToastRegion>
        <Button
            attr:id="test-toast-single-add"
            on_press=move |_| {
                count.update_value(|count| *count += 1);
                queue.add(
                    format!("Alert {}", count.get_value()),
                    ToastOptions {
                        timeout: None,
                        on_close: None,
                    },
                );
            }
        >
            "Alert"
        </Button>
    }
}
