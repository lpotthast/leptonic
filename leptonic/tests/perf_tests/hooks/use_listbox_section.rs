// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the hook `use_listbox_section` (`/perf/hooks/use_listbox_section`, plain
//! sections of a `use_listbox` listbox): what each thing it offers costs, measured
//! (`Page::measure`) and checked against its budget
//! (`tests/budgets/hooks/use_listbox_section.json`): mounting and unmounting 2 sections of 5
//! options and 100 of 10, the collection changing under them (each section looks for its header
//! again), the focus moving from one section to the next, and a click on a heading (which keeps the
//! focus in the listbox).
use assertr::prelude::*;
use browser_test::{TestGroup, browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::{harness::fixture_test, pages::Page};

const PATH: &str = "/perf/hooks/use_listbox_section";

/// Registers the performance tests of `use_listbox_section`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(MountAndUnmountLarge))
        .with(fixture_test(CollectionChange))
        .with(fixture_test(CollectionChangeLarge))
        .with(fixture_test(KeyboardFocusToTheNextSection))
        .with(fixture_test(ClickOnAHeading))
}
const SECTIONS: &str = "[data-perf-section]";
/// The number of the first option the control `append` appends to the first section.
const APPENDED: u32 = 10_000;

/// The option "Option `i`".
fn option(i: u32) -> String {
    format!("[data-perf-option='{i}']")
}

/// The heading of section `i`.
fn heading(i: u32) -> String {
    format!("[data-perf-section='{i}'] > span")
}

/// Mounting `count` sections and unmounting them.
async fn mount_and_unmount_in(page: &Page<'_>, count: usize) -> Result<(), Report> {
    page.control("unmount").await?;
    page.wait_for_count(SECTIONS, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.wait_for_count("[role=group][aria-labelledby]", count)
        .await?;
    let id = page.element(heading(0)).await?.attr("id").await?;
    assert_that!(
        page.element("[data-perf-section='0']")
            .await?
            .attr("aria-labelledby")
            .await?
    )
    .is_equal_to(id);
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(SECTIONS, 0).await?;
    Ok(())
}

/// Mounting 2 sections of 5 options and unmounting them.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    mount_and_unmount_in(page, 2).await
}

/// Mounting 100 sections of 10 options and unmounting them.
#[browser_test]
pub async fn mount_and_unmount_large(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("large").await?;
    page.element(option(999)).await?;
    mount_and_unmount_in(page, 100).await
}

/// An option appended to the first section and removed again.
async fn collection_change_in(page: &Page<'_>) -> Result<(), Report> {
    page.measure("append", page.control("append")).await?;
    page.element(format!("[data-perf-section='0'] {}", option(APPENDED)))
        .await?;
    page.measure("remove", page.control("remove")).await?;
    page.wait_for_count(option(APPENDED), 0).await?;
    Ok(())
}

/// The collection of 2 sections changing.
#[browser_test]
pub async fn collection_change(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    collection_change_in(page).await
}

/// The collection of 100 sections changing.
#[browser_test]
pub async fn collection_change_large(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("large").await?;
    page.element(option(999)).await?;
    collection_change_in(page).await
}

/// Focuses the first option with the keyboard.
async fn tab_in(page: &Page<'_>) -> Result<WebElement, Report> {
    page.element("#perf-before").await?.click().await?;
    page.send_keys(Key::Tab).await?;
    let first = page.element(option(0)).await?;
    page.wait_for_focus(&first).await?;
    Ok(first)
}

/// ArrowDown moving the focus from the last option of the first section to the first option of
/// the second.
#[browser_test]
pub async fn keyboard_focus_to_the_next_section(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    tab_in(page).await?;
    for i in 1..5 {
        page.send_keys(Key::Down).await?;
        page.wait_for_focus(&page.element(option(i)).await?).await?;
    }
    page.measure("arrow down", page.send_keys(Key::Down))
        .await?;
    page.wait_for_focus(
        &page
            .element(format!("[data-perf-section='1'] {}", option(5)))
            .await?,
    )
    .await?;
    Ok(())
}

/// A click on the second section's heading while an option has focus: the focus stays.
#[browser_test]
pub async fn click_on_a_heading(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let first = tab_in(page).await?;
    let second = page.element(heading(1)).await?;
    page.measure("click", second.click()).await?;
    page.focus_stays(&first, std::time::Duration::from_millis(100))
        .await?;
    Ok(())
}
