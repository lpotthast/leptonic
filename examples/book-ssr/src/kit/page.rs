use leptonic::{
    components::prelude::{AnchorLink, Button, ButtonColor, ButtonSize, ButtonVariant},
    utils::live_announcer::announce_polite,
};
use leptos::{context::Provider, prelude::*};
use leptos_router::hooks::use_location;

use super::section::{Parent, heading, slug};

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

/// A documentation page: the `<article>` (exported as Markdown) and a table of contents generated from its
/// [`Section`](super::Section)s.
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

    view! {
        <article class="doc-article">
            <CopyAsMarkdownButton/>
            {heading(1, id, title)}
            {content}
        </article>
        <TableOfContents entries=toc.0.get_value()/>
    }
}

#[component]
fn TableOfContents(entries: Vec<TocEntry>) -> impl IntoView {
    view! {
        <nav id="book-toc" aria-label="Table of contents">
            <h2>"Contents"</h2>
            <ul>
                {entries
                    .into_iter()
                    .map(|TocEntry { level, id, title }| {
                        view! {
                            <li data-level=level>
                                <AnchorLink href=format!("#{id}")>{title}</AnchorLink>
                            </li>
                        }
                    })
                    .collect_view()}
            </ul>
        </nav>
    }
}

/// Copies the Markdown export of the current page (`<path>.md`) to the clipboard, and confirms it visibly and to screen
/// readers.
#[component]
fn CopyAsMarkdownButton() -> impl IntoView {
    let location = use_location();
    let copied = RwSignal::new(false);

    let copy = move |_| {
        let md_url = format!("{}.md", location.pathname.get_untracked());
        leptos::task::spawn_local(async move {
            if copy_url_contents(&md_url).await.is_some() {
                copied.set(true);
                announce_polite("Copied the page as Markdown");
                set_timeout(move || copied.set(false), std::time::Duration::from_secs(2));
            }
        });
    };

    view! {
        <Button
            on_press=copy
            variant=ButtonVariant::Outlined
            color=ButtonColor::Secondary
            size=ButtonSize::Small
            classes="copy-md-button"
            attr:title="Copy page as Markdown"
        >
            {move || if copied.get() { "Copied!" } else { "Copy as MD" }}
        </Button>
    }
}

/// Fetches `url` and writes the response text to the clipboard. `None` if any step fails.
#[cfg(not(feature = "ssr"))]
async fn copy_url_contents(url: &str) -> Option<()> {
    use wasm_bindgen::JsCast;
    use wasm_bindgen_futures::JsFuture;

    let window = leptos_use::use_window();
    let window = window.as_ref()?;
    let response: web_sys::Response = JsFuture::from(window.fetch_with_str(url))
        .await
        .ok()?
        .unchecked_into();
    let text = JsFuture::from(response.text().ok()?)
        .await
        .ok()?
        .as_string()?;
    JsFuture::from(window.navigator().clipboard().write_text(&text))
        .await
        .ok()?;
    Some(())
}

/// Click handlers never run on the server.
#[cfg(feature = "ssr")]
#[allow(clippy::unused_async)]
async fn copy_url_contents(_url: &str) -> Option<()> {
    None
}
