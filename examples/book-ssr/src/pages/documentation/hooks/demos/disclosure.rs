use leptonic::{components::prelude::*, hooks::*, prelude::icondata};
use leptos::prelude::*;

#[component]
pub fn DisclosureDemo() -> impl IntoView {
    let disabled = RwSignal::new(false);

    let state = use_disclosure_state(UseDisclosureStateInput::default());
    let UseDisclosureReturn {
        button,
        panel_props,
        ..
    } = use_disclosure(UseDisclosureInput {
        is_disabled: disabled.into(),
        state,
    });
    // The trigger is a button: `use_button` with the disclosure's configuration.
    let (button_attrs, button_styles) = use_button(button).props.into_parts();

    view! {
        <div class="demo-disclosure">
            <h4 class="demo-disclosure-heading">
                // Styled through `aria-expanded` and `disabled`, which the hooks set.
                <button {..button_attrs} style=button_styles class="demo-disclosure-trigger demo-hook-disclosure-trigger">
                    "What is a disclosure?"
                    <Icon icon=icondata::BsChevronDown classes="demo-disclosure-chevron"/>
                </button>
            </h4>

            // Collapsed, the panel is `hidden="until-found"`: find in page still finds (and expands) it.
            <div {..panel_props.into_attrs()} class="demo-disclosure-panel">
                <p>
                    "A button that shows and hides a panel of content. You find it in FAQs, accordions and "
                    "collapsible sections."
                </p>
            </div>
        </div>

        <div class="demo-controls">
            <Checkbox is_selected=disabled set_selected=disabled>"Disabled"</Checkbox>
        </div>
        <p class="demo-status">{move || if state.is_expanded.get() { "Expanded" } else { "Collapsed" }}</p>
    }
}
