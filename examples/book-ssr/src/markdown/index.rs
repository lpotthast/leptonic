use std::fmt::Write as _;

use super::{CachedDoc, MarkdownCache};
use crate::nav::PageKind;

/// Order and headings of the page groups in the LLM index.
const GROUPS: [(PageKind, &str); 6] = [
    (PageKind::Guide, "Getting Started"),
    (PageKind::Concept, "Concept Overviews"),
    (PageKind::Domain, "Behavioral Domains"),
    (PageKind::Hook, "Hook Deep-Dives"),
    (PageKind::Atom, "Atom Deep-Dives"),
    (PageKind::Component, "Component Deep-Dives"),
];

impl MarkdownCache {
    /// Index of all pages for LLMs: one link per page (with its description) and one per `##` section, grouped by
    /// page kind.
    pub async fn generate_llm_index(&self) -> String {
        self.wait_until_warmed().await;
        let docs = self.docs.read().await;
        let mut pages: Vec<(&String, &CachedDoc)> = docs.iter().collect();
        pages.sort_by_key(|(path, _)| *path);

        let mut md = String::from(
            "# Leptonic Documentation Index\n\n\
             This index lists all documentation pages in markdown format.\n\
             Pages are organized by architectural layer.\n\n\
             | Layer | Count |\n|-------|-------|\n",
        );
        for (kind, heading) in GROUPS {
            let count = pages.iter().filter(|(_, doc)| doc.kind == kind).count();
            if count > 0 {
                let _ = writeln!(md, "| {heading} | {count} |");
            }
        }

        for (kind, heading) in GROUPS {
            let mut group = pages.iter().filter(|(_, doc)| doc.kind == kind).peekable();
            if group.peek().is_none() {
                continue;
            }
            let _ = writeln!(md, "\n## {heading}\n");
            for (path, doc) in group {
                if doc.description.is_empty() {
                    let _ = writeln!(md, "- [{}]({path})", doc.title);
                } else {
                    let _ = writeln!(md, "- [{}]({path}) — {}", doc.title, doc.description);
                }
                for section in &doc.sections {
                    let _ = writeln!(md, "  - [{}]({path}#{})", section.text, section.id);
                }
            }
        }
        md
    }
}
