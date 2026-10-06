use leptonic::components::prelude::*;
use leptos::prelude::*;
use strum::IntoEnumIterator;
use uuid::Uuid;

#[component]
pub fn ToastCreationDemo() -> impl IntoView {
    let (variant, set_variant) = signal(ToastVariant::Success);
    let (timeout, set_timeout) = signal(ToastTimeout::DefaultDelay);
    let (header, set_header) = signal("Header".to_owned());
    let (body, set_body) = signal("Body".to_owned());

    let toasts = expect_context::<Toasts>();

    let create_toast = move |_| {
        let header = header.get_untracked();
        let body = body.get_untracked();
        toasts.push(Toast {
            id: Uuid::new_v4(),
            created_at: time::OffsetDateTime::now_utc(),
            variant: variant.get_untracked(),
            header: (move || header.clone()).into(),
            body: (move || body.clone()).into(),
            timeout: timeout.get_untracked(),
        });
    };

    view! {
        <div class="demo-clf-form">
            <TextField label="Header" state=(header, set_header)/>
            <TextField label="Body" state=(body, set_body)/>

            <Select
                options={ToastVariant::iter().collect::<Vec<_>>()}
                selected=variant
                set_selected=set_variant
                search_text_provider=move |o| format!("{o}")
                render_option=move |o| format!("{o:?}").into_view()
            />

            <Select
                options=vec![ToastTimeout::None, ToastTimeout::DefaultDelay]
                selected=timeout
                set_selected=set_timeout
                search_text_provider=move |o| format!("{o}")
                render_option=move |o| format!("{o:?}").into_view()
            />

            <div>
                <Button on_press=create_toast>"Create Toast"</Button>
            </div>
        </div>
    }
}
