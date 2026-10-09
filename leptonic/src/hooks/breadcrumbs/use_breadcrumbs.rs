// Upstream: react-aria/src/breadcrumbs/useBreadcrumbs.ts @ 99e6102368
use leptos::{attr, attr::Attr, prelude::*};

use crate::{
    IntoAttrs,
    utils::intl_strings::{BreadcrumbsStrings, use_localized_strings},
};

// No deviations from react-aria beyond the project-wide API conventions.

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
/// [`use_breadcrumb_item`](fn@super::use_breadcrumb_item).
pub fn use_breadcrumbs(input: UseBreadcrumbsInput) -> UseBreadcrumbsReturn {
    let UseBreadcrumbsInput { aria_label } = input;
    let strings = use_localized_strings::<BreadcrumbsStrings>();
    UseBreadcrumbsReturn {
        props: UseBreadcrumbsProps {
            aria_label: Signal::derive(move || {
                aria_label
                    .get()
                    .unwrap_or_else(|| strings.read().breadcrumbs())
            }),
        },
    }
}
