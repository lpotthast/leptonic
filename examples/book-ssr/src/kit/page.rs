use leptonic::{
    atoms::prelude::{AnchorLink, Button},
    utils::{clipboard::write_text_deferred, live_announcer::announce_polite},
};
use leptos::{context::Provider, prelude::*};
use leptos_meta::{Meta, Title};
use leptos_router::hooks::use_location;

use super::section::{Parent, heading, slug};
use crate::{
    app::{MainLandmark, NavLandmark, SITE_DESCRIPTION},
    nav::nav,
};

/// One heading of the page, as listed in the table of contents.
#[derive(Debug, Clone)]
struct TocEntry {
    level: u8,
    id: Oco<'static, str>,
    title: &'static str,
}

/// Collects the headings of a [`DocPage`] while its content renders.
///
/// Sections register themselves when they are created. Component bodies run eagerly and in document order (on the
/// server and while hydrating), so once the page content is built, the registry holds every heading in order.
#[derive(Debug, Clone, Copy)]
pub(super) struct TocRegistry(StoredValue<Vec<TocEntry>>);

impl TocRegistry {
    pub(super) fn register(self, level: u8, id: Oco<'static, str>, title: &'static str) {
        self.0
            .update_value(|entries| entries.push(TocEntry { level, id, title }));
    }

    pub(super) fn contains(self, id: &str) -> bool {
        self.0
            .with_value(|entries| entries.iter().any(|entry| entry.id == id))
    }
}

/// Content a layout around the page shows at the top of a [`DocPage`]'s `<main>`, above the article: the concept name
/// and tabs of [`ConceptLayout`](crate::pages::documentation::concept_layout::ConceptLayout).
#[derive(Clone)]
pub struct DocPageHeader(pub ViewFn);

/// A documentation page: the page's `<main>` with the `<article>` (exported as Markdown), and next to it a table of
/// contents generated from its [`Section`](super::Section)s. Sets the document title and description.
///
/// The `title` becomes the page's `<h1>`. Its anchor id is the slug of the title unless `id` is given.
#[component]
pub fn DocPage(
    title: &'static str,
    #[prop(optional)] id: Option<&'static str>,
    children: Children,
) -> impl IntoView {
    let toc = TocRegistry(StoredValue::new(Vec::new()));
    let id: Oco<'static, str> = id.map_or_else(|| slug(title).into(), Oco::Borrowed);
    toc.register(1, id.clone(), title);

    let page = Parent {
        level: 1,
        id: id.clone(),
    };
    let content = view! {
        <Provider value=toc>
            <Provider value=page>{children()}</Provider>
        </Provider>
    };
    let header = use_context::<DocPageHeader>().map(|header| move || header.0.run());
    let description = use_location()
        .pathname
        .with_untracked(|path| page_description(path, title));

    view! {
        <Title text=format!("{title} \u{2013} Leptonic")/>
        <Meta name="description" content=description/>
        <MainLandmark class="doc-main">
            {header}
            <article class="doc-article">
                <div class="doc-article-header">
                    {heading(1, id, title)}
                    <CopyAsMarkdownButton/>
                </div>
                {content}
            </article>
        </MainLandmark>
        <TableOfContents entries=toc.0.get_value()/>
    }
}

/// The description of the page at `path` for search engines and link previews, from its entry in the navigation: the
/// summary of a page, the summary of a concept for its layer pages, the members of a group for its overview.
fn page_description(path: &str, title: &str) -> String {
    for group in nav().groups() {
        if group.overview.as_deref() == Some(path) && !group.entries.is_empty() {
            let members: Vec<&str> = group.entries.iter().map(|entry| entry.title).collect();
            return format!("{} in leptonic: {}.", group.title, list(&members));
        }
        for entry in &group.entries {
            if entry.href == path {
                return format!("{}.", entry.summary);
            }
            if entry.tabs.iter().any(|tab| tab.href == path) {
                return format!("{title}: {}.", lowercase_first(entry.summary));
            }
        }
    }
    SITE_DESCRIPTION.to_owned()
}

/// `["A", "B", "C"]` as "A, B and C".
fn list(items: &[&str]) -> String {
    match items {
        [] => String::new(),
        [single] => (*single).to_owned(),
        [init @ .., last] => format!("{} and {last}", init.join(", ")),
    }
}

/// Lowercases the first letter of a sentence, unless it starts an acronym or identifier ("UI", "ARIA").
fn lowercase_first(text: &str) -> String {
    let mut chars = text.chars();
    match (chars.next(), chars.next()) {
        (Some(first), Some(second)) if first.is_uppercase() && second.is_lowercase() => first
            .to_lowercase()
            .chain(text[first.len_utf8()..].chars())
            .collect(),
        _ => text.to_owned(),
    }
}

#[component]
fn TableOfContents(entries: Vec<TocEntry>) -> impl IntoView {
    view! {
        <NavLandmark id="book-toc" label="Table of contents">
            <h2>"Contents"</h2>
            <ul>
                {entries
                    .into_iter()
                    .map(|TocEntry { level, id, title }| {
                        view! {
                            <li data-level=level>
                                <AnchorLink href=format!("#{id}") classes="doc-toc-link">{title}</AnchorLink>
                            </li>
                        }
                    })
                    .collect_view()}
            </ul>
        </NavLandmark>
    }
}

/// The outcome of the last copy, shown on the button for a moment.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CopyState {
    Idle,
    Copied,
    Failed,
}

/// Copies the Markdown export of the current page (`<path>.md`) to the clipboard, and confirms it (or the failure)
/// visibly and to screen readers.
///
/// The export is downloaded only when the button is pressed, never on navigation, and kept for
/// [`MARKDOWN_CACHE_TTL`], so pressing again (or coming back to the page) copies without downloading.
#[component]
fn CopyAsMarkdownButton() -> impl IntoView {
    let md_url = StoredValue::new(format!("{}.md", use_location().pathname.get_untracked()));
    let state = RwSignal::new(CopyState::Idle);

    let copy = move |_| {
        let url = md_url.get_value();
        // The clipboard write starts during the press (Safari only allows it there); the text follows once it is
        // downloaded, or comes from the cache.
        let written = write_text_deferred(async move {
            if let Some(text) = cached_markdown(&url) {
                return Some(text);
            }
            let text = fetch_text(&url).await?;
            cache_markdown(&url, text.clone());
            Some(text)
        });
        leptos::task::spawn_local(async move {
            let copied = written.await.is_ok();
            state.set(if copied {
                CopyState::Copied
            } else {
                CopyState::Failed
            });
            announce_polite(if copied {
                "Copied the page as Markdown"
            } else {
                "Couldn\u{2019}t copy the page"
            });
            set_timeout(
                move || state.set(CopyState::Idle),
                std::time::Duration::from_secs(2),
            );
        });
    };

    view! {
        <Button on_press=copy classes="doc-copy-markdown">
            {move || match state.get() {
                CopyState::Idle => "Copy as Markdown",
                CopyState::Copied => "Copied",
                CopyState::Failed => "Copy failed",
            }}
        </Button>
    }
}

/// How long a downloaded Markdown export is reused.
const MARKDOWN_CACHE_TTL: std::time::Duration = std::time::Duration::from_mins(5);

thread_local! {
    /// Downloaded Markdown exports by URL. An entry is removed [`MARKDOWN_CACHE_TTL`] after its download.
    static MARKDOWN_CACHE: std::cell::RefCell<std::collections::HashMap<String, String>> =
        std::cell::RefCell::new(std::collections::HashMap::new());
}

fn cached_markdown(url: &str) -> Option<String> {
    MARKDOWN_CACHE.with_borrow(|cache| cache.get(url).cloned())
}

fn cache_markdown(url: &str, text: String) {
    MARKDOWN_CACHE.with_borrow_mut(|cache| cache.insert(url.to_owned(), text));
    let url = url.to_owned();
    set_timeout(
        move || {
            MARKDOWN_CACHE.with_borrow_mut(|cache| cache.remove(&url));
        },
        MARKDOWN_CACHE_TTL,
    );
}

/// Downloads `path` (a path of this site) as text. `None` if the request fails.
async fn fetch_text(path: &str) -> Option<String> {
    use wasm_bindgen::JsCast;
    use wasm_bindgen_futures::JsFuture;

    let promise = leptos_use::use_window().as_ref()?.fetch_with_str(path);
    let response: web_sys::Response = JsFuture::from(promise).await.ok()?.dyn_into().ok()?;
    if !response.ok() {
        return None;
    }
    JsFuture::from(response.text().ok()?).await.ok()?.as_string()
}

#[cfg(test)]
mod tests {
    use assertr::prelude::*;

    use super::{list, lowercase_first, page_description};
    use crate::{app::SITE_DESCRIPTION, nav::nav};

    #[test]
    fn lists_items_in_prose() {
        assert_that!(list(&["A"])).is_equal_to("A");
        assert_that!(list(&["A", "B", "C"])).is_equal_to("A, B and C");
    }

    #[test]
    fn lowercases_sentences_but_not_acronyms() {
        assert_that!(lowercase_first("Triggers an action")).is_equal_to("triggers an action");
        assert_that!(lowercase_first("UI elements")).is_equal_to("UI elements");
    }

    /// Every page of the navigation gets a description of its own.
    #[test]
    fn every_page_has_a_description() {
        let mut missing = Vec::new();
        for page in nav().pages() {
            let description = page_description(page, "Title");
            if description == SITE_DESCRIPTION
                && !nav().groups().any(|group| {
                    group.overview.as_deref() == Some(page) && group.entries.is_empty()
                })
            {
                missing.push(page);
            }
        }
        assert_that!(missing).is_empty();
    }
}
