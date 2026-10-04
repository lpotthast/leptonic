use std::borrow::Cow;

use browser_test::{BrowserTest, async_trait};
use rootcause::Report;

use assertr::prelude::*;
use browser_test::thirtyfour::WebDriver;

use crate::pages::button::ButtonPage;

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

        // Test: disabled button doesn't increment counter
        assert_that!(page.read_disabled_count().await?).is_equal_to(0);
        page.click_disabled_button().await?;
        assert_that!(page.read_disabled_count().await?).is_equal_to(0);

        Ok(())
    }
}
