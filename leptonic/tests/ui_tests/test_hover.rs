// Upstream: react-aria/test/interactions/useHover.test.js @ 99e6102368
//! `use_hover`: hover start/end with the hooked element as target (also over inner elements),
//! no hover by touch, hover ends when disabled and when the hovered child is removed.
use browser_test::{browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::pages::{ElementActions, Page, PointerKind, PointerType, SyntheticEvent};

const PATH: &str = "/hooks/hover";

const START: &str = "start:mouse:test-hover-target,change:true";
const START_END: &str =
    "start:mouse:test-hover-target,change:true,end:mouse:test-hover-target,change:false";

/// The log of hover events.
async fn log(page: &Page<'_>) -> Result<WebElement, Report> {
    page.element("#test-hover-log").await
}

/// Moving the mouse onto an inner element starts hover with the hooked element as target, and
/// moving away ends it ("hover event target should be the same element we attached listeners to
/// even if we hover over inner elements").
#[browser_test]
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
    Ok(())
}

/// Touch pointers entering and leaving the element fire no hover events ("should not fire hover
/// events when pointerType is touch").
#[browser_test]
pub async fn no_hover_by_touch(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let target = page.element("#test-hover-target").await?;
    for kind in [PointerKind::Enter, PointerKind::Leave] {
        target
            .dispatch(SyntheticEvent::pointer(kind).pointer_type(PointerType::Touch))
            .await?;
    }
    log(page)
        .await?
        .inner_text_stays("", std::time::Duration::from_millis(100))
        .await?;
    Ok(())
}

/// Disabling the hovered element ends its hover, and hovering it again while disabled starts none
/// ("should end hover when disabled", "does not handle hover events if disabled").
#[browser_test]
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
    log.inner_text_stays(START_END, std::time::Duration::from_millis(100))
        .await?;
    Ok(())
}

/// Removing the hovered child, which shrinks the target away from the pointer without a
/// `pointerleave`, ends the hover on the next pointer move ("should trigger onHoverEnd after an
/// element is removed").
#[browser_test]
pub async fn hover_ends_when_the_element_is_removed(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let log = log(page).await?;
    let remove = page.element("#test-hover-remove").await?;
    remove.hover().await?;
    log.wait_for_inner_text(START).await?;
    remove.click().await?;
    page.wait_for_count("#test-hover-remove", 0).await?;
    page.low_level()
        .driver()
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

/// The `Hoverable` atom sets `data-hovered` on its child while the mouse is over it.
#[browser_test]
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

/// A touch: pointer down, enter, leave and up of a touch pointer on the target.
async fn touch(target: &WebElement) -> Result<(), Report> {
    for kind in [
        PointerKind::Down,
        PointerKind::Enter,
        PointerKind::Leave,
        PointerKind::Up,
    ] {
        target
            .dispatch(SyntheticEvent::pointer(kind).pointer_type(PointerType::Touch))
            .await?;
    }
    Ok(())
}

/// The mouse pointer events iOS emulates right after a touch start no hover ("ignores emulated
/// mouse events following touch events").
#[browser_test]
pub async fn ignores_emulated_mouse_events_after_touch(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let target = page.element("#test-hover-target").await?;
    touch(&target).await?;
    // Within the 500 ms after the touch's pointer up.
    for kind in [PointerKind::Enter, PointerKind::Leave] {
        target.dispatch(SyntheticEvent::pointer(kind)).await?;
    }
    log(page)
        .await?
        .inner_text_stays("", std::time::Duration::from_millis(100))
        .await?;
    Ok(())
}

/// Mouse pointer events more than 500 ms after a touch hover again ("ignores supports mouse events
/// following touch events after a delay").
#[browser_test]
pub async fn mouse_hovers_again_after_a_touch(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let target = page.element("#test-hover-target").await?;
    touch(&target).await?;
    // A real timer: past the 500 ms in which emulated mouse events are ignored.
    tokio::time::sleep(std::time::Duration::from_millis(600)).await;
    for kind in [PointerKind::Enter, PointerKind::Leave] {
        target.dispatch(SyntheticEvent::pointer(kind)).await?;
    }
    log(page).await?.wait_for_inner_text(START_END).await?;
    Ok(())
}
