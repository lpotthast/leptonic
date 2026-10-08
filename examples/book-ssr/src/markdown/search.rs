use super::MarkdownCache;
use crate::search::SearchResult;

const MAX_RESULTS: usize = 20;

/// Characters of context shown on each side of a match in the text.
const SNIPPET_CONTEXT: usize = 60;

/// What search matches a page by: its title, its sections and its plain text (without frontmatter, Markdown syntax
/// and demos), lowercased once when the page is cached instead of on every query.
#[derive(Debug, Clone)]
pub struct SearchText {
    /// The page's plain text, as shown in snippets.
    text: String,
    text_lower: String,
    title_lower: String,
    /// The sections (`##`, and `###` naming an item): lowercased title, title and anchor id.
    sections: Vec<(String, String, String)>,
}

impl SearchText {
    pub fn new(title: &str, text: String, sections: &[super::convert::Heading]) -> Self {
        Self {
            text_lower: text.to_lowercase(),
            text,
            title_lower: title.to_lowercase(),
            sections: sections
                .iter()
                .map(|section| {
                    (
                        section.text.to_lowercase(),
                        section.text.clone(),
                        section.id.clone(),
                    )
                })
                .collect(),
        }
    }
}

impl MarkdownCache {
    /// Pages containing `query` (case-insensitive), best first: title matches before section matches before matches in
    /// the text, which rank by frequency and by how early the first match is. A section match links the section.
    pub async fn search(&self, query: &str) -> Vec<SearchResult> {
        let query = query.trim().to_lowercase();
        if query.is_empty() {
            return Vec::new();
        }
        self.wait_until_warmed().await;
        let docs = self.docs.read().await;

        let mut results: Vec<(usize, &str, &super::CachedDoc, Option<usize>)> = docs
            .iter()
            .filter_map(|(md_path, doc)| {
                let search = &doc.search;
                let title_match = search.title_lower.contains(&query);
                let section = search
                    .sections
                    .iter()
                    .position(|(lower, _, _)| lower.contains(&query));
                let first_match = search.text_lower.find(&query);
                if !title_match && section.is_none() && first_match.is_none() {
                    return None;
                }

                let mut score = 0;
                if search.title_lower == query {
                    score += 1000;
                } else if title_match {
                    score += 500;
                }
                if section.is_some() {
                    score += 200;
                }
                score += search.text_lower.matches(&query).count().min(20) * 10;
                score += (1000 - first_match.unwrap_or(1000).min(1000)) / 10;
                Some((score, md_path.as_str(), doc, section))
            })
            .collect();

        results.sort_by(|(a_score, _, a, _), (b_score, _, b, _)| {
            b_score.cmp(a_score).then_with(|| a.title.cmp(&b.title))
        });
        results.truncate(MAX_RESULTS);
        results
            .into_iter()
            .map(|(_, md_path, doc, section)| {
                let path = md_path.strip_suffix(".md").unwrap_or(md_path);
                let search = &doc.search;
                // The title says it all: describe the page. Otherwise show where the query occurs.
                let snippet = if search.title_lower.contains(&query) && !doc.description.is_empty()
                {
                    doc.description.clone()
                } else {
                    search.text_lower.find(&query).map_or_else(
                        || doc.description.clone(),
                        |at| snippet(&search.text, &search.text_lower, at, query.len()),
                    )
                };
                let section = section.map(|index| &search.sections[index]);
                SearchResult {
                    path: match section {
                        Some((_, _, id)) => format!("{path}#{id}"),
                        None => path.to_owned(),
                    },
                    title: doc.title.clone(),
                    section: section.map(|(_, title, _)| title.clone()),
                    snippet,
                }
            })
            .collect()
    }
}

/// The text around a match at byte `start` (of `len` bytes) of `lower`, the lowercased `text`, widened to whole words
/// and marked with "…" where it is cut.
fn snippet(text: &str, lower: &str, start: usize, len: usize) -> String {
    // Lowercasing can change byte lengths (`İ` becomes `i̇`), so map the match back to `text` char by char.
    let start = original_offset(text, lower, start);
    let end = text.ceil_char_boundary(start + len);
    let from = word_start(text, start.saturating_sub(SNIPPET_CONTEXT));
    let to = word_end(text, end + SNIPPET_CONTEXT);
    let prefix = if from > 0 { "\u{2026}" } else { "" };
    let suffix = if to < text.len() { "\u{2026}" } else { "" };
    format!("{prefix}{}{suffix}", text[from..to].trim())
}

/// The byte offset in `text` of the char whose lowercase form starts at byte `lower_offset` of `lower`.
fn original_offset(text: &str, lower: &str, lower_offset: usize) -> usize {
    if text.len() == lower.len() {
        return text.floor_char_boundary(lower_offset);
    }
    let mut lowered = 0;
    for (offset, c) in text.char_indices() {
        if lowered >= lower_offset {
            return offset;
        }
        lowered += c.to_lowercase().map(char::len_utf8).sum::<usize>();
    }
    text.len()
}

/// Start of the word containing `index`, looking back at most 15 bytes.
fn word_start(text: &str, index: usize) -> usize {
    let index = text.floor_char_boundary(index);
    let window = text.floor_char_boundary(index.saturating_sub(15));
    text[window..index]
        .char_indices()
        .rev()
        .find(|(_, c)| c.is_whitespace())
        .map_or(index, |(offset, c)| window + offset + c.len_utf8())
}

/// End of the word containing `index`, looking ahead at most 15 bytes.
fn word_end(text: &str, index: usize) -> usize {
    let index = text.ceil_char_boundary(index);
    text[index..]
        .char_indices()
        .take_while(|(offset, _)| *offset <= 15)
        .find(|(_, c)| c.is_whitespace())
        .map_or(index, |(offset, _)| index + offset)
}

#[cfg(test)]
mod tests {
    use assertr::prelude::*;

    use super::*;

    #[test]
    fn snippet_widens_to_whole_words() {
        let text = "aaaa bbbb cccc dddd";
        assert_that!(word_start(text, 7)).is_equal_to(5);
        assert_that!(word_end(text, 7)).is_equal_to(9);
        assert_that!(word_end(text, 100)).is_equal_to(text.len());
    }

    #[test]
    fn snippet_marks_cuts_with_an_ellipsis() {
        let text = format!("{} needle {}", "word ".repeat(30), "word ".repeat(30));
        let lower = text.to_lowercase();
        let at = lower.find("needle").unwrap();
        let snippet = snippet(&text, &lower, at, "needle".len());
        assert_that!(snippet.as_str()).starts_with("\u{2026}word");
        assert_that!(snippet.as_str()).ends_with("word\u{2026}");
        assert_that!(snippet.as_str()).contains("needle");
    }

    #[test]
    fn snippet_maps_matches_back_after_lowercasing_changed_lengths() {
        let text = "İİİ Needle";
        let lower = text.to_lowercase();
        let at = lower.find("needle").unwrap();
        assert_that!(snippet(text, &lower, at, "needle".len())).is_equal_to("İİİ Needle");
    }
}
