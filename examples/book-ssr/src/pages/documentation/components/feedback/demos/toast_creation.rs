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

    view! {
        <TextInput get=header set=set_header placeholder=Oco::Borrowed("Header text") attr:style="margin-bottom: 1em;"/>
        <TextInput get=body set=set_body placeholder=Oco::Borrowed("Body text") attr:style="margin-bottom: 1em;"/>

        <Select
            options={ToastVariant::iter().collect::<Vec<_>>()}
            selected=variant
            set_selected=set_variant
            search_text_provider=move |o| format!("{o}")
            render_option=move |o| format!("{o:?}").into_view()
            attr:style="margin-bottom: 1em;"
        />

        <Select
            options=vec![ToastTimeout::None, ToastTimeout::DefaultDelay]
            selected=timeout
            set_selected=set_timeout
            search_text_provider=move |o| format!("{o}")
            render_option=move |o| format!("{o:?}").into_view()
            attr:style="margin-bottom: 1em;"
        />

        <Button on_press=move |_| { toasts.push(
            Toast {
                id: Uuid::new_v4(),
                created_at: time::OffsetDateTime::now_utc(),
                variant: variant.get_untracked(),
                header: (move || header.get()).into(),
                body: (move || body.get()).into(),
                timeout: timeout.get_untracked(),
            }); }>
            "Create Toast"
        </Button>
    }
}
