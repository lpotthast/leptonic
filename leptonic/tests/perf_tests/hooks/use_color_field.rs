// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the hook `use_color_field` (`/perf/hooks/use_color_field`, on a plain
//! `<input>`, with its state): what each thing it offers costs, measured (`Page::measure`) and
//! checked against its budget (`tests/budgets/hooks/use_color_field.json`): mounting and
//! unmounting it, typing and committing a color (with Enter and on blur), each key it handles,
//! the scroll wheel, keyboard focus, and changes of its inputs (disabled, read-only, with a key
//! it ignores).
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

const PATH: &str = "/perf/hooks/use_color_field";

/// Registers the performance tests of `use_color_field`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(TypeAndCommit))
        .with(fixture_test(CommitOnBlur))
        .with(fixture_test(Keys))
        .with(fixture_test(ScrollWheel))
        .with(fixture_test(KeyboardFocusAndBlur))
        .with(fixture_test(DisableAndEnable))
        .with(fixture_test(ReadOnly))
}
const INPUT: &str = ".perf-input";
/// The color at first (`#808080`).
const GRAY: f64 = 8_421_504.0;

/// The committed color (`0xRRGGBB`, `-1`: none).
async fn value(page: &Page<'_>) -> Result<f64, Report> {
    page.control("value").await
}

/// Focuses the input with the keyboard (Tab from the plain button before it).
async fn focus_with_tab(page: &Page<'_>) -> Result<WebElement, Report> {
    page.element("#perf-before").await?.click().await?;
    page.send_keys(Key::Tab).await?;
    let input = page.element(INPUT).await?;
    page.wait_for_focus(&input).await?;
    Ok(input)
}

/// Empties the focused input.
async fn empty(page: &Page<'_>, input: &WebElement) -> Result<(), Report> {
    page.send_keys(page.primary_modifier().await? + "a").await?;
    page.send_keys(Key::Backspace).await?;
    input.wait_for_prop("value", "").await
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
        .wait_for_prop("value", "#808080")
        .await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(INPUT, 0).await?;
    Ok(())
}

/// Typing a character into the emptied field, and Enter committing the typed color.
#[browser_test]
pub async fn type_and_commit(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = focus_with_tab(page).await?;
    prime_announcer(page).await?;
    empty(page, &input).await?;
    page.measure("type", page.send_keys("1")).await?;
    input.wait_for_prop("value", "1").await?;
    page.send_keys("23").await?;
    page.measure("enter", page.send_keys(Key::Enter)).await?;
    input.wait_for_prop("value", "#112233").await?;
    assert_that!(value(page).await?).is_equal_to(f64::from(0x11_22_33));
    Ok(())
}

/// Tab leaving the field with a typed color: it is committed.
#[browser_test]
pub async fn commit_on_blur(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = focus_with_tab(page).await?;
    empty(page, &input).await?;
    page.send_keys("123").await?;
    page.measure("blur", page.send_keys(Key::Tab)).await?;
    input.wait_for_prop("value", "#112233").await?;
    assert_that!(value(page).await?).is_equal_to(f64::from(0x11_22_33));
    Ok(())
}

/// Each key the field handles: the arrows step the color, PageUp/PageDown by a page,
/// Home/End to black and white. Every key changes the color.
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
        ("end", Key::End.into()),
        ("home", Key::Home.into()),
    ];
    for (name, keys) in steps {
        let before = value(page).await?;
        page.measure(name, page.send_keys(keys)).await?;
        assert_that!(value(page).await?).is_not_equal_to(before);
    }
    Ok(())
}

/// The scroll wheel over the focused field stepping the color.
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
    assert_that!(value(page).await?).is_not_equal_to(GRAY);
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

/// Disabling the field and enabling it again.
#[browser_test]
pub async fn disable_and_enable(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = page.element(INPUT).await?;
    page.measure("disable", page.control("disable")).await?;
    input.wait_for_attr("disabled", Some("true")).await?;
    page.measure("enable", page.control("enable")).await?;
    input.wait_for_attr("disabled", None).await?;
    Ok(())
}

/// Making the field read-only (an arrow key it ignores) and editable again.
#[browser_test]
pub async fn read_only(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = focus_with_tab(page).await?;
    page.measure("read-only", page.control("make-read-only"))
        .await?;
    input.wait_for_attr("readonly", Some("true")).await?;
    page.measure("ignored arrow up", page.send_keys(Key::Up))
        .await?;
    assert_that!(value(page).await?).is_equal_to(GRAY);
    page.measure("editable", page.control("make-editable"))
        .await?;
    input.wait_for_attr("readonly", None).await?;
    Ok(())
}
