use leptonic::components::button::{Button, ButtonSize, ButtonVariant, LinkButton};
use leptos::prelude::*;

/// The styled buttons' theme states: enabled and disabled per variant, the sizes, and a
/// disabled link button.
#[component]
pub fn PageComponentButton() -> impl IntoView {
    let variants = [
        ("flat", ButtonVariant::Flat),
        ("outlined", ButtonVariant::Outlined),
        ("filled", ButtonVariant::Filled),
    ];
    view! {
        <div id="test-page-component-button">
            <h1>"Button components"</h1>
            {variants
                .into_iter()
                .map(|(name, variant)| {
                    view! {
                        <Button attr:id=format!("test-cbtn-{name}") variant=variant>{name}</Button>
                        <Button attr:id=format!("test-cbtn-{name}-disabled") variant=variant is_disabled=true>
                            {name}
                        </Button>
                    }
                })
                .collect_view()}
            <Button attr:id="test-cbtn-small" size=ButtonSize::Small>"Small"</Button>
            <Button attr:id="test-cbtn-normal">"Normal"</Button>
            <Button attr:id="test-cbtn-big" size=ButtonSize::Big>"Big"</Button>
            <LinkButton attr:id="test-cbtn-link-disabled" href="/" is_disabled=true>"Link"</LinkButton>
        </div>
    }
}
