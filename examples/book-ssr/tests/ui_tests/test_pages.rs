//! Checks every documentation page: errors, the dark theme, links and narrow screens.

use std::{
    borrow::Cow,
    collections::{BTreeSet, HashMap},
    sync::{LazyLock, Mutex, PoisonError},
};

use assertr::prelude::*;
use browser_test::{
    BrowserTest, async_trait,
    thirtyfour::{By, WebDriver},
};
use leptos_browser_test::Report;

use crate::pages::BookPage;

/// Whether the dark theme applies: the body background is dark.
pub const IS_DARK: &str = "const c = getComputedStyle(document.body).backgroundColor.match(/\\d+/g); \
                       return !!c && (+c[0] + +c[1] + +c[2]) / 3 < 80;";

/// Whether all CSS transitions finished: elements that transition their colors (buttons, demo targets) show their
/// final theme colors only then. The server renders the reader's theme (it is kept in a cookie), so a page normally
/// starts in the dark theme; this guards against anything that still switches after hydration.
pub const TRANSITIONS_FINISHED: &str = "return document.getAnimations().every(a => !(a instanceof CSSTransition) || a.playState !== 'running');";

/// Demo elements that don't fit the dark theme: bright panels (at least 48 x 32 px, so that small shapes like a
/// switch's knob don't count), or dark text on a dark background.
const DARK_THEME_PROBLEMS: &str = r"
    const lum = c => { const m = c.match(/rgba?\(([^)]+)\)/); if (!m) return 0; const [r, g, b, a = 1] = m[1].split(',').map(Number); return a < 0.5 ? 0 : (0.2126 * r + 0.7152 * g + 0.0722 * b) / 255; };
    const out = [];
    for (const el of document.querySelectorAll('.demo *')) {
        const cs = getComputedStyle(el);
        const label = el.tagName.toLowerCase() + '.' + [...el.classList].join('.');
        if (cs.display !== 'none' && el.offsetWidth >= 48 && el.offsetHeight >= 32 && lum(cs.backgroundColor) > 0.85) out.push('bright background: ' + label + ' “' + el.textContent.trim().slice(0, 30) + '” (' + cs.backgroundColor + ')');
        const ownText = [...el.childNodes].some(n => n.nodeType === 3 && /[A-Za-z0-9]/.test(n.textContent));
        if (ownText && lum(cs.color) < 0.2 && lum(cs.backgroundColor) < 0.3 && lum(getComputedStyle(el.closest('.demo')).backgroundColor) < 0.3) out.push('dark text: ' + label);
    }
    return [...new Set(out)];";

/// The internal links of the page content: `/doc/...` paths and `#fragment`s.
const INTERNAL_LINKS: &str = "return [...document.querySelectorAll('main a[href]')].map(a => a.getAttribute('href')) \
                              .filter(h => h.startsWith('/doc') || h.startsWith('#'));";

/// The ids of all elements of the page.
const IDS: &str = "return [...document.querySelectorAll('[id]')].map(e => e.id);";

/// Whether the book shows its small-screen layout: the app bar's menu button replaces the desktop links.
const SMALL_SCREEN_LAYOUT: &str =
    "return !!document.querySelector('#book-app-bar [aria-label=\"Menu\"]');";

/// The width of a phone screen, in CSS pixels.
const PHONE_WIDTH: u32 = 390;

/// Elements of the page content wider than the viewport, outside of scroll containers.
const OVERFLOWING: &str = r"
    const unclipped = e => { for (let p = e.parentElement; p; p = p.parentElement) if (getComputedStyle(p).overflowX !== 'visible') return false; return true; };
    return [...document.querySelectorAll('main *')]
        .filter(e => e.getBoundingClientRect().right > window.innerWidth + 1 && unclipped(e))
        .slice(0, 3)
        .map(e => e.tagName.toLowerCase() + '.' + e.className + ': ' + e.textContent.trim().slice(0, 40));";

/// The internal links and element ids the page tests found, for [`LinkTests`] (after all page tests).
#[derive(Default)]
struct SiteLinks {
    /// `(page, link)` pairs.
    links: Vec<(String, String)>,
    /// The element ids of every visited page.
    ids: HashMap<String, BTreeSet<String>>,
}

static SITE_LINKS: LazyLock<Mutex<SiteLinks>> = LazyLock::new(Mutex::default);

/// A test per documentation page ([`BookPage::paths`]).
pub fn page_tests() -> impl Iterator<Item = PageContentTest> {
    BookPage::paths()
        .into_iter()
        .map(|path| PageContentTest { path })
}

/// Visits a page in the dark theme: no page errors (`ui_tests::CheckPageErrors`), readable demos, and nothing makes
/// the page wider than a phone screen. Records the page's internal links and ids for [`LinkTests`].
///
/// One visit serves both checks (loading and hydrating the page is most of the time): the page loads at desktop
/// width, then the viewport shrinks to a phone's for the width check. So the phone check sees a resized page, not one
/// loaded at phone width (the shell tests load pages at phone width).
pub struct PageContentTest {
    path: String,
}

#[async_trait]
impl BrowserTest<str> for PageContentTest {
    fn name(&self) -> Cow<'_, str> {
        format!("pages::dark_theme_and_phone_width {}", self.path).into()
    }

    async fn run(&self, driver: &WebDriver, base_url: &str) -> Result<(), Report> {
        let page = BookPage { driver, base_url };
        let path = &self.path;

        // The reader chose the dark theme before, so the server renders the page in it. The desktop layout.
        page.set_theme("dark").await?;
        page.set_viewport(1600, 1000).await?;
        // Hover styles must not count: park the pointer where no page has demos (the app bar's top left corner).
        page.move_pointer_to(0, 0).await?;

        page.goto(path).await?;
        page.wait_until(&format!("{path} shows the dark theme"), IS_DARK)
            .await?;
        page.wait_until(
            &format!("{path} finished its transitions to the dark theme"),
            TRANSITIONS_FINISHED,
        )
        .await?;
        let mut problems = page.strings(DARK_THEME_PROBLEMS).await?;
        let links = page.strings(INTERNAL_LINKS).await?;
        let ids = page.strings(IDS).await?;
        {
            let mut site = SITE_LINKS.lock().unwrap_or_else(PoisonError::into_inner);
            site.links
                .extend(links.into_iter().map(|link| (path.clone(), link)));
            site.ids.insert(path.clone(), ids.into_iter().collect());
        }

        page.set_viewport(PHONE_WIDTH, 844).await?;
        page.wait_until(
            &format!("{path} shows the small-screen layout"),
            SMALL_SCREEN_LAYOUT,
        )
        .await?;
        let width = page
            .number("return document.documentElement.scrollWidth;")
            .await?;
        if width > f64::from(PHONE_WIDTH) + 1.0 {
            let culprits = page.strings(OVERFLOWING).await?;
            problems.push(format!("{width}px wide at {PHONE_WIDTH}px: {culprits:?}"));
        }

        assert_that!(problems).is_empty();
        Ok(())
    }
}

/// Every internal link the [`PageContentTest`]s found resolves to a page, and every fragment to an element id. Runs
/// after all page tests.
pub struct LinkTests {}

#[async_trait]
impl BrowserTest<str> for LinkTests {
    fn name(&self) -> Cow<'_, str> {
        "internal_links_and_anchors_resolve".into()
    }

    async fn run(&self, _driver: &WebDriver, _base_url: &str) -> Result<(), Report> {
        let site = std::mem::take(&mut *SITE_LINKS.lock().unwrap_or_else(PoisonError::into_inner));
        // Links to pages outside `BOOK_TEST_PAGES` (or of a failed page test) can only be checked for existence, not for
        // their fragments.
        let all_pages: BTreeSet<&str> = book_ssr::nav::nav().pages().collect();
        let mut problems = Vec::new();
        for (page_path, link) in &site.links {
            let (target, fragment) = link.split_once('#').unwrap_or((link, ""));
            let target = if target.is_empty() {
                page_path.as_str()
            } else {
                target
            };
            // Markdown exports: the LLM index, or the export of a page.
            let target = match target.strip_suffix(".md") {
                Some("/doc/llm-index") => continue,
                Some(page) => page,
                None => target,
            };
            if !all_pages.contains(target) {
                problems.push(format!("{page_path}: link to {link}: no such page"));
            } else if let Some(target_ids) = site.ids.get(target)
                && !fragment.is_empty()
                && !target_ids.contains(fragment)
            {
                problems.push(format!(
                    "{page_path}: link to {link}: no element with id `{fragment}`"
                ));
            }
        }

        assert_that!(problems)
            .with_detail_message(format!(
                "checked {} links on {} pages",
                site.links.len(),
                site.ids.len()
            ))
            .is_empty();
        Ok(())
    }
}

/// The Markdown export's LLM index lists every page.
pub async fn llm_index_lists_every_page(page: &BookPage<'_>) -> Result<(), Report> {
    page.driver
        .goto(&format!("{}/doc/llm-index.md", page.base_url))
        .await?;
    let index = page.driver.find(By::Tag("body")).await?.text().await?;
    let unlisted: Vec<_> = book_ssr::nav::nav()
        .pages()
        .filter(|path| path.starts_with("/doc/") && !index.contains(&format!("({path}.md)")))
        .collect();
    assert_that!(unlisted)
        .with_detail_message("pages missing from the LLM index")
        .is_empty();
    Ok(())
}

/// Pages are served as Markdown (`<page>.md`), starting with their frontmatter.
pub async fn pages_are_served_as_markdown(page: &BookPage<'_>) -> Result<(), Report> {
    page.driver
        .goto(&format!("{}/doc/button.md", page.base_url))
        .await?;
    let markdown = page.driver.find(By::Tag("body")).await?.text().await?;
    assert_that!(markdown.as_str()).starts_with("---\ntitle: \"Button\"");
    Ok(())
}
