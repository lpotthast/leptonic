use leptonic::{atoms::checkbox::{CheckboxButton, CheckboxField}, hooks::*};
use leptos::prelude::*;

/// One item of the trail: a link, or the current page.
#[component]
fn Crumb(
    href: &'static str,
    is_current: bool,
    is_disabled: Signal<bool>,
    children: Children,
) -> impl IntoView {
    // The current item is a disabled link with `aria-current="page"`: it keeps its element, but loses its `href`.
    let (attrs, styles) = use_breadcrumb_item(UseBreadcrumbItemInput {
        link: UseLinkInput {
            href: Signal::stored(Some(href.to_owned())),
            is_disabled,
            ..UseLinkInput::default()
        },
        is_current: Signal::stored(is_current),
        ..UseBreadcrumbItemInput::default()
    })
    .props
    .into_parts();

    view! {
        <li class="demo-breadcrumb">
            <a {..attrs} style=styles class="demo-navigation-breadcrumb">{children()}</a>
        </li>
    }
}

#[component]
pub fn BreadcrumbsDemo() -> impl IntoView {
    let disabled = RwSignal::new(false);
    let breadcrumbs = use_breadcrumbs(UseBreadcrumbsInput::default());

    view! {
        // The separators are drawn in CSS, hidden from assistive technology.
        <nav aria-label="Breadcrumbs">
            <ol {..breadcrumbs.props.into_attrs()} class="demo-breadcrumbs">
                <Crumb href="/doc/overview" is_current=false is_disabled=disabled.into()>"Docs"</Crumb>
                <Crumb href="/doc/navigation" is_current=false is_disabled=disabled.into()>"Navigation"</Crumb>
                <Crumb href="/doc/breadcrumbs" is_current=false is_disabled=disabled.into()>"Breadcrumbs"</Crumb>
                <Crumb href="/doc/breadcrumbs/hook" is_current=true is_disabled=disabled.into()>"Hooks"</Crumb>
            </ol>
        </nav>

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
