// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the atom `ListBoxSectionHeading` (`/perf/atoms/list_box_section_heading`):
//! what each thing it offers costs, measured (`Page::measure`) and checked against its budget
//! (`tests/budgets/atoms/list_box_section_heading.json`): mounting and unmounting the headings of
//! 2 sections and of 100 (with their sections: a heading takes its props from its section once),
//! a heading's text changing in the collection, at both sizes, and a click on a heading (which
//! keeps the focus in the listbox).
use assertr::prelude::*;
use browser_test::{TestGroup, browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/atoms/list_box_section_heading";

/// Registers the performance tests of `ListBoxSectionHeading`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(MountAndUnmountLarge))
        .with(fixture_test(Rename))
        .with(fixture_test(RenameLarge))
        .with(fixture_test(Click))
}
const HEADINGS: &str = "[data-perf-section] > header";

/// The heading of section `i`.
fn heading(i: u32) -> String {
    format!("[data-perf-section='{i}'] > header")
}

/// Mounting the headings of `count` sections (with the sections) and unmounting them.
async fn mount_and_unmount_in(page: &Page<'_>, count: usize) -> Result<(), Report> {
    page.control("unmount").await?;
    page.wait_for_count(HEADINGS, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.wait_for_count(HEADINGS, count).await?;
    let first = page.element(heading(0)).await?;
    assert_that!(first.inner_text().await?).is_equal_to("Section 0".to_owned());
    let id = first.attr("id").await?;
    assert_that!(
        page.element("[data-perf-section='0']")
            .await?
            .attr("aria-labelledby")
            .await?
    )
    .is_equal_to(id);
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(HEADINGS, 0).await?;
    Ok(())
}

/// Mounting the headings of 2 sections and unmounting them.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    mount_and_unmount_in(page, 2).await
}

/// Mounting the headings of 100 sections and unmounting them.
#[browser_test]
pub async fn mount_and_unmount_large(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("large").await?;
    page.element(heading(99)).await?;
    mount_and_unmount_in(page, 100).await
}

/// The first section renamed in the collection and back: its heading shows the new name.
async fn rename_in(page: &Page<'_>) -> Result<(), Report> {
    let first = page.element(heading(0)).await?;
    page.measure("rename", page.control("rename")).await?;
    first.wait_for_inner_text("Renamed section 0").await?;
    page.measure("restore", page.control("restore-name"))
        .await?;
    first.wait_for_inner_text("Section 0").await?;
    Ok(())
}

/// One of 2 headings renamed and back.
#[browser_test]
pub async fn rename(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    rename_in(page).await
}

/// One of 100 headings renamed and back.
#[browser_test]
pub async fn rename_large(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("large").await?;
    page.element(heading(99)).await?;
    rename_in(page).await
}

/// A click on the second section's heading while an option has focus: the focus stays.
#[browser_test]
pub async fn click(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element("#perf-before").await?.click().await?;
    page.send_keys(Key::Tab).await?;
    let first = page.element("[data-perf-option='0']").await?;
    page.wait_for_focus(&first).await?;
    let second = page.element(heading(1)).await?;
    page.measure("click", second.click()).await?;
    page.focus_stays(&first, std::time::Duration::from_millis(100))
        .await?;
    Ok(())
}
