// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the hook `use_focus_ring` (`/perf/hooks/use_focus_ring`, on a plain
//! `<button>` and on a group): what each thing it offers costs, measured (`Page::measure`) and
//! checked against its budget (`tests/budgets/hooks/use_focus_ring.json`): mounting and
//! unmounting it, keyboard focus (a ring) and pointer focus (none), a key press showing the ring
//! of the focused element, focus moving into, within and out of a group (the within target), and
//! disabling it.
use browser_test::{TestGroup, browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/hooks/use_focus_ring";

/// Registers the performance tests of `use_focus_ring`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(KeyboardFocusAndBlur))
        .with(fixture_test(PointerFocusAndBlur))
        .with(fixture_test(KeyAfterPointerFocus))
        .with(fixture_test(FocusWithin))
        .with(fixture_test(DisableAndEnable))
}
const RINGED: &str = "#perf-focus-ring";
const GROUP: &str = "#perf-focus-ring-group";

/// Mounting the button with its focus ring and unmounting it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(RINGED, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.element(RINGED).await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(RINGED, 0).await?;
    Ok(())
}

/// Tab moving focus to the button (a ring) and on.
#[browser_test]
pub async fn keyboard_focus_and_blur(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element("#perf-before").await?.click().await?;
    let ringed = page.element(RINGED).await?;
    page.measure("focus", page.send_keys(Key::Tab)).await?;
    ringed
        .wait_for_attr("data-focus-visible", Some("true"))
        .await?;
    page.measure("blur", page.send_keys(Key::Tab)).await?;
    ringed.wait_for_attr("data-focus-visible", None).await?;
    ringed.wait_for_attr("data-focused", None).await?;
    Ok(())
}

/// A click focusing the button (no ring), and a click elsewhere taking the focus away.
#[browser_test]
pub async fn pointer_focus_and_blur(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let ringed = page.element(RINGED).await?;
    let after = page.element("#perf-after").await?;
    page.measure("focus", ringed.click()).await?;
    ringed.wait_for_attr("data-focused", Some("true")).await?;
    ringed.wait_for_attr("data-focus-visible", None).await?;
    page.measure("blur", after.click()).await?;
    ringed.wait_for_attr("data-focused", None).await?;
    Ok(())
}

/// A key press (`x`; a modifier alone changes no modality) on the button focused by a click:
/// the ring shows.
#[browser_test]
pub async fn key_after_pointer_focus(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let ringed = page.element(RINGED).await?;
    ringed.click().await?;
    ringed.wait_for_attr("data-focused", Some("true")).await?;
    page.measure("key", page.send_keys("x")).await?;
    ringed
        .wait_for_attr("data-focus-visible", Some("true"))
        .await?;
    Ok(())
}

/// Tab moving focus into the group (a ring), within it, and out of it.
#[browser_test]
pub async fn focus_within(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element(RINGED).await?.click().await?;
    let group = page.element(GROUP).await?;
    page.measure("enter", page.send_keys(Key::Tab)).await?;
    group
        .wait_for_attr("data-focus-visible", Some("true"))
        .await?;
    page.measure("move within", page.send_keys(Key::Tab))
        .await?;
    page.wait_for_focus(&page.element("#perf-group-second").await?)
        .await?;
    group
        .wait_for_attr("data-focus-visible", Some("true"))
        .await?;
    page.measure("leave", page.send_keys(Key::Tab)).await?;
    group.wait_for_attr("data-focus-visible", None).await?;
    group.wait_for_attr("data-focused", None).await?;
    Ok(())
}

/// Disabling the ring of the focused button (it hides), and enabling it again.
#[browser_test]
pub async fn disable_and_enable(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element("#perf-before").await?.click().await?;
    page.send_keys(Key::Tab).await?;
    let ringed = page.element(RINGED).await?;
    ringed
        .wait_for_attr("data-focus-visible", Some("true"))
        .await?;
    page.measure("disable", page.control("disable")).await?;
    ringed.wait_for_attr("data-focus-visible", None).await?;
    page.measure("enable", page.control("enable")).await?;
    page.send_keys(Key::Shift + Key::Tab).await?;
    page.send_keys(Key::Tab).await?;
    ringed
        .wait_for_attr("data-focus-visible", Some("true"))
        .await?;
    Ok(())
}
