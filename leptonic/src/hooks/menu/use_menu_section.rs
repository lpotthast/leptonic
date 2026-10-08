// Upstream: react-aria/src/menu/useMenuSection.ts @ 99e6102368
use leptos::{
    attr::{self, Attr},
    prelude::*,
};

use super::MenuData;
use crate::{
    hooks::{
        IntoAttrs,
        collections::{Key, NodeKind, use_node_aria_label},
    },
    utils::{aria::AriaRole, id::use_id},
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - The heading and `aria-label` come from the collection: the section's `Header` child and its
//   `aria_label`.
//
// =============================================================================

/// Input of [`use_menu_section`].
#[derive(Debug, Clone)]
pub struct UseMenuSectionInput {
    pub menu: MenuData,
    /// The section's key in the menu's collection.
    pub key: Key,
}

/// Return value of [`use_menu_section`].
#[derive(Debug)]
pub struct UseMenuSectionReturn {
    /// For the element wrapping heading and group (e.g. an `<li>` in a `<ul>` menu).
    pub item_props: UseMenuSectionItemProps,
    /// For the heading element; `None` when the section has no header.
    pub heading_props: Option<UseMenuSectionHeadingProps>,
    /// For the element containing the section's items.
    pub group_props: UseMenuSectionGroupProps,
    /// The header text, if any.
    pub heading: Option<String>,
}

#[derive(Debug)]
pub struct UseMenuSectionItemProps {
    pub role: AriaRole,
}

pub type UseMenuSectionItemAttrs = (Attr<attr::Role, AriaRole>,);

impl IntoAttrs for UseMenuSectionItemProps {
    type Attrs = UseMenuSectionItemAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (Attr(attr::Role, self.role),)
    }
}

#[derive(Debug)]
pub struct UseMenuSectionHeadingProps {
    pub id: String,
    pub role: AriaRole,
}

pub type UseMenuSectionHeadingAttrs = (Attr<attr::Id, String>, Attr<attr::Role, AriaRole>);

impl IntoAttrs for UseMenuSectionHeadingProps {
    type Attrs = UseMenuSectionHeadingAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (Attr(attr::Id, self.id), Attr(attr::Role, self.role))
    }
}

#[derive(Debug)]
pub struct UseMenuSectionGroupProps {
    pub role: AriaRole,
    pub aria_label: Signal<Option<String>>,
    pub aria_labelledby: Option<String>,
}

pub type UseMenuSectionGroupAttrs = (
    Attr<attr::Role, AriaRole>,
    Attr<attr::AriaLabel, Signal<Option<String>>>,
    Attr<attr::AriaLabelledby, Option<String>>,
);

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

/// A section of a menu: a group of items with an optional heading.
pub fn use_menu_section(input: UseMenuSectionInput) -> UseMenuSectionReturn {
    let UseMenuSectionInput { menu, key } = input;
    let heading_id = use_id("menu-section-heading");

    // Whether the section has a heading is read once (it decides what renders); its label
    // follows the collection.
    let heading = untrack(|| {
        menu.state.collection.with(|c| {
            c.children(&key)
                .find(|n| n.kind == NodeKind::Header)
                .map(|n| n.text_value.to_string())
        })
    });
    let aria_label = use_node_aria_label(menu.state.collection, key);

    UseMenuSectionReturn {
        item_props: UseMenuSectionItemProps {
            role: AriaRole::Presentation,
        },
        heading_props: heading.as_ref().map(|_| UseMenuSectionHeadingProps {
            id: heading_id.clone(),
            role: AriaRole::Presentation,
        }),
        group_props: UseMenuSectionGroupProps {
            role: AriaRole::Group,
            aria_label: aria_label.into(),
            aria_labelledby: heading.as_ref().map(|_| heading_id),
        },
        heading,
    }
}
