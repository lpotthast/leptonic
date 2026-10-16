// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the atom `Link` (`/perf/atoms/link`): what each thing it offers costs,
//! measured (`Page::measure`) and checked against its budget (`tests/budgets/atoms/link.json`):
//! mounting and unmounting it, presses with the mouse and Enter, its pressed state, hover,
//! keyboard focus, disabling it (it becomes a `<span>`), its target, and following it (a
//! client-side navigation to its page with a query).
use assertr::prelude::*;
use browser_test::{TestGroup, browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/atoms/link";

/// Registers the performance tests of `Link`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(PressWithTheMouse))
        .with(fixture_test(HoldAndRelease))
        .with(fixture_test(PressWithEnter))
        .with(fixture_test(HoverAndLeave))
        .with(fixture_test(KeyboardFocusAndBlur))
        .with(fixture_test(DisableAndEnable))
        .with(fixture_test(Target))
        .with(fixture_test(Follow))
}
const LINK: &str = ".perf-link";

/// How often the link was pressed.
async fn presses(page: &Page<'_>) -> Result<f64, Report> {
    page.control("presses").await
}

/// Mounting the link and unmounting it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(LINK, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.element(LINK).await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(LINK, 0).await?;
    Ok(())
}

/// A click (its navigation prevented by the fixture): press start, press end and the press.
#[browser_test]
pub async fn press_with_the_mouse(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let link = page.element(LINK).await?;
    page.measure("click", link.click()).await?;
    assert_that!(presses(page).await?).is_equal_to(1.0);
    Ok(())
}

/// Holding the pointer down (the pressed state) and releasing it (the press).
#[browser_test]
pub async fn hold_and_release(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let link = page.element(LINK).await?;
    let held = page.measure("hold", link.press_and_hold()).await?;
    link.wait_for_attr("data-pressed", Some("true")).await?;
    page.measure("release", held.release()).await?;
    link.wait_for_attr("data-pressed", None).await?;
    assert_that!(presses(page).await?).is_equal_to(1.0);
    Ok(())
}

/// A press with Enter, the link focused (links don't handle Space).
#[browser_test]
pub async fn press_with_enter(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element("#perf-before").await?.click().await?;
    page.send_keys(Key::Tab).await?;
    let link = page.element(LINK).await?;
    page.wait_for_focus(&link).await?;
    page.measure("enter", page.send_keys(Key::Enter)).await?;
    assert_that!(presses(page).await?).is_equal_to(1.0);
    Ok(())
}

/// The pointer entering and leaving the link.
#[browser_test]
pub async fn hover_and_leave(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let link = page.element(LINK).await?;
    let after = page.element("#perf-after").await?;
    page.measure("hover", link.hover()).await?;
    link.wait_for_attr("data-hovered", Some("true")).await?;
    page.measure("leave", after.hover()).await?;
    link.wait_for_attr("data-hovered", None).await?;
    Ok(())
}

/// Tab moving focus to the link (with a focus ring) and on.
#[browser_test]
pub async fn keyboard_focus_and_blur(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element("#perf-before").await?.click().await?;
    let link = page.element(LINK).await?;
    page.measure("focus", page.send_keys(Key::Tab)).await?;
    link.wait_for_attr("data-focus-visible", Some("true"))
        .await?;
    page.measure("blur", page.send_keys(Key::Tab)).await?;
    link.wait_for_attr("data-focus-visible", None).await?;
    Ok(())
}

/// Disabling the link (it renders as a `<span>`), a click it ignores, and enabling it again (an
/// `<a>` again).
#[browser_test]
pub async fn disable_and_enable(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.measure("disable", page.control("disable")).await?;
    let link = page.element("span.perf-link[aria-disabled=true]").await?;
    page.measure("ignored click", link.click()).await?;
    assert_that!(presses(page).await?).is_equal_to(0.0);
    page.measure("enable", page.control("enable")).await?;
    page.element("a.perf-link[href]").await?;
    Ok(())
}

/// Opening the link in a new tab (`target`, `rel`) and here again.
#[browser_test]
pub async fn target(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let link = page.element(LINK).await?;
    page.measure("new tab", page.control("open-in-new-tab"))
        .await?;
    link.wait_for_attr("target", Some("_blank")).await?;
    link.wait_for_attr("rel", Some("noopener")).await?;
    page.measure("here", page.control("open-here")).await?;
    link.wait_for_attr("target", None).await?;
    Ok(())
}

/// Following the link: a client-side navigation to its page with a query (the page stays, the
/// link stays current).
#[browser_test]
pub async fn follow(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("allow-navigation").await?;
    let link = page.element(LINK).await?;
    page.measure("follow", link.click()).await?;
    let query = page.low_level().driver().current_url().await?;
    assert_that!(query.query()).is_equal_to(Some("followed"));
    link.wait_for_attr("aria-current", Some("page")).await?;
    assert_that!(presses(page).await?).is_equal_to(1.0);
    Ok(())
}
