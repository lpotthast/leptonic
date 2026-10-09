use leptonic::atoms::{
    button::Button,
    checkbox::{CheckboxButton, CheckboxField},
    disclosure::{Disclosure, DisclosurePanel, DisclosureTrigger},
};
use leptos::prelude::*;
use leptos_icons::Icon;

#[component]
pub fn DisclosureConceptDemo() -> impl IntoView {
    let expanded = RwSignal::new(false);
    let disabled = RwSignal::new(false);

    view! {
        <Disclosure is_expanded=expanded set_expanded=expanded is_disabled=disabled classes="demo-disclosure">
            // A heading, so that the section is found by heading navigation.
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
        <p class="demo-status">{move || if expanded.get() { "Expanded." } else { "Collapsed." }}</p>
        <div class="demo-controls">
            <CheckboxField is_selected=disabled set_selected=disabled>
                <CheckboxButton classes="demo-check">
                    <span class="demo-check-box" aria-hidden="true"></span>
                    "Disabled"
                </CheckboxButton>
            </CheckboxField>
        </div>
    }
}
