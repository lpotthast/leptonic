// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the atom `Button` (`/perf/atoms/button`): what each thing it offers costs,
//! measured (`Page::measure`) and checked against its budget (`tests/budgets/atoms/button.json`):
//! mounting and unmounting it, presses with the mouse and the keyboard, its pressed state, hover,
//! keyboard focus, and changes of its reactive props (disabled, pending, tab order, ARIA state).
use assertr::prelude::*;
use browser_test::{TestGroup, browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/atoms/button";

/// Registers the performance tests of `Button`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(PressWithTheMouse))
        .with(fixture_test(HoldAndRelease))
        .with(fixture_test(PressWithTheKeyboard))
        .with(fixture_test(HoverAndLeave))
        .with(fixture_test(KeyboardFocusAndBlur))
        .with(fixture_test(DisableAndEnable))
        .with(fixture_test(Pending))
        .with(fixture_test(TabOrder))
        .with(fixture_test(AriaState))
}
const BUTTON: &str = "#perf-button";

/// The entry the page's first announcement leaves in the live announcer.
const ANNOUNCER_READY: &str = "[data-live-announcer] [aria-live=polite] > div";

/// Loads the page and waits until the live announcer exists (the page creates it on load; the
/// first announcement waits 100 ms for it, which would land in a measurement on some runs).
async fn load(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.wait_for_count(ANNOUNCER_READY, 1).await
}

/// How often the button was pressed.
async fn presses(page: &Page<'_>) -> Result<f64, Report> {
    page.control("presses").await
}

/// Focuses the button with the keyboard (Tab from the plain button before it).
async fn focus_with_tab(page: &Page<'_>) -> Result<WebElement, Report> {
    page.element("#perf-before").await?.click().await?;
    page.send_keys(Key::Tab).await?;
    let button = page.element(BUTTON).await?;
    page.wait_for_focus(&button).await?;
    Ok(button)
}

/// Mounting the button and unmounting it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    load(page).await?;
    page.control("unmount").await?;
    page.wait_for_count(BUTTON, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.element(BUTTON).await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(BUTTON, 0).await?;
    Ok(())
}

/// A click: press start, press end and the press.
#[browser_test]
pub async fn press_with_the_mouse(page: &Page<'_>) -> Result<(), Report> {
    load(page).await?;
    let button = page.element(BUTTON).await?;
    page.measure("click", button.click()).await?;
    assert_that!(presses(page).await?).is_equal_to(1.0);
    Ok(())
}

/// Holding the pointer down (the pressed state) and releasing it (the press).
#[browser_test]
pub async fn hold_and_release(page: &Page<'_>) -> Result<(), Report> {
    load(page).await?;
    let button = page.element(BUTTON).await?;
    let held = page.measure("hold", button.press_and_hold()).await?;
    button.wait_for_attr("data-pressed", Some("true")).await?;
    page.measure("release", held.release()).await?;
    button.wait_for_attr("data-pressed", None).await?;
    assert_that!(presses(page).await?).is_equal_to(1.0);
    Ok(())
}

/// Presses with Enter and with Space, the button focused.
#[browser_test]
pub async fn press_with_the_keyboard(page: &Page<'_>) -> Result<(), Report> {
    load(page).await?;
    focus_with_tab(page).await?;
    page.measure("enter", page.send_keys(Key::Enter)).await?;
    page.measure("space", page.send_keys(" ")).await?;
    assert_that!(presses(page).await?).is_equal_to(2.0);
    Ok(())
}

/// The pointer entering and leaving the button.
#[browser_test]
pub async fn hover_and_leave(page: &Page<'_>) -> Result<(), Report> {
    load(page).await?;
    let button = page.element(BUTTON).await?;
    let after = page.element("#perf-after").await?;
    page.measure("hover", button.hover()).await?;
    button.wait_for_attr("data-hovered", Some("true")).await?;
    page.measure("leave", after.hover()).await?;
    button.wait_for_attr("data-hovered", None).await?;
    Ok(())
}

/// Tab moving focus to the button (with a focus ring) and on.
#[browser_test]
pub async fn keyboard_focus_and_blur(page: &Page<'_>) -> Result<(), Report> {
    load(page).await?;
    page.element("#perf-before").await?.click().await?;
    let button = page.element(BUTTON).await?;
    page.measure("focus", page.send_keys(Key::Tab)).await?;
    button
        .wait_for_attr("data-focus-visible", Some("true"))
        .await?;
    page.measure("blur", page.send_keys(Key::Tab)).await?;
    button.wait_for_attr("data-focus-visible", None).await?;
    Ok(())
}

/// Disabling the button, a click it ignores, and enabling it again.
#[browser_test]
pub async fn disable_and_enable(page: &Page<'_>) -> Result<(), Report> {
    load(page).await?;
    let button = page.element(BUTTON).await?;
    page.measure("disable", page.control("disable")).await?;
    button.wait_for_attr("data-disabled", Some("true")).await?;
    page.measure("ignored click", button.click()).await?;
    assert_that!(presses(page).await?).is_equal_to(0.0);
    page.measure("enable", page.control("enable")).await?;
    button.wait_for_attr("data-disabled", None).await?;
    Ok(())
}

/// The pending state starting, a click it ignores, and the state ending.
#[browser_test]
pub async fn pending(page: &Page<'_>) -> Result<(), Report> {
    load(page).await?;
    let button = page.element(BUTTON).await?;
    page.measure("start", page.control("start-pending")).await?;
    button.wait_for_attr("data-pending", Some("true")).await?;
    page.measure("ignored click", button.click()).await?;
    assert_that!(presses(page).await?).is_equal_to(0.0);
    page.measure("end", page.control("end-pending")).await?;
    button.wait_for_attr("data-pending", None).await?;
    Ok(())
}

/// Taking the button out of the tab order and back.
#[browser_test]
pub async fn tab_order(page: &Page<'_>) -> Result<(), Report> {
    load(page).await?;
    let button = page.element(BUTTON).await?;
    page.measure("exclude", page.control("exclude-from-tab-order"))
        .await?;
    button.wait_for_attr("tabindex", Some("-1")).await?;
    page.measure("include", page.control("include-in-tab-order"))
        .await?;
    button.wait_for_attr("tabindex", Some("0")).await?;
    Ok(())
}

/// Changes of its ARIA state: `aria-pressed` and `aria-expanded`.
#[browser_test]
pub async fn aria_state(page: &Page<'_>) -> Result<(), Report> {
    load(page).await?;
    let button = page.element(BUTTON).await?;
    page.measure("set aria-pressed", page.control("set-aria-pressed"))
        .await?;
    button.wait_for_attr("aria-pressed", Some("true")).await?;
    page.measure("unset aria-pressed", page.control("unset-aria-pressed"))
        .await?;
    button.wait_for_attr("aria-pressed", None).await?;
    page.measure("expand", page.control("expand")).await?;
    button.wait_for_attr("aria-expanded", Some("true")).await?;
    page.measure("collapse", page.control("collapse")).await?;
    button.wait_for_attr("aria-expanded", Some("false")).await?;
    Ok(())
}
