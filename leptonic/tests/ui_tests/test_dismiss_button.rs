// Upstream: react-aria/test/overlays/DismissButton.test.tsx @ 99e6102368
use std::borrow::Cow;

use assertr::prelude::*;
use browser_test::{
    BrowserTest, async_trait,
    thirtyfour::{By, WebDriver, WebElement},
};
use rootcause::Report;

use crate::pages::{BaseActions, Page};

/// `DismissButton`: named "Dismiss" by default, by its `aria_label`, by `aria_labelledby` alone
/// (no `aria-label` then), or by itself and the referenced elements when given both; activating
/// it calls `on_dismiss`.
pub struct DismissButtonTests {}

async fn button_in(page: &Page<'_>, container: &str) -> Result<WebElement, Report> {
    Ok(page
        .element(container)
        .await?
        .find(By::Css("button"))
        .await?)
}

#[async_trait]
impl BrowserTest<str> for DismissButtonTests {
    fn name(&self) -> Cow<'_, str> {
        "dismiss_button_tests".into()
    }

    async fn run(&self, driver: &WebDriver, base_url: &str) -> Result<(), Report> {
        let page = Page { driver, base_url };
        page.goto_path("/atoms/dismiss-button").await?;

        // "should have a default aria-label".
        let default = button_in(&page, "test-dismiss-default").await?;
        assert_that!(default.attr("aria-label").await?).is_equal_to(Some("Dismiss".to_owned()));
        assert_that!(default.attr("tabindex").await?).is_equal_to(Some("-1".to_owned()));

        // "should accept an aria-label".
        let label = button_in(&page, "test-dismiss-label").await?;
        assert_that!(label.attr("aria-label").await?).is_equal_to(Some("foo".to_owned()));

        // "should accept an aria-labelledby".
        let labelledby = button_in(&page, "test-dismiss-labelledby").await?;
        assert_that!(labelledby.attr("aria-labelledby").await?)
            .is_equal_to(Some("test-dismiss-span".to_owned()));
        assert_that!(labelledby.attr("aria-label").await?).is_none();

        // "should accept an aria-labelledby and aria-label".
        let both = button_in(&page, "test-dismiss-both").await?;
        assert_that!(both.attr("aria-labelledby").await?)
            .is_equal_to(Some("self test-dismiss-span".to_owned()));
        assert_that!(both.attr("aria-label").await?).is_equal_to(Some("foo".to_owned()));
        assert_that!(both.attr("id").await?).is_equal_to(Some("self".to_owned()));

        // Activating it (as a screen reader does; it is visually hidden) dismisses.
        page.driver
            .execute(
                "document.querySelector('#test-dismiss-default button').click()",
                vec![],
            )
            .await?;
        page.wait_for_text("test-dismiss-count", "1").await?;
        page.expect_no_page_errors().await
    }
}
