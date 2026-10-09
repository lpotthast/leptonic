//! Checks the contrast of the book's chrome (sidebar, concept tabs, table of contents, links in the article, demo
//! disclosures, app bar) in the light and the dark theme: text meets WCAG AA (4.5:1, 3:1 for large text). The unit
//! test `contrast_check` checks the token values themselves.

use std::borrow::Cow;

use assertr::prelude::*;
use browser_test::{BrowserTest, async_trait, thirtyfour::WebDriver};
use leptos_browser_test::Report;

use super::test_pages::{IS_DARK, TRANSITIONS_FINISHED};
use crate::pages::BookPage;

/// Pages showing every part of the chrome: concept tabs and the current sidebar entry, building-block badges, demo
/// disclosures, the welcome page's cards.
const PAGES: [&str; 4] = [
    "/doc/button",
    "/doc/interactions/use-press",
    "/doc/tooltip/atom",
    "/",
];

/// Visible chrome text below 4.5:1 (3:1 for large text) against its background, which is composed from the
/// backgrounds of the element and its ancestors. Demo content (`.demo`) is left to the demo checks.
const CONTRAST_PROBLEMS: &str = r"
    const SELECTORS = ['.book-nav-item', '.book-badge', '.book-layer-marks span', '.book-nav-part-title',
        '.book-nav-group-title', 'a.book-nav-group-header', '.doc-concept-name', '.doc-concept-tab', '.doc-article p',
        '.doc-article li', '.doc-article a.doc-link', '.doc-disclosure-trigger', '#book-toc .doc-toc-link',
        '#book-toc h2', '#book-app-bar a', '.doc-search-trigger-text', '.book-welcome-card a',
        '.book-welcome-card p', '.book-welcome-tagline'];
    const parse = c => { const m = c.match(/rgba?\(([^)]+)\)/); if (!m) return null;
        const p = m[1].split(/[\s,\/]+/).filter(Boolean).map(Number); return [p[0], p[1], p[2], p.length > 3 ? p[3] : 1]; };
    const over = (top, bottom) => [0, 1, 2].map(i => top[i] * top[3] + bottom[i] * (1 - top[3])).concat([1]);
    const background = el => { const layers = [];
        for (let e = el; e; e = e.parentElement) { const c = parse(getComputedStyle(e).backgroundColor);
            if (c && c[3] > 0) { layers.push(c); if (c[3] >= 1) break; } }
        return layers.reverse().reduce((color, layer) => over(layer, color), [255, 255, 255, 1]); };
    const lum = c => { const f = v => { v /= 255; return v <= 0.04045 ? v / 12.92 : ((v + 0.055) / 1.055) ** 2.4; };
        return 0.2126 * f(c[0]) + 0.7152 * f(c[1]) + 0.0722 * f(c[2]); };
    const ratio = (a, b) => { const [x, y] = [lum(a), lum(b)].sort((p, q) => q - p); return (x + 0.05) / (y + 0.05); };
    const out = [];
    for (const selector of SELECTORS) {
        for (const el of document.querySelectorAll(selector)) {
            const cs = getComputedStyle(el);
            // `checkVisibility` also skips content hidden until found (collapsed sidebar groups), whose styles Chrome doesn't update.
            if (el.closest('.demo') || !el.textContent.trim() || !el.checkVisibility() || cs.visibility === 'hidden') continue;
            const bg = background(el);
            const fg = over(parse(cs.color), bg);
            const size = parseFloat(cs.fontSize);
            const large = size >= 24 || (size >= 18.66 && +cs.fontWeight >= 700);
            const r = ratio(fg, bg);
            if (r < (large ? 3 : 4.5)) out.push(`${selector} “${el.textContent.trim().slice(0, 30)}”: ${r.toFixed(2)} (${cs.color} on rgb(${bg.slice(0, 3).map(Math.round).join(', ')}))`);
        }
    }
    return [...new Set(out)];";

/// A test per theme and page of [`PAGES`].
pub fn contrast_tests() -> impl Iterator<Item = ContrastTest> {
    ["light", "dark"]
        .into_iter()
        .flat_map(|theme| PAGES.map(|path| ContrastTest { theme, path }))
}

/// The chrome of a page in a theme.
pub struct ContrastTest {
    theme: &'static str,
    path: &'static str,
}

#[async_trait]
impl BrowserTest<str> for ContrastTest {
    fn name(&self) -> Cow<'_, str> {
        format!(
            "contrast::chrome_text_meets_wcag_aa {} {}",
            self.theme, self.path
        )
        .into()
    }

    async fn run(&self, driver: &WebDriver, base_url: &str) -> Result<(), Report> {
        let page = BookPage { driver, base_url };
        let path = self.path;
        // The reader chose the theme before, so the server renders the page in it.
        page.set_theme(self.theme).await?;
        page.set_viewport(1600, 1000).await?;
        // Hover styles must not count: park the pointer where no page has content (the app bar's top left corner).
        page.move_pointer_to(0, 0).await?;

        page.goto(path).await?;
        if self.theme == "dark" {
            page.wait_until(&format!("{path} shows the dark theme"), IS_DARK)
                .await?;
        }
        page.wait_until(
            &format!("{path} finished its transitions"),
            TRANSITIONS_FINISHED,
        )
        .await?;
        let problems = page.strings(CONTRAST_PROBLEMS).await?;
        assert_that!(problems).is_empty();
        Ok(())
    }
}
