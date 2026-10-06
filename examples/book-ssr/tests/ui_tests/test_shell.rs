//! Checks the book's shell: the documentation search, the small-screen menu and the demo source toggle.

use std::borrow::Cow;

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
        page.wait_until("results are listed", HAS_RESULTS).await?;

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
            &format!("return location.pathname === '{first}';"),
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
                "//*[contains(@class,'demo-shell')]//button[contains(@class,'doc-disclosure-trigger')][normalize-space(.)='View source']",
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
            "return document.querySelector('.demo-shell .doc-disclosure-trigger').getAttribute('aria-expanded') === 'true';",
        )
        .await?;
        assert_that!(panel.is_displayed().await?)
            .with_detail_message("the code is shown while expanded")
            .is_true();
        assert_that!(panel.text().await?).contains("TooltipTrigger");
        Ok(())
    }
}
