use std::borrow::Cow;

use assertr::prelude::*;
use browser_test::{
    BrowserTest, async_trait,
    thirtyfour::{By, WebDriver},
};
use rootcause::Report;

use crate::pages::{BaseActions, Page};

/// The styled buttons' theme: disabled buttons (the atoms render `data-disabled`) look disabled
/// in every variant, and each size has its own font size.
pub struct ButtonComponentsTests {}

#[async_trait]
impl BrowserTest<str> for ButtonComponentsTests {
    fn name(&self) -> Cow<'_, str> {
        "button_components_tests".into()
    }

    async fn run(&self, driver: &WebDriver, base_url: &str) -> Result<(), Report> {
        let page = Page { driver, base_url };
        page.goto_path("/components/button").await?;

        let css = async |id: &str, property: &str| -> Result<String, Report> {
            Ok(driver.find(By::Id(id)).await?.css_value(property).await?)
        };
        for variant in ["flat", "outlined", "filled"] {
            let disabled = format!("test-cbtn-{variant}-disabled");
            assert_that!(css(&disabled, "background-color").await?)
                .with_detail_message(variant)
                .is_equal_to("rgba(226, 226, 226, 1)".to_owned());
            assert_that!(css(&disabled, "cursor").await?)
                .with_detail_message(variant)
                .is_equal_to("default".to_owned());
            assert_that!(css(&format!("test-cbtn-{variant}"), "background-color").await?)
                .with_detail_message(variant)
                .is_not_equal_to("rgba(226, 226, 226, 1)".to_owned());
        }
        assert_that!(css("test-cbtn-link-disabled", "background-color").await?)
            .is_equal_to("rgba(226, 226, 226, 1)".to_owned());

        let font_size = async |id: &str| -> Result<f64, Report> {
            Ok(css(id, "font-size")
                .await?
                .trim_end_matches("px")
                .parse::<f64>()
                .unwrap_or_default())
        };
        let (small, normal, big) = (
            font_size("test-cbtn-small").await?,
            font_size("test-cbtn-normal").await?,
            font_size("test-cbtn-big").await?,
        );
        assert_that!(small < normal && normal < big)
            .with_detail_message(format!("{small} {normal} {big}"))
            .is_true();

        page.expect_no_page_errors().await
    }
}
