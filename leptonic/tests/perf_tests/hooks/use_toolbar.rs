// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the hook `use_toolbar` (`/perf/hooks/use_toolbar`, on a plain `<div>` of 5
//! plain buttons): what each thing it offers costs, measured (`Page::measure`) and checked
//! against its budget (`tests/budgets/hooks/use_toolbar.json`): mounting and unmounting it, the
//! arrow keys moving the focus, Tab leaving it (from its last element, which its focus manager
//! finds) and Shift+Tab coming back (to the button focused last); its orientation and label. A
//! toolbar of 1,000 buttons is the `Toolbar` atom's.
use browser_test::{TestGroup, browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/hooks/use_toolbar";

/// Registers the performance tests of `use_toolbar`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(Keys))
        .with(fixture_test(Orientation))
        .with(fixture_test(Label))
}
const TOOLBAR: &str = "#perf-toolbar";

/// The toolbar's button `i`.
fn tool(i: usize) -> String {
    format!("#perf-tool-{i}")
}

const TOOLS: usize = 5;

/// Mounting the toolbar and unmounting it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(TOOLBAR, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.element(tool(TOOLS - 1)).await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(TOOLBAR, 0).await?;
    Ok(())
}

/// ArrowRight and ArrowLeft from the first button, Tab leaving the toolbar and Shift+Tab coming
/// back to the first button.
#[browser_test]
pub async fn keys(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
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

/// Making the toolbar vertical and horizontal again.
#[browser_test]
pub async fn orientation(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
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
    page.goto_path(PATH).await?;
    let toolbar = page.element(TOOLBAR).await?;
    page.measure("rename", page.control("rename")).await?;
    toolbar.wait_for_attr("aria-label", Some("Editing")).await?;
    Ok(())
}
