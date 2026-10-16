// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the atom `SearchField` (`leptonic/src/atoms/search_field.rs`, fixture
//! `/perf/atoms/search_field`): what each thing it offers costs, measured (`Page::measure`) and
//! checked against its budget (`tests/budgets/atoms/search_field.json`): mounting and unmounting
//! it with its label, input and clear button, typing, Enter submitting, Escape clearing (and an
//! Escape it ignores while empty), focus by keyboard, and disabling it.
use assertr::prelude::*;
use browser_test::{TestGroup, browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/atoms/search_field";
const FIELD: &str = ".leptonic-SearchField";
const INPUT: &str = "#perf-input";

/// Registers the performance tests of `SearchField`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(Typing))
        .with(fixture_test(SubmitWithEnter))
        .with(fixture_test(ClearWithEscape))
        .with(fixture_test(KeyboardFocusAndBlur))
        .with(fixture_test(DisableAndEnable))
}

/// Focuses the input with the caret at the end of its value.
async fn focus_at_end(page: &Page<'_>) -> Result<WebElement, Report> {
    let input = page.element(INPUT).await?;
    input.type_keys(Key::End).await?;
    page.wait_for_focus(&input).await?;
    Ok(input)
}

/// Mounting the field with its parts and unmounting it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(FIELD, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.element(INPUT)
        .await?
        .wait_for_prop("value", "leptos")
        .await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(FIELD, 0).await?;
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
    let field = page.element(FIELD).await?;
    let input = focus_at_end(page).await?;
    page.measure("escape", page.send_keys(Key::Escape)).await?;
    input.wait_for_prop("value", "").await?;
    field.wait_for_attr("data-empty", Some("true")).await?;
    page.measure("ignored escape", page.send_keys(Key::Escape))
        .await?;
    assert_that!(page.control("clears").await?).is_equal_to(1.0);
    Ok(())
}

/// Tab moving focus to the input (the clear button is not a tab stop) and on.
#[browser_test]
pub async fn keyboard_focus_and_blur(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element("#perf-before").await?.click().await?;
    let input = page.element(INPUT).await?;
    let after = page.element("#perf-after").await?;
    page.measure("focus", page.send_keys(Key::Tab)).await?;
    input
        .wait_for_attr("data-focus-visible", Some("true"))
        .await?;
    page.measure("blur", page.send_keys(Key::Tab)).await?;
    page.wait_for_focus(&after).await?;
    input.wait_for_attr("data-focused", None).await?;
    Ok(())
}

/// Disabling the field and enabling it again.
#[browser_test]
pub async fn disable_and_enable(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let field = page.element(FIELD).await?;
    let clear = page.element(".leptonic-SearchFieldClearButton").await?;
    page.measure("disable", page.control("disable")).await?;
    field.wait_for_attr("data-disabled", Some("true")).await?;
    clear.wait_for_attr("data-disabled", Some("true")).await?;
    page.measure("enable", page.control("enable")).await?;
    field.wait_for_attr("data-disabled", None).await?;
    clear.wait_for_attr("data-disabled", None).await?;
    Ok(())
}
