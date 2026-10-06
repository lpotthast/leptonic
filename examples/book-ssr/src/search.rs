use leptos::prelude::*;
use serde::{Deserialize, Serialize};

/// A page matching a search query.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SearchResult {
    /// Where the result leads: the page, or the matching section of it (`/doc/button#accessibility`).
    pub path: String,
    pub title: String,
    /// The title of the matching section, if the query matches one.
    pub section: Option<String>,
    /// Plain text: the page's description, or the text around the first match.
    pub snippet: String,
}

#[server]
pub async fn search_docs(query: String) -> Result<Vec<SearchResult>, ServerFnError> {
    use crate::markdown::MarkdownCache;

    let cache = expect_context::<MarkdownCache>();
    Ok(cache.search(&query).await)
}
