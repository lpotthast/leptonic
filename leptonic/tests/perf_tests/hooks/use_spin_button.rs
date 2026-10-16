// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the hook `use_spin_button`
//! (`leptonic/src/hooks/spinbutton/use_spin_button.rs`, fixture `/perf/hooks/use_spin_button`):
//! what each thing it offers costs, measured (`Page::measure`) and checked against its budget
//! (`tests/budgets/hooks/use_spin_button.json`): mounting and unmounting a spin button with its
//! increment stepper, focus by keyboard, the keys it handles (arrows, Page Up/Down, Home, End),
//! a click on the stepper, a value change from the page, and disabling it (and a key it then
//! ignores).
//!
//! Holding the stepper down steps repeatedly after 400 ms, every 60 ms until released: a timer
//! no measurement can span deterministically, so the test clicks it.
//!
//! A focused spin button announces each new value. The first announcement creates the live
//! announcer and shows after 100 ms, a timer no measurement may span: the tests step once
//! unmeasured and wait for that announcement first.
use assertr::prelude::*;
use browser_test::{TestGroup, browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/hooks/use_spin_button";
const SPIN_BUTTON: &str = "#perf-spin-button";
const INCREMENT: &str = "#perf-increment";
/// The live announcer's announcement.
const ANNOUNCEMENT: &str = "[data-live-announcer] [aria-live=assertive] div";

/// Registers the performance tests of `use_spin_button`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(KeyboardFocusAndBlur))
        .with(fixture_test(Keys))
        .with(fixture_test(Stepper))
        .with(fixture_test(ValueFromThePage))
        .with(fixture_test(DisableAndEnable))
}

/// The spin button's value.
async fn value(page: &Page<'_>) -> Result<f64, Report> {
    page.control("value").await
}

/// Focuses the spin button with the keyboard (Tab from the plain button before it).
async fn focus_with_tab(page: &Page<'_>) -> Result<WebElement, Report> {
    page.element("#perf-before").await?.click().await?;
    page.send_keys(Key::Tab).await?;
    let spin_button = page.element(SPIN_BUTTON).await?;
    page.wait_for_focus(&spin_button).await?;
    Ok(spin_button)
}

/// Waits for the announcement `text`.
async fn wait_for_announcement(page: &Page<'_>, text: &str) -> Result<(), Report> {
    page.wait_for_count(ANNOUNCEMENT, 1).await?;
    page.element(ANNOUNCEMENT)
        .await?
        .wait_for_inner_text(text)
        .await
}

/// Mounting the spin button and its stepper and unmounting them.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(SPIN_BUTTON, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    let spin_button = page.element(SPIN_BUTTON).await?;
    spin_button
        .wait_for_attr("aria-valuenow", Some("5"))
        .await?;
    spin_button
        .wait_for_attr("role", Some("spinbutton"))
        .await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(SPIN_BUTTON, 0).await?;
    Ok(())
}

/// Tab moving focus to the spin button and on.
#[browser_test]
pub async fn keyboard_focus_and_blur(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element("#perf-before").await?.click().await?;
    let spin_button = page.element(SPIN_BUTTON).await?;
    page.measure("focus", page.send_keys(Key::Tab)).await?;
    page.wait_for_focus(&spin_button).await?;
    page.measure("blur", page.send_keys(Key::Tab)).await?;
    let after = page.element(INCREMENT).await?;
    page.wait_for_focus(&after).await?;
    Ok(())
}

/// The keys it handles: the arrows (by one), Page Up and Page Down (by two), Home (the
/// minimum) and End (the maximum).
#[browser_test]
pub async fn keys(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let spin_button = focus_with_tab(page).await?;
    page.send_keys(Key::Up).await?;
    wait_for_announcement(page, "6").await?;
    page.measure("arrow up", page.send_keys(Key::Up)).await?;
    spin_button
        .wait_for_attr("aria-valuenow", Some("7"))
        .await?;
    page.measure("arrow down", page.send_keys(Key::Down))
        .await?;
    spin_button
        .wait_for_attr("aria-valuenow", Some("6"))
        .await?;
    page.measure("page up", page.send_keys(Key::PageUp)).await?;
    spin_button
        .wait_for_attr("aria-valuenow", Some("8"))
        .await?;
    page.measure("page down", page.send_keys(Key::PageDown))
        .await?;
    spin_button
        .wait_for_attr("aria-valuenow", Some("6"))
        .await?;
    page.measure("home", page.send_keys(Key::Home)).await?;
    spin_button
        .wait_for_attr("aria-valuenow", Some("1"))
        .await?;
    page.measure("end", page.send_keys(Key::End)).await?;
    spin_button
        .wait_for_attr("aria-valuenow", Some("10"))
        .await?;
    wait_for_announcement(page, "10").await?;
    Ok(())
}

/// A click on the increment stepper, the spin button focused (it announces the new value).
#[browser_test]
pub async fn stepper(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let spin_button = focus_with_tab(page).await?;
    page.send_keys(Key::Up).await?;
    wait_for_announcement(page, "6").await?;
    let increment = page.element(INCREMENT).await?;
    page.measure("click", increment.click()).await?;
    spin_button
        .wait_for_attr("aria-valuenow", Some("7"))
        .await?;
    wait_for_announcement(page, "7").await?;
    Ok(())
}

/// The page changing the value (unfocused: nothing is announced).
#[browser_test]
pub async fn value_from_the_page(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let spin_button = page.element(SPIN_BUTTON).await?;
    page.measure("set", page.control("set-value")).await?;
    spin_button
        .wait_for_attr("aria-valuenow", Some("8"))
        .await?;
    spin_button
        .wait_for_attr("aria-valuetext", Some("8"))
        .await?;
    Ok(())
}

/// Disabling the focused spin button, an arrow key it ignores, and enabling it again.
#[browser_test]
pub async fn disable_and_enable(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let spin_button = focus_with_tab(page).await?;
    page.measure("disable", page.control("disable")).await?;
    spin_button
        .wait_for_attr("aria-disabled", Some("true"))
        .await?;
    page.measure("ignored arrow up", page.send_keys(Key::Up))
        .await?;
    assert_that!(value(page).await?).is_equal_to(5.0);
    page.measure("enable", page.control("enable")).await?;
    spin_button.wait_for_attr("aria-disabled", None).await?;
    Ok(())
}
