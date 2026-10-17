// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the hook `use_tag_group` (`/perf/hooks/use_tag_group`, on a plain element):
//! what each thing it offers costs, measured (`Page::measure`) and checked against its budget
//! (`tests/budgets/hooks/use_tag_group.json`): mounting and unmounting the group, keyboard focus
//! coming in and leaving, keyboard navigation (ArrowRight, ArrowLeft, End, Home), and removing tags
//! (the group announces it), with 10 tags and with 1,000.
use browser_test::{TestGroup, browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::{harness::fixture_test, pages::Page};

const PATH: &str = "/perf/hooks/use_tag_group";

/// Registers the performance tests of `use_tag_group`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(MountAndUnmountLarge))
        .with(fixture_test(KeyboardNavigation))
        .with(fixture_test(KeyboardNavigationLarge))
        .with(fixture_test(RemoveWithTheKeyboard))
        .with(fixture_test(RemoveWithTheKeyboardLarge))
}
const GROUP: &str = "[data-perf-group]";
const TAGS: &str = "[data-perf-tag]";

/// Loads the fixture at `path` and creates the live announcer's regions (its first announcement
/// waits for them, so an announcing step measured first would count them, or not).
async fn goto(page: &Page<'_>, path: &str) -> Result<(), Report> {
    page.goto_path(path).await?;
    page.control("prepare-announcer").await?;
    page.wait_for_count("[data-live-announcer] [aria-live=polite] > div", 1)
        .await?;
    Ok(())
}

/// The tag "Tag `i`".
fn tag(i: u32) -> String {
    format!("[data-perf-tag='{i}']")
}

/// Switches to 1,000 tags (unmeasured).
async fn large(page: &Page<'_>) -> Result<(), Report> {
    page.control("large").await?;
    page.element(tag(999)).await?;
    Ok(())
}

/// Focuses the first tag with the keyboard (Tab from the plain button before the group).
async fn tab_in(page: &Page<'_>) -> Result<WebElement, Report> {
    page.element("#perf-before").await?.click().await?;
    page.send_keys(Key::Tab).await?;
    let first = page.element(tag(0)).await?;
    page.wait_for_focus(&first).await?;
    Ok(first)
}

/// Mounting `container` (the group, or its tags) with `count` tags and unmounting it.
async fn mount_and_unmount_in(
    page: &Page<'_>,
    container: &str,
    count: usize,
) -> Result<(), Report> {
    page.control("unmount").await?;
    page.wait_for_count(container, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.wait_for_count(TAGS, count).await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(container, 0).await?;
    Ok(())
}

/// Mounting the group with 10 tags and unmounting it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    goto(page, PATH).await?;
    mount_and_unmount_in(page, GROUP, 10).await
}

/// Mounting the group with 1,000 tags and unmounting it.
#[browser_test]
pub async fn mount_and_unmount_large(page: &Page<'_>) -> Result<(), Report> {
    goto(page, PATH).await?;
    large(page).await?;
    mount_and_unmount_in(page, GROUP, 1_000).await
}

/// Tab focusing the first tag, ArrowRight, ArrowLeft, End and Home moving the focus through the
/// tags (`last + 1`), and Shift+Tab leaving the group.
async fn keyboard_navigation_in(page: &Page<'_>, last: u32) -> Result<(), Report> {
    page.element("#perf-before").await?.click().await?;
    let first = page.element(tag(0)).await?;
    page.measure("focus", page.send_keys(Key::Tab)).await?;
    page.wait_for_focus(&first).await?;
    page.measure("arrow right", page.send_keys(Key::Right))
        .await?;
    page.wait_for_focus(&page.element(tag(1)).await?).await?;
    page.measure("arrow left", page.send_keys(Key::Left))
        .await?;
    page.wait_for_focus(&first).await?;
    page.measure("end", page.send_keys(Key::End)).await?;
    page.wait_for_focus(&page.element(tag(last)).await?).await?;
    page.measure("home", page.send_keys(Key::Home)).await?;
    page.wait_for_focus(&first).await?;
    page.measure("blur", page.send_keys(Key::Shift + Key::Tab))
        .await?;
    page.wait_for_focus(&page.element("#perf-before").await?)
        .await?;
    Ok(())
}

/// Keyboard navigation through 10 tags.
#[browser_test]
pub async fn keyboard_navigation(page: &Page<'_>) -> Result<(), Report> {
    goto(page, PATH).await?;
    keyboard_navigation_in(page, 9).await
}

/// Keyboard navigation through 1,000 tags.
#[browser_test]
pub async fn keyboard_navigation_large(page: &Page<'_>) -> Result<(), Report> {
    goto(page, PATH).await?;
    large(page).await?;
    keyboard_navigation_in(page, 999).await
}

/// Delete removing the focused fourth tag (focus moves to the next) and Backspace removing that
/// one.
async fn remove_with_the_keyboard_in(page: &Page<'_>) -> Result<(), Report> {
    tab_in(page).await?;
    for i in 1..4 {
        page.send_keys(Key::Right).await?;
        page.wait_for_focus(&page.element(tag(i)).await?).await?;
    }
    page.measure("delete", page.send_keys(Key::Delete)).await?;
    page.wait_for_count(tag(3), 0).await?;
    page.wait_for_focus(&page.element(tag(4)).await?).await?;
    page.measure("backspace", page.send_keys(Key::Backspace))
        .await?;
    page.wait_for_count(tag(4), 0).await?;
    Ok(())
}

/// Removing two of 10 tags with the keyboard.
#[browser_test]
pub async fn remove_with_the_keyboard(page: &Page<'_>) -> Result<(), Report> {
    goto(page, PATH).await?;
    remove_with_the_keyboard_in(page).await
}

/// Removing two of 1,000 tags with the keyboard.
#[browser_test]
pub async fn remove_with_the_keyboard_large(page: &Page<'_>) -> Result<(), Report> {
    goto(page, PATH).await?;
    large(page).await?;
    remove_with_the_keyboard_in(page).await
}
