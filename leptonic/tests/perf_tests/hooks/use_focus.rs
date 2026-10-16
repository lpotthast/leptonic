// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the hook `use_focus` (`/perf/hooks/use_focus`, on a plain `<button>`):
//! what each thing it offers costs, measured (`Page::measure`) and checked against its budget
//! (`tests/budgets/hooks/use_focus.json`): mounting and unmounting it, focus and blur by the
//! keyboard and by the pointer, and disabling it (focus it ignores).
use assertr::prelude::*;
use browser_test::{TestGroup, browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/hooks/use_focus";

/// Registers the performance tests of `use_focus`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(KeyboardFocusAndBlur))
        .with(fixture_test(PointerFocusAndBlur))
        .with(fixture_test(DisableAndEnable))
}
const FOCUSED: &str = "#perf-focus";

/// How often the button got focus.
async fn focuses(page: &Page<'_>) -> Result<f64, Report> {
    page.control("focuses").await
}

/// Mounting the button with its focus handling and unmounting it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(FOCUSED, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.element(FOCUSED).await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(FOCUSED, 0).await?;
    Ok(())
}

/// Tab moving focus to the button and on.
#[browser_test]
pub async fn keyboard_focus_and_blur(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element("#perf-before").await?.click().await?;
    let focused = page.element(FOCUSED).await?;
    page.measure("focus", page.send_keys(Key::Tab)).await?;
    focused.wait_for_attr("data-focused", Some("true")).await?;
    page.measure("blur", page.send_keys(Key::Tab)).await?;
    focused.wait_for_attr("data-focused", None).await?;
    assert_that!(focuses(page).await?).is_equal_to(1.0);
    Ok(())
}

/// A click focusing the button, and a click elsewhere taking the focus away.
#[browser_test]
pub async fn pointer_focus_and_blur(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let focused = page.element(FOCUSED).await?;
    let after = page.element("#perf-after").await?;
    page.measure("focus", focused.click()).await?;
    focused.wait_for_attr("data-focused", Some("true")).await?;
    page.measure("blur", after.click()).await?;
    focused.wait_for_attr("data-focused", None).await?;
    assert_that!(focuses(page).await?).is_equal_to(1.0);
    Ok(())
}

/// Disabling the focus handling, focus it ignores, and enabling it again.
#[browser_test]
pub async fn disable_and_enable(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element("#perf-before").await?.click().await?;
    let focused = page.element(FOCUSED).await?;
    page.measure("disable", page.control("disable")).await?;
    page.measure("ignored focus", page.send_keys(Key::Tab))
        .await?;
    page.wait_for_focus(&focused).await?;
    assert_that!(focuses(page).await?).is_equal_to(0.0);
    page.send_keys(Key::Shift + Key::Tab).await?;
    page.measure("enable", page.control("enable")).await?;
    page.send_keys(Key::Tab).await?;
    focused.wait_for_attr("data-focused", Some("true")).await?;
    assert_that!(focuses(page).await?).is_equal_to(1.0);
    Ok(())
}
