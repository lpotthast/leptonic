// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the atom `Focusable` (`/perf/atoms/focusable`, on a
//! `<span role="button">`): what each thing it offers costs, measured (`Page::measure`) and
//! checked against its budget (`tests/budgets/atoms/focusable.json`): mounting and unmounting
//! it, focus and blur by the keyboard and the pointer, a key its handler sees, and changes of its
//! reactive props (tab order, disabled).
use assertr::prelude::*;
use browser_test::{TestGroup, browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/atoms/focusable";

/// Registers the performance tests of `Focusable`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(KeyboardFocusAndBlur))
        .with(fixture_test(PointerFocusAndBlur))
        .with(fixture_test(KeyDown))
        .with(fixture_test(TabOrder))
        .with(fixture_test(DisableAndEnable))
}
const FOCUSABLE: &str = "#perf-focusable";

/// Focuses the element with the keyboard (Tab from the plain button before it).
async fn focus_with_tab(page: &Page<'_>) -> Result<WebElement, Report> {
    page.element("#perf-before").await?.click().await?;
    page.send_keys(Key::Tab).await?;
    let focusable = page.element(FOCUSABLE).await?;
    page.wait_for_focus(&focusable).await?;
    Ok(focusable)
}

/// Mounting the focusable child and unmounting it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(FOCUSABLE, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.element(FOCUSABLE).await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(FOCUSABLE, 0).await?;
    Ok(())
}

/// Tab moving focus to the element and on.
#[browser_test]
pub async fn keyboard_focus_and_blur(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element("#perf-before").await?.click().await?;
    let focusable = page.element(FOCUSABLE).await?;
    page.measure("focus", page.send_keys(Key::Tab)).await?;
    focusable
        .wait_for_attr("data-focused", Some("true"))
        .await?;
    page.measure("blur", page.send_keys(Key::Tab)).await?;
    focusable.wait_for_attr("data-focused", None).await?;
    Ok(())
}

/// A click focusing the element, and a click elsewhere taking the focus away.
#[browser_test]
pub async fn pointer_focus_and_blur(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let focusable = page.element(FOCUSABLE).await?;
    let after = page.element("#perf-after").await?;
    page.measure("focus", focusable.click()).await?;
    focusable
        .wait_for_attr("data-focused", Some("true"))
        .await?;
    page.measure("blur", after.click()).await?;
    focusable.wait_for_attr("data-focused", None).await?;
    Ok(())
}

/// A key its handler sees (Shift), the element focused.
#[browser_test]
pub async fn key_down(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    focus_with_tab(page).await?;
    page.measure("key", page.send_keys(Key::Shift)).await?;
    assert_that!(page.control("key-downs").await?).is_equal_to(1.0);
    Ok(())
}

/// Taking the element out of the tab order and back.
#[browser_test]
pub async fn tab_order(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let focusable = page.element(FOCUSABLE).await?;
    page.measure("exclude", page.control("exclude-from-tab-order"))
        .await?;
    focusable.wait_for_attr("tabindex", Some("-1")).await?;
    page.measure("include", page.control("include-in-tab-order"))
        .await?;
    focusable.wait_for_attr("tabindex", Some("0")).await?;
    Ok(())
}

/// Disabling the element (not focusable), a click it ignores, and enabling it again.
#[browser_test]
pub async fn disable_and_enable(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let focusable = page.element(FOCUSABLE).await?;
    page.measure("disable", page.control("disable")).await?;
    focusable.wait_for_attr("tabindex", None).await?;
    page.measure("ignored click", focusable.click()).await?;
    focusable.wait_for_attr("data-focused", None).await?;
    page.measure("enable", page.control("enable")).await?;
    focusable.wait_for_attr("tabindex", Some("0")).await?;
    Ok(())
}
