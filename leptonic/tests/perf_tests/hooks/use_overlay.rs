// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the hook `use_overlay` (`/perf/hooks/use_overlay`): what each thing it offers
//! costs, measured (`Page::measure`) and checked against its budget
//! (`tests/budgets/hooks/use_overlay.json`): mounting and unmounting it, opening (the overlay
//! stacked) and closing by the app's state, Escape, a click outside and the focus leaving closing
//! it, and each ignored while disabled.
use std::time::Duration;

use browser_test::{TestGroup, browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::{harness::fixture_test, pages::Page};

const PATH: &str = "/perf/hooks/use_overlay";
const OVERLAY: &str = ".perf-overlay";
const OPENER: &str = "#perf-opener";

/// Registers the performance tests of `use_overlay`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(OpenAndClose))
        .with(fixture_test(EscapeCloses))
        .with(fixture_test(OutsideClickCloses))
        .with(fixture_test(BlurCloses))
        .with(fixture_test(IgnoredEscape))
        .with(fixture_test(IgnoredOutsideClick))
        .with(fixture_test(IgnoredBlur))
}

/// Opens the overlay (the opener focused before) and waits until its button has the focus.
async fn open(page: &Page<'_>) -> Result<(), Report> {
    page.element(OPENER).await?.click().await?;
    page.control("open").await?;
    page.wait_for_focus(&page.element("#perf-overlay-button").await?)
        .await
}

/// Mounting the closed overlay and unmounting it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.measure("mount", page.control("mount")).await?;
    page.control("open").await?;
    page.element(OVERLAY).await?;
    page.control("close").await?;
    page.wait_for_count(OVERLAY, 0).await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.control("open").await?;
    page.count_stays(OVERLAY, 0, Duration::from_millis(100))
        .await?;
    Ok(())
}

/// The app opening the overlay (its button focused) and closing it.
#[browser_test]
pub async fn open_and_close(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element(OPENER).await?.click().await?;
    page.measure("open", page.control("open")).await?;
    page.wait_for_focus(&page.element("#perf-overlay-button").await?)
        .await?;
    page.measure("close", page.control("close")).await?;
    page.wait_for_count(OVERLAY, 0).await?;
    Ok(())
}

/// Escape (in the overlay) closing it.
#[browser_test]
pub async fn escape_closes(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    open(page).await?;
    page.measure("escape", page.send_keys(Key::Escape)).await?;
    page.wait_for_count(OVERLAY, 0).await?;
    Ok(())
}

/// A click outside the overlay (on the opener) closing it.
#[browser_test]
pub async fn outside_click_closes(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    open(page).await?;
    let opener = page.element(OPENER).await?;
    page.measure("outside click", opener.click()).await?;
    page.wait_for_count(OVERLAY, 0).await?;
    Ok(())
}

/// The focus leaving the overlay (Tab to the button after it) closing it.
#[browser_test]
pub async fn blur_closes(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    open(page).await?;
    page.measure("blur", page.send_keys(Key::Tab)).await?;
    page.wait_for_count(OVERLAY, 0).await?;
    Ok(())
}

/// Escape while dismissing with the keyboard is disabled: the overlay stays open.
#[browser_test]
pub async fn ignored_escape(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.measure(
        "disable keyboard dismiss",
        page.control("disable-keyboard-dismiss"),
    )
    .await?;
    open(page).await?;
    page.measure("ignored escape", page.send_keys(Key::Escape))
        .await?;
    page.count_stays(OVERLAY, 1, Duration::from_millis(100))
        .await?;
    page.measure(
        "enable keyboard dismiss",
        page.control("enable-keyboard-dismiss"),
    )
    .await?;
    Ok(())
}

/// A click outside while the overlay isn't dismissable (nor closing on blur): it stays open.
#[browser_test]
pub async fn ignored_outside_click(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("stay-open-on-blur").await?;
    page.measure("not dismissable", page.control("not-dismissable"))
        .await?;
    open(page).await?;
    let opener = page.element(OPENER).await?;
    page.measure("ignored outside click", opener.click())
        .await?;
    page.count_stays(OVERLAY, 1, Duration::from_millis(100))
        .await?;
    page.measure("dismissable", page.control("dismissable"))
        .await?;
    Ok(())
}

/// The focus leaving the overlay while it doesn't close on blur: it stays open.
#[browser_test]
pub async fn ignored_blur(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.measure("stay open on blur", page.control("stay-open-on-blur"))
        .await?;
    open(page).await?;
    page.measure("ignored blur", page.send_keys(Key::Tab))
        .await?;
    page.wait_for_focus(&page.element("#perf-after").await?)
        .await?;
    page.count_stays(OVERLAY, 1, Duration::from_millis(100))
        .await?;
    page.measure("close on blur", page.control("close-on-blur"))
        .await?;
    Ok(())
}
