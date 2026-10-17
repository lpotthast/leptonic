// Upstream: react-aria/src/gridlist/useGridListSection.ts @ 99e6102368
// Upstream: react-aria/src/utils/useLabels.ts @ 99e6102368
// Upstream: react-aria-components/test/GridList.test.js @ 99e6102368
use leptos::prelude::*;

use super::GridListData;
use crate::{
    Id, IntoAttrs, Part,
    hooks::collections::{Key, NodeKind},
    utils::{
        aria::AriaRole,
        id::use_id,
        part::Remount,
        signal_attr::{FixedAttr, SignalAttr, fixed_attr, signal_attr},
        slot_id::referenced_id,
    },
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - The header text and `aria-label` come from the collection (the section's `Header` child and
//   its `aria_label`) and follow it.
//
// =============================================================================

/// Input of [`use_grid_list_section`].
#[derive(Debug, Clone)]
pub struct UseGridListSectionInput {
    pub list: GridListData,
    /// The section's key in the grid list's collection.
    pub key: Key,
}

/// The return value of [`use_grid_list_section`].
#[derive(Debug)]
pub struct UseGridListSectionReturn {
    /// The header: the row holding it and its cell, which labels the row group while rendered.
    pub header: Part<(UseGridListSectionRowProps, UseGridListSectionRowHeaderProps)>,
    /// The section's row group element.
    pub row_group: Part<UseGridListSectionRowGroupProps>,
    /// The header text from the collection, if any.
    pub heading: Signal<Option<String>>,
}

#[derive(Debug)]
pub struct UseGridListSectionRowProps {
    pub role: AriaRole,
}

pub type UseGridListSectionRowAttrs = (FixedAttr,);

impl IntoAttrs for UseGridListSectionRowProps {
    type Attrs = UseGridListSectionRowAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (fixed_attr("role", self.role),)
    }
}

#[derive(Debug)]
pub struct UseGridListSectionRowHeaderProps {
    pub id: Id,
    pub role: AriaRole,
}

pub type UseGridListSectionRowHeaderAttrs = (FixedAttr, FixedAttr);

impl IntoAttrs for UseGridListSectionRowHeaderProps {
    type Attrs = UseGridListSectionRowHeaderAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (fixed_attr("id", self.id), fixed_attr("role", self.role))
    }
}

#[derive(Debug)]
pub struct UseGridListSectionRowGroupProps {
    pub id: Id,
    pub role: AriaRole,
    pub aria_label: Signal<Option<String>>,
    pub aria_labelledby: Signal<Option<Id>>,
}

impl Remount for UseGridListSectionRowGroupProps {
    fn remount(&self) -> Self {
        Self {
            id: self.id.clone(),
            role: self.role,
            aria_label: self.aria_label,
            aria_labelledby: self.aria_labelledby,
        }
    }
}

pub type UseGridListSectionRowGroupAttrs = (
    FixedAttr,
    FixedAttr,
    SignalAttr<Option<String>>,
    SignalAttr<Option<Id>>,
);

impl IntoAttrs for UseGridListSectionRowGroupProps {
    type Attrs = UseGridListSectionRowGroupAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            fixed_attr("id", self.id),
            fixed_attr("role", self.role),
            signal_attr("aria-label", self.aria_label),
            signal_attr("aria-labelledby", self.aria_labelledby),
        )
    }
}

/// A section of a grid list: a row group with an optional header row. The group is labelled by
/// its rendered header and its `aria_label` (both, as react-aria's `useLabels`).
pub fn use_grid_list_section(input: UseGridListSectionInput) -> UseGridListSectionReturn {
    crate::metrics::subject!();
    let UseGridListSectionInput { list, key } = input;
    let id = use_id("grid-list-section");
    // The row group references the header while it is rendered (react-aria's `useSlotId`).
    let header_id = use_id("grid-list-section-header");
    let header = Part::observed({
        let id = header_id.clone();
        move || {
            (
                UseGridListSectionRowProps {
                    role: AriaRole::Row,
                },
                UseGridListSectionRowHeaderProps {
                    id: id.clone(),
                    role: AriaRole::Rowheader,
                },
            )
        }
    });
    let collection = list.state.collection;
    let key = StoredValue::new(key);
    let aria_label = Memo::new(move |_| {
        key.with_value(|key| {
            collection.with(|c| {
                c.get(key)
                    .and_then(|n| n.aria_label.as_deref().map(str::to_owned))
            })
        })
    });
    let heading = Memo::new(move |_| {
        key.with_value(|key| {
            collection.with(|c| {
                c.children(key)
                    .find(|n| n.kind == NodeKind::Header)
                    .map(|n| n.text_value.to_string())
            })
        })
    });
    let heading_id = referenced_id(&header, header_id);
    let group_id = id.clone();
    let aria_labelledby = Signal::derive(move || {
        let heading_id = heading_id.get()?;
        Some(if aria_label.with(Option::is_some) {
            Id::derived(format_args!("{group_id} {heading_id}"))
        } else {
            heading_id
        })
    });

    UseGridListSectionReturn {
        header,
        row_group: Part::stored(UseGridListSectionRowGroupProps {
            id,
            role: AriaRole::Rowgroup,
            aria_label: aria_label.into(),
            aria_labelledby,
        }),
        heading: heading.into(),
    }
}
