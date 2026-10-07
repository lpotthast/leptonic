//! Checks the book's shell: the documentation search, the small-screen menu and the demo source toggle.

use std::{borrow::Cow, time::Duration};

use assertr::prelude::*;
use browser_test::{
    BrowserTest, async_trait,
    thirtyfour::{By, Key, WebDriver},
};
use leptos_browser_test::{Report, ResultExt};

use crate::pages::BookPage;

/// Whether the search dialog is open.
const SEARCH_OPEN: &str =
    "return !!document.querySelector('[role=dialog][aria-label=\"Search documentation\"]');";

/// Whether the search dialog is closed.
const SEARCH_CLOSED: &str =
    "return !document.querySelector('[role=dialog][aria-label=\"Search documentation\"]');";

/// Whether the search field's input has focus.
const INPUT_FOCUSED: &str =
    "return !!document.activeElement && document.activeElement.matches('.doc-search-field input');";

/// Whether results are listed.
const HAS_RESULTS: &str = "return document.querySelectorAll('.doc-search-result').length > 0;";

/// The search: the app bar button opens it with focus in the field, typing lists results, Escape first empties the
/// field and then closes the search (focus returns to the button), and Enter opens the first result.
pub struct SearchTests {}

#[async_trait]
impl BrowserTest<str> for SearchTests {
    fn name(&self) -> Cow<'_, str> {
        "search_lists_results_clears_closes_and_opens_the_first".into()
    }

    async fn run(&self, driver: &WebDriver, base_url: &str) -> Result<(), Report> {
        let page = BookPage { driver, base_url };

        // The desktop layout, where the app bar shows the search button with its text.
        page.set_viewport(1600, 1000).await?;
        page.goto("/doc/overview").await?;

        // Open the search; focus moves into the field.
        driver
            .find(By::Css(".doc-search-trigger"))
            .await
            .context("the app bar has a search button")?
            .click()
            .await?;
        page.wait_until("the search opens", SEARCH_OPEN).await?;
        page.wait_until("the search field has focus", INPUT_FOCUSED)
            .await?;

        let input = driver
            .find(By::Css(".doc-search-field input"))
            .await
            .context("the search has a field")?;
        input.send_keys("button").await?;
        // The first search waits until the server has converted every page to Markdown, which takes a while right
        // after the server started, under the load of the parallel tests.
        page.wait_until_within("results are listed", HAS_RESULTS, Duration::from_secs(60))
            .await?;

        // Snippets are plain text with the query highlighted: no Markdown, no frontmatter.
        let snippets = page
            .strings("return [...document.querySelectorAll('.doc-search-result-snippet')].map(s => s.textContent);")
            .await?;
        for snippet in &snippets {
            for markup in ["**", "## ", "](", "title: ", "kind: "] {
                assert_that!(snippet.as_str())
                    .with_detail_message(format!("a snippet contains `{markup}`"))
                    .does_not_contain(markup);
            }
        }
        let marks = page
            .number("return document.querySelectorAll('.doc-search-result mark').length;")
            .await?;
        assert_that!(marks)
            .with_detail_message("results highlight the query")
            .is_greater_than(0.0);

        // The first Escape empties the field and keeps the search open.
        input.send_keys(Key::Escape).await?;
        page.wait_until(
            "Escape empties the field",
            "return document.querySelector('.doc-search-field input').value === '';",
        )
        .await?;
        let open_dialogs = page
            .number("return document.querySelectorAll('[role=dialog][aria-label=\"Search documentation\"]').length;")
            .await?;
        assert_that!(open_dialogs)
            .with_detail_message("the first Escape only empties the field")
            .is_equal_to(1.0);

        // The second Escape closes the search, and focus returns to the button.
        input.send_keys(Key::Escape).await?;
        page.wait_until("a second Escape closes the search", SEARCH_CLOSED)
            .await?;
        page.wait_until(
            "focus returns to the search button",
            "return !!document.activeElement && document.activeElement.matches('.doc-search-trigger');",
        )
        .await?;

        // Enter opens the first result.
        driver
            .find(By::Css(".doc-search-trigger"))
            .await?
            .click()
            .await?;
        page.wait_until("the search opens again", SEARCH_OPEN)
            .await?;
        page.wait_until("the search field has focus again", INPUT_FOCUSED)
            .await?;
        let input = driver.find(By::Css(".doc-search-field input")).await?;
        input.send_keys("modal").await?;
        page.wait_until("results for \u{201c}modal\u{201d} are listed", HAS_RESULTS)
            .await?;
        let first = page
            .strings("return [document.querySelector('.doc-search-result').getAttribute('href')];")
            .await?;
        let first = first.first().cloned().unwrap_or_default();
        assert_that!(first.as_str()).starts_with("/doc/");
        input.send_keys(Key::Enter).await?;
        page.wait_until(
            &format!("Enter opens the first result, {first}"),
            &format!("return location.pathname + location.hash === '{first}';"),
        )
        .await?;
        page.wait_until("the search closes after navigating", SEARCH_CLOSED)
            .await?;
        Ok(())
    }
}

/// Whether the small-screen documentation menu is open.
const DOC_MENU_OPEN: &str =
    "return !!document.querySelector('[role=dialog][aria-label=\"Documentation\"]');";

/// The documentation menu at phone width: the app bar's menu button opens it as a dialog, Escape closes it again
/// (focus returns to the button), without page errors.
pub struct DocMenuTests {}

#[async_trait]
impl BrowserTest<str> for DocMenuTests {
    fn name(&self) -> Cow<'_, str> {
        "doc_menu_opens_at_phone_width_and_closes_on_escape".into()
    }

    async fn run(&self, driver: &WebDriver, base_url: &str) -> Result<(), Report> {
        let page = BookPage { driver, base_url };
        page.set_viewport(390, 844).await?;
        page.goto("/doc/overview").await?;

        let button = driver
            .find(By::Css("[aria-label=\"Documentation menu\"]"))
            .await
            .context("the app bar has a documentation menu button")?;
        assert_that!(button.attr("aria-expanded").await?).is_equal_to(Some("false".to_owned()));
        button.click().await?;
        page.wait_until("the documentation menu opens", DOC_MENU_OPEN)
            .await?;
        page.wait_until(
            "focus moves into the menu",
            "return !!document.activeElement && !!document.activeElement.closest('[role=dialog][aria-label=\"Documentation\"]');",
        )
        .await?;

        driver
            .active_element()
            .await?
            .send_keys(Key::Escape)
            .await?;
        page.wait_until(
            "Escape closes the documentation menu",
            "return !document.querySelector('[role=dialog][aria-label=\"Documentation\"]');",
        )
        .await?;
        page.wait_until(
            "focus returns to the menu button",
            "return !!document.activeElement && document.activeElement.matches('[aria-label=\"Documentation menu\"]');",
        )
        .await?;
        assert_that!(page.page_errors().await?).is_empty();
        Ok(())
    }
}

/// The search shortcut: Ctrl+K (Cmd+K on macOS) opens the search with focus in its field.
pub struct SearchShortcutTests {}

#[async_trait]
impl BrowserTest<str> for SearchShortcutTests {
    fn name(&self) -> Cow<'_, str> {
        "search_opens_with_ctrl_k".into()
    }

    async fn run(&self, driver: &WebDriver, base_url: &str) -> Result<(), Report> {
        let page = BookPage { driver, base_url };
        page.set_viewport(1600, 1000).await?;
        page.goto("/doc/overview").await?;

        driver
            .action_chain()
            .key_down(Key::Control)
            .send_keys("k")
            .key_up(Key::Control)
            .perform()
            .await?;
        page.wait_until("Ctrl+K opens the search", SEARCH_OPEN)
            .await?;
        page.wait_until("the search field has focus", INPUT_FOCUSED)
            .await?;
        Ok(())
    }
}

/// A demo's "View source" disclosure: collapsed by default (`aria-expanded="false"`, the code hidden), pressing it
/// expands it and shows the code.
pub struct DemoSourceTests {}

#[async_trait]
impl BrowserTest<str> for DemoSourceTests {
    fn name(&self) -> Cow<'_, str> {
        "demo_view_source_toggles_the_code".into()
    }

    async fn run(&self, driver: &WebDriver, base_url: &str) -> Result<(), Report> {
        let page = BookPage { driver, base_url };
        page.set_viewport(1600, 1000).await?;
        // A page whose demo starts with its source collapsed.
        page.goto("/doc/tooltip/atom").await?;

        let trigger = driver
            .find(By::XPath(
                "//*[contains(@class,'doc-demo')]//button[contains(@class,'doc-disclosure-trigger')][normalize-space(.)='View source']",
            ))
            .await
            .context("the demo has a \u{201c}View source\u{201d} button")?;
        assert_that!(trigger.attr("aria-expanded").await?).is_equal_to(Some("false".to_owned()));
        let panel_id = trigger.attr("aria-controls").await?.unwrap_or_default();
        assert_that!(panel_id.as_str()).is_not_empty();
        let panel = driver.find(By::Id(&panel_id)).await?;
        assert_that!(panel.is_displayed().await?)
            .with_detail_message("the code is hidden while collapsed")
            .is_false();

        // Near the top of the viewport, below the sticky app bar (instantly: the book scrolls smoothly).
        driver
            .execute(
                "arguments[0].scrollIntoView({block: 'start', behavior: 'instant'}); window.scrollBy({top: -200, behavior: 'instant'});",
                vec![trigger.to_json()?],
            )
            .await?;
        trigger.click().await?;
        page.wait_until(
            "pressing \u{201c}View source\u{201d} expands it",
            "return document.querySelector('.doc-demo .doc-disclosure-trigger').getAttribute('aria-expanded') === 'true';",
        )
        .await?;
        assert_that!(panel.is_displayed().await?)
            .with_detail_message("the code is shown while expanded")
            .is_true();
        assert_that!(panel.text().await?).contains("TooltipTrigger");
        Ok(())
    }
}

/// The page structure: a title per page, a description, landmarks (the app bar in a `<header>`, the navigation and the
/// table of contents outside `<main>`), headings for the sidebar parts, distinct names for group toggles, and the skip
/// link as the first focusable element, moving focus to the page content.
pub struct ShellStructureTests {}

#[async_trait]
impl BrowserTest<str> for ShellStructureTests {
    fn name(&self) -> Cow<'_, str> {
        "shell_has_titles_landmarks_and_a_skip_link".into()
    }

    async fn run(&self, driver: &WebDriver, base_url: &str) -> Result<(), Report> {
        let page = BookPage { driver, base_url };
        page.set_viewport(1600, 1000).await?;
        page.goto("/doc/button").await?;

        page.wait_until(
            "the page has its own title",
            "return document.title === 'Button \u{2013} Leptonic';",
        )
        .await?;
        let description = page
            .strings("return [...document.querySelectorAll('meta[name=description]')].map(m => m.content);")
            .await?;
        assert_that!(description.len())
            .with_detail_message("one description")
            .is_equal_to(1);
        assert_that!(description[0].as_str()).starts_with("Triggers an action");

        let structure = page
            .strings(
                "return [
                    String(document.querySelectorAll('main').length),
                    document.querySelector('main').id,
                    String(!!document.querySelector('header #book-app-bar, header#book-app-bar')),
                    String(!!document.querySelector('main .book-doc-nav, main #book-toc')),
                    String(!!document.querySelector('main .doc-concept-tabs')),
                    [...document.querySelectorAll('#book-doc-sidebar nav h2')].map(h => h.textContent).join(', '),
                    document.querySelector('.doc-search-trigger').getAttribute('aria-label'),
                    document.querySelector('.doc-search-trigger').getAttribute('aria-keyshortcuts'),
                ];",
            )
            .await?;
        assert_that!(structure).is_equal_to(
            [
                "1",
                "book-main",
                "true",
                "false",
                "true",
                "Concepts, Building blocks",
                "Search docs",
                "Control+K",
            ]
            .map(str::to_owned)
            .to_vec(),
        );

        // No two controls of the sidebar share a name: a group's toggle is not named like its overview link.
        let duplicates = page
            .strings(
                "const names = [...document.querySelectorAll('#book-doc-sidebar :is(a, button)')]
                    .map(e => (e.getAttribute('aria-label') || e.textContent).trim());
                 return names.filter((name, i) => names.indexOf(name) !== i);",
            )
            .await?;
        assert_that!(duplicates).is_empty();

        // The skip link is the first stop of Tab, and moves focus into the page content.
        driver.action_chain().send_keys(Key::Tab).perform().await?;
        page.wait_until(
            "Tab focuses the skip link first, which shows",
            "const a = document.activeElement; return !!a && a.textContent.trim() === 'Skip to content' \
             && a.getBoundingClientRect().width > 20;",
        )
        .await?;
        driver.active_element().await?.send_keys(Key::Enter).await?;
        page.wait_until(
            "the skip link moves focus to the page content",
            "return document.activeElement && document.activeElement.id === 'book-main';",
        )
        .await?;

        assert_that!(page.page_errors().await?).is_empty();
        Ok(())
    }
}

/// The shell on a phone: the "Copy as Markdown" button doesn't cover the page title, the concept tabs stay on one row
/// and anchored headings stay below them, the main menu leads to the docs. On a tablet, the welcome page's cards don't
/// leave one alone in a row.
pub struct NarrowShellTests {}

#[async_trait]
impl BrowserTest<str> for NarrowShellTests {
    fn name(&self) -> Cow<'_, str> {
        "shell_fits_narrow_screens".into()
    }

    async fn run(&self, driver: &WebDriver, base_url: &str) -> Result<(), Report> {
        let page = BookPage { driver, base_url };
        page.set_viewport(390, 844).await?;

        page.goto("/doc/focus/use-has-tabbable-child").await?;
        let overlap = page
            .number(
                "const a = document.querySelector('.doc-article h1').getBoundingClientRect();
                 const b = document.querySelector('.doc-copy-markdown').getBoundingClientRect();
                 return Math.max(0, Math.min(a.right, b.right) - Math.max(a.left, b.left))
                      * Math.max(0, Math.min(a.bottom, b.bottom) - Math.max(a.top, b.top));",
            )
            .await?;
        assert_that!(overlap)
            .with_detail_message("the copy button doesn't cover the title")
            .is_equal_to(0.0);

        page.goto("/doc/button/atom").await?;
        let rows = page
            .number(
                "return new Set([...document.querySelectorAll('.doc-concept-tab')].map(t => t.offsetTop)).size;",
            )
            .await?;
        assert_that!(rows)
            .with_detail_message("the concept tabs are one row")
            .is_equal_to(1.0);
        driver
            .execute(
                "const h = [...document.querySelectorAll('.doc-article h2[id]')].pop();
                 h.style.scrollBehavior = 'auto';
                 h.scrollIntoView({block: 'start', behavior: 'instant'});",
                vec![],
            )
            .await?;
        page.wait_until(
            "an anchored heading stays below the concept tabs",
            "const h = [...document.querySelectorAll('.doc-article h2[id]')].pop().getBoundingClientRect();
             const tabs = document.querySelector('.doc-concept-tabs').getBoundingClientRect();
             return h.top >= tabs.bottom - 1;",
        )
        .await?;

        page.goto("/").await?;
        driver
            .find(By::Css("[aria-label=\"Menu\"]"))
            .await
            .context("the app bar has a menu button")?
            .click()
            .await?;
        page.wait_until(
            "the main menu links the docs",
            "return !!document.querySelector('[role=dialog] .book-main-menu a[href=\"/doc\"]');",
        )
        .await?;

        page.set_viewport(700, 900).await?;
        page.goto("/").await?;
        let columns = page
            .strings(
                "return [...document.querySelectorAll('.book-welcome-cards')].map(list =>
                    String(new Set([...list.children].map(card => card.offsetLeft)).size));",
            )
            .await?;
        for count in columns {
            assert_that!(count == "1" || count == "3")
                .with_detail_message(format!(
                    "welcome cards: one per row or all in one row, not {count} columns"
                ))
                .is_true();
        }
        assert_that!(page.page_errors().await?).is_empty();
        Ok(())
    }
}
