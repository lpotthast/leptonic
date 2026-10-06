use std::{collections::HashMap, fmt::Write as _};

use super::{CachedDoc, MarkdownCache};
use crate::nav::{NavGroup, nav};

impl MarkdownCache {
    /// Index of all pages for LLMs, in the order of the sidebar: one link per page (with its description) and one per
    /// `##` section. The guides first, then the concepts by group, then the building blocks by area.
    pub async fn generate_llm_index(&self) -> String {
        self.wait_until_warmed().await;
        let docs = self.docs.read().await;

        let mut md = String::from(
            "# Leptonic Documentation Index\n\n\
             Every page of the leptonic documentation as Markdown, in the order of the book\u{2019}s navigation: \
             getting started and the guides, then the concepts (UI elements such as Button or Table, each documented \
             at its layers: hooks, atoms and styled components), then the building blocks (hooks, atoms and utilities \
             that give your own elements a behavior shared by many concepts).\n",
        );
        for part in &nav().parts {
            // The guides' part has no title: its groups ("Getting started", "Guides") are top-level sections.
            let group_level = if let Some(title) = part.title {
                let _ = writeln!(md, "\n## {title}");
                "###"
            } else {
                "##"
            };
            for group in &part.groups {
                let _ = writeln!(md, "\n{group_level} {}\n", group.title);
                for path in group_pages(group) {
                    write_page(&mut md, &docs, path);
                }
            }
        }
        md
    }
}

/// The pages of `group` in the order of the sidebar: the overview, then each entry followed by its layer pages.
fn group_pages(group: &NavGroup) -> impl Iterator<Item = &str> {
    group.overview.as_deref().into_iter().chain(
        group.entries.iter().flat_map(|entry| {
            std::iter::once(entry.href.as_str()).chain(entry.tabs.iter().map(|tab| tab.href.as_str()))
        }),
    )
}

fn write_page(md: &mut String, docs: &HashMap<String, CachedDoc>, path: &str) {
    let md_path = format!("{path}.md");
    let Some(doc) = docs.get(&md_path) else {
        tracing::warn!("{path} is in the navigation, but has no Markdown export");
        return;
    };
    if doc.description.is_empty() {
        let _ = writeln!(md, "- [{}]({md_path})", doc.title);
    } else {
        let _ = writeln!(md, "- [{}]({md_path}) \u{2014} {}", doc.title, doc.description);
    }
    for section in &doc.sections {
        let _ = writeln!(md, "  - [{}]({md_path}#{})", section.text, section.id);
    }
}
