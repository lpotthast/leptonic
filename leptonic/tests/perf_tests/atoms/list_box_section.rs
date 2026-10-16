// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the atom `ListBoxSection` (`/perf/atoms/list_box_section`): what each thing
//! it offers costs, measured (`Page::measure`) and checked against its budget
//! (`tests/budgets/atoms/list_box_section.json`): mounting and unmounting 2 sections of 5 options
//! and 100 of 10, the collection changing under them (an option appended to the first section
//! and removed: each section looks for its header again), the focus moving from one section to
//! the next with the keyboard, and a click selecting an option in a section.
use browser_test::{TestGroup, browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/atoms/list_box_section";

/// Registers the performance tests of `ListBoxSection`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(MountAndUnmountLarge))
        .with(fixture_test(CollectionChange))
        .with(fixture_test(CollectionChangeLarge))
        .with(fixture_test(KeyboardFocusToTheNextSection))
        .with(fixture_test(SelectWithTheMouse))
}
const SECTIONS: &str = "[data-perf-section]";
/// The number of the first option the control `append` appends to the first section.
const APPENDED: u32 = 10_000;

/// The option "Option `i`".
fn option(i: u32) -> String {
    format!("[data-perf-option='{i}']")
}

/// Mounting `count` sections and unmounting them.
async fn mount_and_unmount_in(page: &Page<'_>, count: usize) -> Result<(), Report> {
    page.control("unmount").await?;
    page.wait_for_count(SECTIONS, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.wait_for_count(SECTIONS, count).await?;
    page.wait_for_count("[role=group][aria-labelledby]", count)
        .await?;
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

/// ArrowDown moving the focus from the last option of the first section to the first option of
/// the second.
#[browser_test]
pub async fn keyboard_focus_to_the_next_section(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element("#perf-before").await?.click().await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&page.element(option(0)).await?).await?;
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

/// A click selecting an option of the second section.
#[browser_test]
pub async fn select_with_the_mouse(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let sixth = page.element(option(5)).await?;
    page.measure("select", sixth.click()).await?;
    sixth.wait_for_attr("aria-selected", Some("true")).await?;
    Ok(())
}
