// Upstream: react-aria-components/test/Link.test.js @ 99e6102368
use std::borrow::Cow;

use assertr::prelude::*;
use browser_test::{BrowserTest, async_trait, thirtyfour::WebDriver};
use rootcause::Report;

use crate::pages::{BaseActions, Page};

/// The link atom: `aria-current` by route (prefix or exact), client-side navigation, `replace`,
/// `target`/`rel` for new tabs, a disabled link (no `href`, `aria-disabled`, not followed, no
/// presses), and the props of a surrounding trigger.
pub struct LinkTests {}

#[async_trait]
impl BrowserTest<str> for LinkTests {
    fn name(&self) -> Cow<'_, str> {
        "link_tests".into()
    }

    async fn run(&self, driver: &WebDriver, base_url: &str) -> Result<(), Report> {
        let page = Page { driver, base_url };
        page.goto_path("/atoms/link").await?;

        // The current page by route.
        assert_that!(page.attr_of("test-link-self", "aria-current").await?)
            .is_equal_to(Some("page".to_owned()));
        assert_that!(page.attr_of("test-link-prefix", "aria-current").await?)
            .is_equal_to(Some("page".to_owned()));
        assert_that!(page.attr_of("test-link-exact", "aria-current").await?).is_none();

        // A new tab gets `noopener`.
        assert_that!(page.attr_of("test-link-external", "target").await?)
            .is_equal_to(Some("_blank".to_owned()));
        assert_that!(page.attr_of("test-link-external", "rel").await?)
            .is_equal_to(Some("noopener".to_owned()));

        // A link as a trigger gets the trigger's props.
        // react-aria's menu triggers: `aria-haspopup="true"`.
        assert_that!(page.attr_of("test-link-menu", "aria-haspopup").await?)
            .is_equal_to(Some("true".to_owned()));
        assert_that!(page.attr_of("test-link-menu", "aria-expanded").await?)
            .is_equal_to(Some("false".to_owned()));

        // "should support disabled state", "should not navigate if disabled".
        page.css(".test-link-disableable").await?.click().await?;
        page.wait_for_text("test-link-presses", "1").await?;
        page.click_element_with_id("test-link-toggle-disabled")
            .await?;
        page.wait_for_selector(
            "span.test-link-disableable[role=link][aria-disabled=true][data-disabled]",
        )
        .await?;
        assert_that!(
            page.css(".test-link-disableable")
                .await?
                .attr("href")
                .await?
        )
        .is_none();
        page.css(".test-link-disableable").await?.click().await?;
        assert_that!(page.read_text_of("test-link-presses").await?).is_equal_to("1".to_owned());
        page.click_element_with_id("test-link-toggle-disabled")
            .await?;
        page.wait_for_selector("a.test-link-disableable[href]")
            .await?;

        // `replace`: no history entry.
        let history_length = || async {
            let len: u64 = page
                .driver
                .execute("return history.length;", vec![])
                .await?
                .convert()?;
            Ok::<u64, Report>(len)
        };
        let before = history_length().await?;
        page.click_element_with_id("test-link-replace").await?;
        page.wait_for_selector("body[data-hydrated]").await?;
        let search: String = page
            .driver
            .execute("return location.search;", vec![])
            .await?
            .convert()?;
        assert_that!(search).is_equal_to("?replaced".to_owned());
        assert_that!(history_length().await?).is_equal_to(before);

        // Client-side navigation: the page isn't reloaded.
        page.driver
            .execute("window.__testLinkNoReload = true;", vec![])
            .await?;
        page.click_element_with_id("test-link-toolbar").await?;
        page.wait_for_selector("#test-page-atom-toolbar").await?;
        let kept: bool = page
            .driver
            .execute("return window.__testLinkNoReload === true;", vec![])
            .await?
            .convert()?;
        assert_that!(kept)
            .with_detail_message("the link navigated on the client")
            .is_true();

        page.expect_no_page_errors().await
    }
}
