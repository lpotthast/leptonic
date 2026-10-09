// Upstream: react-aria/src/gridlist/useGridListSection.ts @ 99e6102368
// Upstream: react-aria/src/utils/useLabels.ts @ 99e6102368
// Upstream: react-aria-components/test/GridList.test.js @ 99e6102368
use leptos::{
    attr::{self, Attr},
    prelude::*,
};

use super::GridListData;
use crate::{
    ElementCaptureAttr, IntoAttrs,
    hooks::collections::{Key, NodeKind},
    use_slot,
    utils::{aria::AriaRole, id::use_id},
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

/// Return value of [`use_grid_list_section`].
#[derive(Debug)]
pub struct UseGridListSectionReturn {
    /// For the row holding the header.
    pub row_props: UseGridListSectionRowProps,
    /// For the header cell: it labels the row group while rendered.
    pub row_header_props: UseGridListSectionRowHeaderProps,
    /// For the element containing the section's rows.
    pub row_group_props: UseGridListSectionRowGroupProps,
    /// The header text from the collection, if any.
    pub heading: Signal<Option<String>>,
}

#[derive(Debug)]
pub struct UseGridListSectionRowProps {
    pub role: AriaRole,
}

pub type UseGridListSectionRowAttrs = (Attr<attr::Role, AriaRole>,);

impl IntoAttrs for UseGridListSectionRowProps {
    type Attrs = UseGridListSectionRowAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (Attr(attr::Role, self.role),)
    }
}

#[derive(Debug)]
pub struct UseGridListSectionRowHeaderProps {
    pub id: String,
    pub role: AriaRole,
    /// Detects whether the header is rendered (react-aria's `useSlotId`): the row group
    /// references it only then.
    pub element_capture: ElementCaptureAttr,
}

pub type UseGridListSectionRowHeaderAttrs = (
    Attr<attr::Id, String>,
    Attr<attr::Role, AriaRole>,
    ElementCaptureAttr,
);

impl IntoAttrs for UseGridListSectionRowHeaderProps {
    type Attrs = UseGridListSectionRowHeaderAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            Attr(attr::Id, self.id),
            Attr(attr::Role, self.role),
            self.element_capture,
        )
    }
}

#[derive(Debug)]
pub struct UseGridListSectionRowGroupProps {
    pub id: String,
    pub role: AriaRole,
    pub aria_label: Signal<Option<String>>,
    pub aria_labelledby: Signal<Option<String>>,
}

pub type UseGridListSectionRowGroupAttrs = (
    Attr<attr::Id, String>,
    Attr<attr::Role, AriaRole>,
    Attr<attr::AriaLabel, Signal<Option<String>>>,
    Attr<attr::AriaLabelledby, Signal<Option<String>>>,
);

impl IntoAttrs for UseGridListSectionRowGroupProps {
    type Attrs = UseGridListSectionRowGroupAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            Attr(attr::Id, self.id),
            Attr(attr::Role, self.role),
            Attr(attr::AriaLabel, self.aria_label),
            Attr(attr::AriaLabelledby, self.aria_labelledby),
        )
    }
}

/// A section of a grid list: a row group with an optional header row. The group is labelled by
/// its rendered header and its `aria_label` (both, as react-aria's `useLabels`).
pub fn use_grid_list_section(input: UseGridListSectionInput) -> UseGridListSectionReturn {
    let UseGridListSectionInput { list, key } = input;
    let id = use_id("grid-list-section");
    let header = use_slot("grid-list-section-header");
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
    let heading_id = header.referenced_id;
    let group_id = id.clone();
    let aria_labelledby = Signal::derive(move || {
        let heading_id = heading_id.get()?;
        Some(if aria_label.with(Option::is_some) {
            format!("{group_id} {heading_id}")
        } else {
            heading_id
        })
    });

    UseGridListSectionReturn {
        row_props: UseGridListSectionRowProps {
            role: AriaRole::Row,
        },
        row_header_props: UseGridListSectionRowHeaderProps {
            id: header.props.id,
            role: AriaRole::Rowheader,
            element_capture: header.props.element_capture,
        },
        row_group_props: UseGridListSectionRowGroupProps {
            id,
            role: AriaRole::Rowgroup,
            aria_label: aria_label.into(),
            aria_labelledby,
        },
        heading: heading.into(),
    }
}
