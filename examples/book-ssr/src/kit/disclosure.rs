use leptonic::atoms;
use leptos::prelude::*;

use super::Icon;

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
        <atoms::disclosure::Disclosure default_expanded=default_expanded classes="doc-disclosure">
            <atoms::disclosure::DisclosureTrigger>
                <atoms::button::Button classes="doc-disclosure-trigger">
                    <Icon icon=icondata::BsChevronRight classes="doc-disclosure-chevron"/>
                    {label}
                </atoms::button::Button>
            </atoms::disclosure::DisclosureTrigger>
            <atoms::disclosure::DisclosurePanel classes="doc-disclosure-panel">{children()}</atoms::disclosure::DisclosurePanel>
        </atoms::disclosure::Disclosure>
    }
}
