//! Markdown export of the documentation.
//!
//! Every `/doc/...` page is also served as Markdown at `/doc/....md`: the middleware renders the page through Leptos'
//! SSR, converts its `<article>` and caches the result. `/doc/llm-index.md` lists all pages in the order of the
//! navigation. The book's search runs on the plain text of the cached pages.

mod convert;
mod index;
mod search;

use std::{
    collections::HashMap,
    fmt::Write as _,
    hash::{DefaultHasher, Hash, Hasher},
    sync::Arc,
};

use axum::{
    body::Body,
    extract::{OriginalUri, Request, State},
    http::{HeaderValue, StatusCode, Uri, header},
    middleware::Next,
    response::{IntoResponse, Response},
};
use tokio::sync::{RwLock, watch};

use self::{
    convert::{ConvertedPage, Heading, Link, convert_page},
    search::SearchText,
};
use crate::nav::{PageKind, nav};

const LLM_INDEX_PATH: &str = "/doc/llm-index.md";

/// Upper bound for a rendered page.
const MAX_PAGE_SIZE: usize = 10 * 1024 * 1024;

/// A documentation page converted to Markdown, with the metadata used by the index and search.
#[derive(Debug, Clone)]
pub struct CachedDoc {
    /// Frontmatter and content.
    pub markdown: String,
    pub title: String,
    pub description: String,
    pub kind: PageKind,
    /// The page's sections (`##`, and `###` naming an item), linked from the index.
    pub sections: Vec<Heading>,
    search: SearchText,
}

/// Markdown exports of all documentation pages, keyed by their `.md` path. Filled at startup by
/// [`warm_markdown_cache`], read by the middleware, the index and search.
#[derive(Clone)]
pub struct MarkdownCache {
    docs: Arc<RwLock<HashMap<String, CachedDoc>>>,
    /// Whether [`warm_markdown_cache`] has converted every page. The index and search wait for it, so they never
    /// miss pages (e.g. right after a start).
    warmed: Arc<watch::Sender<bool>>,
}

impl Default for MarkdownCache {
    fn default() -> Self {
        Self {
            docs: Arc::default(),
            warmed: Arc::new(watch::channel(false).0),
        }
    }
}

impl MarkdownCache {
    /// Waits until every page is in the cache.
    async fn wait_until_warmed(&self) {
        // The sender lives in `self`, so the channel can't close while waiting.
        let _ = self.warmed.subscribe().wait_for(|warmed| *warmed).await;
    }

    async fn get(&self, md_path: &str) -> Option<String> {
        self.docs
            .read()
            .await
            .get(md_path)
            .map(|doc| doc.markdown.clone())
    }

    async fn insert(&self, md_path: String, doc: CachedDoc) {
        self.docs.write().await.insert(md_path, doc);
    }
}

/// Serves `/doc/....md` requests: the Markdown export of the page at `/doc/...`, and the LLM index.
///
/// Supports `If-None-Match` conditional requests.
pub async fn markdown_middleware(
    State(cache): State<MarkdownCache>,
    request: Request,
    next: Next,
) -> Response {
    let md_path = request.uri().path().to_owned();
    let Some(page_path) = md_path
        .strip_suffix(".md")
        .filter(|path| path.starts_with("/doc/"))
    else {
        return next.run(request).await;
    };
    let if_none_match = request.headers().get(header::IF_NONE_MATCH).cloned();

    if md_path == LLM_INDEX_PATH {
        let index = cache.generate_llm_index().await;
        return markdown_response(index, if_none_match.as_ref());
    }

    if let Some(markdown) = cache.get(&md_path).await {
        return markdown_response(markdown, if_none_match.as_ref());
    }

    let Ok(page_uri) = page_path.parse::<Uri>() else {
        return (StatusCode::BAD_REQUEST, "Invalid path").into_response();
    };
    // Render the page itself. Leptos reads the path from `OriginalUri`, so it has to be rewritten as well.
    let (mut parts, body) = request.into_parts();
    parts.extensions.insert(OriginalUri(page_uri.clone()));
    parts.uri = page_uri;
    let page = next.run(Request::from_parts(parts, body)).await;

    if !page.status().is_success() {
        return page;
    }
    let (parts, body) = page.into_parts();
    let Ok(html) = axum::body::to_bytes(body, MAX_PAGE_SIZE).await else {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to read the rendered page",
        )
            .into_response();
    };
    let Some(page) = convert_page(&String::from_utf8_lossy(&html)) else {
        // Not a documentation page (no `<article>`).
        return Response::from_parts(parts, Body::from(html));
    };

    let kind = nav().page_kind(page_path).unwrap_or_else(|| {
        tracing::warn!("{page_path} is missing in the navigation (nav.rs)");
        PageKind::Guide
    });
    let doc = to_cached_doc(page_path, kind, page);
    let markdown = doc.markdown.clone();
    cache.insert(md_path, doc).await;
    markdown_response(markdown, if_none_match.as_ref())
}

fn to_cached_doc(path: &str, kind: PageKind, page: ConvertedPage) -> CachedDoc {
    let ConvertedPage {
        title,
        description,
        sections,
        related,
        markdown,
        text,
    } = page;
    CachedDoc {
        markdown: format!(
            "{}{markdown}",
            frontmatter(path, &title, kind, &description, &related)
        ),
        search: SearchText::new(&title, text, &sections),
        title,
        description,
        kind,
        sections,
    }
}

impl PageKind {
    fn as_str(self) -> &'static str {
        match self {
            Self::Guide => "guide",
            Self::Overview => "overview",
            Self::Concept => "concept",
            Self::Hook => "hook",
            Self::Atom => "atom",
            Self::Utility => "utility",
        }
    }
}

fn frontmatter(
    path: &str,
    title: &str,
    kind: PageKind,
    description: &str,
    related: &[Link],
) -> String {
    let mut fm = String::from("---\n");
    let _ = writeln!(fm, "title: {}", yaml_string(title));
    let _ = writeln!(fm, "kind: {}", kind.as_str());
    let _ = writeln!(fm, "path: {}", yaml_string(path));
    if !description.is_empty() {
        let _ = writeln!(fm, "description: {}", yaml_string(description));
    }
    if !related.is_empty() {
        fm.push_str("related:\n");
        for Link { title, path } in related {
            let _ = writeln!(
                fm,
                "  - title: {}\n    path: {}",
                yaml_string(title),
                yaml_string(path)
            );
        }
    }
    fm.push_str("---\n\n");
    fm
}

/// A double-quoted YAML scalar.
fn yaml_string(value: &str) -> String {
    format!("\"{}\"", value.replace('\\', "\\\\").replace('"', "\\\""))
}

/// A `text/markdown` response, or `304 Not Modified` if the client's `If-None-Match` matches.
fn markdown_response(markdown: String, if_none_match: Option<&HeaderValue>) -> Response {
    let mut hasher = DefaultHasher::new();
    markdown.hash(&mut hasher);
    let etag = HeaderValue::from_str(&format!("\"{:x}\"", hasher.finish()))
        .expect("a hex digest is a valid header value");

    let mut response = if if_none_match == Some(&etag) {
        StatusCode::NOT_MODIFIED.into_response()
    } else {
        let mut response = markdown.into_response();
        response.headers_mut().insert(
            header::CONTENT_TYPE,
            HeaderValue::from_static("text/markdown; charset=utf-8"),
        );
        response.headers_mut().insert(
            header::CACHE_CONTROL,
            HeaderValue::from_static("public, max-age=3600, must-revalidate"),
        );
        response
    };
    response.headers_mut().insert(header::ETAG, etag);
    response
}

/// Converts every documentation page once, so that the index and search know all pages.
#[allow(clippy::missing_panics_doc)]
pub async fn warm_markdown_cache(app: axum::Router, cache: MarkdownCache, doc_paths: &[String]) {
    use tower::ServiceExt;

    for path in doc_paths {
        let request = Request::builder()
            .uri(format!("{path}.md"))
            .body(Body::empty())
            .expect("a documentation path is a valid URI");
        let _ = app.clone().oneshot(request).await;
    }
    cache.warmed.send_replace(true);
    tracing::info!("Markdown cache warmed with {} pages", doc_paths.len());
}

#[cfg(test)]
mod tests {
    use assertr::prelude::*;

    use super::*;

    #[test]
    fn frontmatter_quotes_values() {
        let fm = frontmatter(
            "/doc/button",
            "Button",
            PageKind::Concept,
            "Say \"hi\"",
            &[Link {
                title: "use_button".to_owned(),
                path: "/doc/button/hook.md".to_owned(),
            }],
        );
        assert_that!(fm).is_equal_to(
            "---\ntitle: \"Button\"\nkind: concept\npath: \"/doc/button\"\ndescription: \"Say \\\"hi\\\"\"\n\
             related:\n  - title: \"use_button\"\n    path: \"/doc/button/hook.md\"\n---\n\n"
                .to_owned(),
        );
    }
}
