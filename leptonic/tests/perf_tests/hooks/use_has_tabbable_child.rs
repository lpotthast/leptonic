// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the hook `use_has_tabbable_child` (`/perf/hooks/use_has_tabbable_child`,
//! on a group of 10 or 1,000 texts and a tabbable button): what each thing it offers costs,
//! measured (`Page::measure`) and checked against its budget
//! (`tests/budgets/hooks/use_has_tabbable_child.json`): mounting and unmounting it, the
//! tabbable child added and removed and disabled and enabled (it rechecks the group on every
//! mutation, so both sizes), and disabling the hook.
use browser_test::{TestGroup, browser_test};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/hooks/use_has_tabbable_child";

/// Registers the performance tests of `use_has_tabbable_child`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(MountAndUnmount1000))
        .with(fixture_test(AddAndRemoveChild))
        .with(fixture_test(AddAndRemoveChild1000))
        .with(fixture_test(DisableAndEnableChild))
        .with(fixture_test(DisableAndEnableChild1000))
        .with(fixture_test(DisableAndEnable))
}
const GROUP: &str = "#perf-group";
const HAS: &str = "data-has-tabbable-child";

/// Opens the page with `texts` texts in the group (10 or 1,000).
async fn open(page: &Page<'_>, texts: usize) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control(&format!("use-{texts}")).await?;
    page.wait_for_count("#perf-group > span", texts).await?;
    page.wait_for_attr(GROUP, HAS, Some("true")).await?;
    Ok(())
}

/// Mounting the group and unmounting it.
async fn mount_and_unmount_with(page: &Page<'_>, texts: usize) -> Result<(), Report> {
    open(page, texts).await?;
    page.control("unmount").await?;
    page.wait_for_count(GROUP, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.wait_for_attr(GROUP, HAS, Some("true")).await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(GROUP, 0).await?;
    Ok(())
}

/// Removing the tabbable child and adding it again.
async fn add_and_remove_child_with(page: &Page<'_>, texts: usize) -> Result<(), Report> {
    open(page, texts).await?;
    let group = page.element(GROUP).await?;
    page.measure("remove", page.control("remove-child")).await?;
    group.wait_for_attr(HAS, None).await?;
    page.measure("add", page.control("add-child")).await?;
    group.wait_for_attr(HAS, Some("true")).await?;
    Ok(())
}

/// Disabling the tabbable child (an attribute mutation) and enabling it again.
async fn disable_and_enable_child_with(page: &Page<'_>, texts: usize) -> Result<(), Report> {
    open(page, texts).await?;
    let group = page.element(GROUP).await?;
    page.measure("disable", page.control("disable-child"))
        .await?;
    group.wait_for_attr(HAS, None).await?;
    page.measure("enable", page.control("enable-child")).await?;
    group.wait_for_attr(HAS, Some("true")).await?;
    Ok(())
}

/// Mounting the group of 10 texts and unmounting it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    mount_and_unmount_with(page, 10).await
}

/// Mounting the group of 1,000 texts and unmounting it.
#[browser_test]
pub async fn mount_and_unmount_1000(page: &Page<'_>) -> Result<(), Report> {
    mount_and_unmount_with(page, 1000).await
}

/// Removing and adding the tabbable child of the group of 10 texts.
#[browser_test]
pub async fn add_and_remove_child(page: &Page<'_>) -> Result<(), Report> {
    add_and_remove_child_with(page, 10).await
}

/// Removing and adding the tabbable child of the group of 1,000 texts.
#[browser_test]
pub async fn add_and_remove_child_1000(page: &Page<'_>) -> Result<(), Report> {
    add_and_remove_child_with(page, 1000).await
}

/// Disabling and enabling the tabbable child of the group of 10 texts.
#[browser_test]
pub async fn disable_and_enable_child(page: &Page<'_>) -> Result<(), Report> {
    disable_and_enable_child_with(page, 10).await
}

/// Disabling and enabling the tabbable child of the group of 1,000 texts.
#[browser_test]
pub async fn disable_and_enable_child_1000(page: &Page<'_>) -> Result<(), Report> {
    disable_and_enable_child_with(page, 1000).await
}

/// Disabling the hook (no tabbable child reported) and enabling it again.
#[browser_test]
pub async fn disable_and_enable(page: &Page<'_>) -> Result<(), Report> {
    open(page, 10).await?;
    let group = page.element(GROUP).await?;
    page.measure("disable", page.control("disable")).await?;
    group.wait_for_attr(HAS, None).await?;
    page.measure("enable", page.control("enable")).await?;
    group.wait_for_attr(HAS, Some("true")).await?;
    Ok(())
}
