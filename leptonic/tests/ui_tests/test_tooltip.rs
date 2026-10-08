// Upstream: react-aria-components/test/Tooltip.test.js @ 99e6102368
// Upstream: @adobe/react-spectrum/test/tooltip/TooltipTrigger.test.js @ 99e6102368
use std::{
    borrow::Cow,
    time::{Duration, Instant},
};

use assertr::prelude::*;
use browser_test::{
    BrowserTest, async_trait,
    thirtyfour::{Key, WebDriver, WebElement},
};
use rootcause::Report;

use crate::pages::{BaseActions, Page};

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
        hover_focus_and_warm_up(&page).await?;
        close_on_press_disabled_and_close_delay(&page).await?;
        focus_trigger_mode(&page).await?;
        hide_on_scroll(&page).await?;
        page.expect_no_page_errors().await
    }
}

/// "shows on hover", "shows on focus", "can be keyboard force closed", "once opened, it can be
/// closed and opened instantly for a period of time", "can only show one tooltip at a time", "has
/// a trigger described by the tooltip when open".
async fn hover_focus_and_warm_up(page: &Page<'_>) -> Result<(), Report> {
    let edit = page.element("test-tooltip-edit").await?;
    let delete = page.element("test-tooltip-delete").await?;
    let away = page.element("test-tooltip-away").await?;

    // Shows on hover, after the delay, describing the trigger, above it. The pointer moves over
    // the page first (as upstream's test): hovering only counts with pointer modality.
    hover(page.driver, &away).await?;
    assert_that!(edit.attr("aria-describedby").await?).is_none();
    hover(page.driver, &edit).await?;
    assert_that!(page.count_matching(TOOLTIP).await?).is_equal_to(0);
    page.wait_for_selector(TOOLTIP).await?;
    // Visible once it faded in.
    page.wait_for_selector_text(TOOLTIP, "Edit the entry")
        .await?;
    let tooltip = page.css(TOOLTIP).await?;
    let tooltip_id = tooltip.attr("id").await?;
    // Generated once, with one prefix (not "tooltip-tooltip-trigger-…").
    assert_that!(tooltip_id.clone().unwrap_or_default())
        .with_detail_message("the tooltip's generated id")
        .does_not_contain("tooltip-trigger");
    let describedby = edit.attr("aria-describedby").await?;
    assert_that!(describedby).is_equal_to(tooltip_id);
    assert_that!(tooltip.attr("data-placement").await?).is_equal_to(Some("top".to_owned()));
    page.wait_for_no_selector("[role=tooltip][data-entering]")
        .await?;

    // Warm: the next tooltip opens right away and replaces the first, both without animation
    // (`should_skip_animation`): there is never more than one tooltip, and none enters or exits.
    page.driver
        .execute(
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
    hover(page.driver, &delete).await?;
    page.wait_for_selector_text(TOOLTIP, "Delete the entry")
        .await?;
    page.wait_for_count(TOOLTIP, 1).await?;
    let swap: serde_json::Value = page
        .driver
        .execute("return window.__tooltipSwap;", vec![])
        .await?
        .json()
        .clone();
    assert_that!(swap["max"].as_u64())
        .with_detail_message("tooltips shown at once during the swap")
        .is_equal_to(Some(1));
    assert_that!(swap["animated"].as_bool())
        .with_detail_message("whether a tooltip animated during the swap")
        .is_equal_to(Some(false));

    // Leaving closes it after the close delay.
    hover(page.driver, &away).await?;
    page.wait_for_no_selector(TOOLTIP).await?;
    assert_that!(delete.attr("aria-describedby").await?).is_none();

    // Shows on focus right away; Escape closes it.
    page.click_element_with_id("test-tooltip-before").await?;
    page.press_tab().await?;
    page.wait_for_active_id("test-tooltip-edit").await?;
    page.wait_for_selector_text(TOOLTIP, "Edit the entry")
        .await?;
    page.send_keys_to_active(Key::Escape).await?;
    page.wait_for_no_selector(TOOLTIP).await
}

/// "does not close if the trigger is clicked when shouldCloseOnPress is false" (pointer and
/// keyboard), and a close delay: the tooltip stays while the delay runs after the pointer left.
async fn close_on_press_disabled_and_close_delay(page: &Page<'_>) -> Result<(), Report> {
    let save = page.element("test-tooltip-save").await?;
    hover(page.driver, &page.element("test-tooltip-away").await?).await?;
    hover(page.driver, &save).await?;
    page.wait_for_selector_text(TOOLTIP, "Save the entry")
        .await?;
    save.click().await?;
    page.wait_for_text("test-tooltip-saves", "1").await?;
    page.send_keys_to_active(Key::Enter).await?;
    page.wait_for_text("test-tooltip-saves", "2").await?;
    // Settled: still open.
    stays!("the open tooltips", 1, page.count_matching(TOOLTIP).await?);

    // 800 ms close delay.
    let left = Instant::now();
    hover(page.driver, &page.element("test-tooltip-away").await?).await?;
    stays_for!(
        "the open tooltips while the close delay runs",
        Duration::from_millis(400),
        1,
        page.count_matching(TOOLTIP).await?
    );
    page.wait_for_no_selector(TOOLTIP).await?;
    assert_that!(left.elapsed() >= Duration::from_millis(700))
        .with_detail_message("closed after the close delay")
        .is_true();
    Ok(())
}

/// `trigger=Focus`: "will not open for hover", "will open for focus".
async fn focus_trigger_mode(page: &Page<'_>) -> Result<(), Report> {
    let focus_only = page.element("test-tooltip-focus-only").await?;
    hover(page.driver, &focus_only).await?;
    // Settled: hovering opened nothing.
    stays_for!(
        "the open tooltips",
        Duration::from_millis(400),
        0,
        page.count_matching(TOOLTIP).await?
    );

    // Focused by keyboard (from "Save", focused by the press before).
    page.driver
        .execute(
            "document.getElementById('test-tooltip-save').focus()",
            vec![],
        )
        .await?;
    page.press_tab().await?;
    page.wait_for_active_id("test-tooltip-focus-only").await?;
    page.wait_for_selector_text(TOOLTIP, "Shown on focus")
        .await?;
    page.send_keys_to_active(Key::Escape).await?;
    page.wait_for_no_selector(TOOLTIP).await
}

/// "should hide tooltip on scroll": scrolling the trigger's scroll parent closes the tooltip.
async fn hide_on_scroll(page: &Page<'_>) -> Result<(), Report> {
    hover(page.driver, &page.element("test-tooltip-away").await?).await?;
    let trigger = page.element("test-tooltip-scroll-trigger").await?;
    trigger.scroll_into_view().await?;
    hover(page.driver, &trigger).await?;
    page.wait_for_selector_text(TOOLTIP, "In a scrolling container")
        .await?;
    page.driver
        .execute(
            "document.getElementById('test-tooltip-scroll-container').scrollTop = 2;",
            vec![],
        )
        .await?;
    let scrolled = Instant::now();
    page.wait_for_no_selector(TOOLTIP).await?;
    // Right away, not after the close delay (500 ms) of a pointer leaving.
    assert_that!(scrolled.elapsed() < Duration::from_millis(400))
        .with_detail_message("closed right away by the scrolling")
        .is_true();
    Ok(())
}

async fn hover(driver: &WebDriver, element: &WebElement) -> Result<(), Report> {
    driver
        .action_chain()
        .move_to_element_center(element)
        .perform()
        .await?;
    Ok(())
}
