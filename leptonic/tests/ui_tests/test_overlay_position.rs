// Upstream: react-aria/test/overlays/useOverlayPosition.test.tsx @ 99e6102368
//! Positioning with `use_overlay_position` (through the `Popover` atom): a popover placed above its
//! trigger sits `offset` above it, centered, with its arrow at the trigger's center (hidden from
//! assistive technology) and `--trigger-width` set; a popover without room above flips below.
use assertr::prelude::*;
use rootcause::Report;

use crate::pages::{ElementActions, Page, PageActions};

const PATH: &str = "/atoms/overlay-position";

const ABOVE: &str = ".test-op-above-popover";
const FLIP: &str = ".test-op-flip-popover";

/// Scroll the page to its top, where the flip trigger has no room above.
async fn scroll_to_page_top(page: &Page<'_>) -> Result<(), Report> {
    page.eval::<()>("window.scrollTo(0, 0);", vec![]).await
}

/// Above the trigger, `offset` (10px) away, centered, the arrow (hidden from assistive technology)
/// at the trigger's center, `--trigger-width` set. Non-modal: focus stays on the trigger, which
/// toggles the popover.
pub async fn placed_above(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let trigger = page.element("#test-op-above-trigger").await?;
    trigger.click().await?;
    let popover = page.element(format!("{ABOVE}[data-placement=top]")).await?;
    // Measured once its entry animation (a slide) ran.
    page.wait_for_count(format!("{ABOVE}[data-entering]"), 0)
        .await?;
    let arrow = page.element(".test-op-arrow").await?;
    let trigger_rect = trigger.client_rect().await?;
    let popover_rect = popover.client_rect().await?;
    let arrow_rect = arrow.client_rect().await?;
    let trigger_center = f64::midpoint(trigger_rect.left, trigger_rect.right);
    assert_that!(popover_rect.bottom)
        .with_detail_message("the popover's bottom is 10px above the trigger")
        .is_close_to(trigger_rect.top - 10.0, 1.0);
    assert_that!(f64::midpoint(popover_rect.left, popover_rect.right))
        .with_detail_message("the popover is centered on the trigger")
        .is_close_to(trigger_center, 1.0);
    assert_that!(f64::midpoint(arrow_rect.left, arrow_rect.right))
        .with_detail_message("the arrow points at the trigger's center")
        .is_close_to(trigger_center, 1.0);
    assert_that!(arrow_rect.top)
        .with_detail_message("the arrow hangs below the popover's bottom edge")
        .is_close_to(popover_rect.bottom, 1.0);
    assert_that!(arrow.attr("aria-hidden").await?)
        .get_some()
        .is_equal_to("true");
    assert_that!(arrow.attr("data-placement").await?)
        .get_some()
        .is_equal_to("top");
    // From script: WebDriver's CSS value command doesn't read custom properties.
    let trigger_width: String = page
        .eval(
            "return arguments[0].style.getPropertyValue('--trigger-width');",
            vec![popover.to_json()?],
        )
        .await?;
    assert_that!(trigger_width).is_equal_to(format!("{}px", trigger_rect.width));

    trigger.click().await?;
    page.wait_for_count(ABOVE, 0).await?;
    Ok(())
}

/// Reopened, the popover has its arrow again.
pub async fn reopened_with_arrow(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let trigger = page.element("#test-op-above-trigger").await?;
    trigger.click().await?;
    page.element(format!("{ABOVE} .test-op-arrow[data-placement=top]"))
        .await?;
    trigger.click().await?;
    page.wait_for_count(ABOVE, 0).await?;
    Ok(())
}

/// No room above the trigger at the top of the page: the popover flips below, at the default
/// offset (8px).
pub async fn flips_below(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    scroll_to_page_top(page).await?;
    let trigger = page.element("#test-op-flip-trigger").await?;
    trigger.click().await?;
    // In two steps, so that a failure tells whether it opened and where it was placed.
    let popover = page.element(FLIP).await?;
    popover
        .wait_for_attr("data-placement", Some("bottom"))
        .await?;
    page.wait_for_count(format!("{FLIP}[data-entering]"), 0)
        .await?;
    let trigger_rect = trigger.client_rect().await?;
    let popover_rect = popover.client_rect().await?;
    assert_that!(popover_rect.top)
        .with_detail_message("the flipped popover starts below the trigger (default offset 8px)")
        .is_close_to(trigger_rect.bottom + 8.0, 1.0);
    trigger.click().await?;
    page.wait_for_count(FLIP, 0).await?;
    Ok(())
}

/// Reopened where it fits above, the popover doesn't start from the previous opening's position
/// (below): it is inserted unplaced, then placed above.
pub async fn reopened_unplaced(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element("#test-op-shift").await?.click().await?;
    scroll_to_page_top(page).await?;
    // Records the placement each change of `data-placement` replaced.
    page.eval::<()>(
        "const popover = arguments[0];
         window.__placements = [];
         new MutationObserver(records => {
             for (const record of records) {
                 if (record.target.matches?.(popover)) {
                     window.__placements.push(record.oldValue ?? 'none');
                 }
             }
         }).observe(document.body, {
             subtree: true,
             attributes: true,
             attributeOldValue: true,
             attributeFilter: ['data-placement'],
         });",
        vec![FLIP.into()],
    )
    .await?;
    let trigger = page.element("#test-op-flip-trigger").await?;
    trigger.click().await?;
    page.element(format!("{FLIP}[data-placement=top]")).await?;
    let placements: Vec<String> = page.eval("return window.__placements;", vec![]).await?;
    assert_that!(placements)
        .with_detail_message("the data-placement values the reopened popover had before")
        .contains_exactly(["none"]);
    trigger.click().await?;
    page.wait_for_count(FLIP, 0).await?;
    Ok(())
}
