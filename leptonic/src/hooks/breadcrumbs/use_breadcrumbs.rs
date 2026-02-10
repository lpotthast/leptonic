use leptos::attr;
use leptos::attr::Attr;
use leptos::prelude::*;
use uuid::Uuid;

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/@react-aria/breadcrumbs/src/useBreadcrumbs.ts

// TODO: This tries to reimplement behavior already covered by use_press.

/// Input parameters for the `use_breadcrumbs` hook.
#[derive(Debug, Clone)]
pub struct UseBreadcrumbsInput {
    /// The label for the breadcrumbs navigation.
    pub label: Option<String>,

    /// Whether the breadcrumbs are disabled.
    pub is_disabled: Signal<bool>,
}

impl Default for UseBreadcrumbsInput {
    fn default() -> Self {
        Self {
            label: Some("Breadcrumbs".to_string()),
            is_disabled: Signal::derive(|| false),
        }
    }
}

/// The return value of the `use_breadcrumbs` hook.
pub struct UseBreadcrumbsReturn {
    /// Props for the breadcrumbs navigation element.
    pub nav_props: UseBreadcrumbsAttrs,

    /// The ID of the navigation.
    pub nav_id: String,
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
    let UseBreadcrumbsInput {
        label,
        is_disabled: disabled,
    } = input;

    let nav_id = format!("breadcrumbs-{}", Uuid::new_v4());

    UseBreadcrumbsReturn {
        nav_props: (
            Attr(attr::Id, nav_id.clone()),
            Attr(attr::AriaLabel, label),
        ),
        nav_id,
    }
}
