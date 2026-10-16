// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the hook `use_color_channel_field` (`/perf/hooks/use_color_channel_field`,
//! a hue field on plain elements with stepper buttons, with its state): what each thing it
//! offers costs, measured (`Page::measure`) and checked against its budget
//! (`tests/budgets/hooks/use_color_channel_field.json`): mounting and unmounting it, typing and
//! committing a hue, each key it handles, the stepper buttons, the scroll wheel, keyboard focus,
//! and disabling it
//! (with a stepper click it ignores).
use assertr::prelude::*;
use browser_test::{
    TestGroup, browser_test,
    thirtyfour::{TypingData, prelude::*},
};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page, SyntheticEvent},
};

const PATH: &str = "/perf/hooks/use_color_channel_field";

/// Registers the performance tests of `use_color_channel_field`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(TypeAndCommit))
        .with(fixture_test(Keys))
        .with(fixture_test(StepperButtons))
        .with(fixture_test(ScrollWheel))
        .with(fixture_test(KeyboardFocusAndBlur))
        .with(fixture_test(DisableAndEnable))
}
const INPUT: &str = ".perf-input";

/// The committed hue (`-1`: no color).
async fn hue(page: &Page<'_>) -> Result<f64, Report> {
    page.control("hue").await
}

/// Focuses the input with the keyboard (Tab from the plain button before it).
async fn focus_with_tab(page: &Page<'_>) -> Result<WebElement, Report> {
    page.element("#perf-before").await?.click().await?;
    page.send_keys(Key::Tab).await?;
    let input = page.element(INPUT).await?;
    page.wait_for_focus(&input).await?;
    Ok(input)
}

/// The live announcer's announcements (the field announces its value changes).
const ANNOUNCEMENTS: &str = "[data-live-announcer] [aria-live=assertive] > *";

/// Creates the live announcer with a step up and back of the focused field, unmeasured: the
/// announcer's first announcement comes 100 ms late (a timer), later ones at once, so that the
/// measured changes don't depend on when that timer runs.
async fn prime_announcer(page: &Page<'_>) -> Result<(), Report> {
    page.send_keys(Key::Up).await?;
    page.wait_for_count(ANNOUNCEMENTS, 1).await?;
    page.send_keys(Key::Down).await?;
    page.wait_for_count(ANNOUNCEMENTS, 1).await
}

/// Mounting the field and unmounting it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(INPUT, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.element(INPUT)
        .await?
        .wait_for_prop("value", "180°")
        .await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(INPUT, 0).await?;
    Ok(())
}

/// Typing a digit into the emptied field, and Enter committing the typed hue.
#[browser_test]
pub async fn type_and_commit(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = focus_with_tab(page).await?;
    prime_announcer(page).await?;
    page.send_keys(page.primary_modifier().await? + "a").await?;
    page.send_keys(Key::Backspace).await?;
    input.wait_for_prop("value", "").await?;
    page.measure("type", page.send_keys("9")).await?;
    input.wait_for_prop("value", "9").await?;
    page.send_keys("0").await?;
    page.measure("enter", page.send_keys(Key::Enter)).await?;
    input.wait_for_prop("value", "90°").await?;
    assert_that!(hue(page).await?).is_equal_to(90.0);
    Ok(())
}

/// Each key the field handles: the arrows step the hue, PageUp/PageDown by a page, Home/End to
/// its ends. Every key changes the hue.
#[browser_test]
pub async fn keys(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    focus_with_tab(page).await?;
    prime_announcer(page).await?;
    let steps: [(&str, TypingData); 6] = [
        ("arrow up", Key::Up.into()),
        ("arrow down", Key::Down.into()),
        ("page up", Key::PageUp.into()),
        ("page down", Key::PageDown.into()),
        ("home", Key::Home.into()),
        ("end", Key::End.into()),
    ];
    for (name, keys) in steps {
        let before = hue(page).await?;
        page.measure(name, page.send_keys(keys)).await?;
        assert_that!(hue(page).await?).is_not_equal_to(before);
    }
    Ok(())
}

/// Clicks on the increment and the decrement button.
#[browser_test]
pub async fn stepper_buttons(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = focus_with_tab(page).await?;
    prime_announcer(page).await?;
    let increment = page.element(".perf-increment").await?;
    page.measure("increment", increment.click()).await?;
    input.wait_for_prop("value", "181°").await?;
    let decrement = page.element(".perf-decrement").await?;
    page.measure("decrement", decrement.click()).await?;
    input.wait_for_prop("value", "180°").await?;
    Ok(())
}

/// The scroll wheel over the focused field (focus within its group) stepping the hue.
#[browser_test]
pub async fn scroll_wheel(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = focus_with_tab(page).await?;
    prime_announcer(page).await?;
    page.measure(
        "wheel",
        input.dispatch(SyntheticEvent::wheel().delta_y(10.0)),
    )
    .await?;
    input.wait_for_prop("value", "181°").await?;
    Ok(())
}

/// Tab moving focus to the input and on.
#[browser_test]
pub async fn keyboard_focus_and_blur(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element("#perf-before").await?.click().await?;
    let input = page.element(INPUT).await?;
    page.measure("focus", page.send_keys(Key::Tab)).await?;
    page.wait_for_focus(&input).await?;
    let after = page.element("#perf-after").await?;
    page.measure("blur", page.send_keys(Key::Tab)).await?;
    page.wait_for_focus(&after).await?;
    Ok(())
}

/// Disabling the field (its state), a click on the increment button it ignores, and enabling
/// it again.
#[browser_test]
pub async fn disable_and_enable(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = page.element(INPUT).await?;
    page.measure("disable", page.control("disable")).await?;
    input.wait_for_attr("disabled", Some("true")).await?;
    let increment = page.element(".perf-increment").await?;
    page.measure("ignored click", increment.click()).await?;
    assert_that!(hue(page).await?).is_equal_to(180.0);
    input.wait_for_prop("value", "180°").await?;
    page.measure("enable", page.control("enable")).await?;
    input.wait_for_attr("disabled", None).await?;
    Ok(())
}
