// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the atom `RadioButton` (`/perf/atoms/radio_button`): what each thing it
//! offers costs, measured (`Page::measure`) and checked against its budget
//! (`tests/budgets/atoms/radio_button.json`): mounting and unmounting it (in its `RadioField`, in
//! a mounted `RadioGroup`), selecting it with the mouse, the keyboard and a virtual click (and the
//! click on it while selected, which changes nothing), its pressed state, hover, keyboard focus,
//! and the presses it ignores while disabled or read-only. The arrow keys (the group's) are
//! measured in `radio_group.rs`.
use assertr::prelude::*;
use browser_test::{TestGroup, browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/atoms/radio_button";

/// Registers the performance tests of `RadioButton`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(SelectWithTheMouse))
        .with(fixture_test(HoldAndRelease))
        .with(fixture_test(SelectWithTheKeyboard))
        .with(fixture_test(SelectWithAVirtualClick))
        .with(fixture_test(HoverAndLeave))
        .with(fixture_test(KeyboardFocusAndBlur))
        .with(fixture_test(IgnoredWhileDisabled))
        .with(fixture_test(IgnoredWhileReadOnly))
}
const RADIO: &str = "#perf-radio";

/// The selected value as the group's `on_change` reported it (`-1`: none).
async fn selected(page: &Page<'_>) -> Result<f64, Report> {
    page.control("selected").await
}

/// How often the group's selection changed.
async fn changes(page: &Page<'_>) -> Result<f64, Report> {
    page.control("changes").await
}

/// Mounting the radio (its field and button) and unmounting it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(RADIO, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.element(RADIO).await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(RADIO, 0).await?;
    Ok(())
}

/// A click selecting the radio (and focusing it), and a click on the selected radio, which
/// changes nothing.
#[browser_test]
pub async fn select_with_the_mouse(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let radio = page.element(RADIO).await?;
    page.measure("select", radio.click()).await?;
    radio.wait_for_attr("data-selected", Some("true")).await?;
    assert_that!(selected(page).await?).is_equal_to(0.0);
    page.measure("click selected", radio.click()).await?;
    radio.wait_for_attr("data-selected", Some("true")).await?;
    assert_that!(changes(page).await?).is_equal_to(1.0);
    Ok(())
}

/// Holding the pointer down (the pressed state) and releasing it (selecting the radio).
#[browser_test]
pub async fn hold_and_release(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let radio = page.element(RADIO).await?;
    let held = page.measure("hold", radio.press_and_hold()).await?;
    radio.wait_for_attr("data-pressed", Some("true")).await?;
    page.measure("release", held.release()).await?;
    radio.wait_for_attr("data-pressed", None).await?;
    assert_that!(selected(page).await?).is_equal_to(0.0);
    Ok(())
}

/// Space selecting the focused radio.
#[browser_test]
pub async fn select_with_the_keyboard(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element("#perf-before").await?.click().await?;
    page.send_keys(Key::Tab).await?;
    let radio = page.element(RADIO).await?;
    radio
        .wait_for_attr("data-focus-visible", Some("true"))
        .await?;
    page.measure("select", page.send_keys(" ")).await?;
    radio.wait_for_attr("data-selected", Some("true")).await?;
    assert_that!(selected(page).await?).is_equal_to(0.0);
    Ok(())
}

/// A virtual click (as assistive technology clicks, without a pointer) selecting the radio.
#[browser_test]
pub async fn select_with_a_virtual_click(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let radio = page.element(RADIO).await?;
    page.measure("select", radio.virtual_click()).await?;
    radio.wait_for_attr("data-selected", Some("true")).await?;
    assert_that!(selected(page).await?).is_equal_to(0.0);
    Ok(())
}

/// The pointer entering and leaving the radio.
#[browser_test]
pub async fn hover_and_leave(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let radio = page.element(RADIO).await?;
    let after = page.element("#perf-after").await?;
    page.measure("hover", radio.hover()).await?;
    radio.wait_for_attr("data-hovered", Some("true")).await?;
    page.measure("leave", after.hover()).await?;
    radio.wait_for_attr("data-hovered", None).await?;
    Ok(())
}

/// Tab moving focus to the radio (with a focus ring) and on, out of the group.
#[browser_test]
pub async fn keyboard_focus_and_blur(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element("#perf-before").await?.click().await?;
    let radio = page.element(RADIO).await?;
    page.measure("focus", page.send_keys(Key::Tab)).await?;
    radio
        .wait_for_attr("data-focus-visible", Some("true"))
        .await?;
    page.measure("blur", page.send_keys(Key::Tab)).await?;
    radio.wait_for_attr("data-focused", None).await?;
    Ok(())
}

/// A click the disabled radio ignores.
#[browser_test]
pub async fn ignored_while_disabled(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let radio = page.element(RADIO).await?;
    page.control("disable").await?;
    radio.wait_for_attr("data-disabled", Some("true")).await?;
    page.measure("ignored click", radio.click()).await?;
    assert_that!(changes(page).await?).is_equal_to(0.0);
    radio.wait_for_attr("data-selected", None).await?;
    Ok(())
}

/// A click the radio of the read-only group ignores (it still takes focus).
#[browser_test]
pub async fn ignored_while_read_only(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let radio = page.element(RADIO).await?;
    page.control("make-read-only").await?;
    radio.wait_for_attr("data-readonly", Some("true")).await?;
    page.measure("ignored click", radio.click()).await?;
    assert_that!(changes(page).await?).is_equal_to(0.0);
    radio.wait_for_attr("data-selected", None).await?;
    Ok(())
}
