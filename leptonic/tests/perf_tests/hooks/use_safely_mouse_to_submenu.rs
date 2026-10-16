// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the hook `use_safely_mouse_to_submenu`
//! (`/perf/hooks/use_safely_mouse_to_submenu`): what each thing it offers costs, measured
//! (`Page::measure`) and checked against its budget
//! (`tests/budgets/hooks/use_safely_mouse_to_submenu.json`): mounting and unmounting it, the
//! submenu opening (listening to the pointer) and closing, and pointer moves: the first one
//! (remembered), one towards the submenu (the menu stops taking pointer events), one away from it
//! and one leaving the menu (ignored, resetting). The hook handles a move at most every 50 ms:
//! the moves are that far apart, and a move towards the submenu starts a 1 s timer, after the
//! measurement.
use std::time::Duration;

use assertr::prelude::*;
use browser_test::{TestGroup, browser_test};
use rootcause::{Report, prelude::ResultExt};

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/hooks/use_safely_mouse_to_submenu";
const MENU: &str = "#perf-menu";

/// Registers the performance tests of `use_safely_mouse_to_submenu`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(OpenAndClose))
        .with(fixture_test(MovingTowardsTheSubmenu))
        .with(fixture_test(MovingAway))
        .with(fixture_test(LeavingTheMenu))
}

/// Moves the pointer to (`x`, `y`) in the viewport.
async fn move_to(page: &Page<'_>, x: i64, y: i64) -> Result<(), Report> {
    page.low_level()
        .driver()
        .action_chain()
        .move_to(x, y)
        .perform()
        .await
        .context_with(|| format!("failed to move the pointer to ({x}, {y})"))?;
    Ok(())
}

/// The menu's `pointer-events` (`none` while the pointer heads for the submenu).
async fn pointer_events(page: &Page<'_>) -> Result<String, Report> {
    page.element(MENU)
        .await?
        .style_property("pointer-events")
        .await
}

/// Opens the submenu with the pointer's modality (a click in the menu first) and makes the first
/// move (remembered), then waits past the hook's throttling (50 ms).
async fn open_and_move(page: &Page<'_>) -> Result<(), Report> {
    page.element(MENU).await?.click().await?;
    page.control("open").await?;
    move_to(page, 50, 200).await?;
    // Past the throttling: the next move is handled.
    let menu = page.element(MENU).await?;
    menu.prop_stays("id", "perf-menu", Duration::from_millis(60))
        .await
}

/// Mounting the hook (closed) and unmounting it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(MENU, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.element(MENU).await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(MENU, 0).await?;
    Ok(())
}

/// The submenu opening (the hook listening to the pointer) and closing.
#[browser_test]
pub async fn open_and_close(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element(MENU).await?.click().await?;
    page.measure("open", page.control("open")).await?;
    page.measure("close", page.control("close")).await?;
    assert_that!(pointer_events(page).await?).is_equal_to(String::new());
    Ok(())
}

/// The first move (remembered), and a move towards the submenu: the menu stops taking pointer
/// events.
#[browser_test]
pub async fn moving_towards_the_submenu(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element(MENU).await?.click().await?;
    page.control("open").await?;
    page.measure("first move", move_to(page, 50, 200)).await?;
    // Past the throttling (50 ms): the next move is handled.
    page.element(MENU)
        .await?
        .prop_stays("id", "perf-menu", Duration::from_millis(60))
        .await?;
    page.measure("towards the submenu", move_to(page, 120, 200))
        .await?;
    assert_that!(pointer_events(page).await?).is_equal_to("none".to_owned());
    Ok(())
}

/// A move away from the submenu (left): the menu keeps taking pointer events.
#[browser_test]
pub async fn moving_away(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    open_and_move(page).await?;
    page.measure("away", move_to(page, 20, 200)).await?;
    assert_that!(pointer_events(page).await?).is_equal_to(String::new());
    Ok(())
}

/// A move out of the menu (below it): the tracking resets.
#[browser_test]
pub async fn leaving_the_menu(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    open_and_move(page).await?;
    page.measure("leave", move_to(page, 50, 350)).await?;
    assert_that!(pointer_events(page).await?).is_equal_to(String::new());
    Ok(())
}
