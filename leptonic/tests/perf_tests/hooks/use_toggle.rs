// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the hook `use_toggle` (`leptonic/src/hooks/form/use_toggle.rs`, fixture
//! `/perf/hooks/use_toggle`): what each thing it offers costs, measured (`Page::measure`) and
//! checked against its budget (`tests/budgets/hooks/use_toggle.json`): mounting and unmounting
//! a toggle, presses with the mouse (on its label) and the keyboard, its pressed state, focus
//! by keyboard, its selection changed by the page, and changes of its reactive props (disabled
//! and read-only, with clicks they ignore, invalid).
use assertr::prelude::*;
use browser_test::{TestGroup, browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page, css},
};

const PATH: &str = "/perf/hooks/use_toggle";
const INPUT: &str = "#perf-input";

/// Registers the performance tests of `use_toggle`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(PressWithTheMouse))
        .with(fixture_test(HoldAndRelease))
        .with(fixture_test(PressWithTheKeyboard))
        .with(fixture_test(KeyboardFocusAndBlur))
        .with(fixture_test(SelectionFromThePage))
        .with(fixture_test(DisableAndEnable))
        .with(fixture_test(ReadOnly))
        .with(fixture_test(Invalid))
}

/// Whether the toggle is selected (1) or not (0).
async fn selected(page: &Page<'_>) -> Result<f64, Report> {
    page.control("selected").await
}

/// The toggle's label.
async fn label(page: &Page<'_>) -> Result<WebElement, Report> {
    page.element(css("label").text("Subscribe")).await
}

/// Focuses the toggle with the keyboard (Tab from the plain button before it).
async fn focus_with_tab(page: &Page<'_>) -> Result<WebElement, Report> {
    page.element("#perf-before").await?.click().await?;
    page.send_keys(Key::Tab).await?;
    let input = page.element(INPUT).await?;
    page.wait_for_focus(&input).await?;
    Ok(input)
}

/// Mounting the toggle and unmounting it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(INPUT, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    let input = page.element(INPUT).await?;
    input.wait_for_attr("type", Some("checkbox")).await?;
    input.wait_for_prop("checked", "false").await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(INPUT, 0).await?;
    Ok(())
}

/// A click on the label: the toggle is selected.
#[browser_test]
pub async fn press_with_the_mouse(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let label = label(page).await?;
    page.measure("click", label.click()).await?;
    page.element(INPUT)
        .await?
        .wait_for_prop("checked", "true")
        .await?;
    assert_that!(selected(page).await?).is_equal_to(1.0);
    Ok(())
}

/// Holding the pointer down on the label (the pressed state) and releasing it (selected).
#[browser_test]
pub async fn hold_and_release(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let label = label(page).await?;
    let held = page.measure("hold", label.press_and_hold()).await?;
    assert_that!(page.control("pressed").await?).is_equal_to(1.0);
    page.measure("release", held.release()).await?;
    assert_that!(page.control("pressed").await?).is_equal_to(0.0);
    assert_that!(selected(page).await?).is_equal_to(1.0);
    Ok(())
}

/// Space selecting and deselecting the focused toggle.
#[browser_test]
pub async fn press_with_the_keyboard(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = focus_with_tab(page).await?;
    page.measure("check", page.send_keys(" ")).await?;
    input.wait_for_prop("checked", "true").await?;
    page.measure("uncheck", page.send_keys(" ")).await?;
    input.wait_for_prop("checked", "false").await?;
    assert_that!(selected(page).await?).is_equal_to(0.0);
    Ok(())
}

/// Tab moving focus to the toggle and on.
#[browser_test]
pub async fn keyboard_focus_and_blur(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element("#perf-before").await?.click().await?;
    let input = page.element(INPUT).await?;
    let after = page.element("#perf-after").await?;
    page.measure("focus", page.send_keys(Key::Tab)).await?;
    page.wait_for_focus(&input).await?;
    page.measure("blur", page.send_keys(Key::Tab)).await?;
    page.wait_for_focus(&after).await?;
    Ok(())
}

/// The page selecting the toggle (its state).
#[browser_test]
pub async fn selection_from_the_page(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = page.element(INPUT).await?;
    page.measure("select", page.control("select")).await?;
    input.wait_for_prop("checked", "true").await?;
    Ok(())
}

/// Disabling the toggle, a click it ignores, and enabling it again.
#[browser_test]
pub async fn disable_and_enable(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = page.element(INPUT).await?;
    let label = label(page).await?;
    page.measure("disable", page.control("disable")).await?;
    input.wait_for_attr("disabled", Some("true")).await?;
    page.measure("ignored click", label.click()).await?;
    input.wait_for_prop("checked", "false").await?;
    assert_that!(selected(page).await?).is_equal_to(0.0);
    page.measure("enable", page.control("enable")).await?;
    input.wait_for_attr("disabled", None).await?;
    Ok(())
}

/// Making the toggle read-only, a click it ignores, and making it editable again.
#[browser_test]
pub async fn read_only(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = page.element(INPUT).await?;
    let label = label(page).await?;
    page.measure("set", page.control("set-read-only")).await?;
    input.wait_for_attr("aria-readonly", Some("true")).await?;
    page.measure("ignored click", label.click()).await?;
    input.wait_for_prop("checked", "false").await?;
    assert_that!(selected(page).await?).is_equal_to(0.0);
    page.measure("unset", page.control("unset-read-only"))
        .await?;
    input.wait_for_attr("aria-readonly", None).await?;
    Ok(())
}

/// Marking the toggle invalid and valid again.
#[browser_test]
pub async fn invalid(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = page.element(INPUT).await?;
    page.measure("invalidate", page.control("invalidate"))
        .await?;
    input.wait_for_attr("aria-invalid", Some("true")).await?;
    page.measure("uninvalidate", page.control("uninvalidate"))
        .await?;
    input.wait_for_attr("aria-invalid", None).await?;
    Ok(())
}
