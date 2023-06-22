use leptonic::prelude::*;
use leptos::*;
use uuid::Uuid;

#[component]
pub fn PageToast(cx: Scope) -> impl IntoView {
    let (header, set_header) = create_signal(cx, "Header".to_owned());
    let (body, set_body) = create_signal(cx, "Body".to_owned());

    view! { cx,
        <H2>"Toasts"</H2>

        <Input label="Header text" get=header set=move |v| set_header.set(v)></Input>
        <Input label="Body text" get=body set=move |v| set_body.set(v)></Input>

        <Button on_click=move |_| { expect_context::<Toasts>(cx).push(success_toast(cx, header.get(), body.get())) }>
            "Create Toast"
        </Button>
    }
}

fn success_toast(cx: Scope, header: String, body: String) -> Toast {
    Toast {
        id: Uuid::new_v4(),
        created_at: time::OffsetDateTime::now_utc(),
        variant: ToastVariant::Success,
        header: view! { cx, {header} }.into_view(cx),
        body: view! { cx, {body} }.into_view(cx),
        timeout: ToastTimeout::DefaultDelay,
    }
}
