// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the hook `use_radio` (`leptonic/src/hooks/form/use_radio.rs`, fixture
//! `/perf/hooks/use_radio`): what each thing it offers costs, measured (`Page::measure`) and
//! checked against its budget (`tests/budgets/hooks/use_radio.json`), for a radio in a group of
//! four: mounting and unmounting it, selecting it with the mouse (on its label), its pressed
//! state, the arrow key moving the selection to it, focus by keyboard, and disabling it (and a
//! click it then ignores).
use assertr::prelude::*;
use browser_test::{TestGroup, browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page, css},
};

const PATH: &str = "/perf/hooks/use_radio";
const INPUT: &str = "#perf-input";

/// Registers the performance tests of `use_radio`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(PressWithTheMouse))
        .with(fixture_test(HoldAndRelease))
        .with(fixture_test(ArrowKeySelectsIt))
        .with(fixture_test(KeyboardFocusAndBlur))
        .with(fixture_test(DisableAndEnable))
}

/// The selected option (-1: none).
async fn selected(page: &Page<'_>) -> Result<f64, Report> {
    page.control("selected").await
}

/// The radio's label.
async fn label(page: &Page<'_>) -> Result<WebElement, Report> {
    page.element(css("label").text("Option 3")).await
}

/// Mounting the radio in its group and unmounting it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(INPUT, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    let input = page.element(INPUT).await?;
    input.wait_for_attr("type", Some("radio")).await?;
    input.wait_for_attr("name", Some("options")).await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(INPUT, 0).await?;
    Ok(())
}

/// A click on the label: the radio is selected.
#[browser_test]
pub async fn press_with_the_mouse(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let label = label(page).await?;
    page.measure("click", label.click()).await?;
    page.element(INPUT)
        .await?
        .wait_for_prop("checked", "true")
        .await?;
    assert_that!(selected(page).await?).is_equal_to(3.0);
    Ok(())
}

/// Holding the pointer down on the radio (the pressed state) and releasing it (selected).
#[browser_test]
pub async fn hold_and_release(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = page.element(INPUT).await?;
    let held = page.measure("hold", input.press_and_hold()).await?;
    assert_that!(page.control("pressed").await?).is_equal_to(1.0);
    page.measure("release", held.release()).await?;
    assert_that!(page.control("pressed").await?).is_equal_to(0.0);
    input.wait_for_prop("checked", "true").await?;
    assert_that!(selected(page).await?).is_equal_to(3.0);
    Ok(())
}

/// Arrow Down on the radio before it: the selection (and focus) moves to the radio.
#[browser_test]
pub async fn arrow_key_selects_it(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let previous = page.element(css("label").text("Option 2")).await?;
    previous.click().await?;
    let previous = previous.element("input").await?;
    previous.wait_for_prop("checked", "true").await?;
    page.wait_for_focus(&previous).await?;
    let input = page.element(INPUT).await?;
    page.measure("arrow down", page.send_keys(Key::Down))
        .await?;
    page.wait_for_focus(&input).await?;
    input.wait_for_prop("checked", "true").await?;
    assert_that!(selected(page).await?).is_equal_to(3.0);
    Ok(())
}

/// Shift+Tab moving focus to the radio (selected: the group's tab stop) and Tab moving it on.
#[browser_test]
pub async fn keyboard_focus_and_blur(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = page.element(INPUT).await?;
    label(page).await?.click().await?;
    input.wait_for_prop("checked", "true").await?;
    page.element("#perf-after").await?.click().await?;
    page.measure("focus", page.send_keys(Key::Shift + Key::Tab))
        .await?;
    page.wait_for_focus(&input).await?;
    let after = page.element("#perf-after").await?;
    page.measure("blur", page.send_keys(Key::Tab)).await?;
    page.wait_for_focus(&after).await?;
    Ok(())
}

/// Disabling the radio, a click it ignores, and enabling it again.
#[browser_test]
pub async fn disable_and_enable(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = page.element(INPUT).await?;
    let label = label(page).await?;
    page.measure("disable", page.control("disable")).await?;
    input.wait_for_attr("disabled", Some("true")).await?;
    page.measure("ignored click", label.click()).await?;
    assert_that!(selected(page).await?).is_equal_to(0.0);
    page.measure("enable", page.control("enable")).await?;
    input.wait_for_attr("disabled", None).await?;
    Ok(())
}
