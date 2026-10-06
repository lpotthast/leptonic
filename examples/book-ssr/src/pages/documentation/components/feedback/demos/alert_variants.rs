use leptonic::components::prelude::*;
use leptos::prelude::*;

#[component]
pub fn AlertVariantsDemo() -> impl IntoView {
    view! {
        <Alert variant=AlertVariant::Success>
            <AlertTitle slot>"Success"</AlertTitle>
            <AlertContent slot>"Your changes were saved."</AlertContent>
        </Alert>

        <Alert variant=AlertVariant::Info>
            <AlertTitle slot>"Info"</AlertTitle>
            <AlertContent slot>"A new version is available."</AlertContent>
        </Alert>

        <Alert variant=AlertVariant::Warn>
            <AlertTitle slot>"Warn"</AlertTitle>
            <AlertContent slot>"Your session expires in five minutes."</AlertContent>
        </Alert>

        <Alert variant=AlertVariant::Danger>
            <AlertTitle slot>"Danger"</AlertTitle>
            <AlertContent slot>"The file could not be uploaded."</AlertContent>
        </Alert>
    }
}
