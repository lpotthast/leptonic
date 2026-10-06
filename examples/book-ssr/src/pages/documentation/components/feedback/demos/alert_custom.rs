use leptonic::components::prelude::*;
use leptos::prelude::*;

#[component]
pub fn AlertCustomDemo() -> impl IntoView {
    view! {
        // Replaces the icon with custom prepend and append slots.
        <Alert variant=AlertVariant::Success default_icon_slot=AlertIconSlot::None classes="demo-clf-alert-celebration">
            <AlertPrepend slot>"🎉"</AlertPrepend>
            <AlertTitle slot>"Success"</AlertTitle>
            <AlertAppend slot>"🎉"</AlertAppend>
        </Alert>

        // Moves the icon into the title.
        <Alert variant=AlertVariant::Warn default_icon_slot=AlertIconSlot::None classes="demo-clf-alert-loud">
            <AlertTitle slot>
                "Warning"
                <span class="demo-clf-alert-title-icon">
                    <AlertIcon variant=AlertVariant::Warn/>
                </span>
            </AlertTitle>
            <AlertContent slot>"This is dangerous!"</AlertContent>
        </Alert>
    }
}
