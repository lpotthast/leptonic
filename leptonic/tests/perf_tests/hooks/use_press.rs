// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the hook `use_press` (`/perf/hooks/use_press`, on a plain
//! `<div role="button">`): what each thing it offers costs, measured (`Page::measure`) and
//! checked against its budget (`tests/budgets/hooks/use_press.json`): mounting and unmounting
//! it, presses with the mouse, the keyboard (Enter, Space) and assistive technology (a virtual
//! click), its pressed state (held, dragged out and back), double presses, disabling it and
//! forcing its pressed state.
use assertr::prelude::*;
use browser_test::{TestGroup, browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/hooks/use_press";

/// Registers the performance tests of `use_press`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(PressWithTheMouse))
        .with(fixture_test(HoldAndRelease))
        .with(fixture_test(DragOutAndBack))
        .with(fixture_test(PressWithTheKeyboard))
        .with(fixture_test(VirtualClick))
        .with(fixture_test(DoublePress))
        .with(fixture_test(DisableAndEnable))
        .with(fixture_test(ForcePressed))
}
const PRESSED: &str = "#perf-press";

/// How often the element was pressed.
async fn presses(page: &Page<'_>) -> Result<f64, Report> {
    page.control("presses").await
}

/// Mounting the element with its press handling and unmounting it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(PRESSED, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.element(PRESSED).await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(PRESSED, 0).await?;
    Ok(())
}

/// A click: press start, press up, press end and the press.
#[browser_test]
pub async fn press_with_the_mouse(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let pressed = page.element(PRESSED).await?;
    page.measure("click", pressed.click()).await?;
    assert_that!(presses(page).await?).is_equal_to(1.0);
    Ok(())
}

/// Holding the pointer down (the pressed state) and releasing it (the press).
#[browser_test]
pub async fn hold_and_release(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let pressed = page.element(PRESSED).await?;
    let held = page.measure("hold", pressed.press_and_hold()).await?;
    pressed.wait_for_attr("data-pressed", Some("true")).await?;
    page.measure("release", held.release()).await?;
    pressed.wait_for_attr("data-pressed", None).await?;
    assert_that!(presses(page).await?).is_equal_to(1.0);
    Ok(())
}

/// The held pointer leaving the element (the pressed state ends), coming back (it resumes) and
/// releasing (the press).
#[browser_test]
pub async fn drag_out_and_back(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let pressed = page.element(PRESSED).await?;
    let after = page.element("#perf-after").await?;
    let held = pressed.press_and_hold().await?;
    pressed.wait_for_attr("data-pressed", Some("true")).await?;
    page.measure("leave", held.move_to(&after)).await?;
    pressed.wait_for_attr("data-pressed", None).await?;
    page.measure("return", held.move_to(&pressed)).await?;
    pressed.wait_for_attr("data-pressed", Some("true")).await?;
    page.measure("release", held.release()).await?;
    pressed.wait_for_attr("data-pressed", None).await?;
    assert_that!(presses(page).await?).is_equal_to(1.0);
    Ok(())
}

/// Presses with Enter and with Space, the element focused.
#[browser_test]
pub async fn press_with_the_keyboard(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element("#perf-before").await?.click().await?;
    page.send_keys(Key::Tab).await?;
    let pressed = page.element(PRESSED).await?;
    page.wait_for_focus(&pressed).await?;
    page.measure("enter", page.send_keys(Key::Enter)).await?;
    page.measure("space", page.send_keys(" ")).await?;
    assert_that!(presses(page).await?).is_equal_to(2.0);
    Ok(())
}

/// A click without pointer events, as screen readers make (`element.click()`).
#[browser_test]
pub async fn virtual_click(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let pressed = page.element(PRESSED).await?;
    page.measure("virtual click", pressed.virtual_click())
        .await?;
    assert_that!(presses(page).await?).is_equal_to(1.0);
    Ok(())
}

/// A double click: two presses and a double press.
#[browser_test]
pub async fn double_press(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let pressed = page.element(PRESSED).await?;
    page.measure("double click", pressed.double_click()).await?;
    assert_that!(presses(page).await?).is_equal_to(2.0);
    assert_that!(page.control("double-presses").await?).is_equal_to(1.0);
    Ok(())
}

/// Disabling the press handling, a click it ignores, and enabling it again.
#[browser_test]
pub async fn disable_and_enable(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let pressed = page.element(PRESSED).await?;
    page.measure("disable", page.control("disable")).await?;
    page.measure("ignored click", pressed.click()).await?;
    assert_that!(presses(page).await?).is_equal_to(0.0);
    page.measure("enable", page.control("enable")).await?;
    pressed.click().await?;
    assert_that!(presses(page).await?).is_equal_to(1.0);
    Ok(())
}

/// Forcing the pressed state from outside and releasing it.
#[browser_test]
pub async fn force_pressed(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let pressed = page.element(PRESSED).await?;
    page.measure("force", page.control("force-pressed")).await?;
    pressed.wait_for_attr("data-pressed", Some("true")).await?;
    page.measure("unforce", page.control("unforce-pressed"))
        .await?;
    pressed.wait_for_attr("data-pressed", None).await?;
    Ok(())
}
