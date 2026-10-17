// Upstream: react-aria/src/listbox/useListBoxSection.ts @ 99e6102368
// Upstream: react-aria-components/test/ListBox.test.js @ 99e6102368
use leptos::{ev, prelude::*};
use web_sys::MouseEvent;

use super::ListBoxData;
use crate::{
    EventHandler, Id, IntoAttrs, OnEvent, Part,
    hooks::collections::{Key, NodeKind, UseNodeAriaLabelInput, use_node_aria_label},
    utils::{
        aria::AriaRole,
        id::use_id,
        part::Remount,
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

/// Input of [`use_listbox_section`].
#[derive(Debug, Clone)]
pub struct UseListBoxSectionInput {
    pub list: ListBoxData,
    /// The section's key in the listbox's collection.
    pub key: Key,
}

/// The return value of [`use_listbox_section`].
#[derive(Debug)]
pub struct UseListBoxSectionReturn {
    /// The element wrapping heading and group (e.g. an `<li>` in a `<ul>` listbox).
    pub section: Part<UseListBoxSectionItemProps>,
    /// The heading element (the section's header); `None` when the section has no header.
    pub header: Option<Part<UseListBoxSectionHeadingProps>>,
    /// The element containing the section's options.
    pub group: Part<UseListBoxSectionGroupProps>,
    /// The header text, if any (follows the collection).
    pub heading: Signal<Option<String>>,
}

#[derive(Debug)]
pub struct UseListBoxSectionItemProps {
    pub role: AriaRole,
}

impl Remount for UseListBoxSectionItemProps {
    fn remount(&self) -> Self {
        Self { role: self.role }
    }
}

pub type UseListBoxSectionItemAttrs = (FixedAttr,);

impl IntoAttrs for UseListBoxSectionItemProps {
    type Attrs = UseListBoxSectionItemAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (fixed_attr("role", self.role),)
    }
}

#[derive(Debug)]
pub struct UseListBoxSectionHeadingProps {
    pub id: Id,
    pub role: AriaRole,
    /// Keeps focus in the listbox when the heading is clicked.
    pub on_mousedown: EventHandler<MouseEvent>,
}

pub type UseListBoxSectionHeadingAttrs = (FixedAttr, FixedAttr, OnEvent<ev::mousedown>);

impl IntoAttrs for UseListBoxSectionHeadingProps {
    type Attrs = UseListBoxSectionHeadingAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            fixed_attr("id", self.id),
            fixed_attr("role", self.role),
            self.on_mousedown.into_on(ev::mousedown),
        )
    }
}

#[derive(Debug)]
pub struct UseListBoxSectionGroupProps {
    pub role: AriaRole,
    pub aria_label: Signal<Option<String>>,
    pub aria_labelledby: Option<Id>,
}

impl Remount for UseListBoxSectionGroupProps {
    fn remount(&self) -> Self {
        Self {
            role: self.role,
            aria_label: self.aria_label,
            aria_labelledby: self.aria_labelledby.clone(),
        }
    }
}

pub type UseListBoxSectionGroupAttrs = (FixedAttr, SignalAttr<Option<String>>, FixedAttr);

impl IntoAttrs for UseListBoxSectionGroupProps {
    type Attrs = UseListBoxSectionGroupAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            fixed_attr("role", self.role),
            signal_attr("aria-label", self.aria_label),
            fixed_attr("aria-labelledby", self.aria_labelledby),
        )
    }
}

/// A section of a listbox: a group of options with an optional heading.
pub fn use_listbox_section(input: UseListBoxSectionInput) -> UseListBoxSectionReturn {
    crate::metrics::subject!();
    let UseListBoxSectionInput { list, key } = input;
    let heading_id = use_id("listbox-section-heading");

    // Whether the section has a heading is read once (it decides what renders); its text and
    // the section's label follow the collection.
    let heading = {
        let (collection, key) = (list.state.collection, key.clone());
        Memo::new(move |_| {
            collection.with(|c| {
                c.children(&key)
                    .find(|n| n.kind == NodeKind::Header)
                    .map(|n| n.text_value.to_string())
            })
        })
    };
    let has_heading = heading.with_untracked(Option::is_some);
    let aria_label = use_node_aria_label(UseNodeAriaLabelInput {
        selection: list.state.selection,
        key,
    });

    UseListBoxSectionReturn {
        section: Part::stored(UseListBoxSectionItemProps {
            role: AriaRole::Presentation,
        }),
        header: has_heading.then(|| {
            let id = heading_id.clone();
            let on_mousedown = EventHandler::new(|e: MouseEvent| e.prevent_default());
            Part::new(move || UseListBoxSectionHeadingProps {
                id: id.clone(),
                role: AriaRole::Presentation,
                on_mousedown: on_mousedown.clone(),
            })
        }),
        group: Part::stored(UseListBoxSectionGroupProps {
            role: AriaRole::Group,
            aria_label: aria_label.into(),
            aria_labelledby: has_heading.then_some(heading_id),
        }),
        heading: heading.into(),
    }
}
