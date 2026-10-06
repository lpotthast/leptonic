use std::borrow::Cow;

use assertr::prelude::*;
use browser_test::{
    BrowserTest, async_trait,
    thirtyfour::{Key, WebDriver},
};
use rootcause::Report;

use super::test_text_field_atom::referenced_texts;
use crate::pages::{BaseActions, Page};

/// The themed TextField, SearchField and NumberField components: labelled, bound to app state,
/// clear and stepper buttons.
pub struct TextFieldComponentTests {}

#[async_trait]
impl BrowserTest<str> for TextFieldComponentTests {
    fn name(&self) -> Cow<'_, str> {
        "text_field_component_tests".into()
    }

    async fn run(&self, driver: &WebDriver, base_url: &str) -> Result<(), Report> {
        let page = Page { driver, base_url };
        page.goto_path("/components/text-field").await?;

        // Text field: labelled and described, typing updates the bound signal.
        let name = page.css("#cmp-name input").await?;
        assert_that!(referenced_texts(&page, &name, "aria-labelledby").await?)
            .is_equal_to("Name".to_owned());
        assert_that!(name.attr("required").await?).is_some();
        name.click().await?;
        name.send_keys("Ada").await?;
        page.wait_for_text("cmp-name-value", "Ada").await?;

        // Password field: the input type and description follow their signals.
        let password = page.css("#cmp-password input").await?;
        assert_that!(password.attr("type").await?).is_equal_to(Some("password".to_owned()));
        assert_that!(referenced_texts(&page, &password, "aria-describedby").await?)
            .is_equal_to("Hidden.".to_owned());
        page.click_element_with_id("cmp-password-toggle").await?;
        page.wait_for_selector("#cmp-password input[type=text]")
            .await?;
        assert_that!(referenced_texts(&page, &password, "aria-describedby").await?)
            .is_equal_to("Visible.".to_owned());

        // Search field: the clear button empties the bound value.
        let clear = page.css("#cmp-query button").await?;
        clear.click().await?;
        page.wait_for_text("cmp-query-value", "").await?;
        page.wait_for_selector("#cmp-query [data-empty]").await?;

        // Number field: the steppers change the bound value, up to the maximum.
        let increment = page.css("#cmp-count button[aria-label=Increase]").await?;
        increment.click().await?;
        page.wait_for_text("cmp-count-value", "4").await?;
        page.wait_for_selector("#cmp-count button[aria-label=Increase][disabled]")
            .await?;
        let count = page.css("#cmp-count input:not([type=hidden])").await?;
        count.click().await?;
        count.send_keys(Key::Down).await?;
        page.wait_for_text("cmp-count-value", "3").await
    }
}
