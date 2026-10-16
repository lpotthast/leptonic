// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the atom `GridListSection` (`/perf/atoms/grid_list_section`): what each
//! thing it offers costs, measured (`Page::measure`) and checked against its budget
//! (`tests/budgets/atoms/grid_list_section.json`): mounting and unmounting a list of sections, a
//! section added and removed, and the keys moving focus across sections (ArrowDown into the next
//! section, End, Home), with 2 sections (10 items) and with 200 (1,000 items).
use browser_test::{TestGroup, browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::{harness::fixture_test, pages::Page};

const PATH: &str = "/perf/atoms/grid_list_section";

/// Registers the performance tests of `GridListSection`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(MountAndUnmountLarge))
        .with(fixture_test(AddAndRemoveSectionSmall))
        .with(fixture_test(AddAndRemoveSectionLarge))
        .with(fixture_test(KeysAcrossSectionsSmall))
        .with(fixture_test(KeysAcrossSectionsLarge))
}

const LIST: &str = "[role=grid]";
const SECTIONS: &str = "[role=grid] > [role=rowgroup]";

/// Item `i` of section `s` (both from 0; the section's first row is its header).
fn item(s: usize, i: usize) -> String {
    format!(
        "[role=grid] > [role=rowgroup]:nth-child({}) > [role=row]:nth-child({})",
        s + 1,
        i + 2
    )
}

/// Opens the fixture with its 2 sections.
async fn open(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.wait_for_count(SECTIONS, 2).await?;
    Ok(())
}

/// Opens the fixture and mounts the list with 200 sections (unmeasured).
async fn open_large(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    page.control("unmount").await?;
    page.wait_for_count(LIST, 0).await?;
    page.control("large").await?;
    page.control("mount").await?;
    page.wait_for_count(SECTIONS, 200).await?;
    Ok(())
}

/// Mounting the list with its `sections` sections and unmounting it.
async fn mount_and_unmount_sections(page: &Page<'_>, sections: usize) -> Result<(), Report> {
    page.control("unmount").await?;
    page.wait_for_count(LIST, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.wait_for_count(SECTIONS, sections).await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(LIST, 0).await?;
    Ok(())
}

/// Mounting 2 sections of 5 items and unmounting them.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    mount_and_unmount_sections(page, 2).await
}

/// Mounting 200 sections of 5 items and unmounting them.
#[browser_test]
pub async fn mount_and_unmount_large(page: &Page<'_>) -> Result<(), Report> {
    open_large(page).await?;
    mount_and_unmount_sections(page, 200).await
}

/// A section added at the end and removed again.
async fn add_and_remove_section(page: &Page<'_>, sections: usize) -> Result<(), Report> {
    page.measure("add", page.control("add-section")).await?;
    page.wait_for_count(SECTIONS, sections + 1).await?;
    page.measure("remove", page.control("remove-section"))
        .await?;
    page.wait_for_count(SECTIONS, sections).await?;
    Ok(())
}

/// A section added to the 2 and removed again.
#[browser_test]
pub async fn add_and_remove_section_small(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    add_and_remove_section(page, 2).await
}

/// A section added to the 200 and removed again.
#[browser_test]
pub async fn add_and_remove_section_large(page: &Page<'_>) -> Result<(), Report> {
    open_large(page).await?;
    add_and_remove_section(page, 200).await
}

/// From the last item of the first section, ArrowDown to the first item of the second (past its
/// header); End to the last item of the last section, Home back to the first item.
async fn keys_across_sections(page: &Page<'_>, last_section: usize) -> Result<(), Report> {
    page.element("#perf-before").await?.click().await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&page.element(item(0, 0)).await?)
        .await?;
    page.send_keys(Key::End).await?;
    page.wait_for_focus(&page.element(item(last_section, 4)).await?)
        .await?;
    page.send_keys(Key::Home).await?;
    page.wait_for_focus(&page.element(item(0, 0)).await?)
        .await?;
    for _ in 0..4 {
        page.send_keys(Key::Down).await?;
    }
    page.wait_for_focus(&page.element(item(0, 4)).await?)
        .await?;
    page.measure("arrow down into next section", page.send_keys(Key::Down))
        .await?;
    page.wait_for_focus(&page.element(item(1, 0)).await?)
        .await?;
    page.measure("end", page.send_keys(Key::End)).await?;
    page.wait_for_focus(&page.element(item(last_section, 4)).await?)
        .await?;
    page.measure("home", page.send_keys(Key::Home)).await?;
    page.wait_for_focus(&page.element(item(0, 0)).await?)
        .await?;
    Ok(())
}

/// Keys across 2 sections.
#[browser_test]
pub async fn keys_across_sections_small(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    keys_across_sections(page, 1).await
}

/// Keys across 200 sections.
#[browser_test]
pub async fn keys_across_sections_large(page: &Page<'_>) -> Result<(), Report> {
    open_large(page).await?;
    keys_across_sections(page, 199).await
}
