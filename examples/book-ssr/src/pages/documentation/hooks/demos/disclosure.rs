use leptonic::hooks::*;
use leptos::prelude::*;

#[component]
pub fn DisclosureDemo() -> impl IntoView {
    let UseDisclosureStateReturn {
        is_expanded,
        toggle,
        ..
    } = use_disclosure_state(false);

    let UseDisclosureReturn {
        trigger_props,
        content_props,
        ..
    } = use_disclosure(UseDisclosureInput {
        is_expanded,
        is_disabled: Signal::derive(|| false),
        on_expanded_change: None,
    });

    view! {
        <div style="border: 1px solid #ddd; border-radius: 8px; overflow: hidden; margin: 1em 0; max-width: 500px;">
            <button
                {..trigger_props.into_attrs()}
                on:click=move |_| toggle.run(())
                style="
                    width: 100%;
                    padding: 1em;
                    background: #f5f5f5;
                    border: none;
                    cursor: pointer;
                    display: flex;
                    justify-content: space-between;
                    align-items: center;
                    font-size: 1em;
                    font-weight: 500;
                "
            >
                <span>"What is disclosure?"</span>
                <span style=move || format!(
                    "transition: transform 0.2s; {}",
                    if is_expanded.get() { "transform: rotate(180deg);" } else { "" }
                )>"▼"</span>
            </button>

            <div
                {..content_props.into_attrs()}
                style=move || format!(
                    "overflow: hidden; transition: all 0.3s; {}",
                    if is_expanded.get() { "max-height: 200px; padding: 1em;" } else { "max-height: 0; padding: 0 1em;" }
                )
            >
                <p style="margin: 0;">
                    "A disclosure is a widget that shows or hides content. It consists of a button that toggles the visibility of a panel. This is commonly used for FAQs, accordions, and collapsible sections."
                </p>
            </div>
        </div>
    }
}
