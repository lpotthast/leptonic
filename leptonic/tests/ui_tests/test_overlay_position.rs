// Upstream: react-aria/test/overlays/useOverlayPosition.test.tsx @ 99e6102368
use std::borrow::Cow;

use assertr::prelude::*;
use browser_test::{BrowserTest, async_trait, thirtyfour::WebDriver};
use rootcause::Report;

use crate::pages::{BaseActions, Page};

/// Positioning with `use_overlay_position` (through the `Popover` atom): a popover placed above its
/// trigger sits `offset` above it, centered, with its arrow at the trigger's center (hidden from
/// assistive technology) and `--trigger-width` set; a popover without room above flips below.
pub struct OverlayPositionTests {}

/// The bounding rectangle of the first element matching `selector`: `[left, top, right, bottom]`.
async fn rect(page: &Page<'_>, selector: &str) -> Result<[f64; 4], Report> {
    let script = format!(
        "const r = document.querySelector('{selector}').getBoundingClientRect(); return [r.left, r.top, r.right, r.bottom];"
    );
    let values: Vec<f64> = page.driver.execute(&script, vec![]).await?.convert()?;
    Ok([values[0], values[1], values[2], values[3]])
}

#[async_trait]
impl BrowserTest<str> for OverlayPositionTests {
    fn name(&self) -> Cow<'_, str> {
        "overlay_position_tests".into()
    }

    async fn run(&self, driver: &WebDriver, base_url: &str) -> Result<(), Report> {
        let page = Page { driver, base_url };
        page.goto_path("/atoms/overlay-position").await?;

        // Above the trigger, `offset` (10px) away, centered, the arrow at the trigger's center.
        page.click_element_with_id("test-op-above-trigger").await?;
        page.wait_for_selector(".test-op-above-popover[data-placement=top]")
            .await?;
        let trigger = rect(&page, "#test-op-above-trigger").await?;
        let popover = rect(&page, ".test-op-above-popover").await?;
        let arrow = rect(&page, ".test-op-arrow").await?;
        assert_that!(popover[3])
            .with_detail_message("the popover's bottom is 10px above the trigger")
            .is_close_to(trigger[1] - 10.0, 1.0);
        let trigger_center = f64::midpoint(trigger[0], trigger[2]);
        assert_that!(f64::midpoint(popover[0], popover[2]))
            .with_detail_message("the popover is centered on the trigger")
            .is_close_to(trigger_center, 1.0);
        assert_that!(f64::midpoint(arrow[0], arrow[2]))
            .with_detail_message("the arrow points at the trigger's center")
            .is_close_to(trigger_center, 1.0);
        assert_that!(arrow[1])
            .with_detail_message("the arrow hangs below the popover's bottom edge")
            .is_close_to(popover[3], 1.0);
        let arrow_el = page.css(".test-op-arrow").await?;
        assert_that!(arrow_el.attr("aria-hidden").await?).is_equal_to(Some("true".to_owned()));
        assert_that!(arrow_el.attr("data-placement").await?).is_equal_to(Some("top".to_owned()));
        let trigger_width: String = page
            .driver
            .execute(
                "return document.querySelector('.test-op-above-popover').style.getPropertyValue('--trigger-width');",
                vec![],
            )
            .await?
            .convert()?;
        assert_that!(trigger_width).is_equal_to(format!("{}px", trigger[2] - trigger[0]));
        // Non-modal: focus stayed on the trigger, which toggles the popover.
        page.click_element_with_id("test-op-above-trigger").await?;
        page.wait_for_no_selector(".test-op-above-popover").await?;
        // Reopened, the popover has its arrow again.
        page.click_element_with_id("test-op-above-trigger").await?;
        page.wait_for_selector(".test-op-above-popover .test-op-arrow[data-placement=top]")
            .await?;
        page.click_element_with_id("test-op-above-trigger").await?;
        page.wait_for_no_selector(".test-op-above-popover").await?;

        // No room above the trigger at the top of the page: the popover flips below.
        page.driver
            .execute("window.scrollTo(0, 0);", vec![])
            .await?;
        page.click_element_with_id("test-op-flip-trigger").await?;
        page.wait_for_selector(".test-op-flip-popover[data-placement=bottom]")
            .await?;
        let trigger = rect(&page, "#test-op-flip-trigger").await?;
        let popover = rect(&page, ".test-op-flip-popover").await?;
        assert_that!(popover[1])
            .with_detail_message(
                "the flipped popover starts below the trigger (default offset 8px)",
            )
            .is_close_to(trigger[3] + 8.0, 1.0);
        page.click_element_with_id("test-op-flip-trigger").await?;
        page.wait_for_no_selector(".test-op-flip-popover").await?;
        page.expect_no_page_errors().await
    }
}
