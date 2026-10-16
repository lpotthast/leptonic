// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the atom `DisclosurePanel` (`/perf/atoms/disclosure_panel`): what each thing
//! it offers costs, measured (`Page::measure`) and checked against its budget
//! (`tests/budgets/atoms/disclosure_panel.json`): mounting and unmounting it (collapsed and
//! expanded), showing and hiding it, find in page expanding it, and the focus ring within it.
use assertr::prelude::*;
use browser_test::{TestGroup, browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, EventKind, Page, SyntheticEvent},
};

const PATH: &str = "/perf/atoms/disclosure_panel";

/// Registers the performance tests of `DisclosurePanel`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(MountAndUnmountExpanded))
        .with(fixture_test(ShowAndHide))
        .with(fixture_test(FindInPage))
        .with(fixture_test(FocusWithin))
}
const PANEL: &str = "#perf-disclosure .leptonic-DisclosurePanel";

/// Waits until the panel is shown (`shown`) or hidden (`hidden="until-found"`, `aria-hidden`).
async fn wait_for_shown(page: &Page<'_>, shown: bool) -> Result<(), Report> {
    let panel = page.element(PANEL).await?;
    panel
        .wait_for_prop("hidden", if shown { "false" } else { "until-found" })
        .await?;
    assert_that!(panel.attr("aria-hidden").await?).is_equal_to((!shown).then(|| "true".to_owned()));
    Ok(())
}

/// Mounting the collapsed panel and unmounting it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(PANEL, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    wait_for_shown(page, false).await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(PANEL, 0).await?;
    // Mounted again: the trigger's `aria-controls` refers to the panel.
    page.control("mount").await?;
    page.element(PANEL).await?;
    Ok(())
}

/// Mounting the panel of the expanded disclosure (shown) and unmounting it.
#[browser_test]
pub async fn mount_and_unmount_expanded(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(PANEL, 0).await?;
    page.control("expand").await?;
    page.measure("mount", page.control("mount")).await?;
    wait_for_shown(page, true).await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(PANEL, 0).await?;
    // Mounted again: the trigger's `aria-controls` refers to the panel.
    page.control("mount").await?;
    page.element(PANEL).await?;
    Ok(())
}

/// The disclosure expanding (the panel shown) and collapsing (hidden).
#[browser_test]
pub async fn show_and_hide(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.measure("show", page.control("expand")).await?;
    wait_for_shown(page, true).await?;
    page.measure("hide", page.control("collapse")).await?;
    wait_for_shown(page, false).await?;
    Ok(())
}

/// Find in page matching the collapsed panel's content (`beforematch`): it is shown.
#[browser_test]
pub async fn find_in_page(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let panel = page.element(PANEL).await?;
    page.measure(
        "beforematch",
        panel.dispatch(SyntheticEvent::plain(EventKind::BeforeMatch).bubbles(false)),
    )
    .await?;
    wait_for_shown(page, true).await?;
    Ok(())
}

/// Tab moving focus from the trigger into the shown panel (a focus ring within it) and out of it.
#[browser_test]
pub async fn focus_within(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("expand").await?;
    wait_for_shown(page, true).await?;
    page.element("#perf-before").await?.click().await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&page.element("#perf-trigger").await?)
        .await?;
    let panel = page.element(PANEL).await?;
    page.measure("focus", page.send_keys(Key::Tab)).await?;
    panel
        .wait_for_attr("data-focus-visible-within", Some("true"))
        .await?;
    page.wait_for_focus(&page.element("#perf-panel-button").await?)
        .await?;
    page.measure("blur", page.send_keys(Key::Tab)).await?;
    panel
        .wait_for_attr("data-focus-visible-within", None)
        .await?;
    page.wait_for_focus(&page.element("#perf-after").await?)
        .await?;
    Ok(())
}
