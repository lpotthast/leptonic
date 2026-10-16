// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the atom `ClearPressResponder` (`/perf/atoms/clear_press_responder`,
//! around a `Button` inside a `PressResponder`): what each thing it offers costs, measured
//! (`Page::measure`) and checked against its budget
//! (`tests/budgets/atoms/clear_press_responder.json`): mounting and unmounting it with its
//! button, and presses of the button, which the surrounding responder doesn't see.
use assertr::prelude::*;
use browser_test::{TestGroup, browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::{harness::fixture_test, pages::Page};

const PATH: &str = "/perf/atoms/clear_press_responder";

/// Registers the performance tests of `ClearPressResponder`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(PressWithTheMouse))
        .with(fixture_test(PressWithEnter))
}
const BUTTON: &str = "#perf-button";

/// How often the button and the surrounding responder saw a press.
async fn presses(page: &Page<'_>) -> Result<(f64, f64), Report> {
    Ok((
        page.control("presses").await?,
        page.control("responder-presses").await?,
    ))
}

/// Mounting the cleared button and unmounting it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(BUTTON, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.element(BUTTON).await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(BUTTON, 0).await?;
    Ok(())
}

/// A click of the button: its own press only.
#[browser_test]
pub async fn press_with_the_mouse(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let button = page.element(BUTTON).await?;
    page.measure("click", button.click()).await?;
    assert_that!(presses(page).await?).is_equal_to((1.0, 0.0));
    Ok(())
}

/// A press of the button with Enter: its own press only (the responder's press is the trigger's
/// click that put the focus next to the button).
#[browser_test]
pub async fn press_with_enter(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let button = page.element(BUTTON).await?;
    page.element("#perf-trigger").await?.click().await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&button).await?;
    page.measure("enter", page.send_keys(Key::Enter)).await?;
    assert_that!(presses(page).await?).is_equal_to((1.0, 1.0));
    Ok(())
}
