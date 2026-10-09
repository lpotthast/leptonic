// Upstream: react-aria-components/test/Tooltip.test.js @ 99e6102368
// Upstream: @adobe/react-spectrum/test/tooltip/TooltipTrigger.test.js @ 99e6102368
// Upstream: react-aria/test/tooltip/useTooltip.test.js @ 99e6102368
//! `TooltipTrigger` + `Tooltip` atoms on `Button`s: hovering opens the tooltip after the delay,
//! the next one opens right away (warm-up) and replaces the first without animations; leaving
//! closes it after the close delay; focus opens it immediately and Escape closes it. The trigger
//! is described by the tooltip while it is open. `should_close_on_press=false` keeps it open when
//! the trigger is pressed; `trigger=Focus` ignores hovering; scrolling closes it.
use std::time::Duration;

use assertr::{matchers::eq, prelude::*};
use browser_test::{browser_test, thirtyfour::prelude::*};
use rootcause::Report;
use serde::Deserialize;

use crate::pages::{ElementActions, EventKind, Page, PointerKind, StopwatchEnd, WebElement};

const PATH: &str = "/atoms/tooltip";

const TOOLTIP: &str = "[role=tooltip]";

/// What the observer installed by [`warm_tooltip_replaces_without_animation`] saw.
#[derive(Debug, Deserialize)]
struct TooltipSwap {
    /// The most tooltips shown at once.
    max: u64,
    /// Whether a tooltip entered or exited with an animation.
    animated: bool,
}

/// Hovering the trigger opens its tooltip above it after the delay, and the trigger is described by
/// the tooltip while it is open ("shows on hover", "has a trigger described by the tooltip when
/// open").
#[browser_test]
pub async fn shows_on_hover(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let edit = page.element("#test-tooltip-edit").await?;
    // The pointer moves over the page first (as upstream's test): hovering only counts with
    // pointer modality.
    page.element("#test-tooltip-away").await?.hover().await?;
    assert_that!(edit)
        .attribute("aria-describedby")
        .await
        .is_none();
    // Not before the delay (300 ms), timed in the page.
    let stopwatch = page
        .start_stopwatch(&edit, PointerKind::Enter, StopwatchEnd::Appears(TOOLTIP))
        .await?;
    edit.hover().await?;
    let tooltip = page.element(TOOLTIP).await?;
    assert_that!(stopwatch.finish().await?)
        .with_detail_message("from the pointer entering to the tooltip (delay: 300 ms)")
        .is_greater_or_equal_to(Duration::from_millis(295));
    // Visible once it faded in.
    tooltip.wait_for_inner_text("Edit the entry").await?;
    // Generated once, with one prefix (not "tooltip-tooltip-trigger-…").
    let tooltip_id = assert_that!(tooltip)
        .with_detail_message("the tooltip's generated id")
        .has_attribute("id")
        .await
        .does_not_contain("tooltip-trigger")
        .actual()
        .clone();
    assert_that!(edit)
        .has_attribute("aria-describedby")
        .await
        .is_equal_to(tooltip_id);
    assert_that!(tooltip)
        .has_attribute("data-placement")
        .await
        .is_equal_to("top");
    page.wait_for_count("[role=tooltip][data-entering]", 0)
        .await?;
    Ok(())
}

/// While tooltips are warm, hovering the next trigger replaces the open tooltip right away without
/// animation, and leaving closes it ("once opened, it can be closed and opened instantly for a
/// period of time", "can only show one tooltip at a time").
#[browser_test]
pub async fn warm_tooltip_replaces_without_animation(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let delete = page.element("#test-tooltip-delete").await?;
    // Warm: the Edit tooltip opened by hovering (with pointer modality, as in
    // `shows_on_hover`) and finished fading in.
    page.element("#test-tooltip-away").await?.hover().await?;
    page.element("#test-tooltip-edit").await?.hover().await?;
    page.element(TOOLTIP)
        .await?
        .wait_for_inner_text("Edit the entry")
        .await?;
    page.wait_for_count("[role=tooltip][data-entering]", 0)
        .await?;
    page.low_level().eval::<()>(
        "window.__tooltipSwap = { max: 0, animated: false };
         new MutationObserver(() => {
             const tooltips = document.querySelectorAll('[role=tooltip]');
             window.__tooltipSwap.max = Math.max(window.__tooltipSwap.max, tooltips.length);
             if (document.querySelector('[role=tooltip][data-entering], [role=tooltip][data-exiting]')) {
                 window.__tooltipSwap.animated = true;
             }
         }).observe(document.body, {
             subtree: true,
             childList: true,
             attributes: true,
             attributeFilter: ['data-entering', 'data-exiting'],
         });",
        vec![],
    )
    .await?;
    delete.hover().await?;
    page.element(TOOLTIP)
        .await?
        .wait_for_inner_text("Delete the entry")
        .await?;
    page.wait_for_count(TOOLTIP, 1).await?;
    let swap: TooltipSwap = page
        .low_level()
        .eval("return window.__tooltipSwap;", vec![])
        .await?;
    assert_that!(swap.max)
        .with_detail_message("tooltips shown at once during the swap")
        .is_equal_to(1);
    assert_that!(swap.animated)
        .with_detail_message("whether a tooltip animated during the swap")
        .is_false();

    page.element("#test-tooltip-away").await?.hover().await?;
    page.wait_for_count(TOOLTIP, 0).await?;
    assert_that!(delete)
        .attribute("aria-describedby")
        .await
        .is_none();
    Ok(())
}

/// Focusing the trigger with Tab opens its tooltip right away, and Escape closes it ("shows on
/// focus", "can be keyboard force closed").
#[browser_test]
pub async fn shows_on_focus(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element("#test-tooltip-before").await?.click().await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&page.element("#test-tooltip-edit").await?)
        .await?;
    page.element(TOOLTIP)
        .await?
        .wait_for_inner_text("Edit the entry")
        .await?;
    page.send_keys(Key::Escape).await?;
    page.wait_for_count(TOOLTIP, 0).await?;
    Ok(())
}

/// With `should_close_on_press=false`, pressing the trigger by pointer or Enter keeps the tooltip
/// open, and leaving closes it only after the close delay ("does not close if the trigger is
/// clicked when shouldCloseOnPress is false", "does not close if the trigger is clicked with the
/// keyboard when shouldCloseOnPress is false").
#[browser_test]
pub async fn close_on_press_disabled_and_close_delay(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let save = page.element("#test-tooltip-save").await?;
    let away = page.element("#test-tooltip-away").await?;
    away.hover().await?;
    save.hover().await?;
    page.element(TOOLTIP)
        .await?
        .wait_for_inner_text("Save the entry")
        .await?;
    save.click().await?;
    let saves = page.element("#test-tooltip-saves").await?;
    saves.wait_for_inner_text("1").await?;
    page.send_keys(Key::Enter).await?;
    saves.wait_for_inner_text("2").await?;
    // Still open, also past the close delay (400 ms) a press could have started.
    page.settle().await?;
    assert_that!(|| page.count(TOOLTIP))
        .consistently_ok()
        .for_at_least(Duration::from_millis(500))
        .matches(eq(1))
        .await;

    // 400 ms close delay: closed only after it, measured on the page's clock from the pointer
    // leaving.
    let stopwatch = page
        .start_stopwatch(&save, PointerKind::Leave, StopwatchEnd::Disappears(TOOLTIP))
        .await?;
    away.hover().await?;
    page.wait_for_count(TOOLTIP, 0).await?;
    assert_that!(stopwatch.finish().await?)
        .with_detail_message("closed after the close delay")
        .is_greater_or_equal_to(Duration::from_millis(390));
    Ok(())
}

/// A tooltip triggered by focus only doesn't open on hover but opens on keyboard focus, and Escape
/// closes it ("will not open for hover", "will open for focus").
#[browser_test]
pub async fn focus_trigger_mode(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let focus_only = page.element("#test-tooltip-focus-only").await?;
    focus_only.hover().await?;
    // Settled: hovering opened nothing.
    page.settle().await?;
    assert_that!(|| page.count(TOOLTIP))
        .consistently_ok()
        .for_at_least(Duration::from_millis(400))
        .matches(eq(0))
        .await;

    // Focused by keyboard (from "Save", focused by the press before).
    page.element("#test-tooltip-save").await?.focus().await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&focus_only).await?;
    page.element(TOOLTIP)
        .await?
        .wait_for_inner_text("Shown on focus")
        .await?;
    page.send_keys(Key::Escape).await?;
    page.wait_for_count(TOOLTIP, 0).await?;
    Ok(())
}

/// Scrolling the trigger's scroll parent closes its tooltip right away ("should hide tooltip on
/// scroll").
#[browser_test]
pub async fn hide_on_scroll(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let away = page.element("#test-tooltip-away").await?;
    away.hover().await?;
    let trigger = page.element("#test-tooltip-scroll-trigger").await?;
    trigger.scroll_into_view().await?;
    trigger.hover().await?;
    page.element(TOOLTIP)
        .await?
        .wait_for_inner_text("In a scrolling container")
        .await?;
    let container = page.element("#test-tooltip-scroll-container").await?;
    let stopwatch = page
        .start_stopwatch(
            &container,
            EventKind::Scroll,
            StopwatchEnd::Disappears(TOOLTIP),
        )
        .await?;
    container.scroll_to_top(2.0).await?;
    page.wait_for_count(TOOLTIP, 0).await?;
    // Right away (on the page's clock), not after the close delay (500 ms) of a pointer leaving.
    assert_that!(stopwatch.finish().await?)
        .with_detail_message("closed right away by the scrolling")
        .is_less_than(Duration::from_millis(250));
    Ok(())
}

/// Without a delay, hovering the trigger opens the tooltip at once, measured on the page's clock
/// (useTooltip.test.js "opens tooltip immediately on hover with `delay: 0`").
#[browser_test]
pub async fn opens_at_once_without_delay(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element("#test-tooltip-away").await?.hover().await?;
    let trigger = page.element("#test-tooltip-scroll-trigger").await?;
    trigger.scroll_into_view().await?;
    let stopwatch = page
        .start_stopwatch(&trigger, PointerKind::Enter, StopwatchEnd::Appears(TOOLTIP))
        .await?;
    trigger.hover().await?;
    page.element(TOOLTIP).await?;
    assert_that!(stopwatch.finish().await?)
        .with_detail_message("opened without a delay")
        .is_less_than(Duration::from_millis(150));
    Ok(())
}

/// Opens the tooltip of "Save" (close delay 400 ms) by hovering it and returns the trigger and the
/// tooltip.
async fn hover_save(page: &Page<'_>) -> Result<(WebElement, WebElement), Report> {
    page.goto_path(PATH).await?;
    page.element("#test-tooltip-away").await?.hover().await?;
    let save = page.element("#test-tooltip-save").await?;
    save.hover().await?;
    let tooltip = page.element(TOOLTIP).await?;
    tooltip.wait_for_inner_text("Save the entry").await?;
    Ok((save, tooltip))
}

/// The tooltip stays open while the pointer moves from the trigger onto it, past the close delay
/// (useTooltip.test.js "keeps tooltip open when it gets hovered").
#[browser_test]
pub async fn stays_open_while_hovered(page: &Page<'_>) -> Result<(), Report> {
    let (_, tooltip) = hover_save(page).await?;
    tooltip.hover().await?;
    page.settle().await?;
    assert_that!(|| page.count(TOOLTIP))
        .consistently_ok()
        // Past the close delay (400 ms) the trigger's pointer leave started.
        .for_at_least(Duration::from_millis(600))
        .matches(eq(1))
        .await;
    Ok(())
}

/// The pointer leaving the hovered tooltip closes it (useTooltip.test.js "hides tooltip when hover
/// leaves").
#[browser_test]
pub async fn closes_when_the_pointer_leaves_it(page: &Page<'_>) -> Result<(), Report> {
    let (_, tooltip) = hover_save(page).await?;
    tooltip.hover().await?;
    page.element("#test-tooltip-away").await?.hover().await?;
    page.wait_for_count(TOOLTIP, 0).await?;
    Ok(())
}

/// The tooltip stays open when the pointer returns from it to the trigger (useTooltip.test.js
/// "keeps tooltip open when hover returns to trigger from the tooltip").
#[browser_test]
pub async fn stays_open_back_on_the_trigger(page: &Page<'_>) -> Result<(), Report> {
    let (save, tooltip) = hover_save(page).await?;
    tooltip.hover().await?;
    save.hover().await?;
    page.settle().await?;
    assert_that!(|| page.count(TOOLTIP))
        .consistently_ok()
        // Past the close delay (400 ms) the pointer leaving the tooltip started.
        .for_at_least(Duration::from_millis(600))
        .matches(eq(1))
        .await;
    Ok(())
}
