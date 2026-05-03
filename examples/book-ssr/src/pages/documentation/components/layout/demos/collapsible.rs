use leptonic::{components::prelude::*, utils::css::em};
use leptos::prelude::*;

#[component]
pub fn CollapsibleDemo() -> impl IntoView {
    view! {
        <Collapsibles default_on_open=OnOpen::CloseOthers>
            <Stack spacing=em(0.6)>
                <Collapsible>
                    <CollapsibleHeader slot>"Header1"</CollapsibleHeader>
                    <CollapsibleBody class="my-body" slot>"Body1"</CollapsibleBody>
                </Collapsible>
                <Collapsible>
                    <CollapsibleHeader slot>"Header2"</CollapsibleHeader>
                    <CollapsibleBody slot>"Body2"</CollapsibleBody>
                </Collapsible>
                <Collapsible on_open=OnOpen::DoNothing >
                    <CollapsibleHeader slot>"Header3 - on_open::DoNothing"</CollapsibleHeader>
                    <CollapsibleBody slot>"Body3"</CollapsibleBody>
                </Collapsible>
            </Stack>
        </Collapsibles>
    }
}
