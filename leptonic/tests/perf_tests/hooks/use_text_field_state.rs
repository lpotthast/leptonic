// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the hook `use_text_field_state`
//! (`leptonic/src/hooks/form/use_text_field_state.rs`, fixture
//! `/perf/hooks/use_text_field_state`): what each thing it offers costs, measured
//! (`Page::measure`) and checked against its budget
//! (`tests/budgets/hooks/use_text_field_state.json`): creating and disposing of it (its value
//! shown), setting a new value, a value it ignores (the same), and a 10,000 character value.
use assertr::{matchers::eq, prelude::*};
use browser_test::{TestGroup, browser_test};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/hooks/use_text_field_state";
const VALUE: &str = "#perf-value";

/// Registers the performance tests of `use_text_field_state`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(SetTheValue))
        .with(fixture_test(SetALongValue))
}

/// How often the state reported a change.
async fn changes(page: &Page<'_>) -> Result<f64, Report> {
    page.control("changes").await
}

/// Creating the state (its value shown) and disposing of it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(VALUE, 0).await?;
    assert_that!(|| page.control("length"))
        .eventually_ok()
        .matches(eq(0.0))
        .await;
    page.measure("mount", page.control("mount")).await?;
    page.element(VALUE)
        .await?
        .wait_for_inner_text("Ada")
        .await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(VALUE, 0).await?;
    Ok(())
}

/// Setting the value it has (ignored) and a new value (reported as a change).
#[browser_test]
pub async fn set_the_value(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let value = page.element(VALUE).await?;
    page.measure("ignored set", page.control("set-same-value"))
        .await?;
    value.wait_for_inner_text("Ada").await?;
    assert_that!(changes(page).await?).is_equal_to(0.0);
    page.measure("set", page.control("set-value")).await?;
    value.wait_for_inner_text("Grace").await?;
    assert_that!(changes(page).await?).is_equal_to(1.0);
    Ok(())
}

/// Setting a 10,000 character value.
#[browser_test]
pub async fn set_a_long_value(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.measure("set", page.control("set-long-value")).await?;
    assert_that!(page.control("length").await?).is_equal_to(10_000.0);
    assert_that!(changes(page).await?).is_equal_to(1.0);
    Ok(())
}
