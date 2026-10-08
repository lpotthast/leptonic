// Upstream: react-aria/test/interactions/useMove.test.js @ 99e6102368
//! `use_move`: pointer movement (start on the first move, deltas, end on pointer up or cancel),
//! nothing for right clicks, taps or further pointers, no bubbling to a movable parent, arrow
//! keys, other keys passed on.
use browser_test::thirtyfour::prelude::*;
use rootcause::Report;

use crate::pages::{ElementActions, Page, PageActions, SyntheticEvent};

const PATH: &str = "/hooks/move";

/// The log of move events.
async fn log(page: &Page<'_>) -> Result<WebElement, Report> {
    page.element("#test-move-log").await
}

/// Clears the log, by a script click.
async fn reset(page: &Page<'_>) -> Result<(), Report> {
    page.element("#test-move-reset")
        .await?
        .virtual_click()
        .await?;
    log(page).await?.wait_for_inner_text("").await?;
    Ok(())
}

/// A pen pointer event: `kind` of pointer `pointer_id` at (`x`, `y`) with `button` (0: primary,
/// 2: secondary). The hook listens on the document while moving, so it bubbles.
fn pen(kind: &str, pointer_id: i32, x: i32, y: i32, button: i32) -> SyntheticEvent {
    let buttons = if kind == "pointerup" || kind == "pointercancel" {
        0
    } else {
        1 << button.min(1)
    };
    SyntheticEvent::pointer(kind)
        .with("pointerType", "pen")
        .with("pointerId", pointer_id)
        .with("isPrimary", pointer_id == 1)
        .with("clientX", x)
        .with("clientY", y)
        .with("button", button)
        .with("buttons", buttons)
}

/// "responds to pointer events": the first move starts, up ends.
pub async fn responds_to_pointer_events(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let log = log(page).await?;
    let single = page.element("#test-move-single").await?;
    single.dispatch(pen("pointerdown", 1, 1, 30, 0)).await?;
    log.inner_text_stays("").await?;
    single.dispatch(pen("pointermove", 1, 10, 25, 0)).await?;
    log.wait_for_inner_text("single:start:pen,single:move:pen:9:-5")
        .await?;
    single.dispatch(pen("pointerup", 1, 10, 25, 0)).await?;
    log.wait_for_inner_text("single:start:pen,single:move:pen:9:-5,single:end:pen")
        .await?;
    reset(page).await?;
    Ok(())
}

/// "ends with pointercancel".
pub async fn ends_with_pointercancel(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let single = page.element("#test-move-single").await?;
    single.dispatch(pen("pointerdown", 1, 1, 30, 0)).await?;
    single.dispatch(pen("pointermove", 1, 10, 25, 0)).await?;
    single.dispatch(pen("pointercancel", 1, 10, 25, 0)).await?;
    log(page)
        .await?
        .wait_for_inner_text("single:start:pen,single:move:pen:9:-5,single:end:pen")
        .await?;
    reset(page).await?;
    Ok(())
}

/// "doesn't respond to right click", "doesn't fire anything when tapping".
pub async fn ignores_right_clicks_and_taps(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let single = page.element("#test-move-single").await?;
    single.dispatch(pen("pointerdown", 1, 1, 30, 2)).await?;
    single.dispatch(pen("pointermove", 1, 10, 25, 2)).await?;
    single.dispatch(pen("pointerup", 1, 10, 25, 2)).await?;
    single.dispatch(pen("pointerdown", 1, 1, 30, 0)).await?;
    single.dispatch(pen("pointerup", 1, 1, 30, 0)).await?;
    log(page).await?.inner_text_stays("").await?;
    Ok(())
}

/// "ignores any additional pointers".
pub async fn ignores_additional_pointers(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let log = log(page).await?;
    let single = page.element("#test-move-single").await?;
    single.dispatch(pen("pointerdown", 1, 1, 30, 0)).await?;
    single.dispatch(pen("pointerdown", 3, 1, 30, 0)).await?;
    single.dispatch(pen("pointermove", 3, 1, 40, 0)).await?;
    single.dispatch(pen("pointerup", 3, 1, 40, 0)).await?;
    log.inner_text_stays("").await?;
    single.dispatch(pen("pointermove", 1, 10, 25, 0)).await?;
    single.dispatch(pen("pointerup", 1, 10, 25, 0)).await?;
    log.wait_for_inner_text("single:start:pen,single:move:pen:9:-5,single:end:pen")
        .await?;
    reset(page).await?;
    Ok(())
}

/// "doesn't bubble to useMove on parent elements".
pub async fn doesnt_bubble_to_a_movable_parent(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let child = page.element("#test-move-child").await?;
    child.dispatch(pen("pointerdown", 1, 1, 30, 0)).await?;
    child.dispatch(pen("pointermove", 1, 10, 25, 0)).await?;
    child.dispatch(pen("pointerup", 1, 10, 25, 0)).await?;
    log(page)
        .await?
        .wait_for_inner_text("child:start:pen,child:move:pen:9:-5,child:end:pen")
        .await?;
    reset(page).await?;
    Ok(())
}

/// "responds to keypresses", "allows handling other key events".
pub async fn responds_to_keys(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element("#test-move-single").await?.focus().await?;
    for key in [Key::Up, Key::Down, Key::Left, Key::Right, Key::PageUp] {
        page.send_keys(key).await?;
    }
    log(page)
        .await?
        .wait_for_inner_text(
            "single:start:keyboard,single:move:keyboard:0:-1,single:end:keyboard,\
             single:start:keyboard,single:move:keyboard:0:1,single:end:keyboard,\
             single:start:keyboard,single:move:keyboard:-1:0,single:end:keyboard,\
             single:start:keyboard,single:move:keyboard:1:0,single:end:keyboard,\
             keydown:PageUp",
        )
        .await?;
    Ok(())
}
