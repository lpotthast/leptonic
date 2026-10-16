// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the hook `use_formatted_text_field`
//! (`leptonic/src/hooks/form/use_formatted_text_field.rs`, fixture
//! `/perf/hooks/use_formatted_text_field`): what each thing it offers costs, measured
//! (`Page::measure`) and checked against its budget
//! (`tests/budgets/hooks/use_formatted_text_field.json`), on the text of a number: mounting and
//! unmounting it, a typed digit it accepts, a letter it rejects before the browser applies it,
//! and a composition (an IME) ending in invalid text, which it reverts.
use browser_test::{TestGroup, browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/hooks/use_formatted_text_field";
const INPUT: &str = "#perf-input";

/// Registers the performance tests of `use_formatted_text_field`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(Typing))
        .with(fixture_test(InvalidComposition))
}

/// Focuses the input with the caret at the end of its text.
async fn focus_at_end(page: &Page<'_>) -> Result<WebElement, Report> {
    let input = page.element(INPUT).await?;
    input.type_keys(Key::End).await?;
    page.wait_for_focus(&input).await?;
    Ok(input)
}

/// Mounting the input and unmounting it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(INPUT, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.element(INPUT)
        .await?
        .wait_for_prop("value", "5")
        .await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(INPUT, 0).await?;
    Ok(())
}

/// A typed digit (accepted) and a letter (rejected: the text stays).
#[browser_test]
pub async fn typing(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = focus_at_end(page).await?;
    page.measure("accepted digit", page.send_keys("0")).await?;
    input.wait_for_prop("value", "50").await?;
    page.measure("rejected letter", page.send_keys("a")).await?;
    input.wait_for_prop("value", "50").await?;
    Ok(())
}

/// A composition (as an IME makes it) that ends with invalid text: the hook restores the text
/// from before it.
#[browser_test]
pub async fn invalid_composition(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = focus_at_end(page).await?;
    page.measure(
        "invalid composition",
        page.low_level().eval::<()>(
            "const input = arguments[0];
             input.dispatchEvent(new CompositionEvent('compositionstart', {bubbles: true}));
             input.value = '5x';
             input.dispatchEvent(new CompositionEvent('compositionend', {bubbles: true}));",
            vec![input.to_json()?],
        ),
    )
    .await?;
    input.wait_for_prop("value", "5").await?;
    Ok(())
}
