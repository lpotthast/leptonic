// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the atom `ThemeProvider` (`/perf/atoms/theme_provider`, inside the app's
//! root provider): what each thing it offers costs, measured (`Page::measure`) and checked
//! against its budget (`tests/budgets/atoms/theme_provider.json`): mounting and unmounting it,
//! switching its theme through its context (uncontrolled), and switching a controlled one's.
use browser_test::{TestGroup, browser_test};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/atoms/theme_provider";

/// Registers the performance tests of `ThemeProvider`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(SwitchTheme))
        .with(fixture_test(SwitchControlledTheme))
}
const THEMED: &str = ".perf-theme";

/// Mounting the provider and unmounting it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(THEMED, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.wait_for_attr(THEMED, "data-theme", Some("light"))
        .await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(THEMED, 0).await?;
    Ok(())
}

/// Switching the theme through the provider's context to dark and back to light.
#[browser_test]
pub async fn switch_theme(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let themed = page.element(THEMED).await?;
    page.measure("dark", page.control("dark")).await?;
    themed.wait_for_attr("data-theme", Some("dark")).await?;
    page.measure("light", page.control("light")).await?;
    themed.wait_for_attr("data-theme", Some("light")).await?;
    Ok(())
}

/// Switching a controlled provider's theme to dark and back to light.
#[browser_test]
pub async fn switch_controlled_theme(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let themed = page.element(".perf-theme-controlled").await?;
    page.measure("dark", page.control("controlled-dark"))
        .await?;
    themed.wait_for_attr("data-theme", Some("dark")).await?;
    page.measure("light", page.control("controlled-light"))
        .await?;
    themed.wait_for_attr("data-theme", Some("light")).await?;
    Ok(())
}
