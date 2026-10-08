use std::collections::HashSet;

use leptonic::{
    atoms::prelude::{Button, CheckboxButton, CheckboxField, Disclosure, DisclosureGroup, DisclosurePanel, DisclosureTrigger},
    hooks::Key,
};
use leptos::prelude::*;
use leptos_icons::Icon;

#[component]
pub fn DisclosureAtomDemo() -> impl IntoView {
    let disabled = RwSignal::new(false);
    // App state: the expanded disclosures, by `id`.
    let expanded = RwSignal::new(HashSet::from([Key::from("shipping")]));

    view! {
        // An accordion: expanding one disclosure collapses the other.
        <DisclosureGroup is_disabled=disabled expanded_keys=expanded set_expanded_keys=expanded>
            <Disclosure id="shipping" classes="demo-disclosure">
                <h4 class="demo-disclosure-heading">
                    <DisclosureTrigger>
                        <Button classes="demo-disclosure-trigger">
                            "Shipping"
                            <span class="demo-disclosure-chevron" aria-hidden="true"><Icon icon=icondata::BsChevronDown/></span>
                        </Button>
                    </DisclosureTrigger>
                </h4>
                <DisclosurePanel classes="demo-disclosure-panel">
                    <p>"Orders ship within two working days."</p>
                </DisclosurePanel>
            </Disclosure>
            <Disclosure id="returns" classes="demo-disclosure">
                <h4 class="demo-disclosure-heading">
                    <DisclosureTrigger>
                        <Button classes="demo-disclosure-trigger">
                            "Returns"
                            <span class="demo-disclosure-chevron" aria-hidden="true"><Icon icon=icondata::BsChevronDown/></span>
                        </Button>
                    </DisclosureTrigger>
                </h4>
                <DisclosurePanel classes="demo-disclosure-panel">
                    <p>"Return anything within 30 days."</p>
                </DisclosurePanel>
            </Disclosure>
        </DisclosureGroup>

        <div class="demo-controls">
            <CheckboxField is_selected=disabled set_selected=disabled>
                <CheckboxButton classes="demo-check">
                    <span class="demo-check-box" aria-hidden="true"></span>
                    "Disabled"
                </CheckboxButton>
            </CheckboxField>
        </div>
        <p class="demo-status">
            "Expanded: "
            {move || {
                let mut keys: Vec<String> = expanded.get().iter().map(ToString::to_string).collect();
                keys.sort();
                if keys.is_empty() { "none".to_owned() } else { keys.join(", ") }
            }}
        </p>
    }
}
