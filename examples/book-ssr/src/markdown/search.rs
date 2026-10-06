use super::MarkdownCache;
use crate::search::SearchResult;

const MAX_RESULTS: usize = 20;

/// Characters of context shown on each side of a match in the body.
const SNIPPET_CONTEXT: usize = 60;

impl MarkdownCache {
    /// Pages containing `query` (case-insensitive), best first: title matches before section matches before matches in
    /// the body, which rank by frequency and by how early the first match is.
    pub async fn search(&self, query: &str) -> Vec<SearchResult> {
        let query = query.to_lowercase();
        self.wait_until_warmed().await;
        let docs = self.docs.read().await;

        let mut results: Vec<(usize, SearchResult)> = docs
            .iter()
            .filter_map(|(md_path, doc)| {
                let body = doc.markdown.to_lowercase();
                let first_match = body.find(&query)?;
                let title = doc.title.to_lowercase();
                let section = doc
                    .sections
                    .iter()
                    .find(|section| section.text.to_lowercase().contains(&query));

                let mut score = 0;
                if title == query {
                    score += 1000;
                } else if title.contains(&query) {
                    score += 500;
                }
                if section.is_some() {
                    score += 200;
                }
                score += body.matches(&query).count().min(20) * 10;
                score += (1000 - first_match.min(1000)) / 10;

                let snippet = if title.contains(&query) {
                    format!("**{}**", doc.title)
                } else if let Some(section) = section {
                    format!("## {}", section.text)
                } else {
                    body_snippet(&doc.markdown, first_match, query.len())
                };

                Some((
                    score,
                    SearchResult {
                        path: md_path.strip_suffix(".md").unwrap_or(md_path).to_owned(),
                        title: doc.title.clone(),
                        snippet,
                    },
                ))
            })
            .collect();

        results.sort_by(|(a_score, a), (b_score, b)| {
            b_score.cmp(a_score).then_with(|| a.title.cmp(&b.title))
        });
        results
            .into_iter()
            .take(MAX_RESULTS)
            .map(|(_, result)| result)
            .collect()
    }
}

/// The text around a match at byte `start` (of `len` bytes), widened to whole words.
///
/// Lowercasing can change byte lengths, so the positions found in the lowercased text are only approximate here; they
/// are clamped to char boundaries.
fn body_snippet(markdown: &str, start: usize, len: usize) -> String {
    let from = word_start(markdown, start.saturating_sub(SNIPPET_CONTEXT));
    let to = word_end(markdown, start + len + SNIPPET_CONTEXT);
    format!("...{}...", markdown[from..to].replace('\n', " "))
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
}
