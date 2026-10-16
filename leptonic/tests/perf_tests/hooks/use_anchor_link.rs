// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the hook `use_anchor_link` (`/perf/hooks/use_anchor_link`, on a plain
//! `<a>`): what each thing it offers costs, measured (`Page::measure`) and checked against its
//! budget (`tests/budgets/hooks/use_anchor_link.json`): mounting and unmounting it, presses with
//! the mouse and Enter (each scrolls to its target and sets the URL fragment), its pressed state,
//! hover, keyboard focus, and a modified click it leaves to the browser. The props it passes on
//! are `use_link`'s (measured there).
use assertr::prelude::*;
use browser_test::{TestGroup, browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/hooks/use_anchor_link";

/// Registers the performance tests of `use_anchor_link`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(PressWithTheMouse))
        .with(fixture_test(HoldAndRelease))
        .with(fixture_test(PressWithEnter))
        .with(fixture_test(ModifiedClick))
        .with(fixture_test(HoverAndLeave))
        .with(fixture_test(KeyboardFocusAndBlur))
}
const LINK: &str = "#perf-anchor-link";

/// How often the link was pressed.
async fn presses(page: &Page<'_>) -> Result<f64, Report> {
    page.control("presses").await
}

/// The URL fragment and how far the page is scrolled.
async fn fragment_and_scroll(page: &Page<'_>) -> Result<(Option<String>, f64), Report> {
    let url = page.low_level().driver().current_url().await?;
    let scroll_y: f64 = page
        .low_level()
        .eval("return window.scrollY;", vec![])
        .await?;
    Ok((url.fragment().map(str::to_owned), scroll_y))
}

/// Checks that a press scrolled to the target and set the fragment.
async fn assert_followed(page: &Page<'_>) -> Result<(), Report> {
    let (fragment, scroll_y) = fragment_and_scroll(page).await?;
    assert_that!(fragment).is_equal_to(Some("perf-anchor-target".to_owned()));
    assert_that!(scroll_y).is_greater_than(0.0);
    Ok(())
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

/// A click: the press scrolls to the target and sets the fragment.
#[browser_test]
pub async fn press_with_the_mouse(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let link = page.element(LINK).await?;
    page.measure("click", link.click()).await?;
    assert_that!(presses(page).await?).is_equal_to(1.0);
    assert_followed(page).await
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
    assert_followed(page).await
}

/// A press with Enter, the link focused.
#[browser_test]
pub async fn press_with_enter(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element("#perf-before").await?.click().await?;
    page.send_keys(Key::Tab).await?;
    let link = page.element(LINK).await?;
    page.wait_for_focus(&link).await?;
    page.measure("enter", page.send_keys(Key::Enter)).await?;
    assert_that!(presses(page).await?).is_equal_to(1.0);
    assert_followed(page).await
}

/// A Ctrl+click, which the link leaves to the browser (a script prevents the new tab): pressed,
/// but neither scrolling nor changing the fragment.
#[browser_test]
pub async fn modified_click(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let link = page.element(LINK).await?;
    let click = page.low_level().eval::<()>(
        "window.addEventListener('click', e => e.preventDefault(), {once: true});
         arguments[0].dispatchEvent(
             new MouseEvent('click', {bubbles: true, cancelable: true, ctrlKey: true}));",
        vec![link.to_json()?],
    );
    page.measure("ctrl+click", click).await?;
    assert_that!(presses(page).await?).is_equal_to(1.0);
    assert_that!(fragment_and_scroll(page).await?).is_equal_to((None, 0.0));
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
