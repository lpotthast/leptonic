// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the atom `Keys` (`/perf/atoms/keys`): what each thing it offers costs,
//! measured (`Page::measure`) and checked against its budget (`tests/budgets/atoms/keys.json`):
//! mounting and unmounting it (three keys), and its keys changing (to two and back).
use browser_test::{TestGroup, browser_test};
use rootcause::Report;

use crate::{harness::fixture_test, pages::Page};

const PATH: &str = "/perf/atoms/keys";

/// Registers the performance tests of `Keys`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(ChangeKeys))
}
const KEYS: &str = ".perf-keys";
const KEY: &str = ".perf-keys > kbd";

/// Mounting the keys and unmounting them.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(KEYS, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.wait_for_count(KEY, 3).await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(KEYS, 0).await?;
    Ok(())
}

/// Changing the keys to two and back to three.
#[browser_test]
pub async fn change_keys(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.wait_for_count(KEY, 3).await?;
    page.measure("two keys", page.control("two-keys")).await?;
    page.wait_for_count(KEY, 2).await?;
    page.measure("three keys", page.control("three-keys"))
        .await?;
    page.wait_for_count(KEY, 3).await?;
    Ok(())
}
