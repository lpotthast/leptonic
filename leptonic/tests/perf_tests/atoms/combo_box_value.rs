// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the atom `ComboBoxValue` (`/perf/atoms/combo_box_value`, the fixture of
//! `ComboBox`'s performance test): what each thing it offers costs, measured (`Page::measure`) and
//! checked against its budget (`tests/budgets/atoms/combo_box_value.json`): mounting and unmounting
//! it in its combo box, the combo box's value changing (10 and 1,000 options), and its placeholder
//! changing.
use browser_test::{TestGroup, browser_test};
use rootcause::Report;

use super::combo_box::{LARGE, VALUE, controlled_value_in, goto, large};
use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/atoms/combo_box_value";

/// Registers the performance tests of `ComboBoxValue`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(ValueChange))
        .with(fixture_test(ValueChangeLarge))
        .with(fixture_test(Placeholder))
}

/// Mounting the value in its combo box and unmounting it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    goto(page, PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(VALUE, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.element(VALUE)
        .await?
        .wait_for_inner_text("Nothing selected")
        .await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(VALUE, 0).await?;
    Ok(())
}

/// The value of a combo box of `last + 1` options changing: the first option, the last, none.
async fn value_change_in(page: &Page<'_>, last: u32) -> Result<(), Report> {
    controlled_value_in(page, last).await?;
    page.element(VALUE)
        .await?
        .wait_for_attr("data-placeholder", Some("true"))
        .await?;
    Ok(())
}

/// The value of a combo box of 10 options changing.
#[browser_test]
pub async fn value_change(page: &Page<'_>) -> Result<(), Report> {
    goto(page, PATH).await?;
    value_change_in(page, 9).await
}

/// The value of a combo box of 1,000 options changing.
#[browser_test]
pub async fn value_change_large(page: &Page<'_>) -> Result<(), Report> {
    goto(page, PATH).await?;
    large(page).await?;
    value_change_in(page, LARGE - 1).await
}

/// The placeholder changing while nothing is selected.
#[browser_test]
pub async fn placeholder(page: &Page<'_>) -> Result<(), Report> {
    goto(page, PATH).await?;
    let shown = page.element(VALUE).await?;
    page.measure("rename", page.control("rename-placeholder"))
        .await?;
    shown.wait_for_inner_text("No option").await?;
    Ok(())
}
