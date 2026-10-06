use leptonic::{components::prelude::*, utils::css::em};
use leptos::prelude::*;

#[component]
pub fn CollapsibleDemo() -> impl IntoView {
    view! {
        <Collapsibles default_on_open=OnOpen::CloseOthers>
            <Stack spacing=em(0.6)>
                <Collapsible open=true>
                    <CollapsibleHeader slot>"Header 1"</CollapsibleHeader>
                    <CollapsibleBody slot>"Opening this collapsible closes all others."</CollapsibleBody>
                </Collapsible>
                <Collapsible>
                    <CollapsibleHeader slot>"Header 2"</CollapsibleHeader>
                    <CollapsibleBody slot>"Opening this collapsible closes all others."</CollapsibleBody>
                </Collapsible>
                <Collapsible on_open=OnOpen::DoNothing>
                    <CollapsibleHeader slot>"Header 3 \u{2014} OnOpen::DoNothing"</CollapsibleHeader>
                    <CollapsibleBody slot>"Opening this collapsible leaves the others as they are."</CollapsibleBody>
                </Collapsible>
            </Stack>
        </Collapsibles>
    }
}
