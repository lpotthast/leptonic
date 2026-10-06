use std::collections::HashSet;

use leptos::prelude::*;

use crate::{
    Out,
    atoms::{
        button::Button,
        disclosure::{Disclosure, DisclosureGroup, DisclosurePanel, DisclosureTrigger},
    },
    components::icon::Icon,
    hooks::{DisclosureGroupExpansion, collections::Key},
    utils::{classes::Classes, styles::Styles},
};

/// A group of [`Collapsible`]s: by default, opening one closes the others (an accordion). Give
/// each collapsible an `id` to address it in `default_expanded_keys` and `expanded_keys`.
#[component]
#[allow(clippy::implicit_hasher)]
pub fn Collapsibles(
    /// Whether one or several collapsibles can be open at once.
    #[prop(optional)]
    expansion: DisclosureGroupExpansion,
    #[prop(into, optional)] is_disabled: Signal<bool>,
    /// The initially open collapsibles (their `id`s).
    #[prop(into, optional)]
    default_expanded_keys: Vec<Key>,
    /// The open collapsibles (controlled): a value or any signal.
    #[prop(into, optional)]
    expanded_keys: Option<Signal<HashSet<Key>>>,
    /// Receives the open collapsibles: an `RwSignal`, `WriteSignal`, closure, ...
    #[prop(into, optional)]
    set_expanded_keys: Option<Out<HashSet<Key>>>,
    #[prop(into, optional)] on_expanded_change: Option<Callback<HashSet<Key>>>,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: Children,
) -> impl IntoView {
    view! {
        <DisclosureGroup
            expansion=expansion
            is_disabled=is_disabled
            default_expanded_keys=default_expanded_keys
            nostrip:expanded_keys=expanded_keys
            nostrip:set_expanded_keys=set_expanded_keys
            nostrip:on_expanded_change=on_expanded_change
            classes=classes.add("leptonic-collapsibles")
            styles=styles
        >
            {children()}
        </DisclosureGroup>
    }
}

/// A themed collapsible section: a header button showing and hiding the body. In a
/// [`Collapsibles`] group, opening one can close the others.
///
/// ```ignore
/// <Collapsible>
///     <CollapsibleHeader slot>"Details"</CollapsibleHeader>
///     <CollapsibleBody slot>"Content"</CollapsibleBody>
/// </Collapsible>
/// ```
#[component]
pub fn Collapsible(
    /// The collapsible's key in a surrounding [`Collapsibles`].
    #[prop(into, optional)]
    id: Option<Key>,
    /// Whether the body starts open. Ignored with `is_expanded` or in a group.
    #[prop(optional)]
    default_expanded: bool,
    /// Whether the body is open (controlled): a value or any signal.
    #[prop(into, optional)]
    is_expanded: Option<Signal<bool>>,
    /// Receives the open state: an `RwSignal`, `WriteSignal`, closure, ...
    #[prop(into, optional)]
    set_expanded: Option<Out<bool>>,
    #[prop(into, optional)] on_expanded_change: Option<Callback<bool>>,
    #[prop(into, optional)] is_disabled: Signal<bool>,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    collapsible_header: CollapsibleHeader,
    collapsible_body: CollapsibleBody,
) -> impl IntoView {
    let CollapsibleHeader { children: header } = collapsible_header;
    let CollapsibleBody {
        children: body,
        classes: body_classes,
    } = collapsible_body;
    view! {
        <Disclosure
            nostrip:id=id
            default_expanded=default_expanded
            nostrip:is_expanded=is_expanded
            nostrip:set_expanded=set_expanded
            nostrip:on_expanded_change=on_expanded_change
            is_disabled=is_disabled
            classes=classes.add("leptonic-collapsible")
            styles=styles
        >
            <DisclosureTrigger>
                <Button classes="leptonic-collapsible-header">
                    <span class="leptonic-collapsible-header-content">{header()}</span>
                    // Turned while open (`data-expanded` on the collapsible).
                    <span class="leptonic-collapsible-caret">
                        <Icon icon=icondata::BsCaretDownFill />
                    </span>
                </Button>
            </DisclosureTrigger>
            <DisclosurePanel classes=body_classes.add("leptonic-collapsible-body")>
                <div class="leptonic-collapsible-body-content">{body()}</div>
            </DisclosurePanel>
        </Disclosure>
    }
}

/// The header slot of a [`Collapsible`]: the content of its header button.
#[slot]
pub struct CollapsibleHeader {
    children: Children,
}

/// The body slot of a [`Collapsible`].
#[slot]
pub struct CollapsibleBody {
    children: Children,
    #[prop(into, optional)]
    classes: Classes,
}
