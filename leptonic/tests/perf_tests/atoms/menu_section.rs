// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the atom `MenuSection` (`/perf/atoms/menu_section`): what each thing it
//! offers costs, measured (`Page::measure`) and checked against its budget
//! (`tests/budgets/atoms/menu_section.json`): mounting it (a group named by its heading, with its
//! items) and unmounting it, and its own multiple selection: selecting two items and deselecting
//! one.
use browser_test::{TestGroup, browser_test, thirtyfour::prelude::*};
use leptonic::AriaRole;
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page, role},
};

const PATH: &str = "/perf/atoms/menu_section";
const SECTION: &str = ".leptonic-MenuSection";
const GROUP: &str = "[role=group]";

/// Registers the performance tests of `MenuSection`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(Selection))
}

/// The item with the text `text`.
async fn item(page: &Page<'_>, text: &str) -> Result<WebElement, Report> {
    page.element(role(AriaRole::Menuitemcheckbox).text(text))
        .await
}

/// Mounting the section and unmounting it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(SECTION, 1).await?;
    page.measure("mount", page.control("mount")).await?;
    page.wait_for_count(SECTION, 2).await?;
    item(page, "Cut").await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(SECTION, 1).await?;
    Ok(())
}

/// Clicks selecting "Cut" and "Copy" (the section's own multiple selection), and deselecting
/// "Cut".
#[browser_test]
pub async fn selection(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.wait_for_count(GROUP, 2).await?;
    let cut = item(page, "Cut").await?;
    let copy = item(page, "Copy").await?;
    page.measure("select", cut.click()).await?;
    cut.wait_for_attr("aria-checked", Some("true")).await?;
    page.measure("select another", copy.click()).await?;
    copy.wait_for_attr("aria-checked", Some("true")).await?;
    cut.wait_for_attr("aria-checked", Some("true")).await?;
    page.measure("deselect", cut.click()).await?;
    cut.wait_for_attr("aria-checked", Some("false")).await?;
    Ok(())
}
