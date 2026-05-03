use leptos::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SearchResult {
    pub path: String,
    pub title: String,
    pub snippet: String,
}

#[server]
pub async fn search_docs(query: String) -> Result<Vec<SearchResult>, ServerFnError> {
    use crate::markdown::MarkdownCache;

    let cache = expect_context::<MarkdownCache>();
    Ok(cache.search(&query).await)
}
