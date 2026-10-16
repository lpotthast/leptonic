// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the atom `SwitchField` (`/perf/atoms/switch_field`): what each thing it
//! offers costs, measured (`Page::measure`) and checked against its budget
//! (`tests/budgets/atoms/switch_field.json`): mounting and unmounting it (with its
//! `SwitchButton`), its selection controlled by app state (changed by the app, and by a click
//! through `set_selected`), and changes of its reactive props (disabled, read-only, required,
//! invalid). The button's interactions are measured in `switch_button.rs`.
use assertr::prelude::*;
use browser_test::{TestGroup, browser_test};
use rootcause::Report;

use crate::{harness::fixture_test, pages::Page};

const PATH: &str = "/perf/atoms/switch_field";

/// Registers the performance tests of `SwitchField`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(ControlledSelection))
        .with(fixture_test(ClickThroughSetSelected))
        .with(fixture_test(DisableAndEnable))
        .with(fixture_test(ReadOnlyAndEditable))
        .with(fixture_test(RequiredAndOptional))
        .with(fixture_test(InvalidAndValid))
}
const FIELD: &str = "#perf-switch-field";

/// Measures the control `on` (named after it), waits until the field has the flag `attr`, then
/// measures the control `off` and waits until the flag is gone.
async fn flag_round_trip(page: &Page<'_>, attr: &str, on: &str, off: &str) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.measure(on, page.control(on)).await?;
    page.wait_for_attr(FIELD, attr, Some("true")).await?;
    page.measure(off, page.control(off)).await?;
    page.wait_for_attr(FIELD, attr, None).await?;
    Ok(())
}

/// Mounting the field (with its button) and unmounting it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(FIELD, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.element(FIELD).await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(FIELD, 0).await?;
    Ok(())
}

/// The app switching on and off the controlled switch.
#[browser_test]
pub async fn controlled_selection(page: &Page<'_>) -> Result<(), Report> {
    flag_round_trip(page, "data-selected", "select", "deselect").await
}

/// A click on the button switching the controlled switch on: the change goes to the app's state
/// (`set_selected`) and back.
#[browser_test]
pub async fn click_through_set_selected(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let switch = page.element("#perf-switch").await?;
    page.measure("switch on", switch.click()).await?;
    page.wait_for_attr(FIELD, "data-selected", Some("true"))
        .await?;
    assert_that!(page.control("selected").await?).is_equal_to(1.0);
    Ok(())
}

/// Disabling the switch and enabling it again.
#[browser_test]
pub async fn disable_and_enable(page: &Page<'_>) -> Result<(), Report> {
    flag_round_trip(page, "data-disabled", "disable", "enable").await
}

/// Making the switch read-only and editable again.
#[browser_test]
pub async fn read_only_and_editable(page: &Page<'_>) -> Result<(), Report> {
    flag_round_trip(page, "data-readonly", "make-read-only", "make-editable").await
}

/// Requiring the switch and making it optional again.
#[browser_test]
pub async fn required_and_optional(page: &Page<'_>) -> Result<(), Report> {
    flag_round_trip(page, "data-required", "require", "make-optional").await
}

/// Marking the switch invalid and valid again.
#[browser_test]
pub async fn invalid_and_valid(page: &Page<'_>) -> Result<(), Report> {
    flag_round_trip(page, "data-invalid", "invalidate", "validate").await
}
