use leptonic::components::prelude::*;
use leptos::prelude::*;

#[component]
pub fn AlertCustomDemo() -> impl IntoView {
    view! {
        // Replaces the icon with custom prepend and append slots. The emoji are decoration: hidden from screen readers.
        <Alert variant=AlertVariant::Success default_icon_slot=AlertIconSlot::None classes="demo-alert-celebration">
            <AlertPrepend slot><span aria-hidden="true">"🎉"</span></AlertPrepend>
            <AlertTitle slot>"Order placed"</AlertTitle>
            <AlertAppend slot><span aria-hidden="true">"🎉"</span></AlertAppend>
        </Alert>

        // Moves the icon into the title.
        <Alert variant=AlertVariant::Warn default_icon_slot=AlertIconSlot::None classes="demo-alert-loud">
            <AlertTitle slot>
                "Unsaved changes"
                <span class="demo-alert-title-icon">
                    <AlertIcon variant=AlertVariant::Warn/>
                </span>
            </AlertTitle>
            <AlertContent slot>"Leaving this page discards your changes."</AlertContent>
        </Alert>
    }
}
