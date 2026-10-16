// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the atom `ColorField` (`/perf/atoms/color_field`, an RGB field with its
//! `Label` and `Input`): what each thing it offers costs, measured (`Page::measure`) and checked
//! against its budget (`tests/budgets/atoms/color_field.json`): mounting and unmounting it,
//! typing and committing a color, each key it handles, the scroll wheel, keyboard focus, and
//! changes of its reactive props (disabled, read-only, required, invalid, the bound value).
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

const PATH: &str = "/perf/atoms/color_field";

/// Registers the performance tests of `ColorField`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(TypeAndCommit))
        .with(fixture_test(Keys))
        .with(fixture_test(ScrollWheel))
        .with(fixture_test(KeyboardFocusAndBlur))
        .with(fixture_test(DisableAndEnable))
        .with(fixture_test(ReadOnly))
        .with(fixture_test(Required))
        .with(fixture_test(Invalid))
        .with(fixture_test(ChangeTheValue))
}
const FIELD: &str = ".leptonic-ColorField";
const INPUT: &str = ".leptonic-ColorField input";
/// The bound value at first (`#808080`).
const GRAY: f64 = 8_421_504.0;

/// The bound value (`0xRRGGBB`, `-1`: none).
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
    page.wait_for_count(FIELD, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.element(INPUT)
        .await?
        .wait_for_prop("value", "#808080")
        .await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(FIELD, 0).await?;
    Ok(())
}

/// Typing a character into the emptied field, and Enter committing the typed color.
#[browser_test]
pub async fn type_and_commit(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = focus_with_tab(page).await?;
    prime_announcer(page).await?;
    page.send_keys(page.primary_modifier().await? + "a").await?;
    page.send_keys(Key::Backspace).await?;
    input.wait_for_prop("value", "").await?;
    page.measure("type", page.send_keys("1")).await?;
    input.wait_for_prop("value", "1").await?;
    page.send_keys("23").await?;
    page.measure("enter", page.send_keys(Key::Enter)).await?;
    input.wait_for_prop("value", "#112233").await?;
    assert_that!(value(page).await?).is_equal_to(f64::from(0x11_22_33));
    Ok(())
}

/// Each key the field handles: the arrows step the color, PageUp/PageDown by a page,
/// Home/End to black and white. Every key changes the value.
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

/// Tab moving focus to the field's input and on.
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
    let field = page.element(FIELD).await?;
    page.measure("disable", page.control("disable")).await?;
    field.wait_for_attr("data-disabled", Some("true")).await?;
    page.measure("enable", page.control("enable")).await?;
    field.wait_for_attr("data-disabled", None).await?;
    Ok(())
}

/// Making the field read-only and editable again.
#[browser_test]
pub async fn read_only(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let field = page.element(FIELD).await?;
    page.measure("read-only", page.control("make-read-only"))
        .await?;
    field.wait_for_attr("data-readonly", Some("true")).await?;
    page.measure("editable", page.control("make-editable"))
        .await?;
    field.wait_for_attr("data-readonly", None).await?;
    Ok(())
}

/// Making the field required and optional again.
#[browser_test]
pub async fn required(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let field = page.element(FIELD).await?;
    page.measure("require", page.control("require")).await?;
    field.wait_for_attr("data-required", Some("true")).await?;
    page.measure("unrequire", page.control("unrequire")).await?;
    field.wait_for_attr("data-required", None).await?;
    Ok(())
}

/// Marking the field invalid and valid again.
#[browser_test]
pub async fn invalid(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let field = page.element(FIELD).await?;
    page.measure("invalidate", page.control("invalidate"))
        .await?;
    field.wait_for_attr("data-invalid", Some("true")).await?;
    page.measure("validate", page.control("validate")).await?;
    field.wait_for_attr("data-invalid", None).await?;
    Ok(())
}

/// Changing the bound value (the input's text) and back.
#[browser_test]
pub async fn change_the_value(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = page.element(INPUT).await?;
    page.measure("set red", page.control("set-red")).await?;
    input.wait_for_prop("value", "#FF0000").await?;
    page.measure("set gray", page.control("set-gray")).await?;
    input.wait_for_prop("value", "#808080").await?;
    Ok(())
}
