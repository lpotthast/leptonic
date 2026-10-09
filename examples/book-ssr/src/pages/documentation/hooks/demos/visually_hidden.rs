use leptonic::{
    IntoAttrs,
    atoms::{button::Button, link::AnchorLink},
    hooks::visually_hidden::{UseVisuallyHiddenInput, use_visually_hidden},
};
use leptos::prelude::*;
use leptos_icons::Icon;

#[component]
pub fn VisuallyHiddenDemo() -> impl IntoView {
    let archived = RwSignal::new(0u32);

    // Shown while focus is within it: a skip link.
    let skip_link = use_visually_hidden(UseVisuallyHiddenInput {
        is_focusable: true.into(),
    });
    // Never shown: names the icon-only button for screen readers.
    let button_label = use_visually_hidden(UseVisuallyHiddenInput::default());

    view! {
        <div {..skip_link.props.into_attrs()}>
            <AnchorLink href="#visually-hidden-demo-status">"Skip to the status"</AnchorLink>
        </div>
        <div class="demo-control-row">
            <Button on_press=move |_| archived.update(|n| *n += 1) classes=["demo-btn", "demo-icon-btn"]>
                <Icon icon=icondata::BsArchive/>
                <span {..button_label.props.into_attrs()}>"Archive message"</span>
            </Button>
        </div>
        <p id="visually-hidden-demo-status" class="demo-status">
            {move || match archived.get() {
                1 => String::from("Archived 1 message."),
                n => format!("Archived {n} messages."),
            }}
        </p>
    }
}
