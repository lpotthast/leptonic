use leptonic::components::prelude::*;
use leptos::prelude::*;

#[component]
pub fn ToastConceptDemo() -> impl IntoView {
    // Provided by `Root` (through its `ToastRoot`).
    let toasts = expect_context::<Toasts>();

    let save = move |_| {
        toasts.push(Toast {
            variant: ToastVariant::Success,
            header: (|| "Saved").into(),
            body: (|| "Your changes were saved.").into(),
            timeout: ToastTimeout::DefaultDelay,
        });
    };

    view! {
        <Button on_press=save>"Save"</Button>
        <p class="demo-status">
            {move || match toasts.queue().visible_toasts.with(Vec::len) {
                1 => "1 toast shown.".to_owned(),
                count => format!("{count} toasts shown."),
            }}
        </p>
    }
}
