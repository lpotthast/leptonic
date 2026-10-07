// Upstream: react-aria/src/breadcrumbs/useBreadcrumbs.ts @ 99e6102368
use leptos::{attr, attr::Attr, prelude::*};

use crate::hooks::IntoAttrs;

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## DIFFERENT BEHAVIOR
// - The default label is English ("Breadcrumbs") until localized strings are ported.
//
// =============================================================================

/// The label of breadcrumbs without an `aria_label`.
const DEFAULT_LABEL: &str = "Breadcrumbs";

/// Input of [`use_breadcrumbs`].
#[derive(Debug, Clone, Default)]
pub struct UseBreadcrumbsInput {
    /// Names the navigation landmark. Default: "Breadcrumbs".
    pub aria_label: MaybeProp<String>,
}

/// Return value of [`use_breadcrumbs`].
#[derive(Debug)]
pub struct UseBreadcrumbsReturn {
    /// For the breadcrumbs' navigation element: a `<nav>` around the list of items (react-aria's
    /// `navProps`). React-aria-components (and the `Breadcrumbs` atom) spread them onto the
    /// `<ol>` itself.
    pub props: UseBreadcrumbsProps,
}

/// The navigation element's props from [`use_breadcrumbs`]: its name.
#[derive(Debug)]
pub struct UseBreadcrumbsProps {
    pub aria_label: Signal<String>,
}

impl IntoAttrs for UseBreadcrumbsProps {
    type Attrs = UseBreadcrumbsAttrs;

    fn into_attrs(self) -> Self::Attrs {
        Attr(attr::AriaLabel, self.aria_label)
    }
}

/// Spread onto the navigation element: `<nav {..attrs}>`.
pub type UseBreadcrumbsAttrs = Attr<attr::AriaLabel, Signal<String>>;

/// Breadcrumbs: the trail of links to the current page, a navigation landmark (named
/// "Breadcrumbs" by default) around a list of items. Render each item with
/// [`use_breadcrumb_item`](super::use_breadcrumb_item).
pub fn use_breadcrumbs(input: UseBreadcrumbsInput) -> UseBreadcrumbsReturn {
    let UseBreadcrumbsInput { aria_label } = input;
    UseBreadcrumbsReturn {
        props: UseBreadcrumbsProps {
            aria_label: Signal::derive(move || {
                aria_label.get().unwrap_or_else(|| DEFAULT_LABEL.to_owned())
            }),
        },
    }
}
