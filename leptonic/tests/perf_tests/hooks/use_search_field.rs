// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the hook `use_search_field`
//! (`leptonic/src/hooks/form/use_search_field.rs`, fixture `/perf/hooks/use_search_field`): what
//! each thing it offers costs, measured (`Page::measure`) and checked against its budget
//! (`tests/budgets/hooks/use_search_field.json`): mounting and unmounting a search field with its
//! clear button, typing, Enter submitting, Escape clearing (and Escapes it ignores: on an empty
//! field, while read-only), and a click on the clear button.
use assertr::prelude::*;
use browser_test::{TestGroup, browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/hooks/use_search_field";
const INPUT: &str = "#perf-input";
const CLEAR: &str = "#perf-clear";

/// Registers the performance tests of `use_search_field`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(Typing))
        .with(fixture_test(SubmitWithEnter))
        .with(fixture_test(ClearWithEscape))
        .with(fixture_test(IgnoredWhileReadOnly))
        .with(fixture_test(ClearButton))
}

/// How often the field cleared.
async fn clears(page: &Page<'_>) -> Result<f64, Report> {
    page.control("clears").await
}

/// Focuses the input with the caret at the end of its value.
async fn focus_at_end(page: &Page<'_>) -> Result<WebElement, Report> {
    let input = page.element(INPUT).await?;
    input.type_keys(Key::End).await?;
    page.wait_for_focus(&input).await?;
    Ok(input)
}

/// Mounting the search field with its clear button and unmounting it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(INPUT, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    let input = page.element(INPUT).await?;
    input.wait_for_attr("type", Some("search")).await?;
    page.element(CLEAR)
        .await?
        .wait_for_attr("tabindex", Some("-1"))
        .await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(INPUT, 0).await?;
    Ok(())
}

/// Typing a character.
#[browser_test]
pub async fn typing(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = focus_at_end(page).await?;
    page.measure("type a character", page.send_keys("s"))
        .await?;
    input.wait_for_prop("value", "leptoss").await?;
    Ok(())
}

/// Enter submitting the search.
#[browser_test]
pub async fn submit_with_enter(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    focus_at_end(page).await?;
    page.measure("enter", page.send_keys(Key::Enter)).await?;
    assert_that!(page.control("submits").await?).is_equal_to(1.0);
    Ok(())
}

/// Escape clearing the field, and a second Escape it ignores (the field is empty).
#[browser_test]
pub async fn clear_with_escape(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = focus_at_end(page).await?;
    page.measure("escape", page.send_keys(Key::Escape)).await?;
    input.wait_for_prop("value", "").await?;
    page.measure("ignored escape", page.send_keys(Key::Escape))
        .await?;
    assert_that!(clears(page).await?).is_equal_to(1.0);
    Ok(())
}

/// Enter and Escape on a read-only field: it neither submits nor clears.
#[browser_test]
pub async fn ignored_while_read_only(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = focus_at_end(page).await?;
    page.control("set-read-only").await?;
    input.wait_for_attr("readonly", Some("true")).await?;
    page.measure("ignored enter", page.send_keys(Key::Enter))
        .await?;
    page.measure("ignored escape", page.send_keys(Key::Escape))
        .await?;
    input.wait_for_prop("value", "leptos").await?;
    assert_that!(page.control("submits").await?).is_equal_to(0.0);
    assert_that!(clears(page).await?).is_equal_to(0.0);
    Ok(())
}

/// A click on the clear button: the field clears and its input takes focus.
#[browser_test]
pub async fn clear_button(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = page.element(INPUT).await?;
    let clear = page.element(CLEAR).await?;
    page.measure("click", clear.click()).await?;
    input.wait_for_prop("value", "").await?;
    page.wait_for_focus(&input).await?;
    assert_that!(clears(page).await?).is_equal_to(1.0);
    Ok(())
}
