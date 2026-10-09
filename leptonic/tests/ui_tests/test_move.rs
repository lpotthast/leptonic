// Upstream: react-aria/test/interactions/useMove.test.js @ 99e6102368
//! `use_move`: pointer movement (start on the first move, deltas, end on pointer up or cancel),
//! nothing for right clicks, taps or further pointers, no bubbling to a movable parent, arrow
//! keys, other keys passed on.
use browser_test::{browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::pages::{
    ElementActions, MouseButton, Page, PointerKind, PointerType, SyntheticEvent, interface::Pointer,
};

const PATH: &str = "/hooks/move";

/// The log of move events.
async fn log(page: &Page<'_>) -> Result<WebElement, Report> {
    page.element("#test-move-log").await
}

/// A pen pointer event: `kind` of pointer `pointer_id` at (`x`, `y`) with `button` (0: primary,
/// 2: secondary). The hook listens on the document while moving, so it bubbles.
fn pen(
    kind: PointerKind,
    pointer_id: i32,
    x: f64,
    y: f64,
    button: MouseButton,
) -> SyntheticEvent<Pointer> {
    let buttons: &[MouseButton] = match kind {
        PointerKind::Up | PointerKind::Cancel => &[],
        _ => &[button],
    };
    SyntheticEvent::pointer(kind)
        .pointer_type(PointerType::Pen)
        .pointer_id(pointer_id)
        .at(x, y)
        .button(button)
        .buttons(buttons)
}

/// A pointer down starts nothing, the first move fires move start and a move with its delta, and
/// the pointer up fires move end ("responds to pointer events").
#[browser_test]
pub async fn responds_to_pointer_events(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let log = log(page).await?;
    let single = page.element("#test-move-single").await?;
    single
        .dispatch(pen(PointerKind::Down, 1, 1.0, 30.0, MouseButton::Primary))
        .await?;
    log.inner_text_stays("", std::time::Duration::from_millis(100))
        .await?;
    single
        .dispatch(pen(PointerKind::Move, 1, 10.0, 25.0, MouseButton::Primary))
        .await?;
    log.wait_for_inner_text("single:start:pen,single:move:pen:9:-5")
        .await?;
    single
        .dispatch(pen(PointerKind::Up, 1, 10.0, 25.0, MouseButton::Primary))
        .await?;
    log.wait_for_inner_text("single:start:pen,single:move:pen:9:-5,single:end:pen")
        .await?;
    Ok(())
}

/// A pointer cancel during a move fires move end ("ends with pointercancel").
#[browser_test]
pub async fn ends_with_pointercancel(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let single = page.element("#test-move-single").await?;
    single
        .dispatch(pen(PointerKind::Down, 1, 1.0, 30.0, MouseButton::Primary))
        .await?;
    single
        .dispatch(pen(PointerKind::Move, 1, 10.0, 25.0, MouseButton::Primary))
        .await?;
    single
        .dispatch(pen(
            PointerKind::Cancel,
            1,
            10.0,
            25.0,
            MouseButton::Primary,
        ))
        .await?;
    log(page)
        .await?
        .wait_for_inner_text("single:start:pen,single:move:pen:9:-5,single:end:pen")
        .await?;
    Ok(())
}

/// A secondary-button drag and a tap without movement fire no move events ("doesn't respond to
/// right click", "doesn't fire anything when tapping").
#[browser_test]
pub async fn ignores_right_clicks_and_taps(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let single = page.element("#test-move-single").await?;
    single
        .dispatch(pen(PointerKind::Down, 1, 1.0, 30.0, MouseButton::Secondary))
        .await?;
    single
        .dispatch(pen(
            PointerKind::Move,
            1,
            10.0,
            25.0,
            MouseButton::Secondary,
        ))
        .await?;
    single
        .dispatch(pen(PointerKind::Up, 1, 10.0, 25.0, MouseButton::Secondary))
        .await?;
    single
        .dispatch(pen(PointerKind::Down, 1, 1.0, 30.0, MouseButton::Primary))
        .await?;
    single
        .dispatch(pen(PointerKind::Up, 1, 1.0, 30.0, MouseButton::Primary))
        .await?;
    log(page)
        .await?
        .inner_text_stays("", std::time::Duration::from_millis(100))
        .await?;
    Ok(())
}

/// During a move, a second pointer's events are ignored while the first pointer's still move
/// ("ignores any additional pointers").
#[browser_test]
pub async fn ignores_additional_pointers(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let log = log(page).await?;
    let single = page.element("#test-move-single").await?;
    single
        .dispatch(pen(PointerKind::Down, 1, 1.0, 30.0, MouseButton::Primary))
        .await?;
    single
        .dispatch(pen(PointerKind::Down, 3, 1.0, 30.0, MouseButton::Primary))
        .await?;
    single
        .dispatch(pen(PointerKind::Move, 3, 1.0, 40.0, MouseButton::Primary))
        .await?;
    single
        .dispatch(pen(PointerKind::Up, 3, 1.0, 40.0, MouseButton::Primary))
        .await?;
    log.inner_text_stays("", std::time::Duration::from_millis(100))
        .await?;
    single
        .dispatch(pen(PointerKind::Move, 1, 10.0, 25.0, MouseButton::Primary))
        .await?;
    single
        .dispatch(pen(PointerKind::Up, 1, 10.0, 25.0, MouseButton::Primary))
        .await?;
    log.wait_for_inner_text("single:start:pen,single:move:pen:9:-5,single:end:pen")
        .await?;
    Ok(())
}

/// Moving a movable child fires move events only on the child, not on its movable parent
/// ("doesn't bubble to useMove on parent elements").
#[browser_test]
pub async fn doesnt_bubble_to_a_movable_parent(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let child = page.element("#test-move-child").await?;
    child
        .dispatch(pen(PointerKind::Down, 1, 1.0, 30.0, MouseButton::Primary))
        .await?;
    child
        .dispatch(pen(PointerKind::Move, 1, 10.0, 25.0, MouseButton::Primary))
        .await?;
    child
        .dispatch(pen(PointerKind::Up, 1, 10.0, 25.0, MouseButton::Primary))
        .await?;
    let log = log(page).await?;
    log.wait_for_inner_text("child:start:pen,child:move:pen:9:-5,child:end:pen")
        .await?;
    // Nothing reaches the parent.
    log.inner_text_stays(
        "child:start:pen,child:move:pen:9:-5,child:end:pen",
        std::time::Duration::from_millis(100),
    )
    .await?;
    Ok(())
}

/// Each arrow key fires move start, a one-pixel move in its direction and move end, while other
/// keys reach the element's own key handler ("responds to keypresses: $Key", "allows handling
/// other key events").
#[browser_test]
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
