use leptonic::prelude::*;
use leptos::*;

#[component]
pub fn PageAlert(cx: Scope) -> impl IntoView {
    let (centered, set_centered) = create_signal(cx, false);
    view! { cx,
        <H2>"Alerts"</H2>

        <Alert variant=AlertVariant::Success title=move |_cx| "Success" centered=centered>"Action completed."</Alert>
        <Alert variant=AlertVariant::Info title=move |_cx| "Info" centered=centered>"This concept is based on [...]"</Alert>
        <Alert variant=AlertVariant::Warn title=move |_cx| "Warn" centered=centered>"This seems not plausible."</Alert>
        <Alert variant=AlertVariant::Danger title=move |_cx| "Danger" centered=centered>"There was an error!"</Alert>

        <P>"Alerts can be dynamically centered using a signal."</P>

        <Button on_click=move |_| set_centered.update(|it| *it = !*it)>"Center toggle"</Button>
    }
}
