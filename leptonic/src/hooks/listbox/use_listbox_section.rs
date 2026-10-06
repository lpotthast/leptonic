// Upstream: react-aria/src/listbox/useListBoxSection.ts @ 99e6102368
use leptos::{
    attr::{self, Attr},
    ev,
    ev::{On, SharedEventCallback},
    prelude::*,
};
use web_sys::MouseEvent;

use super::ListBoxData;
use crate::{
    hooks::{
        IntoAttrs,
        collections::{Key, NodeKind},
    },
    utils::{EventHandler, aria::AriaRole, id::use_id},
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

/// Input of [`use_listbox_section`].
#[derive(Debug, Clone)]
pub struct UseListBoxSectionInput {
    pub list: ListBoxData,
    /// The section's key in the listbox's collection.
    pub key: Key,
}

/// Return value of [`use_listbox_section`].
#[derive(Debug)]
pub struct UseListBoxSectionReturn {
    /// For the element wrapping heading and group (e.g. an `<li>` in a `<ul>` listbox).
    pub item_props: UseListBoxSectionItemProps,
    /// For the heading element; `None` when the section has no header.
    pub heading_props: Option<UseListBoxSectionHeadingProps>,
    /// For the element containing the section's options.
    pub group_props: UseListBoxSectionGroupProps,
    /// The header text, if any.
    pub heading: Option<String>,
}

#[derive(Debug)]
pub struct UseListBoxSectionItemProps {
    pub role: AriaRole,
}

pub type UseListBoxSectionItemAttrs = (Attr<attr::Role, AriaRole>,);

impl IntoAttrs for UseListBoxSectionItemProps {
    type Attrs = UseListBoxSectionItemAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (Attr(attr::Role, self.role),)
    }
}

#[derive(Debug)]
pub struct UseListBoxSectionHeadingProps {
    pub id: String,
    pub role: AriaRole,
    /// Keeps focus in the listbox when the heading is clicked.
    pub on_mousedown: EventHandler<MouseEvent>,
}

pub type UseListBoxSectionHeadingAttrs = (
    Attr<attr::Id, String>,
    Attr<attr::Role, AriaRole>,
    On<ev::mousedown, SharedEventCallback<MouseEvent>>,
);

impl IntoAttrs for UseListBoxSectionHeadingProps {
    type Attrs = UseListBoxSectionHeadingAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            Attr(attr::Id, self.id),
            Attr(attr::Role, self.role),
            self.on_mousedown.into_on(ev::mousedown),
        )
    }
}

#[derive(Debug)]
pub struct UseListBoxSectionGroupProps {
    pub role: AriaRole,
    pub aria_label: Option<String>,
    pub aria_labelledby: Option<String>,
}

pub type UseListBoxSectionGroupAttrs = (
    Attr<attr::Role, AriaRole>,
    Attr<attr::AriaLabel, Option<String>>,
    Attr<attr::AriaLabelledby, Option<String>>,
);

impl IntoAttrs for UseListBoxSectionGroupProps {
    type Attrs = UseListBoxSectionGroupAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            Attr(attr::Role, self.role),
            Attr(attr::AriaLabel, self.aria_label),
            Attr(attr::AriaLabelledby, self.aria_labelledby),
        )
    }
}

/// A section of a listbox: a group of options with an optional heading.
pub fn use_listbox_section(input: UseListBoxSectionInput) -> UseListBoxSectionReturn {
    let UseListBoxSectionInput { list, key } = input;
    let heading_id = use_id("listbox-section-heading");

    let (aria_label, heading) = untrack(|| {
        list.state.collection.with(|c| {
            let aria_label = c
                .get(&key)
                .and_then(|n| n.aria_label.as_deref().map(str::to_owned));
            let heading = c
                .children(&key)
                .find(|n| n.kind == NodeKind::Header)
                .map(|n| n.text_value.to_string());
            (aria_label, heading)
        })
    });

    UseListBoxSectionReturn {
        item_props: UseListBoxSectionItemProps {
            role: AriaRole::Presentation,
        },
        heading_props: heading.as_ref().map(|_| UseListBoxSectionHeadingProps {
            id: heading_id.clone(),
            role: AriaRole::Presentation,
            on_mousedown: EventHandler::new(|e: MouseEvent| e.prevent_default()),
        }),
        group_props: UseListBoxSectionGroupProps {
            role: AriaRole::Group,
            aria_label,
            aria_labelledby: heading.as_ref().map(|_| heading_id),
        },
        heading,
    }
}
