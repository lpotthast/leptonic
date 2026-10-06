use leptonic::{components::prelude::Icon, hooks::*, prelude::icondata};
use leptos::prelude::*;

/// A button showing or hiding its content, built on leptonic's `use_disclosure`.
///
/// The hook marks the collapsed panel `aria-hidden`; `_demo.scss` hides it based on that attribute.
#[component]
pub fn Disclosure(
    /// The trigger's text, e.g. `"View source"`.
    label: &'static str,
    #[prop(optional)] default_expanded: bool,
    children: Children,
) -> impl IntoView {
    let UseDisclosureStateReturn {
        is_expanded,
        expand,
        collapse,
        ..
    } = use_disclosure_state(default_expanded);

    let UseDisclosureReturn {
        trigger_props,
        content_props,
        ..
    } = use_disclosure(UseDisclosureInput {
        is_expanded,
        on_expanded_change: Some(Callback::new(move |expanded: bool| {
            if expanded {
                expand.run(());
            } else {
                collapse.run(());
            }
        })),
        ..UseDisclosureInput::default()
    });

    view! {
        <div class="doc-disclosure">
            <button {..trigger_props.into_attrs()} type="button" class="doc-disclosure-trigger">
                <Icon icon=icondata::BsChevronRight classes="doc-disclosure-chevron"/>
                {label}
            </button>
            <div {..content_props.into_attrs()} class="doc-disclosure-panel">
                {children()}
            </div>
        </div>
    }
}
