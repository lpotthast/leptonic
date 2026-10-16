// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the atom `Toolbar` (`/perf/atoms/toolbar`, of 5 or 1,000 plain buttons):
//! what each thing it offers costs, measured (`Page::measure`) and checked against its budget
//! (`tests/budgets/atoms/toolbar.json`): mounting and unmounting it, the arrow keys moving the
//! focus, Tab leaving it (from its last element, which its focus manager finds) and Shift+Tab
//! coming back (to the button focused last), at both sizes; its orientation and label.
use browser_test::{TestGroup, browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/atoms/toolbar";

/// Registers the performance tests of `Toolbar`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(MountAndUnmount1000))
        .with(fixture_test(Keys))
        .with(fixture_test(Keys1000))
        .with(fixture_test(Orientation))
        .with(fixture_test(Label))
}
const TOOLBAR: &str = "#perf-toolbar";

/// The toolbar's button `i`.
fn tool(i: usize) -> String {
    format!("#perf-tool-{i}")
}

/// Opens the page with `tools` buttons in the toolbar (5 or 1,000).
async fn open(page: &Page<'_>, tools: usize) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control(&format!("use-{tools}")).await?;
    page.wait_for_count("#perf-toolbar button", tools).await?;
    Ok(())
}

/// Mounting the toolbar and unmounting it.
async fn mount_and_unmount_with(page: &Page<'_>, tools: usize) -> Result<(), Report> {
    open(page, tools).await?;
    page.control("unmount").await?;
    page.wait_for_count(TOOLBAR, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.element(tool(tools - 1)).await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(TOOLBAR, 0).await?;
    Ok(())
}

/// ArrowRight and ArrowLeft from the first button, Tab leaving the toolbar and Shift+Tab coming
/// back to the first button.
async fn keys_with(page: &Page<'_>, tools: usize) -> Result<(), Report> {
    open(page, tools).await?;
    let first = page.element(tool(0)).await?;
    first.click().await?;
    page.wait_for_focus(&first).await?;
    page.measure("arrow right", page.send_keys(Key::Right))
        .await?;
    page.wait_for_focus(&page.element(tool(1)).await?).await?;
    page.measure("arrow left", page.send_keys(Key::Left))
        .await?;
    page.wait_for_focus(&first).await?;
    page.measure("tab out", page.send_keys(Key::Tab)).await?;
    page.wait_for_focus(&page.element("#perf-after").await?)
        .await?;
    page.measure("shift+tab in", page.send_keys(Key::Shift + Key::Tab))
        .await?;
    page.wait_for_focus(&first).await?;
    Ok(())
}

/// Mounting the toolbar of 5 buttons and unmounting it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    mount_and_unmount_with(page, 5).await
}

/// Mounting the toolbar of 1,000 buttons and unmounting it.
#[browser_test]
pub async fn mount_and_unmount_1000(page: &Page<'_>) -> Result<(), Report> {
    mount_and_unmount_with(page, 1000).await
}

/// The keys in the toolbar of 5 buttons.
#[browser_test]
pub async fn keys(page: &Page<'_>) -> Result<(), Report> {
    keys_with(page, 5).await
}

/// The keys in the toolbar of 1,000 buttons.
#[browser_test]
pub async fn keys_1000(page: &Page<'_>) -> Result<(), Report> {
    keys_with(page, 1000).await
}

/// Making the toolbar vertical and horizontal again.
#[browser_test]
pub async fn orientation(page: &Page<'_>) -> Result<(), Report> {
    open(page, 5).await?;
    let toolbar = page.element(TOOLBAR).await?;
    page.measure("vertical", page.control("vertical")).await?;
    toolbar
        .wait_for_attr("aria-orientation", Some("vertical"))
        .await?;
    page.measure("horizontal", page.control("horizontal"))
        .await?;
    toolbar
        .wait_for_attr("aria-orientation", Some("horizontal"))
        .await?;
    Ok(())
}

/// Renaming the toolbar.
#[browser_test]
pub async fn label(page: &Page<'_>) -> Result<(), Report> {
    open(page, 5).await?;
    let toolbar = page.element(TOOLBAR).await?;
    page.measure("rename", page.control("rename")).await?;
    toolbar.wait_for_attr("aria-label", Some("Editing")).await?;
    Ok(())
}
