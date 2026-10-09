// Upstream: react-aria-components/test/ProgressBar.test.js @ 99e6102368
// Upstream: react-aria-components/test/Meter.test.js @ 99e6102368
//! The progress bar and meter atoms: role and value attributes, the label, the value text and
//! the fill width; custom and empty ranges, indeterminate progress, a value label, a meter.
use assertr::{matchers::eq, prelude::*};
use browser_test::thirtyfour::prelude::*;
use rootcause::Report;

use crate::pages::{ElementActions, Page, PageActions};

const PATH: &str = "/atoms/progress-bar";

/// The visible value text of the progress bar or meter `element`.
async fn value_text(element: &WebElement) -> Result<String, Report> {
    element.element(".value").await?.inner_text().await
}

/// The fill of the progress bar or meter `element`.
async fn fill(element: &WebElement) -> Result<WebElement, Report> {
    element.element(".fill").await
}

/// "renders": named by its label, 25 of 100, the fill a quarter of the 200px track.
pub async fn renders(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let basic = page.element("#test-pb-basic").await?;
    assert_that!(basic.attr("role").await?)
        .get_some()
        .is_equal_to("progressbar");
    assert_that!(basic.attr("aria-valuenow").await?)
        .get_some()
        .is_equal_to("25");
    assert_that!(basic.referenced_text("aria-labelledby").await?).is_equal_to("Loading\u{2026}");
    assert_that!(value_text(&basic).await?).is_equal_to("25%");
    assert_that!(fill(&basic).await?.css_value("width").await?).is_equal_to("50px");
    Ok(())
}

/// The value follows its signal.
pub async fn follows_its_value(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let basic = page.element("#test-pb-basic").await?;
    page.element("#test-pb-more").await?.click().await?;
    basic.wait_for_attr("aria-valuenow", Some("50")).await?;
    assert_that!(basic.attr("aria-valuetext").await?)
        .get_some()
        .is_equal_to("50%");
    let fill = fill(&basic).await?;
    assert_that!(|| fill.css_value("width"))
        .eventually_ok()
        .matches(eq("100px"))
        .await;
    Ok(())
}

/// "supports a custom range".
pub async fn custom_range(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let custom = page.element("[aria-label='Custom range']").await?;
    assert_that!(custom.attr("aria-valuenow").await?)
        .get_some()
        .is_equal_to("3");
    assert_that!(custom.attr("aria-valuemin").await?)
        .get_some()
        .is_equal_to("0");
    assert_that!(custom.attr("aria-valuemax").await?)
        .get_some()
        .is_equal_to("6");
    assert_that!(custom.attr("aria-valuetext").await?)
        .get_some()
        .is_equal_to("50%");
    assert_that!(value_text(&custom).await?).is_equal_to("50%");
    Ok(())
}

/// "renders 0 percent for an empty range with a non-zero bound".
pub async fn empty_range(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let empty = page.element("[aria-label='Empty range']").await?;
    assert_that!(empty.attr("aria-valuenow").await?)
        .get_some()
        .is_equal_to("5");
    assert_that!(empty.attr("aria-valuetext").await?)
        .get_some()
        .is_equal_to("0%");
    Ok(())
}

/// "supports indeterminate state": no value, no width of its own.
pub async fn indeterminate(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let indeterminate = page.element("[aria-label=Indeterminate]").await?;
    assert_that!(indeterminate.attr("data-indeterminate").await?).is_some();
    assert_that!(indeterminate.attr("aria-valuenow").await?).is_none();
    assert_that!(indeterminate.attr("aria-valuetext").await?).is_none();
    assert_that!(value_text(&indeterminate).await?).is_empty();
    assert_that!(
        fill(&indeterminate)
            .await?
            .attr("style")
            .await?
            .unwrap_or_default()
    )
    .does_not_contain("width");
    Ok(())
}

/// useProgressBar.test.js "with custom text value".
pub async fn custom_text_value(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let files = page.element("[aria-label=Files]").await?;
    assert_that!(files.attr("aria-valuetext").await?)
        .get_some()
        .is_equal_to("1 of 4");
    Ok(())
}

/// The label reference follows the rendered `Label` (RAC's `useSlot`).
pub async fn label_follows_the_rendered_label(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let unlabelled = page.element("#test-pb-unlabelled").await?;
    unlabelled.wait_for_attr("aria-labelledby", None).await?;
    let both = page.element("#test-pb-both").await?;
    assert_that!(both.attr("aria-label").await?)
        .get_some()
        .is_equal_to("Named");
    let labelled_by = both.attr("aria-labelledby").await?.unwrap_or_default();
    let ids: Vec<&str> = labelled_by.split(' ').collect();
    assert_that!(&ids).has_length(2);
    // Itself first, as react-aria's `useLabels`.
    assert_that!(ids[0]).is_equal_to("test-pb-both");
    let label = page.element(format!("#{}", ids[1])).await?;
    assert_that!(label.inner_text().await?).is_equal_to("Visible");
    Ok(())
}

/// Meter.test.js "renders".
pub async fn meter(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let meter = page.element("[role=meter]").await?;
    assert_that!(meter.attr("aria-valuenow").await?)
        .get_some()
        .is_equal_to("75");
    assert_that!(meter.referenced_text("aria-labelledby").await?).is_equal_to("Storage");
    assert_that!(value_text(&meter).await?).is_equal_to("75%");
    assert_that!(fill(&meter).await?.css_value("width").await?).is_equal_to("150px");
    Ok(())
}
