use leptos::attr;
use leptos::attr::Attr;
use uuid::Uuid;

use crate::utils::aria::AriaHidden;

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/@react-aria/listbox/src/useListBoxSection.ts

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// No intentional deviations from the react-aria implementation.
//
// =============================================================================

/// Input parameters for the `use_listbox_section` hook.
#[derive(Debug, Clone, Default)]
pub struct UseListBoxSectionInput {
    /// The heading text for the section.
    pub heading: Option<String>,
}

/// The return value of the `use_listbox_section` hook.
#[derive(Debug)]
pub struct UseListBoxSectionReturn {
    /// Props for the section group element.
    pub group_props: UseListBoxSectionGroupProps,

    /// Props for the section heading element.
    pub heading_props: UseListBoxSectionHeadingProps,

    /// Props for section items container.
    pub items_props: UseListBoxSectionItemsProps,
}

/// Props for the section group element.
#[derive(Debug)]
pub struct UseListBoxSectionGroupProps {
    /// The role attribute.
    pub role: &'static str,

    /// The aria-labelledby attribute.
    pub aria_labelledby: Option<String>,
}

impl UseListBoxSectionGroupProps {
    /// Convert to spreadable attributes for Leptos views, consuming self.
    #[must_use]
    pub fn into_attrs(self) -> UseListBoxSectionGroupAttrs {
        (
            Attr(attr::Role, self.role),
            Attr(attr::AriaLabelledby, self.aria_labelledby),
        )
    }
}

/// Attributes for the section group element.
pub type UseListBoxSectionGroupAttrs = (
    Attr<attr::Role, &'static str>,
    Attr<attr::AriaLabelledby, Option<String>>,
);

/// Props for the section heading element.
#[derive(Debug)]
pub struct UseListBoxSectionHeadingProps {
    /// The id of the heading element.
    pub id: String,

    /// The role attribute for the heading.
    pub role: &'static str,

    /// The aria-hidden attribute.
    pub aria_hidden: AriaHidden,
}

impl UseListBoxSectionHeadingProps {
    /// Convert to spreadable attributes for Leptos views, consuming self.
    #[must_use]
    pub fn into_attrs(self) -> UseListBoxSectionHeadingAttrs {
        (
            Attr(attr::Id, self.id),
            Attr(attr::Role, self.role),
            Attr(attr::AriaHidden, self.aria_hidden),
        )
    }
}

/// Attributes for the section heading element.
pub type UseListBoxSectionHeadingAttrs = (
    Attr<attr::Id, String>,
    Attr<attr::Role, &'static str>,
    Attr<attr::AriaHidden, AriaHidden>,
);

/// Props for section items container.
#[derive(Debug)]
pub struct UseListBoxSectionItemsProps {
    /// The role attribute.
    pub role: &'static str,
}

impl UseListBoxSectionItemsProps {
    /// Convert to spreadable attributes for Leptos views, consuming self.
    #[must_use]
    pub fn into_attrs(self) -> UseListBoxSectionItemsAttrs {
        (Attr(attr::Role, self.role),)
    }
}

/// Attributes for the section items container.
pub type UseListBoxSectionItemsAttrs = (Attr<attr::Role, &'static str>,);

/// Provides accessibility attributes for a section within a listbox.
///
/// Listbox sections group related options together with an optional heading.
///
/// # Example
///
/// ```ignore
/// let section = use_listbox_section(UseListBoxSectionInput {
///     heading: Some("Fruits".to_string()),
/// });
///
/// view! {
///     <li role=section.group_props.role aria-labelledby=section.group_props.aria_labelledby>
///         <span
///             id=section.heading_props.id
///             role=section.heading_props.role
///             aria-hidden=section.heading_props.aria_hidden
///         >
///             "Fruits"
///         </span>
///         <ul role=section.items_props.role>
///             // Options here
///         </ul>
///     </li>
/// }
/// ```
#[allow(clippy::needless_pass_by_value)]
pub fn use_listbox_section(input: UseListBoxSectionInput) -> UseListBoxSectionReturn {
    let UseListBoxSectionInput { heading } = input;

    let heading_id = format!("listbox-section-heading-{}", Uuid::new_v4());

    let aria_labelledby = if heading.is_some() {
        Some(heading_id.clone())
    } else {
        None
    };

    UseListBoxSectionReturn {
        group_props: UseListBoxSectionGroupProps {
            role: "presentation",
            aria_labelledby,
        },
        heading_props: UseListBoxSectionHeadingProps {
            id: heading_id,
            role: "presentation",
            aria_hidden: AriaHidden::True,
        },
        items_props: UseListBoxSectionItemsProps { role: "group" },
    }
}
