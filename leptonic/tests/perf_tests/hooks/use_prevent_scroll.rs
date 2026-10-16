// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the hook `use_prevent_scroll` (`/perf/hooks/use_prevent_scroll`): what each
//! thing it offers costs, measured (`Page::measure`) and checked against its budget
//! (`tests/budgets/hooks/use_prevent_scroll.json`): mounting and unmounting it (disabled, and
//! preventing scrolling), and starting and stopping to prevent scrolling: the page's root
//! `overflow: hidden` with a stable scrollbar gutter, and on an iPhone (WebKit) the touch handling
//! and the injected style.
use assertr::prelude::*;
use browser_test::{TestGroup, browser_test};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{Page, Platform},
};

const PATH: &str = "/perf/hooks/use_prevent_scroll";
const SUBJECT: &str = "#perf-subject";

/// Registers the performance tests of `use_prevent_scroll`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(MountAndUnmountPreventing))
        .with(fixture_test(EnableAndDisable))
        .with(fixture_test(EnableAndDisableOnAnIphone))
}

/// The page root's inline `overflow`.
async fn root_overflow(page: &Page<'_>) -> Result<String, Report> {
    page.low_level()
        .eval("return document.documentElement.style.overflow;", vec![])
        .await
}

/// Waits until the page root's inline `overflow` is `expected` (`hidden` while preventing).
async fn wait_for_root_overflow(page: &Page<'_>, expected: &str) -> Result<(), Report> {
    assert_that!(|| root_overflow(page))
        .eventually_ok()
        .satisfies(|overflow| {
            overflow.is_equal_to(expected.to_owned());
        })
        .await;
    Ok(())
}

/// Mounting the disabled hook and unmounting it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(SUBJECT, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.element(SUBJECT).await?;
    assert_that!(root_overflow(page).await?).is_equal_to(String::new());
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(SUBJECT, 0).await?;
    Ok(())
}

/// Mounting the hook while it prevents scrolling (locking the page) and unmounting it (unlocking).
#[browser_test]
pub async fn mount_and_unmount_preventing(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(SUBJECT, 0).await?;
    page.control("enable").await?;
    page.measure("mount", page.control("mount")).await?;
    wait_for_root_overflow(page, "hidden").await?;
    page.measure("unmount", page.control("unmount")).await?;
    wait_for_root_overflow(page, "").await?;
    Ok(())
}

/// Starting to prevent scrolling (locking the page) and stopping (unlocking it).
#[browser_test]
pub async fn enable_and_disable(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element(SUBJECT).await?;
    page.measure("enable", page.control("enable")).await?;
    wait_for_root_overflow(page, "hidden").await?;
    page.measure("disable", page.control("disable")).await?;
    wait_for_root_overflow(page, "").await?;
    Ok(())
}

/// On an iPhone (WebKit): starting to prevent scrolling (touch handling, an injected style) and
/// stopping.
#[browser_test]
pub async fn enable_and_disable_on_an_iphone(page: &Page<'_>) -> Result<(), Report> {
    page.emulate_platform(Platform::IPhone).await?;
    page.goto_path(PATH).await?;
    page.element(SUBJECT).await?;
    let styles = async || -> Result<usize, Report> { page.count("head > style").await };
    let before = styles().await?;
    page.measure("enable", page.control("enable")).await?;
    assert_that!(|| styles())
        .eventually_ok()
        .satisfies(|count| {
            count.is_equal_to(before + 1);
        })
        .await;
    page.measure("disable", page.control("disable")).await?;
    assert_that!(styles().await?).is_equal_to(before);
    Ok(())
}
