// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the hook `use_keyboard` (`/perf/hooks/use_keyboard`, on a focusable
//! `<div>`): what each thing it offers costs, measured (`Page::measure`) and checked against its
//! budget (`tests/budgets/hooks/use_keyboard.json`): mounting and unmounting it, a key (down and
//! up), its shortcut, a held key (repeats, which its shortcut ignores), and disabling it (a key
//! it ignores).
use assertr::prelude::*;
use browser_test::{TestGroup, browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::{harness::fixture_test, pages::Page};

const PATH: &str = "/perf/hooks/use_keyboard";

/// Registers the performance tests of `use_keyboard`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(KeyDownAndUp))
        .with(fixture_test(ShortcutKey))
        .with(fixture_test(HeldKey))
        .with(fixture_test(DisableAndEnable))
}
const KEYED: &str = "#perf-keyboard";

/// The counts of key downs, key ups and shortcut uses. The key ups include the one of the Tab
/// that focused the element.
async fn counts(page: &Page<'_>) -> Result<(f64, f64, f64), Report> {
    Ok((
        page.control("key-downs").await?,
        page.control("key-ups").await?,
        page.control("shortcuts").await?,
    ))
}

/// Focuses the element with the keyboard (Tab from the plain button before it).
async fn focus_with_tab(page: &Page<'_>) -> Result<(), Report> {
    page.element("#perf-before").await?.click().await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&page.element(KEYED).await?).await?;
    Ok(())
}

/// Mounting the element with its key handling and unmounting it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(KEYED, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.element(KEYED).await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(KEYED, 0).await?;
    Ok(())
}

/// A key (Shift) pressed and released on the focused element.
#[browser_test]
pub async fn key_down_and_up(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    focus_with_tab(page).await?;
    page.measure("key", page.send_keys(Key::Shift)).await?;
    assert_that!(counts(page).await?).is_equal_to((1.0, 2.0, 0.0));
    Ok(())
}

/// The shortcut `A` on the focused element.
#[browser_test]
pub async fn shortcut_key(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    focus_with_tab(page).await?;
    page.measure("shortcut", page.send_keys("a")).await?;
    assert_that!(counts(page).await?).is_equal_to((1.0, 2.0, 1.0));
    Ok(())
}

/// The shortcut's key held down: a key down, 10 repeats (which the shortcut ignores) and a key
/// up.
#[browser_test]
pub async fn held_key(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    focus_with_tab(page).await?;
    page.measure("hold", page.hold_key("a", 10)).await?;
    assert_that!(counts(page).await?).is_equal_to((11.0, 2.0, 1.0));
    Ok(())
}

/// Disabling the key handling, a key it ignores, and enabling it again.
#[browser_test]
pub async fn disable_and_enable(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    focus_with_tab(page).await?;
    page.measure("disable", page.control("disable")).await?;
    page.measure("ignored key", page.send_keys("a")).await?;
    assert_that!(counts(page).await?).is_equal_to((0.0, 1.0, 0.0));
    page.measure("enable", page.control("enable")).await?;
    page.send_keys("a").await?;
    assert_that!(counts(page).await?).is_equal_to((1.0, 2.0, 1.0));
    Ok(())
}
