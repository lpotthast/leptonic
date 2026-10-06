use std::borrow::Cow;

use assertr::prelude::*;
use browser_test::{
    BrowserTest, async_trait,
    thirtyfour::{Key, WebDriver},
};
use rootcause::Report;

use crate::pages::{BaseActions, Page};

/// The styled table: a pressable header is reachable by Tab, pressed with Enter and announces its
/// sort order; a plain header isn't focusable.
pub struct TableComponentsTests {}

#[async_trait]
impl BrowserTest<str> for TableComponentsTests {
    fn name(&self) -> Cow<'_, str> {
        "table_components_tests".into()
    }

    async fn run(&self, driver: &WebDriver, base_url: &str) -> Result<(), Report> {
        let page = Page { driver, base_url };
        page.goto_path("/components/table").await?;

        let name = page
            .css("#test-page-component-table th:nth-child(1)")
            .await?;
        let age = page
            .css("#test-page-component-table th:nth-child(2)")
            .await?;
        assert_that!(name.attr("tabindex").await?).is_equal_to(Some("0".to_owned()));
        assert_that!(age.attr("tabindex").await?).is_none();
        assert_that!(name.attr("aria-sort").await?).is_equal_to(Some("none".to_owned()));

        page.element("test-ctable-before").await?.focus().await?;
        page.press_tab().await?;
        page.send_keys_to_active(Key::Enter).await?;
        page.wait_for_attr(&name, "aria-sort", Some("ascending"))
            .await?;
        page.send_keys_to_active(Key::Space).await?;
        page.wait_for_attr(&name, "aria-sort", Some("descending"))
            .await?;

        page.expect_no_page_errors().await
    }
}
