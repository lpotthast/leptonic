use leptonic::{components::prelude::*, hooks::*};
use leptos::prelude::*;

#[component]
pub fn DisclosureDemo() -> impl IntoView {
    let (disabled, set_disabled) = signal(false);

    let UseDisclosureStateReturn {
        is_expanded,
        expand,
        collapse,
        ..
    } = use_disclosure_state(false);

    let UseDisclosureReturn {
        trigger_props,
        content_props,
        ..
    } = use_disclosure(UseDisclosureInput {
        is_expanded,
        is_disabled: disabled.into(),
        on_expanded_change: Some(Callback::new(move |expanded: bool| {
            if expanded {
                expand.run(());
            } else {
                collapse.run(());
            }
        })),
    });

    view! {
        <div class="demo-disclosure">
            <button {..trigger_props.into_attrs()} class="demo-disclosure-trigger">
                <span>"What is a disclosure?"</span>
                <span class="demo-disclosure-chevron" aria-hidden="true">"\u{25bc}"</span>
            </button>

            // The hook sets `aria-hidden` on the collapsed panel; the stylesheet hides it based on that attribute.
            <div {..content_props.into_attrs()} class="demo-disclosure-panel">
                <p>
                    "A disclosure is a widget that shows or hides content. It consists of a button that toggles the "
                    "visibility of a panel. You find it in FAQs, accordions and collapsible sections."
                </p>
            </div>
        </div>

        <Checkbox state=(disabled, set_disabled) classes=["demo-form-row", "demo-mt-1"]>"Disabled"</Checkbox>

        <p>"Expanded: " {move || is_expanded.get()}</p>
    }
}
