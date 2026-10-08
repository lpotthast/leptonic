//! Conversion of a rendered documentation page (HTML) into Markdown plus the metadata used by the index and search.

use std::{fmt::Write as _, rc::Rc, sync::LazyLock};

use htmd::{
    Element, HtmlToMarkdown,
    element_handler::{HandlerResult, Handlers},
};
use markup5ever_rcdom::{Node, NodeData};
use scraper::{ElementRef, Html, Selector};

/// A documentation page converted to Markdown.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConvertedPage {
    /// Text of the page's `<h1>`.
    pub title: String,
    /// The page's first paragraph, shortened to [`DESCRIPTION_MAX_LEN`] bytes.
    pub description: String,
    /// The page's `<h2>` sections, and the `<h3>` sections naming an item no `<h2>` names (see [`names_item`]), in
    /// page order.
    pub sections: Vec<Heading>,
    /// Links of the "See Also" section.
    pub related: Vec<Link>,
    /// The `<article>` as Markdown.
    pub markdown: String,
    /// The `<article>` as plain text, without demos, for search.
    pub text: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Heading {
    pub text: String,
    pub id: String,
    /// 2 for an `<h2>`, 3 for an `<h3>`.
    pub level: u8,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Link {
    pub title: String,
    pub path: String,
}

const DESCRIPTION_MAX_LEN: usize = 200;

/// Converts the `<article>` of a page. `None` if the page has no article.
pub fn convert_page(html: &str) -> Option<ConvertedPage> {
    static ARTICLE: LazyLock<Selector> = LazyLock::new(|| selector("article"));
    static TITLE: LazyLock<Selector> = LazyLock::new(|| selector("h1"));
    static SECTIONS: LazyLock<Selector> = LazyLock::new(|| selector("h2[id]"));
    static SUBSECTIONS: LazyLock<Selector> = LazyLock::new(|| selector("h2[id], h3[id]"));
    static PARAGRAPHS: LazyLock<Selector> = LazyLock::new(|| selector("p"));
    static LINKS: LazyLock<Selector> = LazyLock::new(|| selector("a[href]"));

    let document = Html::parse_document(html);
    let article = document.select(&ARTICLE).next()?;

    let title = article
        .select(&TITLE)
        .next()
        .map(heading_text)
        .unwrap_or_default();

    let headings: Vec<Heading> = article
        .select(&SUBSECTIONS)
        .map(|heading| Heading {
            text: heading_text(heading),
            id: heading.attr("id").unwrap_or_default().to_owned(),
            level: if heading.value().name() == "h2" { 2 } else { 3 },
        })
        .collect();
    let sections = headings
        .iter()
        .filter(|heading| {
            heading.level == 2
                || (names_item(&heading.text)
                    && !headings
                        .iter()
                        .any(|h2| h2.level == 2 && h2.text == heading.text))
        })
        .cloned()
        .collect();

    let description = article
        .select(&PARAGRAPHS)
        .next()
        .map(|p| {
            let mut text = String::new();
            plain_text(p, &mut text);
            shorten(&normalize_whitespace(&text), DESCRIPTION_MAX_LEN)
        })
        .unwrap_or_default();

    let related = article
        .select(&SECTIONS)
        .find(|h2| h2.attr("id") == Some("see-also"))
        .and_then(|h2| h2.parent().and_then(ElementRef::wrap))
        .filter(|parent| parent.value().name() == "section")
        .map(|see_also| {
            see_also
                .select(&LINKS)
                .filter(|a| !is_anchor_link(*a))
                .map(|a| Link {
                    title: normalize_whitespace(&a.text().collect::<String>()),
                    path: markdown_href(a.attr("href").unwrap_or_default()),
                })
                .collect()
        })
        .unwrap_or_default();

    let markdown = CONVERTER
        .convert(&article.inner_html())
        .unwrap_or_else(|err| {
            tracing::warn!("Markdown conversion failed: {err}");
            String::new()
        });

    let mut text = String::new();
    plain_text(article, &mut text);
    let text = normalize_whitespace(&text);

    Some(ConvertedPage {
        title,
        description,
        sections,
        related,
        markdown,
        text,
    })
}

/// Elements whose start and end separate words.
const BLOCK_ELEMENTS: &[&str] = &[
    "article",
    "section",
    "div",
    "p",
    "h1",
    "h2",
    "h3",
    "h4",
    "h5",
    "h6",
    "ul",
    "ol",
    "li",
    "dl",
    "dt",
    "dd",
    "table",
    "thead",
    "tbody",
    "tr",
    "th",
    "td",
    "pre",
    "blockquote",
    "br",
    "hr",
];

/// Whether a heading names a Rust item (`use_drag_session`, `ListState`): one word with an underscore or an inner
/// uppercase letter. Such `<h3>` sections are listed with the `<h2>` ones, so that the index and the search find the
/// hooks, atoms and types documented in subsections, and generic subsections (`Props`, `Example`) are not.
fn names_item(text: &str) -> bool {
    !text.contains(char::is_whitespace)
        && (text.contains('_')
            || text
                .chars()
                .zip(text.chars().skip(1))
                .any(|(a, b)| a.is_lowercase() && b.is_uppercase()))
}

/// Appends the text of `element` as a screen reader reads it: without demos (`Demo`), buttons, the `#` anchors of
/// headings and `aria-hidden` parts, so that a key shown as a glyph reads as its name ("Escape", not "Esc" followed
/// by "Escape"). The `+` between the keys of a combination stays.
fn plain_text(element: ElementRef<'_>, out: &mut String) {
    let value = element.value();
    if matches!(value.name(), "button" | "script" | "style" | "svg")
        || value.has_class("doc-demo", scraper::CaseSensitivity::CaseSensitive)
        || is_anchor_link(element)
        || (value.attr("aria-hidden") == Some("true") && value.attr("data-separator").is_none())
    {
        return;
    }
    let block = BLOCK_ELEMENTS.contains(&value.name());
    if block {
        out.push(' ');
    }
    for child in element.children() {
        match child.value() {
            scraper::Node::Text(text) => out.push_str(text),
            scraper::Node::Element(_) => {
                if let Some(child) = ElementRef::wrap(child) {
                    plain_text(child, out);
                }
            }
            _ => {}
        }
    }
    if block {
        out.push(' ');
    }
}

fn selector(css: &str) -> Selector {
    Selector::parse(css).expect("static selector is valid")
}

/// Text of a heading without its "direct link" anchor (`#`).
fn heading_text(heading: ElementRef<'_>) -> String {
    let text: String = heading
        .children()
        .filter(|child| !ElementRef::wrap(*child).is_some_and(is_anchor_link))
        .map(|child| match child.value() {
            scraper::Node::Text(text) => text.to_string(),
            scraper::Node::Element(_) => ElementRef::wrap(child)
                .map(|element| element.text().collect())
                .unwrap_or_default(),
            _ => String::new(),
        })
        .collect();
    normalize_whitespace(&text)
}

fn is_anchor_link(element: ElementRef<'_>) -> bool {
    element.value().has_class(
        "doc-heading-anchor",
        scraper::CaseSensitivity::CaseSensitive,
    )
}

fn normalize_whitespace(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// Shortens `text` to at most `max_len` bytes at a word boundary, marking the cut with "…".
fn shorten(text: &str, max_len: usize) -> String {
    if text.len() <= max_len {
        return text.to_owned();
    }
    let mut end = max_len;
    while !text.is_char_boundary(end) {
        end -= 1;
    }
    let cut = &text[..end];
    let cut = cut.rfind(' ').map_or(cut, |space| &cut[..space]);
    format!("{cut}\u{2026}")
}

/// Links to documentation pages point to their Markdown export: `/doc/button#props` becomes `/doc/button.md#props`.
fn markdown_href(href: &str) -> String {
    // Other sites, and links that already point to Markdown (the index, `/doc/llm-index.md`).
    let path = href.split('#').next().unwrap_or_default();
    if !href.starts_with("/doc/")
        || std::path::Path::new(path)
            .extension()
            .is_some_and(|ext| ext == "md")
    {
        return href.to_owned();
    }
    let (path, fragment) = href
        .split_once('#')
        .map_or((href, None), |(path, fragment)| (path, Some(fragment)));
    match fragment {
        Some(fragment) => format!("{path}.md#{fragment}"),
        None => format!("{path}.md"),
    }
}

static CONVERTER: LazyLock<HtmlToMarkdown> = LazyLock::new(|| {
    HtmlToMarkdown::builder()
        .skip_tags(vec!["script", "style", "button", "svg"])
        .add_handler(vec!["div"], handle_div)
        .add_handler(vec!["code"], handle_code)
        .add_handler(vec!["a"], handle_anchor)
        .add_handler(vec!["kbd"], handle_kbd)
        .build()
});

fn attr<'a>(element: &'a Element<'_>, name: &str) -> Option<&'a str> {
    element
        .attrs
        .iter()
        .find(|attr| &*attr.name.local == name)
        .map(|attr| &*attr.value)
}

fn has_class(element: &Element<'_>, class: &str) -> bool {
    attr(element, "class").is_some_and(|classes| classes.split_whitespace().any(|c| c == class))
}

/// Demos (`Demo`) become a placeholder naming their description, followed by their Rust source.
#[allow(clippy::needless_pass_by_value)] // Signature required by `ElementHandler`.
fn handle_div(handlers: &dyn Handlers, element: Element<'_>) -> Option<HandlerResult> {
    if !has_class(&element, "doc-demo") {
        return handlers.fallback(element);
    }
    let description = find_element(element.node, &|node| {
        element_attr(node, "data-demo-description").is_some()
    })
    .and_then(|demo| element_attr(&demo, "data-demo-description"))
    .unwrap_or_default();
    let mut markdown = format!("\n\n*\\[Interactive Demo: {description}\\]*\n\n");
    let source = find_element(element.node, &|node| {
        element_attr(node, "data-language").as_deref() == Some("rust")
    });
    if let Some(source) = source {
        let _ = write!(
            markdown,
            "```rust\n{}\n```\n\n",
            text_content(&source).trim_matches('\n')
        );
    }
    Some(markdown.into())
}

/// The first descendant element of `node` matching `predicate`, depth first.
fn find_element(node: &Rc<Node>, predicate: &dyn Fn(&Node) -> bool) -> Option<Rc<Node>> {
    node.children.borrow().iter().find_map(|child| {
        if matches!(child.data, NodeData::Element { .. }) && predicate(child) {
            Some(Rc::clone(child))
        } else {
            find_element(child, predicate)
        }
    })
}

fn element_attr(node: &Node, name: &str) -> Option<String> {
    match &node.data {
        NodeData::Element { attrs, .. } => attrs
            .borrow()
            .iter()
            .find(|attr| &*attr.name.local == name)
            .map(|attr| attr.value.to_string()),
        _ => None,
    }
}

fn text_content(node: &Node) -> String {
    match &node.data {
        NodeData::Text { contents } => contents.borrow().to_string(),
        _ => node
            .children
            .borrow()
            .iter()
            .map(|child| text_content(child))
            .collect(),
    }
}

/// The kit's `Code` renders a bare `<code class="doc-code">` (no `<pre>`) for both inline code and blocks; `data-inline`
/// tells them apart, `data-language` names the language of a block.
#[allow(clippy::needless_pass_by_value)] // Signature required by `ElementHandler`.
fn handle_code(handlers: &dyn Handlers, element: Element<'_>) -> Option<HandlerResult> {
    if !has_class(&element, "doc-code") {
        return handlers.fallback(element);
    }
    if attr(&element, "data-inline") == Some("true") {
        let content = handlers.walk_children(element.node).content;
        return Some(format!("`{}`", content.trim()).into());
    }
    // A block's code is its raw text (the copy button next to it has none): converting the highlighter's spans as
    // Markdown would drop the line breaks at their ends.
    let code = find_element(element.node, &|node| {
        element_attr(node, "class").is_some_and(|classes| {
            classes
                .split_whitespace()
                .any(|class| class == "doc-code-text")
        })
    })
    .map_or_else(|| text_content(element.node), |text| text_content(&text));
    let language = attr(&element, "data-language").unwrap_or_default();
    Some(format!("\n\n```{language}\n{}\n```\n\n", code.trim_matches('\n')).into())
}

/// Drops the `#` anchors next to headings (`doc-heading-anchor`); links to documentation pages point to their Markdown export.
#[allow(clippy::needless_pass_by_value)] // Signature required by `ElementHandler`.
fn handle_anchor(handlers: &dyn Handlers, element: Element<'_>) -> Option<HandlerResult> {
    if has_class(&element, "doc-heading-anchor") {
        return Some(String::new().into());
    }
    match attr(&element, "href") {
        Some(href) if href.starts_with("/doc/") => {
            let content = handlers.walk_children(element.node).content;
            Some(format!("[{}]({})", content.trim(), markdown_href(href)).into())
        }
        _ => handlers.fallback(element),
    }
}

/// Keys (the kit's `Keys`, leptonic's `Keys` atom: a `<kbd class="doc-keys">` around a `<kbd>` per key) become inline
/// code, a combination keeps its keys apart: `Shift + Tab` becomes `` `⇧` + `↹` ``. A key shown as a glyph keeps the
/// glyph (its `aria-hidden` part), not its visually hidden name.
#[allow(clippy::needless_pass_by_value, clippy::unnecessary_wraps)] // Signature required by `ElementHandler`.
fn handle_kbd(handlers: &dyn Handlers, element: Element<'_>) -> Option<HandlerResult> {
    if has_class(&element, "doc-keys") {
        let content = handlers.walk_children(element.node).content;
        return Some(content.trim().replace('+', " + ").into());
    }
    let shown = find_element(element.node, &|node| {
        element_attr(node, "aria-hidden").as_deref() == Some("true")
    })
    .map_or_else(
        || handlers.walk_children(element.node).content,
        |glyph| text_content(&glyph),
    );
    Some(format!("`{}`", shown.trim()).into())
}

#[cfg(test)]
mod tests {
    use assertr::prelude::*;

    use super::*;

    fn page(article: &str) -> ConvertedPage {
        convert_page(&format!(
            "<html><body><article>{article}</article></body></html>"
        ))
        .unwrap()
    }

    #[test]
    fn extracts_title_sections_and_description_without_anchor_links() {
        let page = page(
            r##"<h1 id="use-button">use_button<a class="doc-link doc-heading-anchor" href="#use-button">#</a></h1>
            <p>The   use_button hook.</p>
            <section><h2 id="input">Input<a class="doc-link doc-heading-anchor" href="#input">#</a></h2><p>x</p></section>"##,
        );
        assert_that!(page.title).is_equal_to("use_button");
        assert_that!(page.description).is_equal_to("The use_button hook.");
        assert_that!(page.sections).is_equal_to(vec![Heading {
            text: "Input".to_owned(),
            id: "input".to_owned(),
            level: 2,
        }]);
        assert_that!(page.markdown.as_str()).does_not_contain("#](");
        assert_that!(page.markdown.as_str()).contains("## Input");
    }

    #[test]
    fn sections_include_subsections_naming_items() {
        let page = page(
            r#"<h1>T</h1>
            <section><h2 id="collections">Collections</h2>
                <section><h3 id="use-drag-session">use_drag_session</h3></section>
                <section><h3 id="props">Props</h3></section>
                <section><h3 id="list-state">ListState</h3></section></section>
            <section><h2 id="text-field">TextField</h2></section>
            <section><h2 id="data-attributes">Data Attributes</h2>
                <section><h3 id="data-text-field">TextField</h3></section></section>"#,
        );
        let sections: Vec<_> = page
            .sections
            .iter()
            .map(|section| (section.text.as_str(), section.level))
            .collect();
        assert_that!(sections).is_equal_to(vec![
            ("Collections", 2),
            ("use_drag_session", 3),
            ("ListState", 3),
            ("TextField", 2),
            ("Data Attributes", 2),
        ]);
    }

    #[test]
    fn see_also_links_point_to_markdown_exports() {
        let page = page(
            r##"<h1>T</h1><section><h2 id="see-also">See Also<a class="doc-link doc-heading-anchor" href="#see-also">#</a></h2><ul>
            <li><a href="/doc/button">Button overview</a></li>
            <li><a href="/doc/focus/use-focus-ring#keyboard">use_focus_ring</a></li>
            </ul></section>"##,
        );
        assert_that!(page.related).is_equal_to(vec![
            Link {
                title: "Button overview".to_owned(),
                path: "/doc/button.md".to_owned(),
            },
            Link {
                title: "use_focus_ring".to_owned(),
                path: "/doc/focus/use-focus-ring.md#keyboard".to_owned(),
            },
        ]);
        assert_that!(page.markdown.as_str()).contains("[Button overview](/doc/button.md)");
        assert_that!(markdown_href("/doc/llm-index.md")).is_equal_to("/doc/llm-index.md");
    }

    #[test]
    fn converts_demos_code_and_keys() {
        let page = page(
            r#"<h1>T</h1>
            <div class="doc-demo"><div class="demo" data-demo-description="Press counter"><button>x</button></div>
            <div class="doc-disclosure"><button aria-expanded="false">View source</button>
            <div aria-hidden="true"><code class="doc-code" data-language="rust">fn demo() {}</code></div></div></div>
            <p><code class="doc-code" data-inline="true">use_press</code> and <kbd class="leptonic-Keys doc-keys"><kbd>Enter</kbd></kbd></p>
            <p><kbd class="leptonic-Keys doc-keys"><kbd><span aria-hidden="true">⇧</span><span style="position:absolute">Shift</span></kbd><span data-separator="" aria-hidden="true">+</span><kbd>↹</kbd></kbd></p>
            <code class="doc-code" data-language="rust">let x = 1;</code>"#,
        );
        assert_that!(page.markdown.as_str())
            .contains("*\\[Interactive Demo: Press counter\\]*\n\n```rust\nfn demo() {}\n```");
        assert_that!(page.markdown.as_str()).contains("`use_press` and `Enter`");
        assert_that!(page.markdown.as_str()).contains("`⇧` + `↹`");
        assert_that!(page.markdown.as_str()).contains("```rust\nlet x = 1;\n```");
    }

    #[test]
    fn code_blocks_keep_line_breaks_at_the_end_of_highlighted_spans() {
        let page = page(
            r#"<h1>T</h1><code class="doc-code" data-language="rust"><span class="doc-code-text">fn app() {
    <span class="syn-comment">// The theme.
</span>    <span class="syn-storage">let</span> theme = 1;
    <span class="syn-punctuation">}</span></span></code>"#,
        );
        assert_that!(page.markdown.as_str())
            .contains("```rust\nfn app() {\n    // The theme.\n    let theme = 1;\n    }\n```");
    }

    #[test]
    fn plain_text_separates_blocks_and_leaves_out_demos_and_anchors() {
        let page = page(
            r##"<div class="doc-article-header"><h1 id="t">Title<a class="doc-link doc-heading-anchor" href="#t">#</a></h1>
            <button>Copy as Markdown</button></div><p>The <code>use_press</code>es hook.</p>
            <div class="doc-demo"><div class="demo" data-demo-description="Counter"><button>Press</button><p>Pressed 0 times</p></div></div>
            <section><h2 id="input">Input</h2><ul><li>One</li><li>Two</li></ul></section>"##,
        );
        assert_that!(page.text).is_equal_to("Title The use_presses hook. Input One Two");
    }

    #[test]
    fn plain_text_and_description_name_keys_shown_as_glyphs() {
        let page = page(
            r#"<h1>T</h1><p><kbd class="leptonic-Keys doc-keys"><kbd><span aria-hidden="true">Esc</span><span style="position:absolute">Escape</span></kbd></kbd>
            clears it, <kbd class="leptonic-Keys doc-keys"><kbd><span aria-hidden="true">⇧</span><span style="position:absolute">Shift</span></kbd><span data-separator="" aria-hidden="true">+</span><kbd>Enter</kbd></kbd> submits.</p>"#,
        );
        assert_that!(page.description).is_equal_to("Escape clears it, Shift+Enter submits.");
        assert_that!(page.text).is_equal_to("T Escape clears it, Shift+Enter submits.");
    }

    #[test]
    fn shorten_cuts_at_word_boundary() {
        assert_that!(shorten("short", 10)).is_equal_to("short");
        assert_that!(shorten("one two three", 9)).is_equal_to("one two\u{2026}");
    }
}
