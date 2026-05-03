use crate::search::SearchResult;
use axum::{
    body::Body,
    extract::{Request, State},
    http::{StatusCode, Uri, header},
    middleware::Next,
    response::{IntoResponse, Response},
};
use htmd::{
    Element, HtmlToMarkdown,
    element_handler::{HandlerResult, Handlers},
};
use std::{
    collections::HashMap,
    fmt::Write as _,
    hash::{DefaultHasher, Hash, Hasher},
    sync::Arc,
};
use tokio::sync::RwLock;

/// Cached documentation page with derived metadata.
#[derive(Clone, Debug)]
pub struct CachedDoc {
    /// The converted markdown content (including frontmatter).
    pub markdown: String,
    /// Page title, extracted from the first `# ` heading.
    pub title: String,
    /// Architectural layer, inferred from URL structure.
    pub layer: DocLayer,
    /// Sub-items documented on this page (heading text, anchor ID).
    /// Extracted from `## ` headings. Enables LLM discoverability
    /// of individual hooks/atoms/components on multi-item pages.
    pub headings: Vec<(String, String)>,
    /// Short description extracted from the first prose paragraph.
    pub description: String,
    /// Related pages extracted from "See Also" sections (title, path).
    pub related: Vec<(String, String)>,
}

/// Architectural layer of a documentation page, inferred from its URL structure.
/// Used in frontmatter metadata and for grouping in the LLM index.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DocLayer {
    /// Getting-started pages and other non-layer-specific content
    /// (e.g. `/doc/overview`, `/doc/installation`).
    Guide,
    /// Concept overview pages that group related hook/atom/component deep-dives
    /// (e.g. `/doc/button`, `/doc/slider`, `/doc/checkbox`).
    Concept,
    /// Hook deep-dive pages documenting low-level interaction logic
    /// (e.g. `/doc/button/hook`, `/doc/hooks/use-label`).
    Hook,
    /// Atom deep-dive pages documenting headless single-element components
    /// (e.g. `/doc/button/atom`, `/doc/slider/atom`).
    Atom,
    /// Component deep-dive pages documenting pre-built, styled components
    /// (e.g. `/doc/button/component`, `/doc/components/toast`).
    Component,
    /// Behavioral domain overview pages grouping related hooks
    /// (e.g. `/doc/interactions`, `/doc/focus`, `/doc/input`).
    Domain,
}

impl DocLayer {
    fn as_str(self) -> &'static str {
        match self {
            Self::Guide => "guide",
            Self::Concept => "concept",
            Self::Hook => "hook",
            Self::Atom => "atom",
            Self::Component => "component",
            Self::Domain => "domain",
        }
    }

    fn heading(self) -> &'static str {
        match self {
            Self::Guide => "Getting Started",
            Self::Concept => "Concept Overviews",
            Self::Hook => "Hook Deep-Dives",
            Self::Atom => "Atom Deep-Dives",
            Self::Component => "Component Deep-Dives",
            Self::Domain => "Behavioral Domains",
        }
    }
}

/// Order in which layers appear in the grouped LLM index.
const LAYER_ORDER: [DocLayer; 6] = [
    DocLayer::Guide,
    DocLayer::Concept,
    DocLayer::Domain,
    DocLayer::Hook,
    DocLayer::Atom,
    DocLayer::Component,
];

/// Thread-safe cache of Markdown-rendered documentation pages.
/// Populated during startup warming. Read by middleware, search, and LLM index.
#[derive(Clone, Default)]
pub struct MarkdownCache {
    inner: Arc<RwLock<HashMap<String, CachedDoc>>>,
}

impl MarkdownCache {
    /// Retrieve a cached page's markdown by its `.md` path key.
    pub async fn get_markdown(&self, path: &str) -> Option<String> {
        self.inner
            .read()
            .await
            .get(path)
            .map(|d| d.markdown.clone())
    }

    /// Insert a converted page into the cache.
    pub async fn insert(&self, path: String, doc: CachedDoc) {
        self.inner.write().await.insert(path, doc);
    }

    /// Search cached pages for a query string. Returns matching results
    /// ranked by relevance (title match > heading match > frequency > position).
    pub async fn search(&self, query: &str) -> Vec<SearchResult> {
        let cache = self.inner.read().await;
        let query_lower = query.to_lowercase();

        let mut scored: Vec<(i64, SearchResult)> = cache
            .iter()
            .filter_map(|(path, doc)| {
                let md_lower = doc.markdown.to_lowercase();
                let first_idx = md_lower.find(&query_lower)?;

                let mut score: i64 = 0;
                let title_lower = doc.title.to_lowercase();

                // Exact title match
                if title_lower == query_lower {
                    score += 1000;
                } else if title_lower.contains(&query_lower) {
                    score += 500;
                }

                // Heading match
                if doc
                    .headings
                    .iter()
                    .any(|(h, _)| h.to_lowercase().contains(&query_lower))
                {
                    score += 200;
                }

                // Occurrence frequency (capped at 20)
                let occurrence_count = md_lower.matches(&query_lower).count();
                score += (occurrence_count.min(20) * 10) as i64;

                // Early position bonus
                score += ((1000 - first_idx.min(1000)) / 10) as i64;

                // Build snippet with context
                let snippet = if title_lower.contains(&query_lower) {
                    format!("**{}**", doc.title)
                } else if let Some((heading, _)) = doc
                    .headings
                    .iter()
                    .find(|(h, _)| h.to_lowercase().contains(&query_lower))
                {
                    format!("## {heading}")
                } else {
                    let start = snap_to_word_start(&doc.markdown, first_idx.saturating_sub(60));
                    let end = snap_to_word_end(&doc.markdown, first_idx + query.len() + 60);
                    format!("...{}...", &doc.markdown[start..end].replace('\n', " "))
                };

                Some((
                    score,
                    SearchResult {
                        path: path.strip_suffix(".md").unwrap_or(path).to_owned(),
                        title: doc.title.clone(),
                        snippet,
                    },
                ))
            })
            .collect();

        scored.sort_by(|a, b| b.0.cmp(&a.0).then_with(|| a.1.title.cmp(&b.1.title)));
        scored.truncate(20);
        scored.into_iter().map(|(_, r)| r).collect()
    }

    /// Generate the LLM index markdown from all cached pages, grouped by layer.
    pub async fn generate_llm_index(&self) -> String {
        let cache = self.inner.read().await;

        // Bucket entries by layer.
        let mut groups: HashMap<DocLayer, Vec<(&String, &CachedDoc)>> = HashMap::new();
        for (path, doc) in cache.iter() {
            groups.entry(doc.layer).or_default().push((path, doc));
        }
        // Sort each group alphabetically by path.
        for entries in groups.values_mut() {
            entries.sort_by_key(|(path, _)| (*path).clone());
        }

        let mut md = String::from("# Leptonic Documentation Index\n\n");
        md.push_str("This index lists all documentation pages in markdown format.\n");
        md.push_str("Pages are organized by architectural layer.\n\n");

        // Counts table.
        md.push_str("| Layer | Count |\n|-------|-------|\n");
        for layer in &LAYER_ORDER {
            if let Some(entries) = groups.get(layer) {
                let _ = writeln!(md, "| {} | {} |", layer.heading(), entries.len());
            }
        }
        md.push('\n');

        // Grouped sections.
        for layer in &LAYER_ORDER {
            let Some(entries) = groups.get(layer) else {
                continue;
            };

            let _ = writeln!(md, "## {}\n", layer.heading());

            for (path, doc) in entries {
                if doc.description.is_empty() {
                    let _ = writeln!(md, "- [{}]({path})", doc.title);
                } else {
                    let _ = writeln!(md, "- [{}]({path}) — {}", doc.title, doc.description);
                }
                for (heading, anchor) in &doc.headings {
                    let base = path.strip_suffix(".md").unwrap_or(path);
                    if anchor.is_empty() {
                        let _ = writeln!(md, "  - [{heading}]({base}.md)");
                    } else {
                        let _ = writeln!(md, "  - [{heading}]({base}.md#{anchor})");
                    }
                }
            }
            md.push('\n');
        }

        md
    }
}

/// Adjust a byte index so it falls on a char boundary (searching backward).
fn floor_char_boundary(s: &str, idx: usize) -> usize {
    let mut i = idx.min(s.len());
    while i > 0 && !s.is_char_boundary(i) {
        i -= 1;
    }
    i
}

/// Adjust a byte index so it falls on a char boundary (searching forward).
fn ceil_char_boundary(s: &str, idx: usize) -> usize {
    let mut i = idx.min(s.len());
    while i < s.len() && !s.is_char_boundary(i) {
        i += 1;
    }
    i
}

/// Walk backward from `idx` (up to 15 chars) to find a whitespace boundary.
/// Falls back to the nearest char boundary if no whitespace is found.
fn snap_to_word_start(s: &str, idx: usize) -> usize {
    let start = floor_char_boundary(s, idx);
    // Walk backward up to 15 bytes looking for whitespace.
    let limit = start.saturating_sub(15);
    let mut i = start;
    while i > limit {
        let prev = floor_char_boundary(s, i.saturating_sub(1));
        if prev == i {
            break;
        }
        if s[prev..i].starts_with(char::is_whitespace) {
            return i;
        }
        i = prev;
    }
    start
}

/// Walk forward from `idx` (up to 15 chars) to find a whitespace boundary.
/// Falls back to the nearest char boundary if no whitespace is found.
fn snap_to_word_end(s: &str, idx: usize) -> usize {
    let end = ceil_char_boundary(s, idx);
    // Walk forward up to 15 bytes looking for whitespace.
    let limit = (end + 15).min(s.len());
    let mut i = end;
    while i < limit {
        let next = ceil_char_boundary(s, i + 1);
        if s[i..next].starts_with(char::is_whitespace) {
            return i;
        }
        i = next;
    }
    end
}

fn compute_etag(content: &str) -> String {
    let mut hasher = DefaultHasher::new();
    content.hash(&mut hasher);
    format!("\"{}\"", hasher.finish())
}

fn infer_layer(path: &str) -> DocLayer {
    if path.ends_with("/hook") {
        return DocLayer::Hook;
    }
    if path.ends_with("/atom") {
        return DocLayer::Atom;
    }
    if path.ends_with("/component") {
        return DocLayer::Component;
    }
    if path.starts_with("/doc/hooks/") {
        return DocLayer::Hook;
    }
    if path.starts_with("/doc/components/") {
        return DocLayer::Component;
    }
    let domains = [
        "/doc/interactions",
        "/doc/focus",
        "/doc/overlays",
        "/doc/selection",
        "/doc/input",
        "/doc/data-display",
        "/doc/layout",
        "/doc/feedback",
        "/doc/navigation",
        "/doc/general",
    ];
    if domains.contains(&path) {
        return DocLayer::Domain;
    }
    const CONCEPTS: &[&str] = &[
        "/doc/button",
        "/doc/checkbox",
        "/doc/radio",
        "/doc/toggle",
        "/doc/slider",
        "/doc/select",
        "/doc/text-field",
        "/doc/popover",
        "/doc/modal",
        "/doc/grid",
        "/doc/table",
        "/doc/tabs",
        "/doc/separator",
        "/doc/collapsible",
        "/doc/progress",
        "/doc/chip",
        "/doc/combobox",
        "/doc/listbox",
        "/doc/menu",
        "/doc/tooltip",
        "/doc/link",
    ];
    if CONCEPTS.contains(&path) {
        return DocLayer::Concept;
    }
    DocLayer::Guide
}

/// Replace `\_` with `_` in metadata strings (titles, headings, descriptions).
///
/// htmd escapes underscores to prevent markdown emphasis interpretation.
/// This is correct in body markdown but wrong in YAML frontmatter and index titles
/// where markdown rendering doesn't apply.
fn unescape_underscores(s: &str) -> String {
    s.replace("\\_", "_")
}

/// Extracted metadata from a converted markdown page.
struct ExtractedMetadata {
    title: String,
    headings: Vec<(String, String)>,
    description: String,
}

fn extract_metadata(markdown: &str) -> ExtractedMetadata {
    let title = markdown
        .lines()
        .find(|l| l.starts_with("# "))
        .map_or_else(String::new, |l| unescape_underscores(l[2..].trim()));

    let headings: Vec<(String, String)> = markdown
        .lines()
        .filter(|l| l.starts_with("## "))
        .map(|l| {
            let text = unescape_underscores(l[3..].trim());
            let anchor = text
                .to_lowercase()
                .replace(
                    |c: char| !c.is_alphanumeric() && c != ' ' && c != '-' && c != '_',
                    "",
                )
                .replace([' ', '_'], "-");
            (text, anchor)
        })
        .collect();

    let description = extract_description(markdown);

    ExtractedMetadata {
        title,
        headings,
        description,
    }
}

/// Extract a short description from the first prose paragraph of a markdown document.
///
/// Skips headings, tables, lists, code fences, and blockquotes. Takes the first
/// non-empty group of consecutive prose lines and truncates at 200 chars on a word boundary.
fn extract_description(markdown: &str) -> String {
    let mut paragraph = String::new();
    let mut in_paragraph = false;

    for line in markdown.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            if in_paragraph {
                break; // End of first prose paragraph.
            }
            continue;
        }
        if trimmed.starts_with('#')
            || trimmed.starts_with('|')
            || trimmed.starts_with('-')
            || trimmed.starts_with('*')
            || trimmed.starts_with('`')
            || trimmed.starts_with('>')
        {
            if in_paragraph {
                break;
            }
            continue;
        }
        if !paragraph.is_empty() {
            paragraph.push(' ');
        }
        paragraph.push_str(trimmed);
        in_paragraph = true;
    }

    let paragraph = unescape_underscores(&paragraph);

    if paragraph.len() <= 200 {
        return paragraph;
    }

    // Truncate at word boundary.
    let truncated = &paragraph[..200];
    if let Some(last_space) = truncated.rfind(' ') {
        format!("{}...", &truncated[..last_space])
    } else {
        format!("{truncated}...")
    }
}

/// Extract related pages from a "See Also" section.
///
/// Finds a `## See Also` heading (case-insensitive) and extracts `[title](path)` links
/// from lines until the next `## ` heading or end of document.
fn extract_related(markdown: &str) -> Vec<(String, String)> {
    let mut related = Vec::new();
    let mut in_see_also = false;

    for line in markdown.lines() {
        if line.starts_with("## ") {
            if in_see_also {
                break; // Next section, stop.
            }
            if line[3..].trim().eq_ignore_ascii_case("see also") {
                in_see_also = true;
            }
            continue;
        }
        if !in_see_also {
            continue;
        }
        // Extract [title](path) patterns from this line.
        let mut rest = line;
        while let Some(bracket_start) = rest.find('[') {
            let after_bracket = &rest[bracket_start + 1..];
            let Some(bracket_end) = after_bracket.find("](") else {
                rest = &rest[bracket_start + 1..];
                continue;
            };
            let title = &after_bracket[..bracket_end];
            let after_paren = &after_bracket[bracket_end + 2..];
            let Some(paren_end) = after_paren.find(')') else {
                rest = &after_bracket[bracket_end + 2..];
                continue;
            };
            let path = &after_paren[..paren_end];
            if !title.is_empty() && !path.is_empty() {
                related.push((title.to_owned(), path.to_owned()));
            }
            rest = &after_paren[paren_end + 1..];
        }
    }

    related
}

/// Rewrite internal documentation links to end with `.md` for standalone consumption.
///
/// Transforms `](/doc/button/hook)` → `](/doc/button/hook.md)` and
/// `](/doc/button#props)` → `](/doc/button.md#props)`.
/// Skips links already ending with `.md` and avoids rewriting inside fenced code blocks.
fn rewrite_internal_links(markdown: &str) -> String {
    let mut result = String::with_capacity(markdown.len());
    let mut in_code_fence = false;

    for line in markdown.lines() {
        if line.trim_start().starts_with("```") {
            in_code_fence = !in_code_fence;
        }

        if in_code_fence {
            result.push_str(line);
            result.push('\n');
            continue;
        }

        let mut rest = line.as_bytes();
        let mut line_result = String::with_capacity(line.len());
        let pattern = b"](/doc/";

        while let Some(pos) = find_bytes(rest, pattern) {
            // Copy everything up to and including the `](`
            line_result.push_str(&line[line.len() - rest.len()..line.len() - rest.len() + pos + 2]);
            // Now find the closing `)` for this link
            let link_start = pos + 2; // after `](`
            let link_rest = &rest[link_start..];
            if let Some(paren_end) = link_rest.iter().position(|&b| b == b')') {
                let link = std::str::from_utf8(&link_rest[..paren_end]).unwrap_or("");
                if link.ends_with(".md") || link.contains(".md#") {
                    // Already has .md, keep as-is.
                    line_result.push_str(link);
                } else if let Some(hash_pos) = link.find('#') {
                    // Insert .md before the fragment.
                    line_result.push_str(&link[..hash_pos]);
                    line_result.push_str(".md");
                    line_result.push_str(&link[hash_pos..]);
                } else {
                    // Append .md.
                    line_result.push_str(link);
                    line_result.push_str(".md");
                }
                line_result.push(')');
                rest = &rest[link_start + paren_end + 1..];
            } else {
                // No closing paren, keep as-is.
                line_result.push_str(std::str::from_utf8(&rest[pos + 2..]).unwrap_or(""));
                rest = &[];
            }
        }
        // Append remainder.
        line_result.push_str(std::str::from_utf8(rest).unwrap_or(""));
        result.push_str(&line_result);
        result.push('\n');
    }

    // Remove the trailing newline we added for the last line if the original didn't have one.
    if !markdown.ends_with('\n') && result.ends_with('\n') {
        result.pop();
    }

    result
}

/// Find the byte position of a pattern within a byte slice.
fn find_bytes(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack
        .windows(needle.len())
        .position(|window| window == needle)
}

/// Data needed to generate YAML frontmatter for a documentation page.
struct FrontmatterData<'a> {
    path: &'a str,
    title: &'a str,
    layer: DocLayer,
    description: &'a str,
    related: &'a [(String, String)],
}

fn generate_frontmatter(data: &FrontmatterData<'_>) -> String {
    let mut fm = format!(
        "---\ntitle: \"{}\"\nlayer: {}\npath: \"{}\"\n",
        data.title,
        data.layer.as_str(),
        data.path,
    );
    if !data.description.is_empty() {
        // Escape quotes in description for valid YAML.
        let desc = data.description.replace('"', "\\\"");
        let _ = writeln!(fm, "description: \"{desc}\"");
    }
    if !data.related.is_empty() {
        fm.push_str("related:\n");
        for (title, path) in data.related {
            let _ = writeln!(fm, "  - title: \"{title}\"\n    path: \"{path}\"");
        }
    }
    fm.push_str("---\n\n");
    fm
}

/// Axum middleware that intercepts requests for `.md` variants of doc routes.
///
/// When a request targets e.g. `/doc/hooks/use-press.md`, this middleware:
/// 1. Checks the cache for a previously converted result
/// 2. If not cached: rewrites the path to `/doc/hooks/use-press`, lets Leptos SSR render the HTML,
///    extracts the `<article>` content, cleans it, converts to markdown via `htmd`, and caches it
/// 3. Returns the markdown with `Content-Type: text/markdown; charset=utf-8`
///
/// Supports `ETag` / `If-None-Match` conditional requests and sets `Cache-Control` headers.
pub async fn markdown_middleware(
    State(cache): State<MarkdownCache>,
    request: Request,
    next: Next,
) -> Response {
    let path = request.uri().path().to_owned();

    // Only intercept /doc/.../.md requests
    if !path.starts_with("/doc/") || !path.as_bytes().ends_with(b".md") {
        return next.run(request).await;
    }

    // Extract If-None-Match before consuming the request.
    let if_none_match = request
        .headers()
        .get(header::IF_NONE_MATCH)
        .and_then(|v| v.to_str().ok())
        .map(String::from);

    // Handle the LLM index directly — no SSR needed.
    if path == "/doc/llm-index.md" {
        let index = cache.generate_llm_index().await;
        return with_etag(index, if_none_match.as_deref());
    }

    // Check cache first.
    if let Some(markdown) = cache.get_markdown(&path).await {
        return with_etag(markdown, if_none_match.as_deref());
    }

    // Strip .md suffix to get the real route path.
    let real_path = &path[..path.len() - 3];

    // Rewrite the URI — the path was already validated to end with ".md"
    let Ok(new_uri) = real_path.parse::<Uri>() else {
        return (StatusCode::BAD_REQUEST, "Invalid path").into_response();
    };

    let (mut parts, body) = request.into_parts();
    parts.uri = new_uri;
    let rewritten = Request::from_parts(parts, body);

    // Let Leptos SSR handle the request.
    let response = next.run(rewritten).await;

    // Collect the full response body (streaming SSR).
    let (parts, body) = response.into_parts();
    let Ok(bytes) = axum::body::to_bytes(body, 10 * 1024 * 1024).await else {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to read SSR response body",
        )
            .into_response();
    };

    let html = String::from_utf8_lossy(&bytes);

    // Extract <article> content and convert to markdown.
    let Some((article_html, explicit_docs)) = extract_article(&html) else {
        // If no <article> found, return the original response.
        return Response::from_parts(parts, Body::from(bytes));
    };

    let cleaned = clean_html(&article_html);
    let markdown = convert_to_markdown(&cleaned);
    let markdown = rewrite_internal_links(&markdown);

    let layer = infer_layer(real_path);
    let ExtractedMetadata {
        title,
        mut headings,
        description,
    } = extract_metadata(&markdown);
    let related = extract_related(&markdown);

    // Merge explicit data-documents items (no anchor, link to page root).
    for doc_item in explicit_docs {
        if !headings.iter().any(|(h, _)| h == &doc_item) {
            headings.push((doc_item, String::new()));
        }
    }

    let frontmatter = generate_frontmatter(&FrontmatterData {
        path: real_path,
        title: &title,
        layer,
        description: &description,
        related: &related,
    });
    let full_markdown = format!("{frontmatter}{markdown}");

    let doc = CachedDoc {
        markdown: full_markdown.clone(),
        title,
        layer,
        headings,
        description,
        related,
    };
    cache.insert(path, doc).await;

    with_etag(full_markdown, if_none_match.as_deref())
}

/// Compute `ETag`, check against `If-None-Match`, and return 304 or full markdown response.
fn with_etag(markdown: String, if_none_match: Option<&str>) -> Response {
    let etag = compute_etag(&markdown);
    if if_none_match == Some(etag.as_str()) {
        return not_modified_response(&etag);
    }
    markdown_response(markdown, &etag)
}

/// Build a `text/markdown` response with `ETag` and `Cache-Control` headers.
fn markdown_response(markdown: String, etag: &str) -> Response {
    let mut resp = (StatusCode::OK, markdown).into_response();
    let headers = resp.headers_mut();
    headers.insert(
        header::CONTENT_TYPE,
        "text/markdown; charset=utf-8".parse().unwrap(),
    );
    headers.insert(header::ETAG, etag.parse().unwrap());
    headers.insert(
        header::CACHE_CONTROL,
        "public, max-age=3600, must-revalidate".parse().unwrap(),
    );
    resp
}

/// Build a `304 Not Modified` response with the matching `ETag`.
fn not_modified_response(etag: &str) -> Response {
    let mut resp = StatusCode::NOT_MODIFIED.into_response();
    resp.headers_mut()
        .insert(header::ETAG, etag.parse().unwrap());
    resp
}

/// Extract the inner HTML and `data-documents` attribute from the first `<article>` element.
fn extract_article(html: &str) -> Option<(String, Vec<String>)> {
    let document = scraper::Html::parse_document(html);
    let Ok(selector) = scraper::Selector::parse("article") else {
        return None;
    };
    let article = document.select(&selector).next()?;

    let explicit_docs: Vec<String> = article
        .attr("data-documents")
        .map(|v| {
            v.split(',')
                .map(|s| s.trim().to_owned())
                .filter(|s| !s.is_empty())
                .collect()
        })
        .unwrap_or_default();

    Some((article.inner_html(), explicit_docs))
}

/// Clean HTML before markdown conversion:
/// - Strip Leptos `<!>` fragment markers
/// - Strip Leptos hydration comments
/// - Strip `style` attributes
/// - Strip all `data-*` attributes except `data-inline` and `data-demo-description`
fn clean_html(html: &str) -> String {
    let mut result = html.to_owned();

    // Strip Leptos fragment markers (SSR emits `<!>`, not `<!--...-->`)
    result = result.replace("<!>", "");

    // Strip Leptos hydration comments: <!--hk=...--> and <!--/--> etc.
    result = strip_html_comments(&result);

    result = strip_attribute(&result, "style");
    result = strip_data_attributes(&result);

    result
}

/// Strip all HTML comments from the string.
fn strip_html_comments(html: &str) -> String {
    let mut result = String::with_capacity(html.len());
    let mut rest = html;

    while let Some(start) = rest.find("<!--") {
        result.push_str(&rest[..start]);
        if let Some(end) = rest[start..].find("-->") {
            rest = &rest[start + end + 3..];
        } else {
            // Unclosed comment, keep the rest
            rest = &rest[start..];
            break;
        }
    }
    result.push_str(rest);
    result
}

/// Check whether `position` in `html` falls inside an HTML tag (between `<` and `>`).
///
/// # Constraints
/// Assumes Leptos SSR output: attribute values are always double-quoted,
/// no unescaped quotes inside values, no multi-line values.
#[allow(clippy::similar_names)]
fn is_inside_tag(html: &str, position: usize) -> bool {
    let before = &html[..position];
    let last_lt = before.rfind('<');
    let last_gt = before.rfind('>');
    match (last_lt, last_gt) {
        (Some(lt), Some(gt)) => lt > gt,
        (Some(_), None) => true,
        _ => false,
    }
}

/// Strip all occurrences of a given attribute (e.g. `style="..."`) from HTML tags.
///
/// # Constraints
/// Assumes Leptos SSR output: attribute values are always double-quoted,
/// no unescaped quotes inside values, no multi-line values.
fn strip_attribute(html: &str, attr_name: &str) -> String {
    let mut result = String::with_capacity(html.len());
    let mut rest = html;
    let mut offset = 0;

    let pattern = format!(" {attr_name}=\"");

    while let Some(start) = rest.find(&pattern) {
        let abs_pos = offset + start;
        if !is_inside_tag(html, abs_pos) {
            // Match is in text content, not in a tag — skip past it
            result.push_str(&rest[..start + pattern.len()]);
            rest = &rest[start + pattern.len()..];
            offset = abs_pos + pattern.len();
            continue;
        }
        result.push_str(&rest[..start]);
        let after_eq = start + pattern.len();
        if let Some(end) = rest[after_eq..].find('"') {
            rest = &rest[after_eq + end + 1..];
            offset = abs_pos + pattern.len() + end + 1;
        } else {
            rest = &rest[start..];
            break;
        }
    }
    result.push_str(rest);
    result
}

/// Returns `Some(attr_name_len)` if the data attribute starting at `after_space`
/// should be preserved during stripping.
fn preserved_data_attr_len(after_space: &str) -> Option<usize> {
    const KEEP: &[&str] = &["data-inline", "data-demo-description", "data-language"];
    KEEP.iter()
        .find(|attr| after_space.starts_with(**attr))
        .map(|attr| attr.len())
}

/// Strip all `data-*` attributes except those in the preserve list.
///
/// # Constraints
/// Assumes Leptos SSR output: attribute values are always double-quoted,
/// no unescaped quotes inside values, no multi-line values.
fn strip_data_attributes(html: &str) -> String {
    let mut result = String::with_capacity(html.len());
    let mut rest = html;
    let mut offset = 0;

    while let Some(start) = rest.find(" data-") {
        let abs_pos = offset + start;
        if !is_inside_tag(html, abs_pos) {
            // Match is in text content, not in a tag — skip past it
            result.push_str(&rest[..start + 6]);
            rest = &rest[start + 6..];
            offset = abs_pos + 6;
            continue;
        }

        // Check if this data attribute should be preserved.
        let after_space = &rest[start + 1..];
        if let Some(attr_len) = preserved_data_attr_len(after_space) {
            result.push_str(&rest[..start + 1 + attr_len]);
            rest = &rest[start + 1 + attr_len..];
            offset = abs_pos + 1 + attr_len;
            continue;
        }

        // Strip this data attribute.
        result.push_str(&rest[..start]);
        let after_data = &rest[start + 1..]; // skip the leading space

        // Find the `="..."` part
        if let Some(eq_pos) = after_data.find("=\"") {
            let value_start = eq_pos + 2;
            if let Some(quote_end) = after_data[value_start..].find('"') {
                let skip_len = 1 + value_start + quote_end + 1;
                rest = &rest[start + skip_len..];
                offset = abs_pos + skip_len;
                continue;
            }
        }

        // No valid attribute value found, keep as-is
        result.push_str(&rest[start..start + 6]);
        rest = &rest[start + 6..];
        offset = abs_pos + 6;
    }
    result.push_str(rest);
    result
}

// ── Markdown conversion handlers ──────────────────────────────

/// Handler for `<div>` elements: detects demo containers (`DemoShell`)
/// and replaces them with an *\[Interactive Demo\]* placeholder.
#[allow(clippy::needless_pass_by_value)] // Signature required by `ElementHandler` trait
fn handle_div(handlers: &dyn Handlers, element: Element<'_>) -> Option<HandlerResult> {
    let is_demo = element
        .attrs
        .iter()
        .any(|a| a.name.local.as_ref() == "class" && a.value.contains("demo-shell"));

    if is_demo {
        let desc = element.attrs.iter().find_map(|a| {
            (a.name.local.as_ref() == "data-demo-description").then(|| a.value.to_string())
        });
        let content = match desc {
            Some(d) => format!("\n\n*\\[Interactive Demo: {d}\\]*\n\n"),
            None => "\n\n*\\[Interactive Demo\\]*\n\n".to_owned(),
        };
        return Some(HandlerResult {
            content,
            markdown_translated: true,
        });
    }

    handlers.fallback(element)
}

/// Handler for `<code>` elements: distinguishes leptonic block code (no `<pre>` wrapper)
/// from inline code using the `data-inline` attribute.
#[allow(clippy::needless_pass_by_value)] // Signature required by `ElementHandler` trait
fn handle_code(handlers: &dyn Handlers, element: Element<'_>) -> Option<HandlerResult> {
    let is_leptonic = element
        .attrs
        .iter()
        .any(|a| a.name.local.as_ref() == "class" && a.value.contains("leptonic-code"));
    if !is_leptonic {
        return handlers.fallback(element); // Fall back to default htmd handler
    }

    let is_inline = element
        .attrs
        .iter()
        .any(|a| a.name.local.as_ref() == "data-inline" && &*a.value == "true");

    // walk_children processes <span> text; <button> is in skip_tags
    let content = handlers.walk_children(element.node).content;

    if is_inline {
        let trimmed = content.trim();
        Some(format!("`{trimmed}`").into())
    } else {
        let trimmed = content.trim_matches('\n');
        let lang = element
            .attrs
            .iter()
            .find(|a| a.name.local.as_ref() == "data-language")
            .map(|a| a.value.as_ref());
        match lang {
            Some(l) => Some(format!("\n\n```{l}\n{trimmed}\n```\n\n").into()),
            None => Some(format!("\n\n```\n{trimmed}\n```\n\n").into()),
        }
    }
}

/// Handler for `<a>` elements: strips leptonic anchor-link `#` markers from headings.
#[allow(clippy::needless_pass_by_value)] // Signature required by `ElementHandler` trait
fn handle_anchor(handlers: &dyn Handlers, element: Element<'_>) -> Option<HandlerResult> {
    let is_anchor_link = element
        .attrs
        .iter()
        .any(|a| a.name.local.as_ref() == "class" && a.value.contains("leptonic-anchor-link"));
    if is_anchor_link {
        Some(HandlerResult {
            content: String::new(),
            markdown_translated: true,
        })
    } else {
        handlers.fallback(element) // Fall back to default handler for normal links
    }
}

/// Handler for `<nav>` elements: skip navigation (`ToC` sidebar).
#[allow(clippy::needless_pass_by_value, clippy::unnecessary_wraps)] // Signature required by `ElementHandler` trait
fn handle_nav(_handlers: &dyn Handlers, _element: Element<'_>) -> Option<HandlerResult> {
    Some(HandlerResult {
        content: String::new(),
        markdown_translated: true,
    })
}

/// Singleton `HtmlToMarkdown` converter — built once, reused for every request.
static CONVERTER: std::sync::LazyLock<HtmlToMarkdown> = std::sync::LazyLock::new(|| {
    HtmlToMarkdown::builder()
        .skip_tags(vec!["script", "style", "button", "svg"])
        .add_handler(vec!["div"], handle_div)
        .add_handler(vec!["nav"], handle_nav)
        .add_handler(vec!["code"], handle_code)
        .add_handler(vec!["a"], handle_anchor)
        .build()
});

/// Convert cleaned HTML to markdown using `htmd`.
fn convert_to_markdown(html: &str) -> String {
    match CONVERTER.convert(html) {
        Ok(md) => md,
        Err(e) => {
            tracing::warn!("Markdown conversion failed: {e}");
            String::new()
        }
    }
}

/// Warm the markdown cache by sending synthetic requests for every doc route.
#[allow(clippy::missing_panics_doc)]
pub async fn warm_markdown_cache(app: axum::Router, doc_paths: &[String]) {
    use tower::ServiceExt;

    for path in doc_paths {
        let uri = format!("{path}.md");
        let req = Request::builder().uri(&uri).body(Body::empty()).unwrap();
        let _ = app.clone().oneshot(req).await;
    }
    tracing::info!("Markdown cache warmed with {} entries", doc_paths.len());
}
