use leptonic::prelude::*;
use leptos::*;

#[component]
pub fn PageAlert(cx: Scope) -> impl IntoView {
    let (center_alert, set_center_alert) = create_signal(cx, false);
    view! { cx,
        <H2>"Alerts"</H2>

        <Button on_click=move |_| set_center_alert.update(|it| *it = !*it)>"Center toggle"</Button>
        <Alert variant=AlertVariant::Success title="asd" centered=center_alert.into()>"Success alert"</Alert>
        <Alert variant=AlertVariant::Info title="asd" centered=true.into()>"Info alert"</Alert>
        <Alert variant=AlertVariant::Warn title="asd">"Warn alert"</Alert>
        <Alert variant=AlertVariant::Danger title="asd">"Danger alert"</Alert>
    }
}
