use leptonic::components::prelude::*;
use leptos::prelude::*;

#[component]
pub fn AlertVariantsDemo() -> impl IntoView {
    view! {
        <Alert variant=AlertVariant::Success>
            <AlertTitle slot>"Success"</AlertTitle>
            <AlertContent slot>"Action completed."</AlertContent>
        </Alert>

        <Alert variant=AlertVariant::Info>
            <AlertTitle slot>"Info"</AlertTitle>
            <AlertContent slot>"This concept is based on [...]"</AlertContent>
        </Alert>

        <Alert variant=AlertVariant::Warn>
            <AlertTitle slot>"Warn"</AlertTitle>
            <AlertContent slot>"This is not plausible."</AlertContent>
        </Alert>

        <Alert variant=AlertVariant::Danger>
            <AlertTitle slot>"Danger"</AlertTitle>
            <AlertContent slot>"There was an error!"</AlertContent>
        </Alert>
    }
}
