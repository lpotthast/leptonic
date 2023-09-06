use indoc::indoc;
use leptonic::prelude::*;
use leptos::*;

#[component]
pub fn PageAlert() -> impl IntoView {
    let (centered, set_centered) = create_signal(false);
    view! {
        <H1>"Alerts"</H1>

        <Code>
            {indoc!(r#"
                <Alert variant=AlertVariant::Success title=|_| "Success">"Action completed."</Alert>
            "#)}
        </Code>

        <Alert variant=AlertVariant::Success title=|| "Success" centered=centered>"Action completed."</Alert>
        <Alert variant=AlertVariant::Info title=|| "Info" centered=centered>"This concept is based on [...]"</Alert>
        <Alert variant=AlertVariant::Warn title=|| "Warn" centered=centered>"This seems not plausible."</Alert>
        <Alert variant=AlertVariant::Danger title=|| "Danger" centered=centered>"There was an error!"</Alert>

        <P>"Alerts can be dynamically centered using a signal."</P>

        <Button on_click=move |_| set_centered.update(|it| *it = !*it)>"Center toggle"</Button>

        <H2>"Styling"</H2>

        <P>"You may overwrite any of the following CSS variables to meet your styling needs."</P>

        <Code>
            {indoc!(r#"
                --alert-margin
                --alert-padding
                --alert-primary-background-color
                --alert-primary-color
                --alert-info-background-color
                --alert-info-color
                --alert-success-background-color
                --alert-success-color
                --alert-warn-background-color
                --alert-warn-color
                --alert-danger-background-color
                --alert-danger-color
            "#)}
        </Code>
    }
}
