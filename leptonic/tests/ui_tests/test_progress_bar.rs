// Upstream: react-aria-components/test/ProgressBar.test.js @ 99e6102368
// Upstream: react-aria-components/test/Meter.test.js @ 99e6102368
//! The progress bar and meter atoms: role and value attributes, the label, the value text and
//! the fill width; custom and empty ranges, indeterminate progress, a value label, a meter.
use assertr::{matchers::eq, prelude::*};
use browser_test::{browser_test, thirtyfour::prelude::*};
use leptonic::AriaRole;
use rootcause::Report;

use crate::pages::{ElementActions, Page, css, role};

const PATH: &str = "/atoms/progress-bar";

/// The value label of the progress bar or meter `element`.
async fn value_label(element: &WebElement) -> Result<WebElement, Report> {
    element.element(".value").await
}

/// The fill of the progress bar or meter `element`.
async fn fill(element: &WebElement) -> Result<WebElement, Report> {
    element.element(".fill").await
}

/// A progress bar at 25 of 100 is named by its label, shows "25%" and fills a quarter of its
/// track ("renders").
#[browser_test]
pub async fn renders(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let basic = page.element("#test-pb-basic").await?;
    assert_that!(basic)
        .has_attribute("role")
        .await
        .is_equal_to("progressbar");
    assert_that!(basic)
        .has_attribute("aria-valuenow")
        .await
        .is_equal_to("25");
    assert_that!(basic)
        .accessible_name()
        .await
        .is_equal_to("Loading\u{2026}");
    assert_that!(value_label(&basic).await?)
        .inner_text()
        .await
        .is_equal_to("25%");
    assert_that!(fill(&basic).await?.css_value("width").await?).is_equal_to("50px");
    Ok(())
}

/// Raising the value signal to 50 updates `aria-valuenow`, `aria-valuetext` and the fill width.
#[browser_test]
pub async fn follows_its_value(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let basic = page.element("#test-pb-basic").await?;
    page.element("#test-pb-more").await?.click().await?;
    basic.wait_for_attr("aria-valuenow", Some("50")).await?;
    assert_that!(basic)
        .has_attribute("aria-valuetext")
        .await
        .is_equal_to("50%");
    let fill = fill(&basic).await?;
    assert_that!(|| fill.css_value("width"))
        .eventually_ok()
        .matches(eq("100px"))
        .await;
    Ok(())
}

/// A value of 3 in the range 0 to 6 is announced and shown as "50%" ("supports a custom range").
#[browser_test]
pub async fn custom_range(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let custom = page.element("[aria-label='Custom range']").await?;
    assert_that!(custom)
        .has_attribute("aria-valuenow")
        .await
        .is_equal_to("3");
    assert_that!(custom)
        .has_attribute("aria-valuemin")
        .await
        .is_equal_to("0");
    assert_that!(custom)
        .has_attribute("aria-valuemax")
        .await
        .is_equal_to("6");
    assert_that!(custom)
        .has_attribute("aria-valuetext")
        .await
        .is_equal_to("50%");
    assert_that!(value_label(&custom).await?)
        .inner_text()
        .await
        .is_equal_to("50%");
    Ok(())
}

/// A progress bar whose range is the single value 5 announces "0%" ("renders 0 percent for an
/// empty range with a non-zero bound").
#[browser_test]
pub async fn empty_range(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let empty = page.element("[aria-label='Empty range']").await?;
    assert_that!(empty)
        .has_attribute("aria-valuenow")
        .await
        .is_equal_to("5");
    assert_that!(empty)
        .has_attribute("aria-valuetext")
        .await
        .is_equal_to("0%");
    Ok(())
}

/// An indeterminate progress bar is marked `data-indeterminate`, has no value, no value text and no
/// fill width of its own ("supports indeterminate state").
#[browser_test]
pub async fn indeterminate(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let indeterminate = page.element("[aria-label=Indeterminate]").await?;
    assert_that!(indeterminate)
        .has_attribute("data-indeterminate")
        .await;
    assert_that!(indeterminate)
        .attribute("aria-valuenow")
        .await
        .is_none();
    assert_that!(indeterminate)
        .attribute("aria-valuetext")
        .await
        .is_none();
    assert_that!(value_label(&indeterminate).await?)
        .inner_text()
        .await
        .is_empty();
    assert_that!(fill(&indeterminate).await?)
        .attribute("style")
        .await
        .map_owned(Option::unwrap_or_default)
        .does_not_contain("width");
    Ok(())
}

/// A value label replaces the percentage in `aria-valuetext` ("1 of 4"; useProgressBar.test.js
/// "with custom text value").
#[browser_test]
pub async fn custom_text_value(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let files = page.element("[aria-label=Files]").await?;
    assert_that!(files)
        .has_attribute("aria-valuetext")
        .await
        .is_equal_to("1 of 4");
    Ok(())
}

/// Without a rendered `Label` the progress bar has no `aria-labelledby` (only its `aria_label`);
/// with one and an `aria_label`, it is labelled by itself and then the `Label` (RAC's `useSlot`).
#[browser_test]
pub async fn label_follows_the_rendered_label(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let custom = page.element("[aria-label='Custom range']").await?;
    custom.wait_for_attr("aria-labelledby", None).await?;
    let both = page.element("#test-pb-both").await?;
    assert_that!(both)
        .has_attribute("aria-label")
        .await
        .is_equal_to("Named");
    let label_id = {
        let labelled_by = assert_that!(both).has_attribute("aria-labelledby").await;
        let ids = labelled_by
            .derive_owned(|value| value.split(' ').collect::<Vec<_>>())
            .has_length(2);
        // Itself first, as react-aria's `useLabels`.
        ids.derive_owned(|ids| ids[0]).is_equal_to("test-pb-both");
        ids.actual()[1].to_owned()
    };
    let label = page.element(format!("#{label_id}")).await?;
    assert_that!(label)
        .inner_text()
        .await
        .is_equal_to("Visible");
    Ok(())
}

/// A meter at 75 is named by its label, shows "75%" and fills three quarters of its track
/// (Meter.test.js "renders").
#[browser_test]
pub async fn meter(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let meter = page
        .element(role(AriaRole::Meter).has(css(".leptonic-Label").text("Storage")))
        .await?;
    assert_that!(meter)
        .has_attribute("aria-valuenow")
        .await
        .is_equal_to("75");
    assert_that!(meter)
        .accessible_name()
        .await
        .is_equal_to("Storage");
    assert_that!(value_label(&meter).await?)
        .inner_text()
        .await
        .is_equal_to("75%");
    assert_that!(fill(&meter).await?.css_value("width").await?).is_equal_to("150px");
    Ok(())
}

/// A progress bar whose range is 0 to 0 announces and shows "0%" (not "NaN%") and fills none of
/// its track ("renders 0 percent for an empty range").
#[browser_test]
pub async fn zero_range(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let zero = page.element("[aria-label='Zero range']").await?;
    assert_that!(zero)
        .has_attribute("aria-valuenow")
        .await
        .is_equal_to("0");
    assert_that!(zero)
        .has_attribute("aria-valuemax")
        .await
        .is_equal_to("0");
    assert_that!(zero)
        .has_attribute("aria-valuetext")
        .await
        .is_equal_to("0%");
    assert_that!(value_label(&zero).await?)
        .inner_text()
        .await
        .is_equal_to("0%");
    assert_that!(fill(&zero).await?.css_value("width").await?).is_equal_to("0px");
    Ok(())
}

/// A meter at 3 in the range 0 to 6 announces and shows "50%" and fills half its track (Meter.test.js
/// "supports a custom range").
#[browser_test]
pub async fn meter_custom_range(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let meter = page.element("[aria-label='Meter custom range']").await?;
    assert_that!(meter)
        .has_attribute("aria-valuenow")
        .await
        .is_equal_to("3");
    assert_that!(meter)
        .has_attribute("aria-valuemin")
        .await
        .is_equal_to("0");
    assert_that!(meter)
        .has_attribute("aria-valuemax")
        .await
        .is_equal_to("6");
    assert_that!(meter)
        .has_attribute("aria-valuetext")
        .await
        .is_equal_to("50%");
    assert_that!(value_label(&meter).await?)
        .inner_text()
        .await
        .is_equal_to("50%");
    assert_that!(fill(&meter).await?.css_value("width").await?).is_equal_to("100px");
    Ok(())
}

/// A meter whose range is 0 to 0 announces and shows "0%" and fills none of its track
/// (Meter.test.js "renders 0 percent for an empty range").
#[browser_test]
pub async fn meter_empty_range(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let meter = page.element("[aria-label='Meter empty range']").await?;
    assert_that!(meter)
        .has_attribute("aria-valuetext")
        .await
        .is_equal_to("0%");
    assert_that!(value_label(&meter).await?)
        .inner_text()
        .await
        .is_equal_to("0%");
    assert_that!(fill(&meter).await?.css_value("width").await?).is_equal_to("0px");
    Ok(())
}

/// The parts carry their default classes, and the fills hold their percentage in `--percent`
/// (`100%` while indeterminate) for styles ("renders": `react-aria-ProgressBar`).
#[browser_test]
pub async fn default_classes_and_percent(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let basic = page.element("#test-pb-basic").await?;
    assert_that!(basic)
        .has_attribute("class")
        .await
        .is_equal_to("leptonic-ProgressBar");
    assert_that!(value_label(&basic).await?)
        .has_attribute("class")
        .await
        .is_equal_to("leptonic-ProgressBarValueText value");
    let basic_fill = fill(&basic).await?;
    assert_that!(basic_fill)
        .has_attribute("class")
        .await
        .is_equal_to("leptonic-ProgressBarFill fill");
    assert_that!(basic_fill.style_property("--percent").await?).is_equal_to("25%");
    let indeterminate = fill(&page.element("[aria-label=Indeterminate]").await?).await?;
    assert_that!(indeterminate.style_property("--percent").await?).is_equal_to("100%");

    let meter = page
        .element(role(AriaRole::Meter).has(css(".leptonic-Label").text("Storage")))
        .await?;
    assert_that!(meter)
        .has_attribute("class")
        .await
        .is_equal_to("leptonic-Meter");
    assert_that!(value_label(&meter).await?)
        .has_attribute("class")
        .await
        .is_equal_to("leptonic-MeterValueText value");
    let meter_fill = fill(&meter).await?;
    assert_that!(meter_fill)
        .has_attribute("class")
        .await
        .is_equal_to("leptonic-MeterFill fill");
    assert_that!(meter_fill.style_property("--percent").await?).is_equal_to("75%");
    Ok(())
}
