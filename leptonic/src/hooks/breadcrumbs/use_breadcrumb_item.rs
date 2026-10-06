// Upstream: react-aria/src/breadcrumbs/useBreadcrumbItem.ts @ 99e6102368
use leptos::prelude::*;

use crate::{
    hooks::{UseLinkInput, UseLinkReturn, use_link},
    utils::aria::AriaCurrent,
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - The link settings are a nested `UseLinkInput`; `isCurrent` is a signal and `aria-current` the
//   typed `current` (default `AriaCurrent::Page`).
//
// ## OMITTED FEATURES
// - Heading items (`elementType: 'h1'..'h6'`, attributes without link behavior): render the
//   current item as a heading without the hook.
// - `autoFocus` of the current item.
//
// =============================================================================

/// Input of [`use_breadcrumb_item`].
#[derive(Debug, Clone)]
pub struct UseBreadcrumbItemInput {
    /// The item's link. Its `is_disabled` disables the item.
    pub link: UseLinkInput,
    /// Whether the item is the current page (the last one): it is then no working link.
    pub is_current: Signal<bool>,
    /// What the current item is. Default: `AriaCurrent::Page`.
    pub current: AriaCurrent,
}

impl Default for UseBreadcrumbItemInput {
    fn default() -> Self {
        Self {
            link: UseLinkInput::default(),
            is_current: Signal::default(),
            current: AriaCurrent::Page,
        }
    }
}

/// An item of breadcrumbs: a link to an ancestor page, or the current page, which is announced
/// as current (`aria-current`) and can't be followed. The item is disabled while current.
pub fn use_breadcrumb_item(input: UseBreadcrumbItemInput) -> UseLinkReturn {
    let UseBreadcrumbItemInput {
        link,
        is_current,
        current,
    } = input;
    let is_disabled = link.is_disabled;
    let aria_current = link.aria_current;
    use_link(UseLinkInput {
        is_disabled: Signal::derive(move || is_disabled.get() || is_current.get()),
        aria_current: Signal::derive(move || {
            is_current
                .get()
                .then_some(current)
                .or_else(|| aria_current.get())
        }),
        ..link
    })
}
