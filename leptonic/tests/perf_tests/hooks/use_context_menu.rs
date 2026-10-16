// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the hook `use_context_menu` (`/perf/hooks/use_context_menu`, on a
//! focusable `<div>`): what each thing it offers costs, measured (`Page::measure`) and checked
//! against its budget (`tests/budgets/hooks/use_context_menu.json`): mounting and unmounting it,
//! a right click (a request), and Control+Enter, which requests the menu on macOS only (ignored
//! elsewhere: the tests run on Linux).
use assertr::prelude::*;
use browser_test::{TestGroup, browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::{harness::fixture_test, pages::Page};

const PATH: &str = "/perf/hooks/use_context_menu";

/// Registers the performance tests of `use_context_menu`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(RightClick))
        .with(fixture_test(ControlEnter))
}
const TARGET: &str = "#perf-context-menu";

/// How often the menu was requested.
async fn requests(page: &Page<'_>) -> Result<f64, Report> {
    page.control("requests").await
}

/// Mounting the element with its context menu handling and unmounting it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(TARGET, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.element(TARGET).await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(TARGET, 0).await?;
    Ok(())
}

/// A right click: a request (the browser's menu prevented).
#[browser_test]
pub async fn right_click(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let target = page.element(TARGET).await?;
    let right_click = page
        .low_level()
        .driver()
        .action_chain()
        .context_click_element(&target);
    page.measure("right click", right_click.perform()).await?;
    assert_that!(requests(page).await?).is_equal_to(1.0);
    Ok(())
}

/// Control+Enter on the focused element, which isn't a request outside macOS.
#[browser_test]
pub async fn control_enter(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element("#perf-before").await?.click().await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&page.element(TARGET).await?).await?;
    page.measure("control+enter", page.send_keys(Key::Control + Key::Enter))
        .await?;
    assert_that!(requests(page).await?).is_equal_to(0.0);
    Ok(())
}
