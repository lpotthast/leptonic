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
use leptos_browser_test::{Report, ResultExt};

use crate::pages::BookPage;

/// Whether the dark theme applies: the body background is dark.
pub const IS_DARK: &str = "const c = getComputedStyle(document.body).backgroundColor.match(/\\d+/g); \
                       return !!c && (+c[0] + +c[1] + +c[2]) / 3 < 80;";

/// Whether all CSS transitions finished. The server renders the light theme (the theme choice lives in the browser's
/// local storage), and hydration switches to the dark one: until their transitions finish, elements that transition
/// their colors (buttons, demo targets) still show light-theme colors.
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

/// Elements of the page content wider than the viewport, outside of scroll containers.
const OVERFLOWING: &str = r"
    const unclipped = e => { for (let p = e.parentElement; p; p = p.parentElement) if (getComputedStyle(p).overflowX !== 'visible') return false; return true; };
    return [...document.querySelectorAll('main *')]
        .filter(e => e.getBoundingClientRect().right > window.innerWidth + 1 && unclipped(e))
        .slice(0, 3)
        .map(e => e.tagName.toLowerCase() + '.' + e.className + ': ' + e.textContent.trim().slice(0, 40));";

/// A part of the pages: every `count`-th page, starting at `index`. The page walks are split into shards that run in
/// parallel, as visiting (and hydrating) every page one after the other takes minutes.
#[derive(Debug, Clone, Copy)]
pub struct Shard {
    pub index: usize,
    pub count: usize,
}

impl Shard {
    /// All shards of a walk split into `count` parts.
    pub fn all(count: usize) -> impl Iterator<Item = Shard> {
        (0..count).map(move |index| Shard { index, count })
    }

    /// This shard's pages.
    fn paths(self) -> Vec<String> {
        BookPage::paths()
            .into_iter()
            .skip(self.index)
            .step_by(self.count)
            .collect()
    }

    fn label(self) -> String {
        format!("{}/{}", self.index + 1, self.count)
    }
}

/// The internal links and element ids the dark-theme shards found, for [`LinkTests`] (after all shards).
#[derive(Default)]
struct SiteLinks {
    /// `(page, link)` pairs.
    links: Vec<(String, String)>,
    /// The element ids of every visited page.
    ids: HashMap<String, BTreeSet<String>>,
}

static SITE_LINKS: LazyLock<Mutex<SiteLinks>> = LazyLock::new(Mutex::default);

/// Visits a shard of the pages in the dark theme: no page errors, readable demos. Records the pages' internal links
/// and ids for [`LinkTests`].
pub struct PageContentTests {
    pub shard: Shard,
}

#[async_trait]
impl BrowserTest<str> for PageContentTests {
    fn name(&self) -> Cow<'_, str> {
        format!(
            "pages_load_cleanly_in_the_dark_theme ({})",
            self.shard.label()
        )
        .into()
    }

    async fn run(&self, driver: &WebDriver, base_url: &str) -> Result<(), Report> {
        let page = BookPage { driver, base_url };
        let paths = self.shard.paths();

        // The desktop layout, where the app bar shows the theme toggle.
        page.set_viewport(1600, 1000).await?;

        // Switch to the dark theme as a user would; it is remembered for all following pages.
        page.goto("/doc/overview").await?;
        driver
            .find(By::Css("#book-app-bar .book-theme-toggle"))
            .await
            .context("the app bar has a theme toggle")?
            .click()
            .await?;
        page.wait_until("the dark theme applies", IS_DARK).await?;
        // Hover styles must not count: park the pointer where no page has demos (the app bar's top left corner).
        page.move_pointer_to(0, 0).await?;

        let mut problems = Vec::new();
        for path in &paths {
            page.goto(path).await?;
            page.wait_until(&format!("{path} shows the dark theme"), IS_DARK)
                .await?;
            page.wait_until(
                &format!("{path} finished its transitions to the dark theme"),
                TRANSITIONS_FINISHED,
            )
            .await?;
            for error in page.page_errors().await? {
                problems.push(format!("{path}: page error: {error}"));
            }
            for problem in page.strings(DARK_THEME_PROBLEMS).await? {
                problems.push(format!("{path}: {problem}"));
            }
            let links = page.strings(INTERNAL_LINKS).await?;
            let ids = page.strings(IDS).await?;
            let mut site = SITE_LINKS.lock().unwrap_or_else(PoisonError::into_inner);
            site.links
                .extend(links.into_iter().map(|link| (path.clone(), link)));
            site.ids.insert(path.clone(), ids.into_iter().collect());
        }

        assert_that!(problems)
            .with_detail_message(format!("checked {} pages", paths.len()))
            .is_empty();
        Ok(())
    }
}

/// Every internal link the [`PageContentTests`] shards found resolves to a page, and every fragment to an element id.
/// Runs after all shards.
pub struct LinkTests {}

#[async_trait]
impl BrowserTest<str> for LinkTests {
    fn name(&self) -> Cow<'_, str> {
        "internal_links_and_anchors_resolve".into()
    }

    async fn run(&self, _driver: &WebDriver, _base_url: &str) -> Result<(), Report> {
        let site = std::mem::take(&mut *SITE_LINKS.lock().unwrap_or_else(PoisonError::into_inner));
        // Links to pages outside `BOOK_TEST_PAGES` (or of a failed shard) can only be checked for existence, not for
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

/// Visits a shard of the pages on a phone-sized viewport: nothing makes the page wider than the screen.
pub struct NarrowScreenTests {
    pub shard: Shard,
}

const PHONE_WIDTH: u32 = 390;

#[async_trait]
impl BrowserTest<str> for NarrowScreenTests {
    fn name(&self) -> Cow<'_, str> {
        format!("pages_fit_a_phone_screen ({})", self.shard.label()).into()
    }

    async fn run(&self, driver: &WebDriver, base_url: &str) -> Result<(), Report> {
        let page = BookPage { driver, base_url };
        let paths = self.shard.paths();

        page.set_viewport(PHONE_WIDTH, 844).await?;

        let mut problems = Vec::new();
        for path in &paths {
            page.goto(path).await?;
            let width = page
                .number("return document.documentElement.scrollWidth;")
                .await?;
            if width > f64::from(PHONE_WIDTH) + 1.0 {
                let culprits = page.strings(OVERFLOWING).await?;
                problems.push(format!("{path}: {width}px wide: {culprits:?}"));
            }
        }

        assert_that!(problems)
            .with_detail_message(format!("checked {} pages at {PHONE_WIDTH}px", paths.len()))
            .is_empty();
        Ok(())
    }
}

/// The Markdown export: the LLM index lists every page, and pages are served as Markdown with frontmatter.
pub struct MarkdownExportTests {}

#[async_trait]
impl BrowserTest<str> for MarkdownExportTests {
    fn name(&self) -> Cow<'_, str> {
        "markdown_export_lists_and_serves_every_page".into()
    }

    async fn run(&self, driver: &WebDriver, base_url: &str) -> Result<(), Report> {
        driver.goto(&format!("{base_url}/doc/llm-index.md")).await?;
        let index = driver.find(By::Tag("body")).await?.text().await?;
        let unlisted: Vec<_> = book_ssr::nav::nav()
            .pages()
            .filter(|path| path.starts_with("/doc/") && !index.contains(&format!("({path}.md)")))
            .collect();
        assert_that!(unlisted)
            .with_detail_message("pages missing from the LLM index")
            .is_empty();

        driver.goto(&format!("{base_url}/doc/button.md")).await?;
        let markdown = driver.find(By::Tag("body")).await?.text().await?;
        assert_that!(markdown.as_str()).starts_with("---\ntitle: \"Button\"");
        Ok(())
    }
}
