// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the hook `use_form_validation_state`
//! (`leptonic/src/hooks/form/use_form_validation_state.rs`, fixture
//! `/perf/hooks/use_form_validation_state`): what each thing it offers costs, measured
//! (`Page::measure`) and checked against its budget
//! (`tests/budgets/hooks/use_form_validation_state.json`), with native validation (errors show
//! once committed): creating and disposing it, a value change and its commit, a native validity
//! update, commit and reset, the controlled `is_invalid`, and the server's errors.
use assertr::{matchers::eq, prelude::*};
use browser_test::{TestGroup, browser_test};
use rootcause::Report;

use crate::{harness::fixture_test, pages::Page};

const PATH: &str = "/perf/hooks/use_form_validation_state";

/// Registers the performance tests of `use_form_validation_state`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(ValueChangeAndCommit))
        .with(fixture_test(NativeValidity))
        .with(fixture_test(ControlledInvalid))
        .with(fixture_test(ServerErrors))
}

/// Whether the state shows the value invalid (1) or not (0).
async fn invalid(page: &Page<'_>) -> Result<f64, Report> {
    page.control("invalid").await
}

/// The number of errors the state shows.
async fn errors(page: &Page<'_>) -> Result<f64, Report> {
    page.control("errors").await
}

/// Creating the state and disposing of it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    assert_that!(|| page.control("exists"))
        .eventually_ok()
        .matches(eq(0.0))
        .await;
    page.measure("mount", page.control("mount")).await?;
    assert_that!(page.control("exists").await?).is_equal_to(1.0);
    assert_that!(invalid(page).await?).is_equal_to(0.0);
    page.measure("unmount", page.control("unmount")).await?;
    assert_that!(page.control("exists").await?).is_equal_to(0.0);
    Ok(())
}

/// The value becoming invalid (nothing shows before a commit) and the commit showing its
/// error.
#[browser_test]
pub async fn value_change_and_commit(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.measure("invalid value", page.control("set-short"))
        .await?;
    assert_that!(invalid(page).await?).is_equal_to(0.0);
    page.measure("commit", page.control("commit")).await?;
    assert_that!(invalid(page).await?).is_equal_to(1.0);
    assert_that!(errors(page).await?).is_equal_to(1.0);
    Ok(())
}

/// The input's native validity reporting an error (deferred), its commit showing it, and a
/// reset clearing it.
#[browser_test]
pub async fn native_validity(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.measure("update", page.control("report-native-error"))
        .await?;
    assert_that!(invalid(page).await?).is_equal_to(0.0);
    page.measure("commit", page.control("commit")).await?;
    assert_that!(invalid(page).await?).is_equal_to(1.0);
    page.measure("reset", page.control("reset")).await?;
    assert_that!(invalid(page).await?).is_equal_to(0.0);
    Ok(())
}

/// The controlled `is_invalid` marking the value invalid (shown at once) and no longer.
#[browser_test]
pub async fn controlled_invalid(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.measure("invalidate", page.control("invalidate"))
        .await?;
    assert_that!(invalid(page).await?).is_equal_to(1.0);
    assert_that!(errors(page).await?).is_equal_to(0.0);
    page.measure("uninvalidate", page.control("uninvalidate"))
        .await?;
    assert_that!(invalid(page).await?).is_equal_to(0.0);
    Ok(())
}

/// The server's errors for the field's name arriving (shown at once) and a commit (the user
/// changed the value) clearing them.
#[browser_test]
pub async fn server_errors(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.measure("set", page.control("set-server-errors"))
        .await?;
    assert_that!(invalid(page).await?).is_equal_to(1.0);
    assert_that!(errors(page).await?).is_equal_to(1.0);
    page.measure("commit", page.control("commit")).await?;
    assert_that!(invalid(page).await?).is_equal_to(0.0);
    Ok(())
}
