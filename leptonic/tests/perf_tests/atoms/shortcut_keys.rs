// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the atom `ShortcutKeys` (`/perf/atoms/shortcut_keys`, `Mod+Shift+K`): what
//! it costs, measured (`Page::measure`) and checked against its budget
//! (`tests/budgets/atoms/shortcut_keys.json`): mounting it (it renders the platform's keys once
//! mounted) and unmounting it. Its shortcut isn't reactive.
use browser_test::{TestGroup, browser_test};
use rootcause::Report;

use crate::{harness::fixture_test, pages::Page};

const PATH: &str = "/perf/atoms/shortcut_keys";

/// Registers the performance tests of `ShortcutKeys`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group.with(fixture_test(MountAndUnmount))
}
const KEYS: &str = ".perf-shortcut-keys";

/// Mounting the keys of the shortcut and unmounting them.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(KEYS, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.wait_for_count(".perf-shortcut-keys > kbd", 3).await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(KEYS, 0).await?;
    Ok(())
}
