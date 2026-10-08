use leptonic::{atoms::checkbox::{CheckboxButton, CheckboxField}, hooks::*};
use leptos::prelude::*;

#[component]
pub fn LinkDisabledDemo() -> impl IntoView {
    let disabled = RwSignal::new(false);

    let link = use_link(UseLinkInput {
        href: Signal::stored(Some("/doc/link/atom".to_owned())),
        is_disabled: disabled.into(),
        ..UseLinkInput::default()
    });
    let (link_attrs, link_styles) = link.props.into_parts();

    view! {
        // Disabled, the link loses its `href` and gets `aria-disabled="true"`, which the styles use.
        <p><a {..link_attrs} class="demo-link" style=link_styles>"Link Atoms"</a></p>

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
