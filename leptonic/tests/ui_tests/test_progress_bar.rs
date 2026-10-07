// Upstream: react-aria-components/test/ProgressBar.test.js @ 99e6102368
// Upstream: react-aria-components/test/Meter.test.js @ 99e6102368
use std::borrow::Cow;

use assertr::prelude::*;
use browser_test::{
    BrowserTest, async_trait,
    thirtyfour::{By, WebDriver, WebElement},
};
use rootcause::Report;

use crate::pages::{BaseActions, Page};

/// The progress bar and meter atoms: role and value attributes, the label, the value text and
/// the fill width; custom and empty ranges, indeterminate progress, a value label, a meter.
pub struct ProgressBarTests {}

async fn attr(element: &WebElement, name: &str) -> Result<Option<String>, Report> {
    Ok(element.attr(name).await?)
}

async fn value_text(element: &WebElement) -> Result<String, Report> {
    Ok(element.find(By::Css(".value")).await?.text().await?)
}

#[async_trait]
impl BrowserTest<str> for ProgressBarTests {
    fn name(&self) -> Cow<'_, str> {
        "progress_bar_tests".into()
    }

    async fn run(&self, driver: &WebDriver, base_url: &str) -> Result<(), Report> {
        let page = Page { driver, base_url };
        page.goto_path("/atoms/progress-bar").await?;

        // "renders": named by its label, 25 of 100, the fill a quarter of the 200px track.
        let basic = page.element("test-pb-basic").await?;
        assert_that!(attr(&basic, "role").await?).is_equal_to(Some("progressbar".to_owned()));
        assert_that!(attr(&basic, "aria-valuenow").await?).is_equal_to(Some("25".to_owned()));
        let label_id = attr(&basic, "aria-labelledby").await?.unwrap_or_default();
        assert_that!(page.element(&label_id).await?.text().await?)
            .is_equal_to("Loading\u{2026}".to_owned());
        assert_that!(value_text(&basic).await?).is_equal_to("25%".to_owned());
        let fill = basic.find(By::Css(".fill")).await?;
        assert_that!(fill.css_value("width").await?).is_equal_to("50px".to_owned());

        // The value follows its signal.
        page.element("test-pb-more").await?.click().await?;
        page.wait_for_attr(&basic, "aria-valuenow", Some("50"))
            .await?;
        assert_that!(attr(&basic, "aria-valuetext").await?).is_equal_to(Some("50%".to_owned()));
        assert_that!(fill.css_value("width").await?).is_equal_to("100px".to_owned());

        // "supports a custom range".
        let custom = page.css("[aria-label='Custom range']").await?;
        assert_that!(attr(&custom, "aria-valuenow").await?).is_equal_to(Some("3".to_owned()));
        assert_that!(attr(&custom, "aria-valuemin").await?).is_equal_to(Some("0".to_owned()));
        assert_that!(attr(&custom, "aria-valuemax").await?).is_equal_to(Some("6".to_owned()));
        assert_that!(attr(&custom, "aria-valuetext").await?).is_equal_to(Some("50%".to_owned()));
        assert_that!(value_text(&custom).await?).is_equal_to("50%".to_owned());

        // "renders 0 percent for an empty range with a non-zero bound".
        let empty = page.css("[aria-label='Empty range']").await?;
        assert_that!(attr(&empty, "aria-valuenow").await?).is_equal_to(Some("5".to_owned()));
        assert_that!(attr(&empty, "aria-valuetext").await?).is_equal_to(Some("0%".to_owned()));

        // "supports indeterminate state": no value, no width of its own.
        let indeterminate = page.css("[aria-label=Indeterminate]").await?;
        assert_that!(attr(&indeterminate, "data-indeterminate").await?).is_some();
        assert_that!(attr(&indeterminate, "aria-valuenow").await?).is_none();
        assert_that!(attr(&indeterminate, "aria-valuetext").await?).is_none();
        assert_that!(value_text(&indeterminate).await?).is_equal_to(String::new());
        let indeterminate_fill = indeterminate.find(By::Css(".fill")).await?;
        assert_that!(
            attr(&indeterminate_fill, "style")
                .await?
                .unwrap_or_default()
        )
        .does_not_contain("width");

        // useProgressBar.test.js "with custom text value".
        let files = page.css("[aria-label=Files]").await?;
        assert_that!(attr(&files, "aria-valuetext").await?).is_equal_to(Some("1 of 4".to_owned()));

        // The label reference follows the rendered `Label` (RAC's `useSlot`).
        let unlabelled = page.element("test-pb-unlabelled").await?;
        page.wait_for_attr(&unlabelled, "aria-labelledby", None)
            .await?;
        let both = page.element("test-pb-both").await?;
        assert_that!(attr(&both, "aria-label").await?).is_equal_to(Some("Named".to_owned()));
        let labelled_by = attr(&both, "aria-labelledby").await?.unwrap_or_default();
        let ids: Vec<&str> = labelled_by.split(' ').collect();
        assert_that!(ids.len()).is_equal_to(2);
        // Itself first, as react-aria's `useLabels`.
        assert_that!(ids[0]).is_equal_to("test-pb-both");
        assert_that!(page.element(ids[1]).await?.text().await?).is_equal_to("Visible".to_owned());

        // Meter.test.js "renders".
        let meter = page.css("[role=meter]").await?;
        assert_that!(attr(&meter, "aria-valuenow").await?).is_equal_to(Some("75".to_owned()));
        let meter_label = attr(&meter, "aria-labelledby").await?.unwrap_or_default();
        assert_that!(page.element(&meter_label).await?.text().await?)
            .is_equal_to("Storage".to_owned());
        assert_that!(value_text(&meter).await?).is_equal_to("75%".to_owned());
        let meter_fill = meter.find(By::Css(".fill")).await?;
        assert_that!(meter_fill.css_value("width").await?).is_equal_to("150px".to_owned());

        page.expect_no_page_errors().await
    }
}
