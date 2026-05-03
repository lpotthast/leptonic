use leptonic::components::prelude::*;
use leptos::prelude::*;

#[component]
pub fn AlertCustomDemo() -> impl IntoView {
    view! {
        <Alert variant=AlertVariant::Success default_icon_slot=AlertIconSlot::None>
            <AlertPrepend slot style="align-items: center; font-size: 1.8em; margin: 0;">"🎉"</AlertPrepend>
            <AlertTitle slot style=r"
                align-items: center;
                justify-content: center;
                height: 100%;
                text-transform: uppercase;
                font-size: 1.3em;
            ">
                "Success"
            </AlertTitle>
            <AlertAppend slot style="align-items: center; font-size: 1.8em; margin: 0;">"🎉"</AlertAppend>
        </Alert>

        <Alert variant=AlertVariant::Warn default_icon_slot=AlertIconSlot::None>
            <AlertTitle slot style="text-transform: uppercase; font-size: 1.3em;">
                "Warning"
                <AlertIcon variant=AlertVariant::Warn attr:style="margin-left: 0.5em;" />
            </AlertTitle>
            <AlertContent slot>"This is dangerous!"</AlertContent>
        </Alert>
    }
}
