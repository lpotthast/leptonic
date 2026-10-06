// Upstream: react-aria/src/gridlist/useGridListSection.ts @ 99e6102368
use leptos::{
    attr::{self, Attr},
    prelude::*,
};

use super::GridListData;
use crate::{
    hooks::{
        IntoAttrs,
        collections::{Key, NodeKind},
    },
    utils::{aria::AriaRole, id::use_id},
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - The header text and `aria-label` come from the collection (the section's `Header` child and
//   its `aria_label`); the row group is labelled by the header while there is one.
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
    /// For the header cell.
    pub row_header_props: UseGridListSectionRowHeaderProps,
    /// For the element containing the section's rows.
    pub row_group_props: UseGridListSectionRowGroupProps,
    /// The header text, if any.
    pub heading: Option<String>,
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
}

pub type UseGridListSectionRowHeaderAttrs = (Attr<attr::Id, String>, Attr<attr::Role, AriaRole>);

impl IntoAttrs for UseGridListSectionRowHeaderProps {
    type Attrs = UseGridListSectionRowHeaderAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (Attr(attr::Id, self.id), Attr(attr::Role, self.role))
    }
}

#[derive(Debug)]
pub struct UseGridListSectionRowGroupProps {
    pub role: AriaRole,
    pub aria_label: Option<String>,
    pub aria_labelledby: Option<String>,
}

pub type UseGridListSectionRowGroupAttrs = (
    Attr<attr::Role, AriaRole>,
    Attr<attr::AriaLabel, Option<String>>,
    Attr<attr::AriaLabelledby, Option<String>>,
);

impl IntoAttrs for UseGridListSectionRowGroupProps {
    type Attrs = UseGridListSectionRowGroupAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            Attr(attr::Role, self.role),
            Attr(attr::AriaLabel, self.aria_label),
            Attr(attr::AriaLabelledby, self.aria_labelledby),
        )
    }
}

/// A section of a grid list: a row group with an optional header row.
pub fn use_grid_list_section(input: UseGridListSectionInput) -> UseGridListSectionReturn {
    let UseGridListSectionInput { list, key } = input;
    let heading_id = use_id("grid-list-section-header");

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

    // As react-aria's `useLabels`: with both a label and a header, the header labels the group.
    let aria_labelledby = heading.as_ref().map(|_| heading_id.clone());

    UseGridListSectionReturn {
        row_props: UseGridListSectionRowProps {
            role: AriaRole::Row,
        },
        row_header_props: UseGridListSectionRowHeaderProps {
            id: heading_id,
            role: AriaRole::Rowheader,
        },
        row_group_props: UseGridListSectionRowGroupProps {
            role: AriaRole::Rowgroup,
            aria_label,
            aria_labelledby,
        },
        heading,
    }
}
