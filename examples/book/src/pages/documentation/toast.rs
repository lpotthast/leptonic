use indoc::indoc;
use leptonic::prelude::*;
use leptos::*;
use strum::IntoEnumIterator;
use uuid::Uuid;

#[component]
pub fn PageToast(cx: Scope) -> impl IntoView {
    let (variant, set_variant) = create_signal(cx, ToastVariant::Success);
    let (timeout, set_timeout) = create_signal(cx, ToastTimeout::DefaultDelay);
    let (header, set_header) = create_signal(cx, "Header".to_owned());
    let (body, set_body) = create_signal(cx, "Body".to_owned());

    view! { cx,
        <H2>"Toasts"</H2>

        <Input label="Header text" get=header set=move |v| set_header.set(v)></Input>
        <Input label="Body text" get=body set=move |v| set_body.set(v)></Input>

        <Select
            options={ToastVariant::iter().collect::<Vec<_>>()}
            selected=variant
            set_selected=create_callback(cx, move |v| set_variant.set(v))
            render_option=create_callback(cx, move |(_cx, o)| format!("{o:?}"))
        />

        <Select
            options=vec![ToastTimeout::None, ToastTimeout::DefaultDelay]
            selected=timeout
            set_selected=create_callback(cx, move |v| set_timeout.set(v))
            render_option=create_callback(cx, move |(_cx, o)| format!("{o:?}"))
        />

        <Button on_click=move |_| { expect_context::<Toasts>(cx).push(
            Toast {
                id: Uuid::new_v4(),
                created_at: time::OffsetDateTime::now_utc(),
                variant: variant.get_untracked(),
                header: header.get_untracked().into_view(cx),
                body: body.get_untracked().into_view(cx),
                timeout: timeout.get_untracked(),
            }) }>
            "Create Toast"
        </Button>


        <H2>"Styling"</H2>

        <P>"You may overwrite any of the following CSS variables to meet your styling needs."</P>

        <Code>
            {indoc!(r#"
                --toast-border-radius
                --toast-header-border-bottom
                --toast-header-padding
                --toast-message-padding
                --toast-info-header-background
                --toast-info-header-color
                --toast-info-message-background
                --toast-info-message-color
                --toast-success-header-background
                --toast-success-header-color
                --toast-success-message-background
                --toast-success-message-color
                --toast-warn-header-background
                --toast-warn-header-color
                --toast-warn-message-background
                --toast-warn-message-color
                --toast-error-header-background
                --toast-error-header-color
                --toast-error-message-background
                --toast-error-message-color
            "#)}
        </Code>
    }
}
