// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the atom `ToastCloseButton` (`/perf/atoms/toast_close_button`, in the
//! toasts of a `ToastRegion`): what each thing it offers costs, measured (`Page::measure`) and
//! checked against its budget (`tests/budgets/atoms/toast_close_button.json`): mounting and
//! unmounting it in a toast shown, closing its toast with the mouse (and its pressed state) and
//! with Enter and Space, hover, and keyboard focus.
use assertr::prelude::*;
use browser_test::{TestGroup, browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/atoms/toast_close_button";

/// Registers the performance tests of `ToastCloseButton`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(PressWithTheMouse))
        .with(fixture_test(HoldAndRelease))
        .with(fixture_test(PressWithTheKeyboard))
        .with(fixture_test(HoverAndLeave))
        .with(fixture_test(KeyboardFocusAndBlur))
}
const TOAST: &str = "[role=alertdialog]";
const BUTTON: &str = ".leptonic-ToastCloseButton";

/// How many toasts closed.
async fn closes(page: &Page<'_>) -> Result<f64, Report> {
    page.control("closes").await
}

/// Adds a toast and waits until `count` show.
async fn add(page: &Page<'_>, count: usize) -> Result<(), Report> {
    page.control("add").await?;
    page.wait_for_count(TOAST, count).await
}

/// Moves focus to the newest toast with the keyboard (F6 to the region, Tab to the toast).
async fn focus_newest_toast(page: &Page<'_>) -> Result<WebElement, Report> {
    page.element("#perf-before").await?.click().await?;
    page.send_keys(Key::F6).await?;
    page.wait_for_focus(&page.element(".leptonic-ToastRegion").await?)
        .await?;
    page.send_keys(Key::Tab).await?;
    let toast = page.first_element(TOAST).await?;
    page.wait_for_focus(&toast).await?;
    Ok(toast)
}

/// Unmounting the close button of a toast shown, and mounting it again.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    add(page, 1).await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(BUTTON, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.wait_for_attr(BUTTON, "aria-label", Some("Close"))
        .await?;
    Ok(())
}

/// A click on the close button: its toast closes (and the region with it).
#[browser_test]
pub async fn press_with_the_mouse(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    add(page, 1).await?;
    let button = page.element(BUTTON).await?;
    page.measure("click", button.click()).await?;
    page.wait_for_count(TOAST, 0).await?;
    assert_that!(closes(page).await?).is_equal_to(1.0);
    Ok(())
}

/// Holding the pointer down on the close button (the pressed state) and releasing it (closing
/// the toast).
#[browser_test]
pub async fn hold_and_release(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    add(page, 1).await?;
    let button = page.element(BUTTON).await?;
    let held = page.measure("hold", button.press_and_hold()).await?;
    button.wait_for_attr("data-pressed", Some("true")).await?;
    page.measure("release", held.release()).await?;
    page.wait_for_count(TOAST, 0).await?;
    assert_that!(closes(page).await?).is_equal_to(1.0);
    Ok(())
}

/// Enter on the focused close button of the newest of two toasts (the focus moves to the other
/// toast), and Space on the close button of the other.
#[browser_test]
pub async fn press_with_the_keyboard(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    add(page, 1).await?;
    add(page, 2).await?;
    let newest = focus_newest_toast(page).await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&newest.element(BUTTON).await?).await?;
    page.measure("enter", page.send_keys(Key::Enter)).await?;
    page.wait_for_count(TOAST, 1).await?;
    let remaining = page.element(TOAST).await?;
    page.wait_for_focus(&remaining).await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&remaining.element(BUTTON).await?)
        .await?;
    page.measure("space", page.send_keys(" ")).await?;
    page.wait_for_count(TOAST, 0).await?;
    assert_that!(closes(page).await?).is_equal_to(2.0);
    Ok(())
}

/// The pointer entering and leaving the close button.
#[browser_test]
pub async fn hover_and_leave(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    add(page, 1).await?;
    let button = page.element(BUTTON).await?;
    let after = page.element("#perf-after").await?;
    page.measure("hover", button.hover()).await?;
    button.wait_for_attr("data-hovered", Some("true")).await?;
    page.measure("leave", after.hover()).await?;
    button.wait_for_attr("data-hovered", None).await?;
    Ok(())
}

/// Tab moving focus from the toast to its close button, and Shift+Tab back.
#[browser_test]
pub async fn keyboard_focus_and_blur(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    add(page, 1).await?;
    focus_newest_toast(page).await?;
    let button = page.element(BUTTON).await?;
    page.measure("focus", page.send_keys(Key::Tab)).await?;
    button.wait_for_attr("data-focused", Some("true")).await?;
    page.measure("blur", page.send_keys(Key::Shift + Key::Tab))
        .await?;
    button.wait_for_attr("data-focused", None).await?;
    Ok(())
}
