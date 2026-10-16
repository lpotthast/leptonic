// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the atom `TagList` (`/perf/atoms/tag_list`, the fixture of `TagGroup`'s
//! performance test): what each thing it offers costs, measured (`Page::measure`) and checked
//! against its budget (`tests/budgets/atoms/tag_list.json`): mounting and unmounting it with 10
//! tags and with 1,000, keyboard focus coming into it and leaving (its focus state), and its empty
//! state (every tag removed, and tags again).
use browser_test::{TestGroup, browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use super::tag_group::{LIST, TAGS, goto, large, tag};
use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/atoms/tag_list";

/// Registers the performance tests of `TagList`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(MountAndUnmountLarge))
        .with(fixture_test(KeyboardFocusAndBlur))
        .with(fixture_test(Empty))
}

/// Mounting the list with `count` tags and unmounting it.
async fn mount_and_unmount_in(page: &Page<'_>, count: usize) -> Result<(), Report> {
    page.control("unmount").await?;
    page.wait_for_count(LIST, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.wait_for_count(TAGS, count).await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(LIST, 0).await?;
    Ok(())
}

/// Mounting the list with 10 tags and unmounting it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    goto(page, PATH).await?;
    mount_and_unmount_in(page, 10).await
}

/// Mounting the list with 1,000 tags and unmounting it.
#[browser_test]
pub async fn mount_and_unmount_large(page: &Page<'_>) -> Result<(), Report> {
    goto(page, PATH).await?;
    large(page).await?;
    mount_and_unmount_in(page, 1_000).await
}

/// Tab moving focus into the list (its first tag) and Shift+Tab out of it.
#[browser_test]
pub async fn keyboard_focus_and_blur(page: &Page<'_>) -> Result<(), Report> {
    goto(page, PATH).await?;
    page.element("#perf-before").await?.click().await?;
    page.element(LIST).await?;
    page.measure("focus", page.send_keys(Key::Tab)).await?;
    page.wait_for_focus(&page.element(tag(0)).await?).await?;
    page.measure("blur", page.send_keys(Key::Shift + Key::Tab))
        .await?;
    page.wait_for_focus(&page.element("#perf-before").await?)
        .await?;
    Ok(())
}

/// Every tag removed (the list shows its empty state) and 10 tags again.
#[browser_test]
pub async fn empty(page: &Page<'_>) -> Result<(), Report> {
    goto(page, PATH).await?;
    let list = page.element(LIST).await?;
    page.measure("remove all", page.control("remove-all"))
        .await?;
    list.wait_for_attr("data-empty", Some("true")).await?;
    list.wait_for_inner_text("No tags").await?;
    page.measure("restore", page.control("restore")).await?;
    list.wait_for_attr("data-empty", None).await?;
    page.wait_for_count(TAGS, 10).await?;
    Ok(())
}
