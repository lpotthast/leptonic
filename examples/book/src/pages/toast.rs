use leptonic::prelude::*;
use leptos::*;
use uuid::Uuid;

#[component]
pub fn PageToast(cx: Scope) -> impl IntoView {
    let toasts = use_context::<Toasts>(cx).unwrap();

    view! { cx,
        <Typography variant=TypographyVariant::H2>"Toasts"</Typography>

        <Button on_click=move |_| toasts.push(success_toast(cx))>"Create Toast"</Button>
    }
}

fn success_toast(cx: Scope) -> Toast {
    Toast {
        id: Uuid::new_v4(),
        created_at: time::OffsetDateTime::now_utc(),
        variant: ToastVariant::Success,
        header: view! { cx, "Header" }.into_view(cx),
        body: view! { cx, "Body" }.into_view(cx),
        timeout: ToastTimeout::DefaultDelay,
    }
}
