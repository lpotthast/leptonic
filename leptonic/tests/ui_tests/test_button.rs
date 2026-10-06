use std::borrow::Cow;

use assertr::prelude::*;
use browser_test::{BrowserTest, async_trait, thirtyfour::WebDriver};
use rootcause::Report;

use crate::pages::{BaseActions, button::ButtonPage};

pub struct ButtonTests {}

#[async_trait]
impl BrowserTest<str> for ButtonTests {
    fn name(&self) -> Cow<'_, str> {
        "button_tests".into()
    }

    async fn run(&self, driver: &WebDriver, base_url: &str) -> Result<(), Report> {
        let page = ButtonPage { driver, base_url };
        page.goto().await?;

        // Test: basic button click increments counter
        assert_that!(page.read_basic_count().await?).is_equal_to(0);
        page.click_basic_button().await?;
        assert_that!(page.read_basic_count().await?).is_equal_to(1);
        page.click_basic_button().await?;
        assert_that!(page.read_basic_count().await?).is_equal_to(2);
        // The pressed button is focused (`data-focused`).
        page.wait_for_selector("#test-button-basic[data-focused=true]")
            .await?;

        // Test: disabled button doesn't increment counter
        assert_that!(page.read_disabled_count().await?).is_equal_to(0);
        page.click_disabled_button().await?;
        assert_that!(page.read_disabled_count().await?).is_equal_to(0);

        // State as data attributes: disabled.
        let disabled = page.css("#test-button-disabled").await?;
        assert_that!(disabled.attr("data-disabled").await?).is_equal_to(Some("true".to_owned()));
        let basic = page.css("#test-button-basic").await?;
        assert_that!(basic.attr("data-disabled").await?).is_none();

        // ARIA props reach the button element.
        let labelled = page.css("#test-button-labelled").await?;
        assert_that!(labelled.attr("aria-label").await?).is_equal_to(Some("Page 2".to_owned()));
        assert_that!(labelled.attr("aria-current").await?).is_equal_to(Some("page".to_owned()));

        Ok(())
    }
}
