// Upstream: react-aria/src/menu/useMenuSection.ts @ 99e6102368
// Upstream: react-aria/test/menu/useMenu.test.tsx @ 99e6102368
// Upstream: react-aria-components/test/Menu.test.tsx @ 99e6102368
use leptos::prelude::*;

use super::MenuData;
use crate::{
    Id, IntoAttrs,
    hooks::collections::{Key, NodeKind, UseNodeAriaLabelInput, use_node_aria_label},
    utils::{
        aria::AriaRole,
        id::use_id,
        part::{Part, Remount},
        signal_attr::{FixedAttr, SignalAttr, fixed_attr, signal_attr},
    },
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

/// The return value of [`use_menu_section`].
#[derive(Debug)]
pub struct UseMenuSectionReturn {
    /// The element wrapping heading and group.
    pub section: Part<UseMenuSectionItemProps>,
    /// The section's heading element.
    pub header: Part<UseMenuSectionHeadingProps>,
    /// The element containing the section's items.
    pub group: Part<UseMenuSectionGroupProps>,
    /// The header text, while the section has a header (follows the collection).
    pub heading: Signal<Option<String>>,
}

#[derive(Debug)]
pub struct UseMenuSectionItemProps {
    pub role: AriaRole,
}

impl Remount for UseMenuSectionItemProps {
    fn remount(&self) -> Self {
        Self { role: self.role }
    }
}

pub type UseMenuSectionItemAttrs = (FixedAttr,);

impl IntoAttrs for UseMenuSectionItemProps {
    type Attrs = UseMenuSectionItemAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (fixed_attr("role", self.role),)
    }
}

#[derive(Debug)]
pub struct UseMenuSectionHeadingProps {
    pub id: Id,
    pub role: AriaRole,
}

impl Remount for UseMenuSectionHeadingProps {
    fn remount(&self) -> Self {
        Self {
            id: self.id.clone(),
            role: self.role,
        }
    }
}

pub type UseMenuSectionHeadingAttrs = (FixedAttr, FixedAttr);

impl IntoAttrs for UseMenuSectionHeadingProps {
    type Attrs = UseMenuSectionHeadingAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (fixed_attr("id", self.id), fixed_attr("role", self.role))
    }
}

#[derive(Debug)]
pub struct UseMenuSectionGroupProps {
    pub role: AriaRole,
    pub aria_label: Signal<Option<String>>,
    /// The heading, while there is one.
    pub aria_labelledby: Signal<Option<Id>>,
}

impl Remount for UseMenuSectionGroupProps {
    fn remount(&self) -> Self {
        Self {
            role: self.role,
            aria_label: self.aria_label,
            aria_labelledby: self.aria_labelledby,
        }
    }
}

pub type UseMenuSectionGroupAttrs = (
    FixedAttr,
    SignalAttr<Option<String>>,
    SignalAttr<Option<Id>>,
);

impl IntoAttrs for UseMenuSectionGroupProps {
    type Attrs = UseMenuSectionGroupAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            fixed_attr("role", self.role),
            signal_attr("aria-label", self.aria_label),
            signal_attr("aria-labelledby", self.aria_labelledby),
        )
    }
}

/// A section of a menu: a group of items with an optional heading.
pub fn use_menu_section(input: UseMenuSectionInput) -> UseMenuSectionReturn {
    crate::metrics::subject!();
    let UseMenuSectionInput { menu, key } = input;
    let heading_id = use_id("menu-section-heading");

    // The heading and the label follow the collection.
    let aria_label = use_node_aria_label(UseNodeAriaLabelInput {
        selection: menu.state.selection,
        key: key.clone(),
    });
    let collection = menu.state.collection;
    let selection = menu.state.selection;
    // Only this section's changes (its header's included) reach it (`performance.md` R1).
    let heading = Memo::new(move |_| {
        selection.track_node(&key);
        collection.with_untracked(|c| {
            c.children(&key)
                .find(|n| n.kind == NodeKind::Header)
                .map(|n| n.text_value.to_string())
        })
    });
    let labelledby_id = heading_id.clone();

    UseMenuSectionReturn {
        section: Part::stored(UseMenuSectionItemProps {
            role: AriaRole::Presentation,
        }),
        header: Part::stored(UseMenuSectionHeadingProps {
            id: heading_id,
            role: AriaRole::Presentation,
        }),
        group: Part::stored(UseMenuSectionGroupProps {
            role: AriaRole::Group,
            aria_label: aria_label.into(),
            aria_labelledby: Signal::derive(move || {
                heading.with(Option::is_some).then(|| labelledby_id.clone())
            }),
        }),
        heading: heading.into(),
    }
}
