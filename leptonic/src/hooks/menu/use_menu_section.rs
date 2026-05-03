use leptos::{attr, attr::Attr};
use uuid::Uuid;

use crate::{hooks::IntoAttrs, utils::aria::AriaRole};

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/%40react-aria/menu/src/useMenuSection.ts

// No intentional deviations from the react-aria implementation.

/// Input parameters for the `use_menu_section` hook.
#[derive(Debug, Clone, Default)]
pub struct UseMenuSectionInput {
    /// The heading for the section. If provided, it will be used as the label for the group.
    pub heading: Option<String>,

    /// An accessibility label for the section. Required if `heading` is not present.
    pub aria_label: Option<String>,
}

/// The return value of the `use_menu_section` hook.
#[derive(Debug)]
pub struct UseMenuSectionReturn {
    /// Props for the wrapper list item.
    pub item_props: UseMenuSectionItemProps,

    /// Props for the heading element, if any.
    pub heading_props: UseMenuSectionHeadingProps,

    /// Props for the group element.
    pub group_props: UseMenuSectionGroupProps,
}

/// Props for the menu section wrapper item.
#[derive(Debug, Clone, Copy)]
pub struct UseMenuSectionItemProps {
    /// The role attribute.
    pub role: AriaRole,
}

impl IntoAttrs for UseMenuSectionItemProps {
    type Attrs = UseMenuSectionItemAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (Attr(attr::Role, self.role),)
    }
}

/// Attributes for a menu section wrapper item.
pub type UseMenuSectionItemAttrs = (Attr<attr::Role, AriaRole>,);

/// Props for the menu section heading element.
#[derive(Debug)]
pub struct UseMenuSectionHeadingProps {
    /// The id of the heading element, for aria-labelledby.
    pub id: Option<String>,

    /// The role attribute. Set to "presentation" to hide from assistive technology.
    pub role: Option<AriaRole>,
}

impl IntoAttrs for UseMenuSectionHeadingProps {
    type Attrs = UseMenuSectionHeadingAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (Attr(attr::Id, self.id), Attr(attr::Role, self.role))
    }
}

/// Attributes for a menu section heading element.
pub type UseMenuSectionHeadingAttrs = (
    Attr<attr::Id, Option<String>>,
    Attr<attr::Role, Option<AriaRole>>,
);

/// Props for the menu section group element.
#[derive(Debug)]
pub struct UseMenuSectionGroupProps {
    /// The role attribute.
    pub role: AriaRole,

    /// An accessibility label for the section.
    pub aria_label: Option<String>,

    /// The id of the heading that labels this group.
    pub aria_labelledby: Option<String>,
}

impl IntoAttrs for UseMenuSectionGroupProps {
    type Attrs = UseMenuSectionGroupAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            Attr(attr::Role, self.role),
            Attr(attr::AriaLabel, self.aria_label),
            Attr(attr::AriaLabelledby, self.aria_labelledby),
        )
    }
}

/// Attributes for a menu section group element.
pub type UseMenuSectionGroupAttrs = (
    Attr<attr::Role, AriaRole>,
    Attr<attr::AriaLabel, Option<String>>,
    Attr<attr::AriaLabelledby, Option<String>>,
);

/// Provides the behavior and accessibility implementation for a section in a menu.
///
/// Menu sections are used to group related menu items together. Each section can have
/// an optional heading that is announced by screen readers but hidden from the visual
/// DOM structure (using `role="presentation"`).
///
/// # Example
///
/// ```ignore
/// let section = use_menu_section(UseMenuSectionInput {
///     heading: Some("File Actions".to_string()),
///     aria_label: None,
/// });
///
/// view! {
///     <li {..section.item_props.into_attrs()}>
///         <span {..section.heading_props.into_attrs()}>
///             { section_heading }
///         </span>
///         <ul {..section.group_props.into_attrs()}>
///             // Menu items here
///         </ul>
///     </li>
/// }
/// ```
pub fn use_menu_section(input: UseMenuSectionInput) -> UseMenuSectionReturn {
    let UseMenuSectionInput {
        heading,
        aria_label,
    } = input;

    let heading_id = format!("menu-section-heading-{}", Uuid::new_v4());

    let has_heading = heading.is_some();

    UseMenuSectionReturn {
        item_props: UseMenuSectionItemProps {
            role: AriaRole::Presentation,
        },
        heading_props: if has_heading {
            UseMenuSectionHeadingProps {
                // Technically, menus cannot contain headings according to ARIA.
                // We hide the heading from assistive technology, using role="presentation",
                // and only use it as a label for the nested group.
                id: Some(heading_id.clone()),
                role: Some(AriaRole::Presentation),
            }
        } else {
            UseMenuSectionHeadingProps {
                id: None,
                role: None,
            }
        },
        group_props: UseMenuSectionGroupProps {
            role: AriaRole::Group,
            aria_label,
            aria_labelledby: if has_heading { Some(heading_id) } else { None },
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_menu_section_with_heading() {
        let result = use_menu_section(UseMenuSectionInput {
            heading: Some("Actions".to_string()),
            aria_label: None,
        });

        assert_eq!(result.item_props.role, AriaRole::Presentation);
        assert_eq!(result.heading_props.role, Some(AriaRole::Presentation));
        assert!(result.heading_props.id.is_some());
        assert_eq!(result.group_props.role, AriaRole::Group);
        assert!(result.group_props.aria_labelledby.is_some());
    }

    #[test]
    fn test_menu_section_with_aria_label() {
        let result = use_menu_section(UseMenuSectionInput {
            heading: None,
            aria_label: Some("Actions".to_string()),
        });

        assert_eq!(result.item_props.role, AriaRole::Presentation);
        assert!(result.heading_props.id.is_none());
        assert_eq!(result.group_props.aria_label, Some("Actions".to_string()));
        assert!(result.group_props.aria_labelledby.is_none());
    }
}
