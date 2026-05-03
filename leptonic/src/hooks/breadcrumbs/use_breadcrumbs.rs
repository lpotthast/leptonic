use leptos::{attr, attr::Attr};
use uuid::Uuid;

use crate::hooks::IntoAttrs;

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/@react-aria/breadcrumbs/src/useBreadcrumbs.ts

//
// No intentional deviations from the react-aria implementation.
//
// NOTE: This hook intentionally has no `disabled` input.
// `aria-disabled` is meant for interactive elements (buttons, links, inputs),
// not for structural/landmark elements like `<nav>`. Disabling breadcrumbs is
// a per-item concern, handled by `use_breadcrumb_item` via `is_disabled`.
// React-aria's Spectrum `Breadcrumbs` component offers a container-level
// `isDisabled` prop, but it simply forwards it to each child item — it does
// not set anything on the `<nav>` element itself.
//

/// Input parameters for the `use_breadcrumbs` hook.
#[derive(Debug, Clone)]
pub struct UseBreadcrumbsInput {
    /// The label for the breadcrumbs navigation.
    pub label: Option<String>,
}

impl Default for UseBreadcrumbsInput {
    fn default() -> Self {
        Self {
            label: Some("Breadcrumbs".to_string()),
        }
    }
}

/// The return value of the `use_breadcrumbs` hook.
pub struct UseBreadcrumbsReturn {
    /// Props for the breadcrumbs navigation element.
    pub nav_props: UseBreadcrumbsProps,

    /// The ID of the navigation.
    pub nav_id: String,
}

/// Props from `use_breadcrumbs` that can be extracted and merged programmatically.
#[derive(Debug)]
pub struct UseBreadcrumbsProps {
    pub id: String,
    pub aria_label: Option<String>,
}

impl IntoAttrs for UseBreadcrumbsProps {
    type Attrs = UseBreadcrumbsAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            Attr(attr::Id, self.id),
            Attr(attr::AriaLabel, self.aria_label),
        )
    }
}

/// Attributes for the breadcrumbs navigation element.
pub type UseBreadcrumbsAttrs = (
    Attr<attr::Id, String>,
    Attr<attr::AriaLabel, Option<String>>,
);

/// Provides the behavior and accessibility for a breadcrumbs component.
///
/// Breadcrumbs show a navigation trail for the current location.
///
/// # Example
///
/// ```ignore
/// let breadcrumbs = use_breadcrumbs(UseBreadcrumbsInput::default());
///
/// view! {
///     <nav {..breadcrumbs.nav_props}>
///         <ol>
///             // Breadcrumb items...
///         </ol>
///     </nav>
/// }
/// ```
pub fn use_breadcrumbs(input: UseBreadcrumbsInput) -> UseBreadcrumbsReturn {
    let UseBreadcrumbsInput { label } = input;

    let nav_id = format!("breadcrumbs-{}", Uuid::new_v4());

    UseBreadcrumbsReturn {
        nav_props: UseBreadcrumbsProps {
            id: nav_id.clone(),
            aria_label: label,
        },
        nav_id,
    }
}
