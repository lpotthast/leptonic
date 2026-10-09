use leptonic::{
    IntoAttrs,
    atoms::checkbox::{CheckboxButton, CheckboxField},
    hooks::{
        button::use_button,
        disclosure::{
            UseDisclosureInput, UseDisclosureReturn, UseDisclosureStateInput, use_disclosure,
            use_disclosure_state,
        },
    },
};
use leptos::prelude::*;
use leptos_icons::Icon;

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
                    <span class="demo-disclosure-chevron" aria-hidden="true"><Icon icon=icondata::BsChevronDown/></span>
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
            <CheckboxField is_selected=disabled set_selected=disabled>
                <CheckboxButton classes="demo-check">
                    <span class="demo-check-box" aria-hidden="true"></span>
                    "Disabled"
                </CheckboxButton>
            </CheckboxField>
        </div>
        <p class="demo-status">{move || if state.is_expanded.get() { "Expanded" } else { "Collapsed" }}</p>
    }
}
