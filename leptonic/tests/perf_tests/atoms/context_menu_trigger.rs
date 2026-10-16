// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the atom `ContextMenuTrigger` (`/perf/atoms/context_menu_trigger`): what
//! each thing it offers costs, measured (`Page::measure`) and checked against its budget
//! (`tests/budgets/atoms/context_menu_trigger.json`): mounting and unmounting it (with its list and
//! menu), a right click on a row opening the menu at the pointer, the keyboard opening it, an
//! action, Escape and a right click outside closing it (the focus returning to the row), and a
//! right click while disabled (ignored).
use std::time::Duration;

use assertr::prelude::*;
use browser_test::{TestGroup, browser_test, thirtyfour::prelude::*};
use leptonic::AriaRole;
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page, role},
};

const PATH: &str = "/perf/atoms/context_menu_trigger";
const MENU: &str = "[role=menu]";

/// Registers the performance tests of `ContextMenuTrigger`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(RightClickAndAction))
        .with(fixture_test(EscapeCloses))
        .with(fixture_test(KeyboardOpens))
        .with(fixture_test(RightClickOutsideCloses))
        .with(fixture_test(IgnoredWhileDisabled))
}

/// The row with the text `text`.
async fn row(page: &Page<'_>, text: &str) -> Result<WebElement, Report> {
    page.element(role(AriaRole::Row).text(text)).await
}

/// A right click on `element` with the pointer.
async fn context_click(page: &Page<'_>, element: &WebElement) -> Result<(), Report> {
    page.low_level()
        .driver()
        .action_chain()
        .context_click_element(element)
        .perform()
        .await?;
    Ok(())
}

/// Waits until the menu is open, labelled by the row `row`, placed and done entering.
async fn wait_for_open(page: &Page<'_>, row: &WebElement) -> Result<(), Report> {
    let menu = page.element(MENU).await?;
    let row_id = row.attr("id").await?;
    menu.wait_for_attr("aria-labelledby", row_id.as_deref())
        .await?;
    page.element(".leptonic-Popover[data-placement]").await?;
    page.wait_for_count(".leptonic-Popover[data-entering]", 0)
        .await
}

/// Mounting the trigger (closed, with its list) and unmounting it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count("[role=grid]", 0).await?;
    page.measure("mount", page.control("mount")).await?;
    row(page, "Music").await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count("[role=grid]", 0).await?;
    Ok(())
}

/// A right click on a row opening the menu, and an action closing it (the focus back on the row).
#[browser_test]
pub async fn right_click_and_action(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let pictures = row(page, "Pictures").await?;
    page.measure("right click", context_click(page, &pictures))
        .await?;
    wait_for_open(page, &pictures).await?;
    let rename = page
        .element(role(AriaRole::Menuitem).text("Rename"))
        .await?;
    page.measure("action", rename.click()).await?;
    page.wait_for_count(MENU, 0).await?;
    page.wait_for_focus(&pictures).await?;
    assert_that!(page.control("actions").await?).is_equal_to(1.0);
    Ok(())
}

/// Escape closing the menu opened on a row (the focus back on the row).
#[browser_test]
pub async fn escape_closes(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let music = row(page, "Music").await?;
    context_click(page, &music).await?;
    wait_for_open(page, &music).await?;
    page.measure("escape", page.send_keys(Key::Escape)).await?;
    page.wait_for_count(MENU, 0).await?;
    page.wait_for_focus(&music).await?;
    Ok(())
}

/// The keyboard opening the focused row's menu: Shift+F10 (the browser's `contextmenu`), Ctrl+Enter
/// on macOS.
#[browser_test]
pub async fn keyboard_opens(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element("#perf-before").await?.click().await?;
    page.send_keys(Key::Tab).await?;
    let documents = row(page, "Documents").await?;
    page.wait_for_focus(&documents).await?;
    let mac = page
        .reported_platform()
        .await?
        .to_ascii_lowercase()
        .starts_with("mac");
    let keys = if mac {
        Key::Control + Key::Enter
    } else {
        Key::Shift + Key::F10
    };
    page.measure("open", page.send_keys(keys)).await?;
    wait_for_open(page, &documents).await?;
    Ok(())
}

/// A right button press outside the list (on the page's body) closing the open menu.
#[browser_test]
pub async fn right_click_outside_closes(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let music = row(page, "Music").await?;
    context_click(page, &music).await?;
    wait_for_open(page, &music).await?;
    // A right button press on the page's body (the modal popover's underlay covers it for the
    // pointer; the trigger listens on the document).
    let press = page.low_level().eval::<()>(
        "document.body.dispatchEvent(new MouseEvent('mousedown', {bubbles: true, button: 2}));",
        vec![],
    );
    page.measure("right click outside", press).await?;
    page.wait_for_count(MENU, 0).await?;
    Ok(())
}

/// A right click on a row while the trigger is disabled: no menu.
#[browser_test]
pub async fn ignored_while_disabled(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.measure("disable", page.control("disable")).await?;
    let music = row(page, "Music").await?;
    page.measure("ignored right click", context_click(page, &music))
        .await?;
    page.count_stays(MENU, 0, Duration::from_millis(100))
        .await?;
    page.measure("enable", page.control("enable")).await?;
    Ok(())
}
