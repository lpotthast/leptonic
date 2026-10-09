// Upstream: react-aria/test/overlays/useOverlayPosition.test.tsx @ 99e6102368
//! Positioning with `use_overlay_position` (through the `Popover` atom): a popover placed above its
//! trigger sits `offset` above it, centered, with its arrow at the trigger's center (hidden from
//! assistive technology) and `--trigger-width` set; a popover without room above flips below.
use assertr::prelude::*;
use browser_test::browser_test;
use rootcause::Report;

use crate::pages::{ElementActions, EventKind, GlobalTarget, Page, SyntheticEvent, WebElement};

const PATH: &str = "/atoms/overlay-position";
const OPTIONS: &str = "/atoms/overlay-position-options";

const ABOVE: &str = ".test-op-above-popover";
const FLIP: &str = ".test-op-flip-popover";

/// Scroll the page to its top, where the flip trigger has no room above.
async fn scroll_to_page_top(page: &Page<'_>) -> Result<(), Report> {
    page.low_level()
        .eval::<()>("window.scrollTo(0, 0);", vec![])
        .await
}

/// A popover placed above sits its 10px offset above the trigger, centered on it, with its hidden
/// arrow at the trigger's center and `--trigger-width` set, while the focus stays on the trigger
/// ("should position the overlay relative to the trigger at top", "arrow should be hidden when
/// using assistive technologies").
#[browser_test]
pub async fn placed_above(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let trigger = page.element("#test-op-above-trigger").await?;
    trigger.click().await?;
    // In steps, so that a failure tells whether it opened, stayed open and was placed.
    trigger.wait_for_attr("aria-expanded", Some("true")).await?;
    let popover = page.element(ABOVE).await?;
    popover.wait_for_attr("data-placement", Some("top")).await?;
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
    assert_that!(arrow)
        .has_attribute("aria-hidden")
        .await
        .is_equal_to("true");
    assert_that!(arrow)
        .has_attribute("data-placement")
        .await
        .is_equal_to("top");
    assert_that!(popover.style_property("--trigger-width").await?)
        .is_equal_to(format!("{}px", trigger_rect.width));
    page.focus_stays(&trigger, std::time::Duration::from_millis(100))
        .await?;

    trigger.click().await?;
    page.wait_for_count(ABOVE, 0).await?;
    Ok(())
}

/// A popover stays hidden (`opacity: 0`, clipped) until it is placed: none of the styles it has
/// while unplaced (`position: fixed` at the viewport's origin) shows it, also after its trigger's
/// width (a CSS variable in the same `style`) arrives. useEnterAnimation's hiding.
#[browser_test]
pub async fn hidden_until_placed(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let recording = page.record_attr_of(ABOVE, "style").await?;
    let trigger = page.element("#test-op-above-trigger").await?;
    trigger.click().await?;
    let popover = page.element(ABOVE).await?;
    popover.wait_for_attr("data-placement", Some("top")).await?;
    page.wait_for_count(format!("{ABOVE}[data-entering]"), 0)
        .await?;
    let styles: Vec<String> = recording
        .finish()
        .await?
        .into_iter()
        .flatten()
        .map(|style| style.replace(' ', ""))
        .collect();
    let unplaced: Vec<&String> = styles
        .iter()
        .filter(|style| style.contains("position:fixed"))
        .collect();
    assert_that!(&unplaced)
        .with_detail_message("the popover was inserted unplaced")
        .is_not_empty();
    let shown_unplaced: Vec<&&String> = unplaced
        .iter()
        .filter(|style| !style.contains("opacity:0"))
        .collect();
    assert_that!(shown_unplaced)
        .with_detail_message("styles of the unplaced popover that don't hide it")
        .is_empty();
    assert_that!(popover.style_property("--trigger-width").await?).is_not_empty();
    Ok(())
}

/// A popover closed and reopened renders its placed arrow again.
#[browser_test]
pub async fn reopened_with_arrow(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let trigger = page.element("#test-op-above-trigger").await?;
    let arrow = format!("{ABOVE} .test-op-arrow[data-placement=top]");
    trigger.click().await?;
    page.element(&arrow).await?;
    trigger.click().await?;
    page.wait_for_count(ABOVE, 0).await?;

    trigger.click().await?;
    page.element(ABOVE).await?;
    page.wait_for_count(&arrow, 1).await?;
    Ok(())
}

/// A popover without room above its trigger flips below it, at the default offset (8px).
#[browser_test]
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

/// A popover reopened where it now fits above is inserted unplaced and then placed above, never
/// showing the previous opening's placement below.
#[browser_test]
pub async fn reopened_unplaced(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    scroll_to_page_top(page).await?;
    let trigger = page.element("#test-op-flip-trigger").await?;
    // First opened without room above: placed below.
    trigger.click().await?;
    page.element(FLIP)
        .await?
        .wait_for_attr("data-placement", Some("bottom"))
        .await?;
    trigger.click().await?;
    page.wait_for_count(FLIP, 0).await?;
    // Room above from now on.
    page.element("#test-op-shift").await?.click().await?;
    scroll_to_page_top(page).await?;
    // Records the placement each change of `data-placement` replaced.
    page.low_level()
        .eval::<()>(
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
    trigger.click().await?;
    page.element(FLIP)
        .await?
        .wait_for_attr("data-placement", Some("top"))
        .await?;
    let placements: Vec<String> = page
        .low_level()
        .eval("return window.__placements;", vec![])
        .await?;
    assert_that!(placements)
        .with_detail_message("the data-placement values the reopened popover had before")
        .contains_exactly(["none"]);
    trigger.click().await?;
    page.wait_for_count(FLIP, 0).await?;
    Ok(())
}

/// Opens the popover of the options fixture's `name` section and returns its trigger and the
/// popover, once placed (at `side`) and entered.
async fn open_placed(
    page: &Page<'_>,
    name: &str,
    side: &str,
) -> Result<(WebElement, WebElement), Report> {
    page.goto_sections(OPTIONS, &[name]).await?;
    let trigger = page.element(format!("#test-opo-{name}-trigger")).await?;
    trigger.click().await?;
    let selector = format!(".test-opo-{name}-popover");
    let popover = page.element(selector.as_str()).await?;
    popover.wait_for_attr("data-placement", Some(side)).await?;
    page.wait_for_count(format!("{selector}[data-entering]"), 0)
        .await?;
    Ok((trigger, popover))
}

/// The popover's top edge, read with its trigger's bottom edge.
async fn gap_below(trigger: &WebElement, popover: &WebElement) -> Result<f64, Report> {
    Ok(popover.client_rect().await?.top - trigger.client_rect().await?.bottom)
}

/// The horizontal distance from the trigger's center to the popover's center.
async fn center_shift(trigger: &WebElement, popover: &WebElement) -> Result<f64, Report> {
    let (trigger, popover) = (trigger.client_rect().await?, popover.client_rect().await?);
    Ok(f64::midpoint(popover.left, popover.right) - f64::midpoint(trigger.left, trigger.right))
}

/// A changed `offset` moves the open popover ("should update the position on props change").
#[browser_test]
pub async fn repositions_on_props_change(page: &Page<'_>) -> Result<(), Report> {
    let (trigger, popover) = open_placed(page, "offset", "bottom").await?;
    assert_that!(gap_below(&trigger, &popover).await?).is_close_to(0.0, 1.0);
    page.element("#test-opo-offset-toggle")
        .await?
        .click()
        .await?;
    assert_that!(|| gap_below(&trigger, &popover))
        .eventually_ok()
        .satisfies(|gap| {
            gap.is_close_to(20.0, 1.0);
        })
        .await;
    Ok(())
}

/// A window `resize` repositions the open popover at its trigger, which moved without resizing
/// ("should update the position on window resize").
#[browser_test]
pub async fn repositions_on_window_resize(page: &Page<'_>) -> Result<(), Report> {
    let (trigger, popover) = open_placed(page, "resize", "bottom").await?;
    assert_that!(center_shift(&trigger, &popover).await?).is_close_to(0.0, 1.0);
    page.element("#test-opo-move").await?.click().await?;
    assert_that!(|| async { Ok::<_, Report>(trigger.client_rect().await?.left) })
        .eventually_ok()
        .satisfies(|left| {
            left.is_close_to(400.0, 1.0);
        })
        .await;
    page.dispatch_to(
        GlobalTarget::Window,
        SyntheticEvent::plain(EventKind::Resize),
    )
    .await?;
    assert_that!(|| center_shift(&trigger, &popover))
        .eventually_ok()
        .satisfies(|shift| {
            shift.is_close_to(0.0, 1.0);
        })
        .await;
    Ok(())
}

/// A `max_height` smaller than the room available limits the popover's height ("should update the
/// overlay's maxHeight by the given one if it's smaller than available viewport height.").
#[browser_test]
pub async fn max_height_limits(page: &Page<'_>) -> Result<(), Report> {
    let (_, popover) = open_placed(page, "max-height", "bottom").await?;
    assert_that!(popover.client_rect().await?.height).is_close_to(50.0, 1.0);
    Ok(())
}

/// A trigger's margin doesn't move the popover: it opens the default offset (8px) below the
/// trigger's border box (calculatePosition.test.ts "overlay target has margin").
#[browser_test]
pub async fn target_with_margin(page: &Page<'_>) -> Result<(), Report> {
    let (trigger, popover) = open_placed(page, "margin", "bottom").await?;
    assert_that!(gap_below(&trigger, &popover).await?).is_close_to(8.0, 1.0);
    assert_that!(center_shift(&trigger, &popover).await?).is_close_to(0.0, 1.0);
    Ok(())
}

/// `cross_offset` shifts the popover along the trigger's side.
#[browser_test]
pub async fn cross_offset_shifts(page: &Page<'_>) -> Result<(), Report> {
    let (trigger, popover) = open_placed(page, "cross-offset", "bottom").await?;
    assert_that!(center_shift(&trigger, &popover).await?).is_close_to(30.0, 1.0);
    Ok(())
}

/// In a right-to-left subtree, `Start` opens to the right of the trigger and `BottomStart` aligns
/// the right edges.
#[browser_test]
pub async fn start_and_end_follow_rtl(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(OPTIONS, &["rtl"]).await?;
    let start = page.element("#test-opo-start-trigger").await?;
    start.click().await?;
    let popover = page.element(".test-opo-start-popover").await?;
    popover
        .wait_for_attr("data-placement", Some("right"))
        .await?;
    let (trigger, placed) = (start.client_rect().await?, popover.client_rect().await?);
    assert_that!(placed.left - trigger.right).is_close_to(8.0, 1.0);
    start.click().await?;
    page.wait_for_count(".test-opo-start-popover", 0).await?;

    let bottom_start = page.element("#test-opo-bottom-start-trigger").await?;
    bottom_start.click().await?;
    let popover = page.element(".test-opo-bottom-start-popover").await?;
    popover
        .wait_for_attr("data-placement", Some("bottom"))
        .await?;
    let (trigger, placed) = (
        bottom_start.client_rect().await?,
        popover.client_rect().await?,
    );
    assert_that!(placed.right).is_close_to(trigger.right, 1.0);
    Ok(())
}

/// `target_rect` replaces the trigger's rectangle: the popover opens below the given point,
/// centered on it.
#[browser_test]
pub async fn target_rect_replaces_the_trigger(page: &Page<'_>) -> Result<(), Report> {
    let (_, popover) = open_placed(page, "target-rect", "bottom").await?;
    let rect = popover.client_rect().await?;
    assert_that!(rect.top).is_close_to(108.0, 1.0);
    assert_that!(f64::midpoint(rect.left, rect.right)).is_close_to(400.0, 1.0);
    Ok(())
}

/// The arrow keeps `arrow_boundary_offset` (30px) from the popover's edge, also when the trigger's
/// center is beyond it (a trigger at the viewport's edge).
#[browser_test]
pub async fn arrow_boundary_offset(page: &Page<'_>) -> Result<(), Report> {
    let (_, popover) = open_placed(page, "arrow", "bottom").await?;
    let arrow = popover
        .element(".test-opo-arrow")
        .await?
        .client_rect()
        .await?;
    let popover = popover.client_rect().await?;
    assert_that!(popover.left)
        .with_detail_message("the popover keeps the container padding (12px)")
        .is_close_to(12.0, 1.0);
    assert_that!(f64::midpoint(arrow.left, arrow.right) - popover.left)
        .with_detail_message("half the arrow (6px) and the boundary offset (30px)")
        .is_close_to(36.0, 1.0);
    Ok(())
}

/// The popover stays within its `boundary` (with the container padding), not the viewport.
#[browser_test]
pub async fn stays_within_the_boundary(page: &Page<'_>) -> Result<(), Report> {
    let (_, popover) = open_placed(page, "boundary", "bottom").await?;
    assert_that!(popover.client_rect().await?.left).is_close_to(312.0, 1.0);
    Ok(())
}
