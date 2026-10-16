// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the hook `use_menu_section` (`/perf/hooks/use_menu_section`): what each thing
//! it offers costs, measured (`Page::measure`) and checked against its budget
//! (`tests/budgets/hooks/use_menu_section.json`): mounting it (a group named by its heading, with
//! its items) and unmounting it, and its heading changing in the collection.
use browser_test::{TestGroup, browser_test};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/hooks/use_menu_section";
const GROUPS: &str = "[role=menu] [role=group]";
const HEADINGS: &str = ".perf-heading";

/// Registers the performance tests of `use_menu_section`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(HeadingChanges))
}

/// Mounting the section and unmounting it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(GROUPS, 1).await?;
    page.measure("mount", page.control("mount")).await?;
    page.wait_for_count(GROUPS, 2).await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(GROUPS, 1).await?;
    Ok(())
}

/// The section's heading renamed ("Edit" to "Clipboard") and back.
#[browser_test]
pub async fn heading_changes(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.wait_for_count(HEADINGS, 2).await?;
    let heading = page.elements(HEADINGS).await?.remove(1);
    heading.wait_for_inner_text("Edit").await?;
    page.measure("rename", page.control("rename")).await?;
    heading.wait_for_inner_text("Clipboard").await?;
    page.measure("rename back", page.control("unrename"))
        .await?;
    heading.wait_for_inner_text("Edit").await?;
    Ok(())
}
