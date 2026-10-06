use leptonic::components::{
    prelude::*,
    table::{TableCell, TableRow},
};
use leptos::prelude::*;

use super::DocTable;
use crate::nav::{NavEntry, nav};

/// The pages of the navigation section whose overview page is `overview`, as a table of name, layers and summary.
/// Keeps section overviews in sync with the sidebar: add pages in `nav.rs`, not here.
#[component]
pub fn SectionMembers(
    /// Path of the section's overview page, e.g. `routes::doc::InputCategory.materialize()`.
    overview: String,
) -> impl IntoView {
    let section = nav()
        .sections
        .iter()
        .find(move |section| section.overview.as_ref() == Some(&overview))
        .expect("`overview` is the overview page of a navigation section");

    view! {
        <DocTable headers=&["Name", "Layers", "Description"]>
            {section.entries.iter().map(member_row).collect_view()}
        </DocTable>
    }
}

fn member_row(entry: &'static NavEntry) -> impl IntoView {
    // A concept links its layer pages; a standalone page names its kind.
    let layers = if entry.tabs.is_empty() {
        entry.kind.label().into_any()
    } else {
        entry
            .tabs
            .iter()
            .enumerate()
            .map(|(i, tab)| {
                view! {
                    {(i > 0).then_some(", ")}
                    <Link href=tab.href.clone()>{tab.label}</Link>
                }
            })
            .collect_view()
            .into_any()
    };

    view! {
        <TableRow>
            <TableCell><Link href=entry.href.clone()>{entry.title}</Link></TableCell>
            <TableCell attr:data-label="Layers">{layers}</TableCell>
            <TableCell attr:data-label="Description">{entry.summary}</TableCell>
        </TableRow>
    }
}
