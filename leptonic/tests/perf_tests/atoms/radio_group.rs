// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the atom `RadioGroup` (`/perf/atoms/radio_group`): what each thing it
//! offers costs, measured (`Page::measure`) and checked against its budget
//! (`tests/budgets/atoms/radio_group.json`), with 10 and with 1,000 radios: mounting and unmounting
//! it, clicks selecting a radio and another, the app selecting radios (its value controlled by app
//! state), the arrow keys moving the selection, and disabling it; with 10, changes of its other
//! reactive props (orientation, read-only, required, invalid). A single radio's costs are measured
//! in `radio_button.rs` and `radio_field.rs`.
use assertr::prelude::*;
use browser_test::{TestGroup, browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/atoms/radio_group";

/// Registers the performance tests of `RadioGroup`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(MountAndUnmountLarge))
        .with(fixture_test(SelectWithTheMouse))
        .with(fixture_test(SelectWithTheMouseLarge))
        .with(fixture_test(SelectByTheApp))
        .with(fixture_test(SelectByTheAppLarge))
        .with(fixture_test(ArrowKeys))
        .with(fixture_test(ArrowKeysLarge))
        .with(fixture_test(DisableAndEnable))
        .with(fixture_test(DisableAndEnableLarge))
        .with(fixture_test(ChangeOrientation))
        .with(fixture_test(ReadOnlyAndEditable))
        .with(fixture_test(RequiredAndOptional))
        .with(fixture_test(InvalidAndValid))
}
const GROUP: &str = "#perf-radio-group";
const RADIOS: &str = "#perf-radio-group .leptonic-RadioButton";
const FIRST: &str = "#perf-radio-group > :first-child .leptonic-RadioButton";
const SECOND: &str = "#perf-radio-group > :nth-child(2) .leptonic-RadioButton";
const LAST: &str = "#perf-radio-group > :last-child .leptonic-RadioButton";

/// The selected value (`-1`: none).
async fn selected(page: &Page<'_>) -> Result<f64, Report> {
    page.control("selected").await
}

/// Replaces the group of 10 radios with one of 1,000 (not measured).
async fn mount_large(page: &Page<'_>) -> Result<(), Report> {
    page.control("unmount").await?;
    page.wait_for_count(GROUP, 0).await?;
    page.control("mount-large").await?;
    page.wait_for_count(RADIOS, 1_000).await?;
    Ok(())
}

/// Mounting and unmounting the group with the control `mount` (`mount-large`) and `count`
/// radios.
async fn mount_and_unmount_with(page: &Page<'_>, mount: &str, count: usize) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(GROUP, 0).await?;
    page.measure("mount", page.control(mount)).await?;
    page.wait_for_count(RADIOS, count).await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(GROUP, 0).await?;
    Ok(())
}

/// A click selecting the first radio, and one selecting the last instead (the group has `count`
/// radios).
async fn select_with_the_mouse_in(page: &Page<'_>, count: u32) -> Result<(), Report> {
    let first = page.element(FIRST).await?;
    let last = page.element(LAST).await?;
    page.measure("select", first.click()).await?;
    first.wait_for_attr("data-selected", Some("true")).await?;
    assert_that!(selected(page).await?).is_equal_to(0.0);
    page.measure("select another", last.click()).await?;
    last.wait_for_attr("data-selected", Some("true")).await?;
    first.wait_for_attr("data-selected", None).await?;
    assert_that!(selected(page).await?).is_equal_to(f64::from(count - 1));
    Ok(())
}

/// The app selecting the first radio, then the last, then none.
async fn select_by_the_app_in(page: &Page<'_>) -> Result<(), Report> {
    page.measure("select first", page.control("select-first"))
        .await?;
    page.wait_for_attr(FIRST, "data-selected", Some("true"))
        .await?;
    page.measure("select last", page.control("select-last"))
        .await?;
    page.wait_for_attr(LAST, "data-selected", Some("true"))
        .await?;
    page.wait_for_attr(FIRST, "data-selected", None).await?;
    page.measure("select none", page.control("select-none"))
        .await?;
    page.wait_for_attr(LAST, "data-selected", None).await?;
    Ok(())
}

/// Tab entering the group at its first radio (not measured), then Down selecting the second radio
/// and Up the first again.
async fn arrow_keys_in(page: &Page<'_>) -> Result<(), Report> {
    page.element("#perf-before").await?.click().await?;
    page.send_keys(Key::Tab).await?;
    let first = page.element(FIRST).await?;
    let second = page.element(SECOND).await?;
    first
        .wait_for_attr("data-focus-visible", Some("true"))
        .await?;
    page.measure("down", page.send_keys(Key::Down)).await?;
    second.wait_for_attr("data-selected", Some("true")).await?;
    assert_that!(selected(page).await?).is_equal_to(1.0);
    page.measure("up", page.send_keys(Key::Up)).await?;
    first.wait_for_attr("data-selected", Some("true")).await?;
    assert_that!(selected(page).await?).is_equal_to(0.0);
    Ok(())
}

/// Disabling the group (and so every radio) and enabling it again.
async fn disable_and_enable_group(page: &Page<'_>) -> Result<(), Report> {
    page.measure("disable", page.control("disable")).await?;
    page.wait_for_attr(GROUP, "data-disabled", Some("true"))
        .await?;
    page.wait_for_attr(LAST, "data-disabled", Some("true"))
        .await?;
    page.measure("enable", page.control("enable")).await?;
    page.wait_for_attr(GROUP, "data-disabled", None).await?;
    page.wait_for_attr(LAST, "data-disabled", None).await?;
    Ok(())
}

/// Measures the control `on` (named after it), waits until the group has the flag `attr`, then
/// measures the control `off` and waits until the flag is gone.
async fn flag_round_trip(page: &Page<'_>, attr: &str, on: &str, off: &str) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.measure(on, page.control(on)).await?;
    page.wait_for_attr(GROUP, attr, Some("true")).await?;
    page.measure(off, page.control(off)).await?;
    page.wait_for_attr(GROUP, attr, None).await?;
    Ok(())
}

/// Mounting the group of 10 radios and unmounting it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    mount_and_unmount_with(page, "mount", 10).await
}

/// Mounting the group of 1,000 radios and unmounting it.
#[browser_test]
pub async fn mount_and_unmount_large(page: &Page<'_>) -> Result<(), Report> {
    mount_and_unmount_with(page, "mount-large", 1_000).await
}

/// Clicks selecting the first of 10 radios, then the last.
#[browser_test]
pub async fn select_with_the_mouse(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    select_with_the_mouse_in(page, 10).await
}

/// Clicks selecting the first of 1,000 radios, then the last.
#[browser_test]
pub async fn select_with_the_mouse_large(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    mount_large(page).await?;
    select_with_the_mouse_in(page, 1_000).await
}

/// The app selecting the first of 10 radios, the last, and none.
#[browser_test]
pub async fn select_by_the_app(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    select_by_the_app_in(page).await
}

/// The app selecting the first of 1,000 radios, the last, and none.
#[browser_test]
pub async fn select_by_the_app_large(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    mount_large(page).await?;
    select_by_the_app_in(page).await
}

/// Down and Up moving the selection among 10 radios.
#[browser_test]
pub async fn arrow_keys(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    arrow_keys_in(page).await
}

/// Down and Up moving the selection among 1,000 radios.
#[browser_test]
pub async fn arrow_keys_large(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    mount_large(page).await?;
    arrow_keys_in(page).await
}

/// Disabling the group of 10 radios and enabling it again.
#[browser_test]
pub async fn disable_and_enable(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    disable_and_enable_group(page).await
}

/// Disabling the group of 1,000 radios and enabling it again.
#[browser_test]
pub async fn disable_and_enable_large(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    mount_large(page).await?;
    disable_and_enable_group(page).await
}

/// Laying the group of 10 out horizontally and vertically again.
#[browser_test]
pub async fn change_orientation(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.measure("make horizontal", page.control("make-horizontal"))
        .await?;
    page.wait_for_attr(GROUP, "data-orientation", Some("horizontal"))
        .await?;
    page.measure("make vertical", page.control("make-vertical"))
        .await?;
    page.wait_for_attr(GROUP, "data-orientation", Some("vertical"))
        .await?;
    Ok(())
}

/// Making the group of 10 read-only and editable again.
#[browser_test]
pub async fn read_only_and_editable(page: &Page<'_>) -> Result<(), Report> {
    flag_round_trip(page, "data-readonly", "make-read-only", "make-editable").await
}

/// Requiring a selection in the group of 10, and making it optional again.
#[browser_test]
pub async fn required_and_optional(page: &Page<'_>) -> Result<(), Report> {
    flag_round_trip(page, "data-required", "require", "make-optional").await
}

/// Marking the group of 10 invalid and valid again.
#[browser_test]
pub async fn invalid_and_valid(page: &Page<'_>) -> Result<(), Report> {
    flag_round_trip(page, "data-invalid", "invalidate", "validate").await
}
