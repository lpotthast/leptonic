// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the atom `DisclosureGroup` (`/perf/atoms/disclosure_group`): what each thing
//! it offers costs, at 10 and at 1,000 disclosures, measured (`Page::measure`) and checked against
//! its budget (`tests/budgets/atoms/disclosure_group.json`): mounting and unmounting it, expanding
//! one disclosure after the other (single expansion collapses the expanded one), multiple
//! expansion and switching back to single, its controlled expanded keys, and disabling it.
use assertr::prelude::*;
use browser_test::{TestGroup, browser_test};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/atoms/disclosure_group";
const GROUP: &str = "#perf-group";
const DISCLOSURES: &str = "#perf-group .leptonic-Disclosure";

/// Registers the performance tests of `DisclosureGroup`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmountSmall))
        .with(fixture_test(MountAndUnmountLarge))
        .with(fixture_test(ExpandOneAfterAnotherSmall))
        .with(fixture_test(ExpandOneAfterAnotherLarge))
        .with(fixture_test(MultipleExpansion))
        .with(fixture_test(ControlledKeysSmall))
        .with(fixture_test(ControlledKeysLarge))
        .with(fixture_test(DisableAndEnableSmall))
        .with(fixture_test(DisableAndEnableLarge))
}

/// The group's sizes, with the control mounting it.
#[derive(Clone, Copy)]
enum Size {
    /// 10 disclosures.
    Small,
    /// 1,000 disclosures.
    Large,
}

impl Size {
    fn disclosures(self) -> usize {
        match self {
            Self::Small => 10,
            Self::Large => 1_000,
        }
    }

    fn mount_control(self) -> &'static str {
        match self {
            Self::Small => "mount-small",
            Self::Large => "mount-large",
        }
    }
}

/// Loads the page with the group of `size`, mounted.
async fn open(page: &Page<'_>, size: Size) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    if let Size::Large = size {
        page.control("unmount").await?;
        page.wait_for_count(GROUP, 0).await?;
        page.control(size.mount_control()).await?;
    }
    page.wait_for_count(DISCLOSURES, size.disclosures()).await?;
    Ok(())
}

/// Waits until the disclosure `i` is `expanded` (`data-expanded`).
async fn wait_for_expanded(page: &Page<'_>, i: usize, expanded: bool) -> Result<(), Report> {
    page.element(format!("#perf-disclosure-{i}"))
        .await?
        .wait_for_attr("data-expanded", expanded.then_some("true"))
        .await
}

/// Mounting the group of `size` and unmounting it.
async fn mount_and_unmount(page: &Page<'_>, size: Size) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(GROUP, 0).await?;
    page.measure("mount", page.control(size.mount_control()))
        .await?;
    page.wait_for_count(DISCLOSURES, size.disclosures()).await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(GROUP, 0).await?;
    Ok(())
}

/// Mounting the group of 10 disclosures and unmounting it.
#[browser_test]
pub async fn mount_and_unmount_small(page: &Page<'_>) -> Result<(), Report> {
    mount_and_unmount(page, Size::Small).await
}

/// Mounting the group of 1,000 disclosures and unmounting it.
#[browser_test]
pub async fn mount_and_unmount_large(page: &Page<'_>) -> Result<(), Report> {
    mount_and_unmount(page, Size::Large).await
}

/// Clicking the first trigger (expanding it), the second (expanding it and collapsing the first)
/// and the second again (collapsing it), in the group of `size`.
async fn expand_one_after_another(page: &Page<'_>, size: Size) -> Result<(), Report> {
    open(page, size).await?;
    let first = page.element("#perf-trigger-0").await?;
    let second = page.element("#perf-trigger-1").await?;
    page.measure("expand first", first.click()).await?;
    wait_for_expanded(page, 0, true).await?;
    page.measure("expand second", second.click()).await?;
    wait_for_expanded(page, 1, true).await?;
    wait_for_expanded(page, 0, false).await?;
    page.measure("collapse second", second.click()).await?;
    wait_for_expanded(page, 1, false).await?;
    Ok(())
}

/// Expanding one disclosure after the other in the group of 10.
#[browser_test]
pub async fn expand_one_after_another_small(page: &Page<'_>) -> Result<(), Report> {
    expand_one_after_another(page, Size::Small).await
}

/// Expanding one disclosure after the other in the group of 1,000.
#[browser_test]
pub async fn expand_one_after_another_large(page: &Page<'_>) -> Result<(), Report> {
    expand_one_after_another(page, Size::Large).await
}

/// Switching to multiple expansion, expanding two disclosures, and switching back to single
/// expansion (which keeps the first).
#[browser_test]
pub async fn multiple_expansion(page: &Page<'_>) -> Result<(), Report> {
    open(page, Size::Small).await?;
    page.measure("multiple", page.control("multiple")).await?;
    let first = page.element("#perf-trigger-0").await?;
    let second = page.element("#perf-trigger-1").await?;
    page.measure("expand first", first.click()).await?;
    wait_for_expanded(page, 0, true).await?;
    page.measure("expand second", second.click()).await?;
    wait_for_expanded(page, 1, true).await?;
    assert_that!(
        page.element("#perf-disclosure-0")
            .await?
            .attr("data-expanded")
            .await?
    )
    .is_equal_to(Some("true".to_owned()));
    page.measure("single", page.control("single")).await?;
    wait_for_expanded(page, 1, false).await?;
    wait_for_expanded(page, 0, true).await?;
    Ok(())
}

/// The app expanding the first disclosure and collapsing all (the controlled expanded keys), in
/// the group of `size`.
async fn controlled_keys(page: &Page<'_>, size: Size) -> Result<(), Report> {
    open(page, size).await?;
    page.measure("expand first", page.control("expand-first"))
        .await?;
    wait_for_expanded(page, 0, true).await?;
    page.measure("collapse all", page.control("collapse-all"))
        .await?;
    wait_for_expanded(page, 0, false).await?;
    Ok(())
}

/// The controlled expanded keys of the group of 10.
#[browser_test]
pub async fn controlled_keys_small(page: &Page<'_>) -> Result<(), Report> {
    controlled_keys(page, Size::Small).await
}

/// The controlled expanded keys of the group of 1,000.
#[browser_test]
pub async fn controlled_keys_large(page: &Page<'_>) -> Result<(), Report> {
    controlled_keys(page, Size::Large).await
}

/// Disabling the group of `size` (all its disclosures), a click on a trigger it ignores, and
/// enabling it again.
async fn disable_and_enable(page: &Page<'_>, size: Size) -> Result<(), Report> {
    open(page, size).await?;
    let group = page.element(GROUP).await?;
    let first = page.element("#perf-trigger-0").await?;
    page.measure("disable", page.control("disable")).await?;
    group.wait_for_attr("data-disabled", Some("true")).await?;
    first.wait_for_attr("data-disabled", Some("true")).await?;
    page.measure("ignored click", first.click()).await?;
    wait_for_expanded(page, 0, false).await?;
    page.measure("enable", page.control("enable")).await?;
    group.wait_for_attr("data-disabled", None).await?;
    first.wait_for_attr("data-disabled", None).await?;
    Ok(())
}

/// Disabling and enabling the group of 10.
#[browser_test]
pub async fn disable_and_enable_small(page: &Page<'_>) -> Result<(), Report> {
    disable_and_enable(page, Size::Small).await
}

/// Disabling and enabling the group of 1,000.
#[browser_test]
pub async fn disable_and_enable_large(page: &Page<'_>) -> Result<(), Report> {
    disable_and_enable(page, Size::Large).await
}
