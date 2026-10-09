// Upstream: react-aria-components/src/Breadcrumbs.tsx @ 99e6102368
use leptos::{context::Provider, prelude::*};
use leptos_classes::Classes;

use super::link::{CurrentMatch, LinkContext};
use crate::{
    IntoAttrs,
    hooks::{
        breadcrumbs::{UseBreadcrumbsInput, use_breadcrumbs},
        collections::Key,
        interactions::PressEvent,
    },
    utils::{
        aria::AriaCurrent, data_attributes::flag, default_class::with_default_class, id::use_id,
        styles::Styles,
    },
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - The items are plain `Breadcrumb` children instead of a collection, so the current item is
//   marked (`is_current`) instead of being the collection's last: without a collection, which
//   item is last isn't known while the items render (and hydrate).
// - Render props become `data-*` attributes plus plain children.
// - The `<ol>` carries `use_breadcrumbs`' props (its name), as react-aria-components does; the
//   `<nav>` around it is the app's.
// - A `Breadcrumb` hands its link the item's settings through the link's context instead of
//   calling `use_breadcrumb_item` (as react-aria-components, which composes a `Link` child the
//   same way).
//
// =============================================================================

/// What the [`Breadcrumbs`] share with their items.
#[derive(Debug, Clone, Copy)]
struct BreadcrumbsContext {
    is_disabled: Signal<bool>,
    on_action: Option<Callback<Key>>,
}

/// The trail of links to the current page: an `<ol>` of [`Breadcrumb`]s, the last of which is
/// the current page (`is_current`). Wrap it in a `<nav>` landmark.
///
/// ```ignore
/// <nav>
///     <Breadcrumbs>
///         <Breadcrumb><Link href="/">"Home"</Link></Breadcrumb>
///         <Breadcrumb is_current=true><Link href="/docs">"Docs"</Link></Breadcrumb>
///     </Breadcrumbs>
/// </nav>
/// ```
///
/// Data attributes: `data-disabled`.
///
/// Default class: `leptonic-Breadcrumbs`.
#[component]
pub fn Breadcrumbs(
    /// Names the breadcrumbs. Default: "Breadcrumbs".
    #[prop(into, optional)]
    aria_label: MaybeProp<String>,
    /// Whether all items are disabled.
    #[prop(into, optional)]
    is_disabled: Signal<bool>,
    /// Called with the `key` of a pressed item.
    #[prop(into, optional)]
    on_action: Option<Callback<Key>>,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: Children,
) -> impl IntoView {
    let classes = with_default_class("leptonic-Breadcrumbs", classes);
    let breadcrumbs = use_breadcrumbs(UseBreadcrumbsInput { aria_label });
    let context = BreadcrumbsContext {
        is_disabled,
        on_action,
    };
    view! {
        <ol
            {..breadcrumbs.props.into_attrs()}
            class=classes
            style=styles
            data-disabled=flag(is_disabled)
        >
            <Provider value=context>{children()}</Provider>
        </ol>
    }
}

/// An item of the [`Breadcrumbs`] around it: an `<li>` with a [`Link`](super::link::Link). Mark
/// the last item `is_current`: its link is then the current page (`aria-current="page"`) and
/// disabled.
///
/// Data attributes: `data-current`, `data-disabled`.
///
/// Default class: `leptonic-Breadcrumb`.
#[component]
pub fn Breadcrumb(
    /// The item's key for the breadcrumbs' `on_action`. Default: generated.
    #[prop(into, optional)]
    key: Option<Key>,
    /// Whether the item is the current page (the last item).
    #[prop(into, optional)]
    is_current: Signal<bool>,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: Children,
) -> impl IntoView {
    let classes = with_default_class("leptonic-Breadcrumb", classes);
    let Some(breadcrumbs) = use_context::<BreadcrumbsContext>() else {
        crate::utils::dev_warn!("A <Breadcrumb> must be inside <Breadcrumbs>.");
        return children().into_any();
    };
    let key = StoredValue::new(key.unwrap_or_else(|| Key::from(use_id("breadcrumb"))));
    let is_disabled = Signal::derive(move || breadcrumbs.is_disabled.get() || is_current.get());
    let link = LinkContext {
        is_disabled,
        aria_current: Signal::derive(move || is_current.get().then_some(AriaCurrent::Page)),
        // Ancestors' routes contain the current location: only the current item is current.
        current_match: Some(CurrentMatch::Exact),
        on_press: breadcrumbs
            .on_action
            .map(|on_action| Callback::new(move |_: PressEvent| on_action.run(key.get_value()))),
    };

    view! {
        <li
            class=classes
            style=styles
            data-current=flag(is_current)
            data-disabled=flag(is_disabled)
        >
            <Provider value=link>{children()}</Provider>
        </li>
    }
    .into_any()
}
