// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the hook `use_global_shortcuts` (`/perf/hooks/use_global_shortcuts`,
//! binding `Mod+K` anywhere and `/` outside text fields): what each thing it offers costs,
//! measured (`Page::measure`) and checked against its budget
//! (`tests/budgets/hooks/use_global_shortcuts.json`): binding and unbinding them (mounting and
//! unmounting), each shortcut, `Mod+K` while typing, and what it ignores: `/` typed into a text
//! field, a key bound to nothing.
use assertr::prelude::*;
use browser_test::{TestGroup, browser_test};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/hooks/use_global_shortcuts";

/// Registers the performance tests of `use_global_shortcuts`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(Shortcuts))
        .with(fixture_test(WhileTyping))
        .with(fixture_test(UnboundKey))
}
const BOUND: &str = "#perf-shortcuts";

/// How often `Mod+K` and `/` ran.
async fn counts(page: &Page<'_>) -> Result<(f64, f64), Report> {
    Ok((page.control("palette").await?, page.control("slash").await?))
}

/// Binding the shortcuts (mounting) and unbinding them (unmounting).
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(BOUND, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.element(BOUND).await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(BOUND, 0).await?;
    Ok(())
}

/// `Mod+K` and `/`, the focus on a button.
#[browser_test]
pub async fn shortcuts(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element("#perf-before").await?.click().await?;
    let modifier = page.primary_modifier().await?;
    page.measure("mod+k", page.send_keys(modifier + "k"))
        .await?;
    page.measure("slash", page.send_keys("/")).await?;
    assert_that!(counts(page).await?).is_equal_to((1.0, 1.0));
    Ok(())
}

/// In a text field: `/` is typed (not a shortcut there), `Mod+K` works.
#[browser_test]
pub async fn while_typing(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = page.element("#perf-input").await?;
    input.click().await?;
    page.measure("typed slash", page.send_keys("/")).await?;
    input.wait_for_prop("value", "/").await?;
    let modifier = page.primary_modifier().await?;
    page.measure("mod+k", page.send_keys(modifier + "k"))
        .await?;
    assert_that!(counts(page).await?).is_equal_to((1.0, 0.0));
    Ok(())
}

/// A key bound to nothing (`x`), the focus on a button.
#[browser_test]
pub async fn unbound_key(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element("#perf-before").await?.click().await?;
    page.measure("x", page.send_keys("x")).await?;
    assert_that!(counts(page).await?).is_equal_to((0.0, 0.0));
    Ok(())
}
