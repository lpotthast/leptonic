// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the atom `VisuallyHidden` (`/perf/atoms/visually_hidden`, around a button,
//! focusable): what each thing it offers costs, measured (`Page::measure`) and checked against
//! its budget (`tests/budgets/atoms/visually_hidden.json`): mounting and unmounting it, focus
//! entering it (it shows) and leaving it (hidden again), and switching whether it is focusable,
//! with focus inside it while not (it stays hidden).
use assertr::prelude::*;
use browser_test::{TestGroup, browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/atoms/visually_hidden";

/// Registers the performance tests of `VisuallyHidden`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(FocusAndBlur))
        .with(fixture_test(Focusable))
}
const HIDDEN: &str = ".perf-hidden";

/// Mounting the hidden content and unmounting it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(HIDDEN, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.element(HIDDEN).await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(HIDDEN, 0).await?;
    Ok(())
}

/// Tab moving focus into the hidden content (it shows) and out again (hidden).
#[browser_test]
pub async fn focus_and_blur(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element("#perf-before").await?.click().await?;
    let hidden = page.element(HIDDEN).await?;
    page.measure("focus", page.send_keys(Key::Tab)).await?;
    hidden.wait_for_attr("style", None).await?;
    page.measure("blur", page.send_keys(Key::Tab)).await?;
    page.wait_for_focus(&page.element("#perf-after").await?)
        .await?;
    assert_hidden(&hidden).await
}

/// Making the content unfocusable, focus within it (it stays hidden), and focusable again (it
/// shows).
#[browser_test]
pub async fn focusable(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element("#perf-before").await?.click().await?;
    let hidden = page.element(HIDDEN).await?;
    page.measure("unfocusable", page.control("make-unfocusable"))
        .await?;
    page.measure("focus", page.send_keys(Key::Tab)).await?;
    page.wait_for_focus(&page.element("#perf-skip").await?)
        .await?;
    assert_hidden(&hidden).await?;
    page.measure("focusable", page.control("make-focusable"))
        .await?;
    page.send_keys(Key::Shift + Key::Tab).await?;
    page.send_keys(Key::Tab).await?;
    hidden.wait_for_attr("style", None).await?;
    Ok(())
}

/// Checks that the content is hidden: it has the hiding styles.
async fn assert_hidden(hidden: &WebElement) -> Result<(), Report> {
    assert_that!(hidden.attr("style").await?).is_some();
    Ok(())
}
