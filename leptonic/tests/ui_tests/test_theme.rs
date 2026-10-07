// No upstream: react-aria-components has no theme provider; leptonic's `ThemeProvider` applies
// `data-theme` and offers the theme to `use_theme`.
use std::borrow::Cow;

use assertr::prelude::*;
use browser_test::{BrowserTest, async_trait, thirtyfour::WebDriver};
use rootcause::Report;

use crate::pages::{BaseActions, Page};

/// The theme provider: its context reaches its children only (not its later siblings), its element
/// carries `data-theme` and its default class, and `use_theme` switches it.
pub struct ThemeTests {}

#[async_trait]
impl BrowserTest<str> for ThemeTests {
    fn name(&self) -> Cow<'_, str> {
        "theme_tests".into()
    }

    async fn run(&self, driver: &WebDriver, base_url: &str) -> Result<(), Report> {
        let page = Page { driver, base_url };
        page.goto_path("/atoms/theme").await?;

        assert_that!(page.read_text_of("test-theme-inside").await?).is_equal_to("Dark".to_owned());
        // The sibling after the dark provider sees the app's provider (light), not the dark one.
        assert_that!(page.read_text_of("test-theme-after").await?).is_equal_to("Light".to_owned());

        let provider = page.css(".test-theme-dark").await?;
        assert_that!(provider.attr("data-theme").await?).is_equal_to(Some("dark".to_owned()));
        assert_that!(provider.attr("class").await?)
            .is_equal_to(Some("leptonic-ThemeProvider test-theme-dark".to_owned()));

        page.click_element_with_id("test-theme-toggle").await?;
        page.wait_for_text("test-theme-inside", "Light").await?;
        page.wait_for_attr(&provider, "data-theme", Some("light"))
            .await?;
        assert_that!(page.read_text_of("test-theme-after").await?).is_equal_to("Light".to_owned());
        page.click_element_with_id("test-theme-toggle").await?;
        page.wait_for_text("test-theme-inside", "Dark").await?;
        // The nested provider leaves `<html data-theme>` to the app's (outermost) provider.
        let html_theme: Option<String> = page
            .driver
            .execute(
                "return document.documentElement.getAttribute('data-theme');",
                vec![],
            )
            .await?
            .convert()?;
        assert_that!(html_theme).is_equal_to(Some("light".to_owned()));

        Ok(())
    }
}
