use leptos::attr;
use leptos::attr::Attr;
use leptos::prelude::*;
use uuid::Uuid;
use crate::hooks::IntoAttrs;

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/@react-aria/breadcrumbs/src/useBreadcrumbs.ts

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// No intentional deviations from the react-aria implementation.
//
// =============================================================================

// TODO: This tries to reimplement behavior already covered by use_press.

/// Input parameters for the `use_breadcrumbs` hook.
#[derive(Debug, Clone)]
pub struct UseBreadcrumbsInput {
    /// The label for the breadcrumbs navigation.
    pub label: Option<String>,

    /// Whether the breadcrumbs are disabled.
    pub disabled: Signal<bool>,
}

impl Default for UseBreadcrumbsInput {
    fn default() -> Self {
        Self {
            label: Some("Breadcrumbs".to_string()),
            disabled: Signal::derive(|| false),
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
    let UseBreadcrumbsInput { label, disabled } = input;

    let nav_id = format!("breadcrumbs-{}", Uuid::new_v4());

    UseBreadcrumbsReturn {
        nav_props: UseBreadcrumbsProps {
            id: nav_id.clone(),
            aria_label: label,
        },
        nav_id,
    }
}
