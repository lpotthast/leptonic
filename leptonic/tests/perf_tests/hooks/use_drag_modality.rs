// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the hook `use_drag_modality` (`/perf/hooks/use_drag_modality`): the drag
//! modality, following the interaction modality. Measured (`Page::measure`) and checked against
//! its budget (`tests/budgets/hooks/use_drag_modality.json`): mounting and unmounting it, and the
//! modality changing with the keyboard (Tab), a mouse click and a virtual click (a screen
//! reader's).
use browser_test::{TestGroup, browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/hooks/use_drag_modality";
const MODALITY: &str = "#perf-modality";

/// Registers the performance tests of `use_drag_modality`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(ModalityChanges))
}

/// Mounting the hook and unmounting it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(MODALITY, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.element(MODALITY).await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(MODALITY, 0).await?;
    Ok(())
}

/// The modality changing: to the keyboard (Tab), to virtual by a mouse click (a fine pointer),
/// to the keyboard again; and a screen reader's virtual click, which changes the interaction
/// modality silently (as react-aria's): the drag modality stays until the next notification.
#[browser_test]
pub async fn modality_changes(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let modality = page.element(MODALITY).await?;
    let before = page.element("#perf-before").await?;
    let after = page.element("#perf-after").await?;
    before.click().await?;
    modality
        .wait_for_attr("data-modality", Some("virtual"))
        .await?;
    page.measure("keyboard", page.send_keys(Key::Tab)).await?;
    modality
        .wait_for_attr("data-modality", Some("keyboard"))
        .await?;
    page.measure("mouse click", after.click()).await?;
    modality
        .wait_for_attr("data-modality", Some("virtual"))
        .await?;
    page.send_keys(Key::Tab).await?;
    modality
        .wait_for_attr("data-modality", Some("keyboard"))
        .await?;
    page.measure("virtual click", before.virtual_click())
        .await?;
    modality
        .attr_stays(
            "data-modality",
            Some("keyboard"),
            std::time::Duration::from_millis(100),
        )
        .await?;
    Ok(())
}
