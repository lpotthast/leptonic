//! Contracts for the browser helpers, independent of component behavior.
use std::time::Duration;

use assertr::prelude::*;
use browser_test::browser_test;
use leptonic::AriaRole;
use rootcause::Report;

use crate::pages::{
    ElementActions, Page, PointerKind, StopwatchEnd, SyntheticEvent, css, health, role,
};

/// Install a small DOM on the fixture origin. The script is fixture construction, not an action API.
async fn document(page: &Page<'_>, html: &str) -> Result<(), Report> {
    page.goto_path("/").await?;
    page.low_level()
        .eval::<()>("document.body.innerHTML = arguments[0];", vec![html.into()])
        .await
}

/// Singular lookup rejects ambiguity; first lookup, whitespace text and descendant filters are explicit.
#[browser_test]
pub async fn lookup_contract(page: &Page<'_>) -> Result<(), Report> {
    document(page, "<button id='a'>One</button><button id='b'>Two</button><div id='group'><span> A   B </span></div>").await?;
    let ambiguous = page.element("button").await;
    assert_that!(ambiguous).is_err();
    assert_that!(page.first_element("button").await?)
        .attribute("id")
        .await
        .is_equal_to(Some("a".to_owned()));
    assert_that!(page.element(role(AriaRole::Button).text("Two")).await?)
        .attribute("id")
        .await
        .is_equal_to(Some("b".to_owned()));
    assert_that!(
        page.element(css("div").has(css("span")).text("A B"))
            .await?
    )
    .attribute("id")
    .await
    .is_equal_to(Some("group".to_owned()));
    let group = page.element("#group").await?;
    assert_that!(group.count("span").await?).is_equal_to(1);
    assert_that!(group.first_element("span").await?)
        .inner_text()
        .await
        .is_equal_to("A B");
    group
        .count_stays("span", 1, Duration::from_millis(50))
        .await?;
    Ok(())
}

/// Browser accessibility computations handle implicit roles, overrides, references and descriptions.
#[browser_test]
pub async fn accessibility_contract(page: &Page<'_>) -> Result<(), Report> {
    document(page, "<button id='native'>Native</button><button id='tab' role='tab'>Tab</button><label for='input'>Name</label><input id='input' aria-describedby='description'><span id='description'><span>Nested description</span></span>").await?;
    assert_that!(page.count(role(AriaRole::Button)).await?).is_equal_to(1);
    assert_that!(page.count(role(AriaRole::Tab)).await?).is_equal_to(1);
    let input = page.element("#input").await?;
    assert_that!(input)
        .accessible_name()
        .await
        .is_equal_to("Name");
    assert_that!(input)
        .accessible_description()
        .await
        .is_equal_to("Nested description");
    Ok(())
}

/// A stale node fails through Result, while a locator wait finds its replacement.
#[browser_test]
pub async fn replacement_contract(page: &Page<'_>) -> Result<(), Report> {
    document(page, "<div id='target' data-state='old'></div>").await?;
    let old = page.element("#target").await?;
    page.low_level()
        .eval::<()>(
            "arguments[0].outerHTML = '<div id=target data-state=new></div>';",
            vec![old.to_json()?],
        )
        .await?;
    assert_that!(old.wait_for_attr("data-state", Some("new")).await).is_err();
    assert_that!(old.element("span").await).is_err();
    page.wait_for_attr("#target", "data-state", Some("new"))
        .await?;
    Ok(())
}

/// Stability checks observe immediately and over time; a zero duration is rejected.
#[browser_test]
pub async fn stability_contract(page: &Page<'_>) -> Result<(), Report> {
    document(page, "<div id='target' data-state='old'></div>").await?;
    let target = page.element("#target").await?;
    assert_that!(
        target
            .attr_stays("data-state", Some("new"), Duration::from_millis(100))
            .await
    )
    .is_err();
    assert_that!(
        target
            .attr_stays("data-state", Some("old"), Duration::ZERO)
            .await
    )
    .is_err();
    page.low_level()
        .eval::<()>(
            "setTimeout(() => arguments[0].setAttribute('data-state', 'new'), 60);",
            vec![target.to_json()?],
        )
        .await?;
    assert_that!(
        target
            .attr_stays("data-state", Some("old"), Duration::from_millis(200))
            .await
    )
    .is_err();
    Ok(())
}

/// Batched mutations retain intermediate values, and finishing or cancelling releases observers.
#[browser_test]
pub async fn recording_contract(page: &Page<'_>) -> Result<(), Report> {
    document(page, "<div id='target'></div>").await?;
    let target = page.element("#target").await?;
    let recording = target.record_attr("data-state").await?;
    page.low_level().eval::<()>("const e = arguments[0]; e.setAttribute('data-state', 'one'); e.setAttribute('data-state', 'two'); e.removeAttribute('data-state');", vec![target.to_json()?]).await?;
    assert_that!(recording.finish().await?).contains_exactly([
        Some("one".to_owned()),
        Some("two".to_owned()),
        None,
    ]);
    target.record_attr("data-state").await?.cancel().await?;
    assert_that!(
        page.low_level()
            .eval::<usize>("return window.__attrRecordings.size;", vec![])
            .await?
    )
    .is_equal_to(0);
    let stopwatch = page
        .start_stopwatch(&target, PointerKind::Enter, StopwatchEnd::Appears("#never"))
        .await?;
    stopwatch.cancel().await?;
    assert_that!(
        page.low_level()
            .eval::<usize>("return window.__stopwatches.size;", vec![])
            .await?
    )
    .is_equal_to(0);
    let completed = page
        .start_stopwatch(&target, PointerKind::Enter, StopwatchEnd::Appears("#end"))
        .await?;
    target
        .dispatch(SyntheticEvent::pointer(PointerKind::Enter))
        .await?;
    page.low_level().eval::<()>("const end = document.createElement('span'); end.id = 'end'; document.body.append(end);", vec![]).await?;
    assert_that!(completed.finish().await?).is_less_than(Duration::from_secs(5));
    // The harness also owns cleanup if the case exits early with live resources.
    let recording = target.record_attr("data-state").await?;
    let stopwatch = page
        .start_stopwatch(&target, PointerKind::Enter, StopwatchEnd::Appears("#never"))
        .await?;
    page.cleanup().await?;
    drop((recording, stopwatch));
    assert_that!(
        page.low_level()
            .eval::<usize>(
                "return window.__attrRecordings.size + window.__stopwatches.size;",
                vec![]
            )
            .await?
    )
    .is_equal_to(0);
    Ok(())
}

/// Missing diagnostics fail closed, and consuming an expected warning preserves unrelated errors.
#[browser_test]
pub async fn diagnostics_contract(page: &Page<'_>) -> Result<(), Report> {
    document(page, "<div></div>").await?;
    page.low_level()
        .eval::<()>("delete window.__panics;", vec![])
        .await?;
    assert_that!(health::diagnostics(page.driver()).await).is_err();
    page.low_level().eval::<()>("window.__panics = []; window.__consoleWarnings.push('expected', 'unrelated'); window.__consoleErrors.push('error');", vec![]).await?;
    crate::fixtures::take_warnings(page, "expected", 1).await?;
    let diagnostics = health::diagnostics(page.driver()).await?;
    assert_that!(diagnostics.console_warnings).contains_exactly(["unrelated".to_owned()]);
    assert_that!(diagnostics.console_errors).contains_exactly(["error".to_owned()]);
    // Restore the deliberately corrupted instrumentation after checking its evidence.
    page.low_level()
        .eval::<()>(
            "window.__consoleWarnings = []; window.__consoleErrors = [];",
            vec![],
        )
        .await?;
    // Consuming an initial fixture warning must retire its allowance, so an identical
    // warning appearing later still fails the fixture policy.
    page.goto_path("/atoms/label-slots").await?;
    let warning = "If you do not provide a visible label, you must specify an aria-label or aria-labelledby attribute for accessibility";
    crate::fixtures::take_warnings(page, warning, 4).await?;
    page.low_level()
        .eval::<()>("console.warn(arguments[0]);", vec![warning.into()])
        .await?;
    assert_that!(crate::fixtures::check_health(page).await).is_err();
    crate::fixtures::take_warnings(page, warning, 1).await?;
    Ok(())
}

/// Typed synthetic pointer events preserve cancellation and dispatch to the selected node.
#[browser_test]
pub async fn event_contract(page: &Page<'_>) -> Result<(), Report> {
    document(page, "<button id='target'>Target</button>").await?;
    let target = page.element("#target").await?;
    page.low_level()
        .eval::<()>(
            "arguments[0].addEventListener('pointerdown', event => event.preventDefault());",
            vec![target.to_json()?],
        )
        .await?;
    let result = target
        .dispatch(SyntheticEvent::pointer(PointerKind::Down))
        .await?;
    assert_that!(result.default_prevented).is_true();
    Ok(())
}
