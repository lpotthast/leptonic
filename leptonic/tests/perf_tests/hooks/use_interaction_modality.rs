// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the hook `use_interaction_modality`
//! (`/perf/hooks/use_interaction_modality`): what each thing it offers costs, measured
//! (`Page::measure`) and checked against its budget
//! (`tests/budgets/hooks/use_interaction_modality.json`): mounting and unmounting it, and the
//! modality changing: to the pointer (a click), the keyboard (a key press), an interaction that
//! keeps it (a second key press), and back to the pointer.
use browser_test::{TestGroup, browser_test};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/hooks/use_interaction_modality";

/// Registers the performance tests of `use_interaction_modality`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(ModalityChanges))
}
const MODALITY: &str = "#perf-modality";

/// Mounting the hook and unmounting it (after a click: the modality is the pointer).
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element("#perf-before").await?.click().await?;
    page.control("unmount").await?;
    page.wait_for_count(MODALITY, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.wait_for_attr(MODALITY, "data-modality", Some("pointer"))
        .await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(MODALITY, 0).await?;
    Ok(())
}

/// A click (the pointer), a key press (the keyboard), another (no change), and a click again
/// (the pointer). The virtual modality takes assistive technology moving the focus: a test can't
/// (a script's `focus()` counts as an event before the focus, a virtual click sets the modality
/// without notifying, both as react-aria's).
#[browser_test]
pub async fn modality_changes(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let before = page.element("#perf-before").await?;
    let modality = page.element(MODALITY).await?;
    page.measure("pointer", before.click()).await?;
    modality
        .wait_for_attr("data-modality", Some("pointer"))
        .await?;
    page.measure("keyboard", page.send_keys("x")).await?;
    modality
        .wait_for_attr("data-modality", Some("keyboard"))
        .await?;
    page.measure("keyboard again", page.send_keys("x")).await?;
    modality
        .wait_for_attr("data-modality", Some("keyboard"))
        .await?;
    page.measure("pointer again", before.click()).await?;
    modality
        .wait_for_attr("data-modality", Some("pointer"))
        .await?;
    Ok(())
}
