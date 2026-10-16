// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the atom `PressResponder` (`/perf/atoms/press_responder`, around a
//! `Button`): what each thing it offers costs, measured (`Page::measure`) and checked against its
//! budget (`tests/budgets/atoms/press_responder.json`): mounting and unmounting it with its
//! button, presses with the mouse and the keyboard (through the responder's handler and the
//! button's), the pressed state, and the props it gives the button (disabled, forced pressed
//! state).
use assertr::prelude::*;
use browser_test::{TestGroup, browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/atoms/press_responder";

/// Registers the performance tests of `PressResponder`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(PressWithTheMouse))
        .with(fixture_test(HoldAndRelease))
        .with(fixture_test(PressWithEnter))
        .with(fixture_test(DisableAndEnable))
        .with(fixture_test(ForcePressed))
}
const BUTTON: &str = "#perf-button";

/// How often the responder and the button saw a press.
async fn presses(page: &Page<'_>) -> Result<(f64, f64), Report> {
    Ok((
        page.control("presses").await?,
        page.control("button-presses").await?,
    ))
}

/// Mounting the responder with its button and unmounting it.
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

/// A click: the responder's press handler and the button's.
#[browser_test]
pub async fn press_with_the_mouse(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let button = page.element(BUTTON).await?;
    page.measure("click", button.click()).await?;
    assert_that!(presses(page).await?).is_equal_to((1.0, 1.0));
    Ok(())
}

/// Holding the pointer down (the pressed state) and releasing it (the press).
#[browser_test]
pub async fn hold_and_release(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let button = page.element(BUTTON).await?;
    let held = page.measure("hold", button.press_and_hold()).await?;
    button.wait_for_attr("data-pressed", Some("true")).await?;
    page.measure("release", held.release()).await?;
    button.wait_for_attr("data-pressed", None).await?;
    assert_that!(presses(page).await?).is_equal_to((1.0, 1.0));
    Ok(())
}

/// A press with Enter, the button focused.
#[browser_test]
pub async fn press_with_enter(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element("#perf-before").await?.click().await?;
    page.send_keys(Key::Tab).await?;
    let button = page.element(BUTTON).await?;
    page.wait_for_focus(&button).await?;
    page.measure("enter", page.send_keys(Key::Enter)).await?;
    assert_that!(presses(page).await?).is_equal_to((1.0, 1.0));
    Ok(())
}

/// Disabling the button through the responder, a click it ignores, and enabling it again.
#[browser_test]
pub async fn disable_and_enable(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let button = page.element(BUTTON).await?;
    page.measure("disable", page.control("disable")).await?;
    button.wait_for_attr("data-disabled", Some("true")).await?;
    page.measure("ignored click", button.click()).await?;
    assert_that!(presses(page).await?).is_equal_to((0.0, 0.0));
    page.measure("enable", page.control("enable")).await?;
    button.wait_for_attr("data-disabled", None).await?;
    Ok(())
}

/// Forcing the button's pressed state through the responder and releasing it.
#[browser_test]
pub async fn force_pressed(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let button = page.element(BUTTON).await?;
    page.measure("force", page.control("force-pressed")).await?;
    button.wait_for_attr("data-pressed", Some("true")).await?;
    page.measure("unforce", page.control("unforce-pressed"))
        .await?;
    button.wait_for_attr("data-pressed", None).await?;
    Ok(())
}
