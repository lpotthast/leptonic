// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the atom `Pressable` (`/perf/atoms/pressable`, on a
//! `<span role="button">`): what each thing it offers costs, measured (`Page::measure`) and
//! checked against its budget (`tests/budgets/atoms/pressable.json`): mounting and unmounting
//! it, presses with the mouse and the keyboard, its pressed state, double presses, keyboard focus
//! (it makes its child focusable), and disabling it.
use assertr::prelude::*;
use browser_test::{TestGroup, browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/atoms/pressable";

/// Registers the performance tests of `Pressable`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(PressWithTheMouse))
        .with(fixture_test(HoldAndRelease))
        .with(fixture_test(PressWithTheKeyboard))
        .with(fixture_test(DoublePress))
        .with(fixture_test(KeyboardFocusAndBlur))
        .with(fixture_test(DisableAndEnable))
}
const PRESSABLE: &str = "#perf-pressable";

/// How often the child was pressed.
async fn presses(page: &Page<'_>) -> Result<f64, Report> {
    page.control("presses").await
}

/// Mounting the pressable child and unmounting it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(PRESSABLE, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.element(PRESSABLE).await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(PRESSABLE, 0).await?;
    Ok(())
}

/// A click: press start, press end and the press.
#[browser_test]
pub async fn press_with_the_mouse(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let pressable = page.element(PRESSABLE).await?;
    page.measure("click", pressable.click()).await?;
    assert_that!(presses(page).await?).is_equal_to(1.0);
    Ok(())
}

/// Holding the pointer down (the pressed state) and releasing it (the press).
#[browser_test]
pub async fn hold_and_release(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let pressable = page.element(PRESSABLE).await?;
    let held = page.measure("hold", pressable.press_and_hold()).await?;
    pressable
        .wait_for_attr("data-pressed", Some("true"))
        .await?;
    page.measure("release", held.release()).await?;
    pressable.wait_for_attr("data-pressed", None).await?;
    assert_that!(presses(page).await?).is_equal_to(1.0);
    Ok(())
}

/// Presses with Enter and with Space, the child focused.
#[browser_test]
pub async fn press_with_the_keyboard(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element("#perf-before").await?.click().await?;
    page.send_keys(Key::Tab).await?;
    let pressable = page.element(PRESSABLE).await?;
    page.wait_for_focus(&pressable).await?;
    page.measure("enter", page.send_keys(Key::Enter)).await?;
    page.measure("space", page.send_keys(" ")).await?;
    assert_that!(presses(page).await?).is_equal_to(2.0);
    Ok(())
}

/// A double click: two presses and a double press.
#[browser_test]
pub async fn double_press(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let pressable = page.element(PRESSABLE).await?;
    page.measure("double click", pressable.double_click())
        .await?;
    assert_that!(presses(page).await?).is_equal_to(2.0);
    assert_that!(page.control("double-presses").await?).is_equal_to(1.0);
    Ok(())
}

/// Tab moving focus to the child (made focusable) and on.
#[browser_test]
pub async fn keyboard_focus_and_blur(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element("#perf-before").await?.click().await?;
    let pressable = page.element(PRESSABLE).await?;
    let after = page.element("#perf-after").await?;
    page.measure("focus", page.send_keys(Key::Tab)).await?;
    page.wait_for_focus(&pressable).await?;
    page.measure("blur", page.send_keys(Key::Tab)).await?;
    page.wait_for_focus(&after).await?;
    Ok(())
}

/// Disabling the child, a click it ignores, and enabling it again.
#[browser_test]
pub async fn disable_and_enable(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let pressable = page.element(PRESSABLE).await?;
    page.measure("disable", page.control("disable")).await?;
    pressable.wait_for_attr("tabindex", None).await?;
    page.measure("ignored click", pressable.click()).await?;
    assert_that!(presses(page).await?).is_equal_to(0.0);
    page.measure("enable", page.control("enable")).await?;
    pressable.wait_for_attr("tabindex", Some("0")).await?;
    Ok(())
}
