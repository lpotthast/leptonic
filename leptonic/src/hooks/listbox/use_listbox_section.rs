use uuid::Uuid;

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/@react-aria/listbox/src/useListBoxSection.ts

/// Input parameters for the `use_listbox_section` hook.
#[derive(Debug, Clone, Default)]
pub struct UseListBoxSectionInput {
    /// The heading text for the section.
    pub heading: Option<String>,
}

/// The return value of the `use_listbox_section` hook.
#[derive(Debug, Clone)]
pub struct UseListBoxSectionReturn {
    /// Props for the section group element.
    pub group_props: UseListBoxSectionGroupProps,

    /// Props for the section heading element.
    pub heading_props: UseListBoxSectionHeadingProps,

    /// Props for section items container.
    pub items_props: UseListBoxSectionItemsProps,
}

/// Props for the section group element.
#[derive(Debug, Clone)]
pub struct UseListBoxSectionGroupProps {
    /// The role attribute.
    pub role: &'static str,

    /// The aria-labelledby attribute.
    pub aria_labelledby: Option<String>,
}

/// Props for the section heading element.
#[derive(Debug, Clone)]
pub struct UseListBoxSectionHeadingProps {
    /// The id of the heading element.
    pub id: String,

    /// The role attribute for the heading.
    pub role: &'static str,

    /// The aria-hidden attribute.
    pub aria_hidden: &'static str,
}

/// Props for section items container.
#[derive(Debug, Clone)]
pub struct UseListBoxSectionItemsProps {
    /// The role attribute.
    pub role: &'static str,
}

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
    let heading_id = format!("listbox-section-heading-{}", Uuid::new_v4());

    let aria_labelledby = if input.heading.is_some() {
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
            aria_hidden: "true",
        },
        items_props: UseListBoxSectionItemsProps { role: "group" },
    }
}
