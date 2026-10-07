// Upstream: react-aria-components/test/Separator.test.js @ 99e6102368
use std::borrow::Cow;

use assertr::prelude::*;
use browser_test::{BrowserTest, async_trait, thirtyfour::WebDriver};
use rootcause::Report;

use crate::pages::{BaseActions, Page};

/// The `Separator` atom: an `<hr>` with the default class, ARIA props, and a `<div
/// role="separator">` while vertical (switching with the orientation).
pub struct SeparatorTests {}

#[async_trait]
impl BrowserTest<str> for SeparatorTests {
    fn name(&self) -> Cow<'_, str> {
        "separator_tests".into()
    }

    async fn run(&self, driver: &WebDriver, base_url: &str) -> Result<(), Report> {
        let page = Page { driver, base_url };
        page.goto_path("/atoms/separator").await?;

        // "should render a separator with default class".
        let plain = page.css(".test-sep-plain").await?;
        assert_that!(plain.tag_name().await?).is_equal_to("hr".to_owned());
        assert_that!(plain.attr("class").await?)
            .is_equal_to(Some("leptonic-Separator test-sep-plain".to_owned()));
        assert_that!(plain.attr("role").await?).is_none();

        // "should support accessibility props".
        let labelled = page.css(".test-sep-labelled").await?;
        assert_that!(labelled.attr("aria-label").await?).is_equal_to(Some("label".to_owned()));
        let by = page.element("test-sep-by").await?;
        assert_that!(by.attr("aria-labelledby").await?)
            .is_equal_to(Some("test-sep-heading".to_owned()));

        // Vertical: a `<div role="separator" aria-orientation="vertical">`; horizontal again: an
        // `<hr>`.
        let switching = page.css(".test-sep-switching").await?;
        assert_that!(switching.tag_name().await?).is_equal_to("div".to_owned());
        assert_that!(switching.attr("role").await?).is_equal_to(Some("separator".to_owned()));
        assert_that!(switching.attr("aria-orientation").await?)
            .is_equal_to(Some("vertical".to_owned()));
        page.click_element_with_id("test-sep-toggle").await?;
        page.wait_for_selector("hr.test-sep-switching").await?;
        let switched = page.css(".test-sep-switching").await?;
        assert_that!(switched.attr("aria-orientation").await?).is_none();
        page.click_element_with_id("test-sep-toggle").await?;
        page.wait_for_selector("div.test-sep-switching[role=separator][aria-orientation=vertical]")
            .await
    }
}
