use leptonic::{
    components::prelude::*,
    hooks::{IntoAttrs, UseVisuallyHiddenInput, use_visually_hidden},
    prelude::icondata,
};
use leptos::prelude::*;

#[component]
pub fn VisuallyHiddenDemo() -> impl IntoView {
    let archived = RwSignal::new(0u32);

    // Shown while focus is within it: a skip link.
    let skip_link = use_visually_hidden(UseVisuallyHiddenInput { is_focusable: true });
    // Never shown: names the icon-only button for screen readers.
    let button_label = use_visually_hidden(UseVisuallyHiddenInput::default());

    view! {
        <div {..skip_link.props.into_attrs()}>
            <AnchorLink href="#visually-hidden-demo-status">"Skip to the status"</AnchorLink>
        </div>
        <div class="demo-control-row">
            <Button variant=ButtonVariant::Flat on_press=move |_| archived.update(|n| *n += 1)>
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
