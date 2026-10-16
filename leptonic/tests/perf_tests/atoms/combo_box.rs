// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the atom `ComboBox` (`/perf/atoms/combo_box`): what each thing it offers
//! costs, measured (`Page::measure`) and checked against its budget
//! (`tests/budgets/atoms/combo_box.json`): mounting and unmounting it, opening it with its button
//! and selecting an option with the mouse, typing (filtering the options), the keyboard (opening,
//! moving through the options, selecting, leaving), dismissing it, and changes of its reactive
//! props (value, input text, disabled, read-only, invalid, required, menu trigger, disabled
//! options), with 10 options and with 1,000.
//!
//! The performance tests of its parts use the same fixture and the helpers here.
use assertr::prelude::*;
use browser_test::{TestGroup, browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/atoms/combo_box";

/// Registers the performance tests of `ComboBox`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(MountAndUnmountLarge))
        .with(fixture_test(OpenAndSelectWithTheMouse))
        .with(fixture_test(OpenAndSelectWithTheMouseLarge))
        .with(fixture_test(Typing))
        .with(fixture_test(TypingLarge))
        .with(fixture_test(Keyboard))
        .with(fixture_test(KeyboardLarge))
        .with(fixture_test(Dismiss))
        .with(fixture_test(DisableAndEnable))
        .with(fixture_test(ReadOnly))
        .with(fixture_test(Invalid))
        .with(fixture_test(Required))
        .with(fixture_test(ControlledValue))
        .with(fixture_test(ControlledValueLarge))
        .with(fixture_test(ControlledInput))
        .with(fixture_test(ControlledInputLarge))
        .with(fixture_test(OpenOnFocus))
        .with(fixture_test(DisabledOption))
}
pub(crate) const COMBO_BOX: &str = ".leptonic-ComboBox";
pub(crate) const INPUT: &str = "[role=combobox]";
pub(crate) const BUTTON: &str = ".leptonic-ComboBoxButton";
pub(crate) const VALUE: &str = ".leptonic-ComboBoxValue";
pub(crate) const POPOVER: &str = ".leptonic-ComboBoxPopover";
pub(crate) const OPTIONS: &str = "[role=option]";
/// Loads the fixture at `path` and creates the live announcer's regions (its first announcement
/// waits for them, so an announcing step measured first would count them, or not).
pub(crate) async fn goto(page: &Page<'_>, path: &str) -> Result<(), Report> {
    page.goto_path(path).await?;
    page.control("prepare-announcer").await?;
    page.wait_for_count("[data-live-announcer] [aria-live=polite] > div", 1)
        .await?;
    Ok(())
}

/// The fixture's options after the control `large`.
pub(crate) const LARGE: u32 = 1_000;

/// The option "Option `i`".
pub(crate) fn option(i: u32) -> String {
    format!("[role=option]:has(> [data-perf-option='{i}'])")
}

/// The combo box's value: the selected option's number, `-1` for none.
pub(crate) async fn value(page: &Page<'_>) -> Result<f64, Report> {
    page.control("value").await
}

/// Switches to [`LARGE`] options (unmeasured).
pub(crate) async fn large(page: &Page<'_>) -> Result<(), Report> {
    page.control("large").await?;
    page.settle().await
}

/// Mounting the combo box (10 options) and unmounting it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    goto(page, PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(COMBO_BOX, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.element(INPUT).await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(COMBO_BOX, 0).await?;
    Ok(())
}

/// Mounting the combo box with 1,000 options and unmounting it.
#[browser_test]
pub async fn mount_and_unmount_large(page: &Page<'_>) -> Result<(), Report> {
    goto(page, PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(COMBO_BOX, 0).await?;
    large(page).await?;
    page.measure("mount", page.control("mount")).await?;
    page.element(INPUT).await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(COMBO_BOX, 0).await?;
    Ok(())
}

/// A click on the button opening the popover with all `count` options, and a click on an option
/// selecting it (closing the popover, the input showing its text).
pub(crate) async fn open_and_select_with_the_mouse_in(
    page: &Page<'_>,
    count: usize,
) -> Result<(), Report> {
    let button = page.element(BUTTON).await?;
    page.measure("open", button.click()).await?;
    page.wait_for_count(OPTIONS, count).await?;
    let third = page.element(option(3)).await?;
    page.measure("select", third.click()).await?;
    page.wait_for_count(POPOVER, 0).await?;
    assert_that!(value(page).await?).is_equal_to(3.0);
    page.element(INPUT)
        .await?
        .wait_for_prop("value", "Option 3")
        .await?;
    Ok(())
}

/// Opening the combo box of 10 options with its button and selecting one with the mouse.
#[browser_test]
pub async fn open_and_select_with_the_mouse(page: &Page<'_>) -> Result<(), Report> {
    goto(page, PATH).await?;
    open_and_select_with_the_mouse_in(page, 10).await
}

/// Opening the combo box of 1,000 options with its button and selecting one with the mouse.
#[browser_test]
pub async fn open_and_select_with_the_mouse_large(page: &Page<'_>) -> Result<(), Report> {
    goto(page, PATH).await?;
    large(page).await?;
    open_and_select_with_the_mouse_in(page, 1_000).await
}

/// Typing into the input: "1" opens the popover with the options containing it (`ones`), "2"
/// narrows them to those containing "12" (`twelves`; none: the popover closes), Backspace widens
/// them again.
pub(crate) async fn typing_in(page: &Page<'_>, ones: usize, twelves: usize) -> Result<(), Report> {
    page.element(INPUT).await?.click().await?;
    page.measure("type", page.send_keys("1")).await?;
    page.wait_for_count(OPTIONS, ones).await?;
    page.measure("type more", page.send_keys("2")).await?;
    page.wait_for_count(OPTIONS, twelves).await?;
    page.measure("delete", page.send_keys(Key::Backspace))
        .await?;
    page.wait_for_count(OPTIONS, ones).await?;
    Ok(())
}

/// Typing into the combo box of 10 options.
#[browser_test]
pub async fn typing(page: &Page<'_>) -> Result<(), Report> {
    goto(page, PATH).await?;
    typing_in(page, 1, 0).await
}

/// Typing into the combo box of 1,000 options.
#[browser_test]
pub async fn typing_large(page: &Page<'_>) -> Result<(), Report> {
    goto(page, PATH).await?;
    large(page).await?;
    typing_in(page, 271, 20).await
}

/// Tab focusing the input, ArrowDown opening the popover (the first option focused), ArrowDown
/// and ArrowUp moving the focus, Enter selecting the focused option (closing the popover), and
/// Tab leaving.
async fn keyboard_in(page: &Page<'_>, count: usize) -> Result<(), Report> {
    page.element("#perf-before").await?.click().await?;
    let input = page.element(INPUT).await?;
    page.measure("focus", page.send_keys(Key::Tab)).await?;
    page.wait_for_focus(&input).await?;
    page.measure("open", page.send_keys(Key::Down)).await?;
    page.wait_for_count(OPTIONS, count).await?;
    let first = page.element(option(0)).await?;
    first.wait_for_attr("data-focused", Some("true")).await?;
    page.measure("arrow down", page.send_keys(Key::Down))
        .await?;
    page.element(option(1))
        .await?
        .wait_for_attr("data-focused", Some("true"))
        .await?;
    page.measure("arrow up", page.send_keys(Key::Up)).await?;
    first.wait_for_attr("data-focused", Some("true")).await?;
    page.measure("select", page.send_keys(Key::Enter)).await?;
    page.wait_for_count(POPOVER, 0).await?;
    input.wait_for_prop("value", "Option 0").await?;
    page.measure("blur", page.send_keys(Key::Tab)).await?;
    page.wait_for_focus(&page.element("#perf-after").await?)
        .await?;
    Ok(())
}

/// The keyboard on the combo box of 10 options.
#[browser_test]
pub async fn keyboard(page: &Page<'_>) -> Result<(), Report> {
    goto(page, PATH).await?;
    keyboard_in(page, 10).await
}

/// The keyboard on the combo box of 1,000 options.
#[browser_test]
pub async fn keyboard_large(page: &Page<'_>) -> Result<(), Report> {
    goto(page, PATH).await?;
    large(page).await?;
    keyboard_in(page, 1_000).await
}

/// Escape and a click outside closing the open popover.
#[browser_test]
pub async fn dismiss(page: &Page<'_>) -> Result<(), Report> {
    goto(page, PATH).await?;
    let button = page.element(BUTTON).await?;
    button.click().await?;
    page.element(option(0)).await?;
    page.measure("escape", page.send_keys(Key::Escape)).await?;
    page.wait_for_count(POPOVER, 0).await?;
    button.click().await?;
    page.element(option(0)).await?;
    page.measure("outside click", page.click_at(5, 5)).await?;
    page.wait_for_count(POPOVER, 0).await?;
    Ok(())
}

/// Disabling the combo box, a click on its button it ignores, and enabling it again.
#[browser_test]
pub async fn disable_and_enable(page: &Page<'_>) -> Result<(), Report> {
    goto(page, PATH).await?;
    let combo_box = page.element(COMBO_BOX).await?;
    page.measure("disable", page.control("disable")).await?;
    combo_box
        .wait_for_attr("data-disabled", Some("true"))
        .await?;
    let button = page.element(BUTTON).await?;
    page.measure("ignored click", button.click()).await?;
    assert_that!(page.count(POPOVER).await?).is_equal_to(0);
    page.measure("enable", page.control("enable")).await?;
    combo_box.wait_for_attr("data-disabled", None).await?;
    Ok(())
}

/// Making the combo box read-only, typing it ignores, and editable again.
#[browser_test]
pub async fn read_only(page: &Page<'_>) -> Result<(), Report> {
    goto(page, PATH).await?;
    let combo_box = page.element(COMBO_BOX).await?;
    let input = page.element(INPUT).await?;
    page.measure("make read-only", page.control("make-read-only"))
        .await?;
    combo_box
        .wait_for_attr("data-readonly", Some("true"))
        .await?;
    input.click().await?;
    page.measure("ignored typing", page.send_keys("1")).await?;
    assert_that!(page.count(POPOVER).await?).is_equal_to(0);
    input.wait_for_prop("value", "").await?;
    page.measure("make editable", page.control("make-editable"))
        .await?;
    combo_box.wait_for_attr("data-readonly", None).await?;
    Ok(())
}

/// Marking the combo box invalid and valid again.
#[browser_test]
pub async fn invalid(page: &Page<'_>) -> Result<(), Report> {
    goto(page, PATH).await?;
    let combo_box = page.element(COMBO_BOX).await?;
    page.measure("invalidate", page.control("invalidate"))
        .await?;
    combo_box
        .wait_for_attr("data-invalid", Some("true"))
        .await?;
    page.measure("validate", page.control("validate")).await?;
    combo_box.wait_for_attr("data-invalid", None).await?;
    Ok(())
}

/// Making the combo box required and optional again.
#[browser_test]
pub async fn required(page: &Page<'_>) -> Result<(), Report> {
    goto(page, PATH).await?;
    let combo_box = page.element(COMBO_BOX).await?;
    page.measure("require", page.control("require")).await?;
    combo_box
        .wait_for_attr("data-required", Some("true"))
        .await?;
    page.measure("unrequire", page.control("unrequire")).await?;
    combo_box.wait_for_attr("data-required", None).await?;
    Ok(())
}

/// The controlled value changing: the first option, the last of `last + 1`, none. The input
/// shows the selected option's text.
pub(crate) async fn controlled_value_in(page: &Page<'_>, last: u32) -> Result<(), Report> {
    let input = page.element(INPUT).await?;
    page.measure("select first", page.control("select-first"))
        .await?;
    input.wait_for_prop("value", "Option 0").await?;
    page.measure("select last", page.control("select-last"))
        .await?;
    input
        .wait_for_prop("value", &format!("Option {last}"))
        .await?;
    page.measure("clear", page.control("clear-value")).await?;
    input.wait_for_prop("value", "").await?;
    Ok(())
}

/// The controlled value of a combo box of 10 options changing.
#[browser_test]
pub async fn controlled_value(page: &Page<'_>) -> Result<(), Report> {
    goto(page, PATH).await?;
    controlled_value_in(page, 9).await
}

/// The controlled value of a combo box of 1,000 options changing.
#[browser_test]
pub async fn controlled_value_large(page: &Page<'_>) -> Result<(), Report> {
    goto(page, PATH).await?;
    large(page).await?;
    controlled_value_in(page, LARGE - 1).await
}

/// The controlled input text set ("Option 5", filtering the options without opening the
/// popover) and cleared.
async fn controlled_input_in(page: &Page<'_>) -> Result<(), Report> {
    let input = page.element(INPUT).await?;
    page.measure("set", page.control("set-text")).await?;
    input.wait_for_prop("value", "Option 5").await?;
    page.measure("clear", page.control("clear-text")).await?;
    input.wait_for_prop("value", "").await?;
    assert_that!(page.count(POPOVER).await?).is_equal_to(0);
    Ok(())
}

/// The controlled input text of a combo box of 10 options changing.
#[browser_test]
pub async fn controlled_input(page: &Page<'_>) -> Result<(), Report> {
    goto(page, PATH).await?;
    controlled_input_in(page).await
}

/// The controlled input text of a combo box of 1,000 options changing.
#[browser_test]
pub async fn controlled_input_large(page: &Page<'_>) -> Result<(), Report> {
    goto(page, PATH).await?;
    large(page).await?;
    controlled_input_in(page).await
}

/// Switching the menu trigger to focus, focus opening the popover, and switching back to input.
#[browser_test]
pub async fn open_on_focus(page: &Page<'_>) -> Result<(), Report> {
    goto(page, PATH).await?;
    page.measure("open on focus", page.control("open-on-focus"))
        .await?;
    let input = page.element(INPUT).await?;
    page.measure("focus", input.click()).await?;
    page.wait_for_count(OPTIONS, 10).await?;
    page.measure("open on input", page.control("open-on-input"))
        .await?;
    page.send_keys(Key::Escape).await?;
    page.wait_for_count(POPOVER, 0).await?;
    Ok(())
}

/// Disabling the first option of the open popover, a click on it that it ignores, and enabling
/// it again.
#[browser_test]
pub async fn disabled_option(page: &Page<'_>) -> Result<(), Report> {
    goto(page, PATH).await?;
    page.element(BUTTON).await?.click().await?;
    let first = page.element(option(0)).await?;
    page.measure("disable", page.control("disable-first"))
        .await?;
    first.wait_for_attr("aria-disabled", Some("true")).await?;
    page.measure("ignored click", first.click()).await?;
    assert_that!(value(page).await?).is_equal_to(-1.0);
    page.measure("enable", page.control("enable-all")).await?;
    first.wait_for_attr("aria-disabled", None).await?;
    Ok(())
}
