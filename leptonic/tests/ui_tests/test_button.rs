use std::time::Duration;

use assertr::prelude::*;
use thirtyfour::{TimeoutConfiguration, WebDriver};

use crate::{pages::button::ButtonPage, ui_tests::UiTest};

pub struct ButtonTests {}

#[async_trait::async_trait]
impl UiTest for ButtonTests {
    fn name(&self) -> String {
        "button_tests".to_string()
    }

    async fn run(&self, driver: &WebDriver, base_url: &str) -> anyhow::Result<()> {
        let mut timeouts = TimeoutConfiguration::default();
        timeouts.set_implicit(Some(Duration::from_secs(3)));
        driver.update_timeouts(timeouts).await?;

        let page = ButtonPage { driver, base_url };
        page.goto().await?;

        // Test: basic button click increments counter
        assert_that(page.read_basic_count().await?).is_equal_to(0);
        page.click_basic_button().await?;
        assert_that(page.read_basic_count().await?).is_equal_to(1);
        page.click_basic_button().await?;
        assert_that(page.read_basic_count().await?).is_equal_to(2);

        // Test: disabled button doesn't increment counter
        assert_that(page.read_disabled_count().await?).is_equal_to(0);
        page.click_disabled_button().await?;
        assert_that(page.read_disabled_count().await?).is_equal_to(0);

        Ok(())
    }
}
