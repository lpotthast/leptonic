// Upstream: react-aria/test/interactions/useHover.test.js @ 99e6102368
//! `use_hover`: hover start/end with the hooked element as target (also over inner elements),
//! no hover by touch, hover ends when disabled and when the hovered child is removed.
use browser_test::thirtyfour::prelude::*;
use rootcause::Report;

use crate::pages::{ElementActions, Page, PageActions, SyntheticEvent};

const PATH: &str = "/hooks/hover";

const START: &str = "start:mouse:test-hover-target,change:true";
const START_END: &str =
    "start:mouse:test-hover-target,change:true,end:mouse:test-hover-target,change:false";

/// The log of hover events.
async fn log(page: &Page<'_>) -> Result<WebElement, Report> {
    page.element("#test-hover-log").await
}

/// Clears the log, by a script click (the pointer stays where it is).
async fn reset(page: &Page<'_>) -> Result<(), Report> {
    page.element("#test-hover-reset")
        .await?
        .virtual_click()
        .await?;
    log(page).await?.wait_for_inner_text("").await?;
    Ok(())
}

/// "hover event target should be the same element we attached listeners to even if we hover over
/// inner elements".
pub async fn target_is_the_hooked_element(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let log = log(page).await?;
    let target = page.element("#test-hover-target").await?;
    page.element("#test-hover-away").await?.hover().await?;
    page.element("#test-hover-inner").await?.hover().await?;
    log.wait_for_inner_text(START).await?;
    target.wait_for_attr("data-hovered", Some("true")).await?;
    page.element("#test-hover-away").await?.hover().await?;
    log.wait_for_inner_text(START_END).await?;
    target.wait_for_attr("data-hovered", None).await?;
    reset(page).await?;
    Ok(())
}

/// "should not fire hover events when pointerType is touch".
pub async fn no_hover_by_touch(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let target = page.element("#test-hover-target").await?;
    for kind in ["pointerenter", "pointerleave"] {
        target
            .dispatch(
                SyntheticEvent::pointer(kind)
                    .with("pointerType", "touch")
                    .with("bubbles", false),
            )
            .await?;
    }
    log(page).await?.inner_text_stays("").await?;
    Ok(())
}

/// "should end hover when disabled" (disabled by a script, the pointer stays), "does not handle
/// hover events if disabled".
pub async fn hover_ends_when_disabled(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let log = log(page).await?;
    let target = page.element("#test-hover-target").await?;
    let disable = page.element("#test-hover-disable").await?;
    page.element("#test-hover-inner").await?.hover().await?;
    log.wait_for_inner_text(START).await?;
    disable.virtual_click().await?;
    log.wait_for_inner_text(START_END).await?;
    target.wait_for_attr("data-hovered", None).await?;

    page.element("#test-hover-away").await?.hover().await?;
    page.element("#test-hover-inner").await?.hover().await?;
    log.inner_text_stays(START_END).await?;
    disable.virtual_click().await?;
    page.element("#test-hover-away").await?.hover().await?;
    reset(page).await?;
    Ok(())
}

/// "should trigger onHoverEnd after an element is removed": the target shrinks, the browser fires
/// no `pointerleave`, only `pointerover` on what is under the pointer now.
pub async fn hover_ends_when_the_element_is_removed(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let log = log(page).await?;
    let remove = page.element("#test-hover-remove").await?;
    remove.hover().await?;
    log.wait_for_inner_text(START).await?;
    remove.click().await?;
    page.wait_for_count("#test-hover-remove", 0).await?;
    page.driver
        .action_chain()
        .move_by_offset(1, 0)
        .perform()
        .await?;
    log.wait_for_inner_text(START_END).await?;
    page.element("#test-hover-target")
        .await?
        .wait_for_attr("data-hovered", None)
        .await?;
    Ok(())
}

/// The `Hoverable` atom sets `data-hovered` on its child.
pub async fn hoverable_atom(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let hoverable = page.element("#test-hoverable").await?;
    hoverable.hover().await?;
    hoverable
        .wait_for_attr("data-hovered", Some("true"))
        .await?;
    page.element("#test-hover-away").await?.hover().await?;
    hoverable.wait_for_attr("data-hovered", None).await?;
    Ok(())
}
