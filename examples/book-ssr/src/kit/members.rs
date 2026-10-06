use leptonic::components::{
    prelude::*,
    table::{TableCell, TableRow},
};
use leptos::prelude::*;

use super::DocTable;
use crate::nav::{NavEntry, nav};

/// The pages of the navigation group whose overview page is `overview`, as a table of name, layers and summary.
/// Keeps group overviews in sync with the sidebar: add pages in `nav.rs`, not here.
///
/// A unit test checks that every `overview` is the overview of a group with pages.
#[component]
#[allow(clippy::needless_pass_by_value)] // Leptos components own their props.
pub fn SectionMembers(
    /// Path of the group's overview page, e.g. `routes::doc::Fields.materialize()`.
    overview: String,
) -> impl IntoView {
    let Some(group) = nav()
        .groups()
        .find(|group| group.overview.as_ref() == Some(&overview))
    else {
        tracing::error!("{overview} is not the overview page of a navigation group");
        return ().into_any();
    };

    view! {
        <DocTable headers=&["Name", "Layers", "Description"]>
            {group.entries.iter().map(member_row).collect_view()}
        </DocTable>
    }
    .into_any()
}

fn member_row(entry: &'static NavEntry) -> impl IntoView {
    // A concept links its layer pages; a single page names its kind.
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
                    <Link href=tab.href.clone()>{tab.label()}</Link>
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

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, fs, path::Path};

    use assertr::prelude::*;
    use syn::{Item, ItemMod, LitStr};

    use crate::nav::nav;

    /// The paths of the routes directly below `/doc` (`routes::doc::<Name>`), by route name: `mod date_time` with
    /// `#[route("/date-time")]` is `DateTime`, at `/doc/date-time`.
    fn doc_routes() -> HashMap<String, String> {
        let source = fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join("src/routes.rs"))
            .expect("routes.rs is readable");
        let file = syn::parse_file(&source).expect("routes.rs parses");
        let module = |items: &[Item], name: &str| -> Option<ItemMod> {
            items.iter().find_map(|item| match item {
                Item::Mod(module) if module.ident == name => Some(module.clone()),
                _ => None,
            })
        };
        let routes = module(&file.items, "routes").expect("routes.rs has `mod routes`");
        let doc = module(&routes.content.expect("`mod routes` has content").1, "doc")
            .expect("`mod routes` has `mod doc`");

        let mut paths = HashMap::new();
        for item in doc.content.expect("`mod doc` has content").1 {
            let Item::Mod(module) = item else { continue };
            let Some(path) = module
                .attrs
                .iter()
                .find(|attr| attr.path().is_ident("route"))
                .and_then(|attr| attr.parse_args::<LitStr>().ok())
            else {
                continue;
            };
            let name: String = module
                .ident
                .to_string()
                .split('_')
                .map(|word| {
                    let mut chars = word.chars();
                    chars
                        .next()
                        .map(|first| first.to_uppercase().chain(chars).collect::<String>())
                        .unwrap_or_default()
                })
                .collect();
            paths.insert(name, format!("/doc{}", path.value()));
        }
        paths
    }

    /// Every `<SectionMembers overview=routes::doc::<Name>.materialize()/>` of the pages names the overview of a
    /// navigation group with pages.
    #[test]
    fn every_section_members_overview_is_a_group_overview() {
        let routes = doc_routes();
        let pages = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/pages");
        let mut found = 0;
        let mut problems = Vec::new();
        let mut dirs = vec![pages];
        while let Some(dir) = dirs.pop() {
            for entry in fs::read_dir(&dir).expect("pages are readable").flatten() {
                let path = entry.path();
                if path.is_dir() {
                    dirs.push(path);
                    continue;
                }
                let source = fs::read_to_string(&path).expect("page is readable");
                for (start, _) in source.match_indices("<SectionMembers") {
                    found += 1;
                    let tag = &source[start..];
                    let tag = &tag[..tag.find("/>").unwrap_or(tag.len())];
                    let Some(name) = tag
                        .split("overview=routes::doc::")
                        .nth(1)
                        .and_then(|rest| rest.strip_suffix(".materialize()"))
                    else {
                        problems.push(format!(
                            "{}: `overview` is not `routes::doc::<Name>.materialize()`: {tag}",
                            path.display()
                        ));
                        continue;
                    };
                    let is_overview = routes.get(name).is_some_and(|route| {
                        nav().groups().any(|group| {
                            group.overview.as_deref() == Some(route) && !group.entries.is_empty()
                        })
                    });
                    if !is_overview {
                        problems.push(format!(
                            "{}: `routes::doc::{name}` is not the overview of a navigation group with pages",
                            path.display()
                        ));
                    }
                }
            }
        }
        assert_that!(found).is_greater_than(0);
        assert_that!(problems).is_empty();
    }
}
