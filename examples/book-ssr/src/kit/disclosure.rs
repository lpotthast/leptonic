use leptonic::{atoms::prelude as atoms, components::prelude::Icon, prelude::icondata};
use leptos::prelude::*;

/// A button showing or hiding its content, built on leptonic's `Disclosure` atoms. While collapsed,
/// the panel is `hidden="until-found"`: hidden, but the browser's find in page still finds (and
/// expands) its content.
#[component]
pub fn Disclosure(
    /// The trigger's text, e.g. `"View source"`.
    label: &'static str,
    #[prop(optional)] default_expanded: bool,
    children: Children,
) -> impl IntoView {
    view! {
        <atoms::Disclosure default_expanded=default_expanded classes="doc-disclosure">
            <atoms::DisclosureTrigger>
                <atoms::Button classes="doc-disclosure-trigger">
                    <Icon icon=icondata::BsChevronRight classes="doc-disclosure-chevron"/>
                    {label}
                </atoms::Button>
            </atoms::DisclosureTrigger>
            <atoms::DisclosurePanel classes="doc-disclosure-panel">{children()}</atoms::DisclosurePanel>
        </atoms::Disclosure>
    }
}
