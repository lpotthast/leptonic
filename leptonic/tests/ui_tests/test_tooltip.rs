// Upstream: react-aria-components/test/Tooltip.test.js @ 99e6102368
// Upstream: @adobe/react-spectrum/test/tooltip/TooltipTrigger.test.js @ 99e6102368
use std::{
    borrow::Cow,
    time::{Duration, Instant},
};

use assertr::prelude::*;
use browser_test::{BrowserTest, async_trait, thirtyfour::prelude::*};
use rootcause::Report;
use serde::Deserialize;

use crate::{
    pages::{ElementActions, Page, PageActions},
    polling::expect,
};

const TOOLTIP: &str = "[role=tooltip]";

/// `TooltipTrigger` + `Tooltip` atoms on `Button`s: hovering opens the tooltip after the delay,
/// the next one opens right away (warm-up) and replaces the first without animations; leaving
/// closes it after the close delay; focus opens it immediately and Escape closes it. The trigger
/// is described by the tooltip while it is open. `should_close_on_press=false` keeps it open when
/// the trigger is pressed; `trigger=Focus` ignores hovering; scrolling closes it.
pub struct TooltipTests {}

#[async_trait]
impl BrowserTest<str> for TooltipTests {
    fn name(&self) -> Cow<'_, str> {
        "tooltip_tests".into()
    }

    async fn run(&self, driver: &WebDriver, base_url: &str) -> Result<(), Report> {
        let page = Page { driver, base_url };
        page.goto_path("/atoms/tooltip").await?;

        cases!(
            shows_on_hover(&page),
            warm_tooltip_replaces_without_animation(&page),
            shows_on_focus(&page),
            close_on_press_disabled_and_close_delay(&page),
            focus_trigger_mode(&page),
            hide_on_scroll(&page),
        );

        Ok(())
    }
}

/// What the observer installed by [`warm_tooltip_replaces_without_animation`] saw.
#[derive(Debug, Deserialize)]
struct TooltipSwap {
    /// The most tooltips shown at once.
    max: u64,
    /// Whether a tooltip entered or exited with an animation.
    animated: bool,
}

/// "shows on hover", "has a trigger described by the tooltip when open": after the delay, above
/// the trigger. The pointer moves over the page first (as upstream's test): hovering only counts
/// with pointer modality.
async fn shows_on_hover(page: &Page<'_>) -> Result<(), Report> {
    let edit = page.element("#test-tooltip-edit").await?;
    page.element("#test-tooltip-away").await?.hover().await?;
    assert_that!(edit.attr("aria-describedby").await?).is_none();
    edit.hover().await?;
    assert_that!(page.count(TOOLTIP).await?).is_equal_to(0);
    let tooltip = page.element(TOOLTIP).await?;
    // Visible once it faded in.
    tooltip.wait_for_inner_text("Edit the entry").await?;
    let tooltip_id = tooltip.id().await?;
    // Generated once, with one prefix (not "tooltip-tooltip-trigger-…").
    assert_that!(tooltip_id.as_deref())
        .with_detail_message("the tooltip's generated id")
        .get_some()
        .does_not_contain("tooltip-trigger");
    assert_that!(edit.attr("aria-describedby").await?).is_equal_to(tooltip_id);
    assert_that!(tooltip.attr("data-placement").await?)
        .get_some()
        .is_equal_to("top");
    page.wait_for_count("[role=tooltip][data-entering]", 0)
        .await?;
    Ok(())
}

/// "once opened, it can be closed and opened instantly for a period of time", "can only show one
/// tooltip at a time": while warm, the next tooltip opens right away and replaces the first, both
/// without animation (`should_skip_animation`). Leaving closes it after the close delay.
async fn warm_tooltip_replaces_without_animation(page: &Page<'_>) -> Result<(), Report> {
    let delete = page.element("#test-tooltip-delete").await?;
    page.element(TOOLTIP)
        .await?
        .wait_for_inner_text("Edit the entry")
        .await?;
    page.eval::<()>(
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
    let swap: TooltipSwap = page.eval("return window.__tooltipSwap;", vec![]).await?;
    assert_that!(swap.max)
        .with_detail_message("tooltips shown at once during the swap")
        .is_equal_to(1);
    assert_that!(swap.animated)
        .with_detail_message("whether a tooltip animated during the swap")
        .is_false();

    page.element("#test-tooltip-away").await?.hover().await?;
    page.wait_for_count(TOOLTIP, 0).await?;
    assert_that!(delete.attr("aria-describedby").await?).is_none();
    Ok(())
}

/// "shows on focus" right away, "can be keyboard force closed" with Escape.
async fn shows_on_focus(page: &Page<'_>) -> Result<(), Report> {
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

/// "does not close if the trigger is clicked when shouldCloseOnPress is false" (pointer and
/// keyboard), and a close delay: the tooltip stays while the delay runs after the pointer left.
async fn close_on_press_disabled_and_close_delay(page: &Page<'_>) -> Result<(), Report> {
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
    // Settled: still open.
    page.count_stays(TOOLTIP, 1).await?;

    // 800 ms close delay: closed only after it, measured from before the pointer left.
    let left = Instant::now();
    away.hover().await?;
    page.wait_for_count(TOOLTIP, 0).await?;
    assert_that!(left.elapsed())
        .with_detail_message("closed after the close delay")
        .is_greater_or_equal_to(Duration::from_millis(700));
    Ok(())
}

/// `trigger=Focus`: "will not open for hover", "will open for focus".
async fn focus_trigger_mode(page: &Page<'_>) -> Result<(), Report> {
    let focus_only = page.element("#test-tooltip-focus-only").await?;
    focus_only.hover().await?;
    // Settled: hovering opened nothing.
    expect("the number of open tooltips")
        .observing(|| page.count(TOOLTIP))
        .for_at_least(Duration::from_millis(400))
        .to_stay_equal_to(0)
        .await?;

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

/// "should hide tooltip on scroll": scrolling the trigger's scroll parent closes the tooltip.
async fn hide_on_scroll(page: &Page<'_>) -> Result<(), Report> {
    let away = page.element("#test-tooltip-away").await?;
    away.hover().await?;
    let trigger = page.element("#test-tooltip-scroll-trigger").await?;
    trigger.scroll_into_view().await?;
    trigger.hover().await?;
    page.element(TOOLTIP)
        .await?
        .wait_for_inner_text("In a scrolling container")
        .await?;
    page.element("#test-tooltip-scroll-container")
        .await?
        .scroll_to_top(2.0)
        .await?;
    let scrolled = Instant::now();
    page.wait_for_count(TOOLTIP, 0).await?;
    // Right away, not after the close delay (500 ms) of a pointer leaving.
    assert_that!(scrolled.elapsed())
        .with_detail_message("closed right away by the scrolling")
        .is_less_than(Duration::from_millis(400));
    Ok(())
}
