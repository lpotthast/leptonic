// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the hook `use_move` (`/perf/hooks/use_move`, on a focusable `<div>`): what
//! each thing it offers costs, measured (`Page::measure`) and checked against its budget
//! (`tests/budgets/hooks/use_move.json`): mounting and unmounting it, a drag with the mouse
//! (press, moves, release), the arrow keys (a one-pixel move each), and disabling it (a drag it
//! ignores).
use assertr::prelude::*;
use browser_test::{TestGroup, browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/hooks/use_move";

/// Registers the performance tests of `use_move`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(Drag))
        .with(fixture_test(ArrowKeys))
        .with(fixture_test(DisableAndEnable))
}
const MOVABLE: &str = "#perf-move";

/// The counts of move starts, moves and move ends.
async fn counts(page: &Page<'_>) -> Result<(f64, f64, f64), Report> {
    Ok((
        page.control("starts").await?,
        page.control("moves").await?,
        page.control("ends").await?,
    ))
}

/// Mounting the element with its move handling and unmounting it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(MOVABLE, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.element(MOVABLE).await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(MOVABLE, 0).await?;
    Ok(())
}

/// A drag with the mouse: the press (nothing yet), the first move (move start and a move), a
/// second move, and the release (move end).
#[browser_test]
pub async fn drag(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let movable = page.element(MOVABLE).await?;
    let held = page.measure("press", movable.press_and_hold()).await?;
    assert_that!(counts(page).await?).is_equal_to((0.0, 0.0, 0.0));
    page.measure("first move", held.move_by(10, 0)).await?;
    page.measure("move", held.move_by(10, 0)).await?;
    assert_that!(counts(page).await?).is_equal_to((1.0, 2.0, 0.0));
    assert_that!(page.control("delta-x").await?).is_equal_to(20.0);
    page.measure("release", held.release()).await?;
    assert_that!(counts(page).await?).is_equal_to((1.0, 2.0, 1.0));
    Ok(())
}

/// ArrowRight and ArrowDown on the focused element: move start, a one-pixel move and move end
/// each.
#[browser_test]
pub async fn arrow_keys(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element("#perf-before").await?.click().await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&page.element(MOVABLE).await?).await?;
    page.measure("arrow right", page.send_keys(Key::Right))
        .await?;
    page.measure("arrow down", page.send_keys(Key::Down))
        .await?;
    assert_that!(counts(page).await?).is_equal_to((2.0, 2.0, 2.0));
    assert_that!(page.control("delta-x").await?).is_equal_to(1.0);
    Ok(())
}

/// Disabling the move handling, a drag it ignores, and enabling it again.
#[browser_test]
pub async fn disable_and_enable(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let movable = page.element(MOVABLE).await?;
    page.measure("disable", page.control("disable")).await?;
    let held = page
        .measure("ignored press", movable.press_and_hold())
        .await?;
    page.measure("ignored move", held.move_by(10, 0)).await?;
    page.measure("ignored release", held.release()).await?;
    assert_that!(counts(page).await?).is_equal_to((0.0, 0.0, 0.0));
    page.measure("enable", page.control("enable")).await?;
    Ok(())
}
