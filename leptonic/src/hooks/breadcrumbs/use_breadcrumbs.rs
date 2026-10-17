// Upstream: react-aria/src/breadcrumbs/useBreadcrumbs.ts @ 99e6102368
// Upstream: react-aria/test/breadcrumbs/useBreadcrumbs.test.js @ 99e6102368
use leptos::prelude::*;

use crate::{
    IntoAttrs,
    utils::{
        intl_strings::{BreadcrumbsStrings, use_localized_strings},
        part::{Part, Remount},
        signal_attr::{SignalAttr, signal_attr},
    },
};

// No deviations from react-aria beyond the project-wide API conventions.

/// Input of [`use_breadcrumbs`].
#[derive(Debug, Clone, Default)]
pub struct UseBreadcrumbsInput {
    /// Names the navigation landmark. Default: "Breadcrumbs".
    pub aria_label: MaybeProp<String>,
}

/// The return value of [`use_breadcrumbs`].
#[derive(Debug)]
pub struct UseBreadcrumbsReturn {
    /// The breadcrumbs' list element.
    pub breadcrumbs: Part<UseBreadcrumbsProps>,
}

/// The navigation element's props from [`use_breadcrumbs`]: its name.
#[derive(Debug)]
pub struct UseBreadcrumbsProps {
    pub aria_label: Signal<String>,
}

impl Remount for UseBreadcrumbsProps {
    fn remount(&self) -> Self {
        Self {
            aria_label: self.aria_label,
        }
    }
}

impl IntoAttrs for UseBreadcrumbsProps {
    type Attrs = UseBreadcrumbsAttrs;

    fn into_attrs(self) -> Self::Attrs {
        signal_attr("aria-label", self.aria_label)
    }
}

/// Spread onto the navigation element: `<nav {..attrs}>`.
pub type UseBreadcrumbsAttrs = SignalAttr<String>;

/// Breadcrumbs: the trail of links to the current page, a navigation landmark (named
/// "Breadcrumbs" by default) around a list of items. Render each item with
/// [`use_breadcrumb_item`](fn@super::use_breadcrumb_item).
pub fn use_breadcrumbs(input: UseBreadcrumbsInput) -> UseBreadcrumbsReturn {
    crate::metrics::subject!();
    let UseBreadcrumbsInput { aria_label } = input;
    let strings = use_localized_strings::<BreadcrumbsStrings>();
    UseBreadcrumbsReturn {
        breadcrumbs: Part::stored(UseBreadcrumbsProps {
            aria_label: Signal::derive(move || {
                aria_label
                    .get()
                    .unwrap_or_else(|| strings.read().breadcrumbs())
            }),
        }),
    }
}
