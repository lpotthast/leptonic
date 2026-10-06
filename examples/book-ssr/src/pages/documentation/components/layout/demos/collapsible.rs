use std::collections::HashSet;

use leptonic::{components::prelude::*, hooks::collections::Key, utils::css::em};
use leptos::prelude::*;

#[component]
pub fn CollapsibleDemo() -> impl IntoView {
    // App state: the open collapsibles, by `id`.
    let open = RwSignal::new(HashSet::from([Key::from("shipping")]));

    view! {
        // An accordion: opening one collapsible closes the others.
        <Collapsibles expanded_keys=open set_expanded_keys=open>
            <Stack spacing=em(0.6)>
                <Collapsible id="shipping">
                    <CollapsibleHeader slot>"Shipping"</CollapsibleHeader>
                    <CollapsibleBody slot>"Orders ship within two working days."</CollapsibleBody>
                </Collapsible>
                <Collapsible id="returns">
                    <CollapsibleHeader slot>"Returns"</CollapsibleHeader>
                    <CollapsibleBody slot>"Return anything within 30 days."</CollapsibleBody>
                </Collapsible>
                <Collapsible id="warranty">
                    <CollapsibleHeader slot>"Warranty"</CollapsibleHeader>
                    <CollapsibleBody slot>"Two years on all products."</CollapsibleBody>
                </Collapsible>
            </Stack>
        </Collapsibles>

        <p class="demo-status">
            "Open: "
            {move || open.get().iter().next().map_or_else(|| "none".to_owned(), ToString::to_string)}
        </p>
    }
}
