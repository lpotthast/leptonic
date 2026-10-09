// Upstream: react-aria/test/landmark/useLandmark.test.tsx @ 99e6102368
//! `use_landmark`: F6/Shift+F6 move between landmarks in document order and wrap; Alt+F6 goes to
//! the main landmark; a landmark regains the element focused in it last; `aria-hidden` landmarks
//! are skipped; added and removed landmarks are followed; a cancelable event fires before
//! wrapping; the focused landmark is `tabindex="-1"` until the focus leaves it; labels update;
//! nested landmarks go in document order; a
//! `LandmarkController` moves between them; landmarks sharing a role without distinct labels are
//! reported.
use assertr::{
    matchers::{eq, satisfying},
    prelude::*,
};
use browser_test::{browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::pages::{ElementActions, Page};

const PATH: &str = "/hooks/landmark";
const NESTED: &str = "/hooks/landmark-nested";

/// F6 and Shift+F6 move the focus between landmarks in both directions, wrapping and skipping
/// `aria-hidden` ones; the focused landmark has `tabindex="-1"` until something else is focused
/// ("can F6 to the next landmark region", "landmark navigation forward wraps", "landmark
/// navigation backward wraps", "skips over aria-hidden landmarks", "loses the tabIndex=-1 if
/// something else is focused", "can F6 to a landmark region", "can shift+F6 to the previous
/// landmark region", "landmark has tabIndex="-1" when focused").
#[browser_test]
pub async fn navigation_order(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let nav = page.element("#test-lm-nav").await?;
    let main = page.element("#test-lm-main").await?;
    page.send_keys(Key::F6).await?;
    page.wait_for_focus(&nav).await?;
    assert_that!(nav)
        .has_attribute("tabindex")
        .await
        .is_equal_to("-1");
    page.send_keys(Key::F6).await?;
    page.wait_for_focus(&main).await?;
    // The region inside `aria-hidden` is skipped; forward wraps.
    page.send_keys(Key::F6).await?;
    page.wait_for_focus(&nav).await?;
    // Backward wraps too.
    page.send_keys(Key::Shift + Key::F6).await?;
    page.wait_for_focus(&main).await?;
    page.send_keys(Key::Shift + Key::F6).await?;
    page.wait_for_focus(&nav).await?;
    // Focusing something else drops the landmark's tabindex.
    page.element("#test-lm-name").await?.click().await?;
    nav.wait_for_attr("tabindex", None).await?;
    Ok(())
}

/// F6 skips a landmark inside an `inert` element (the page behind a modal), as it skips
/// `aria-hidden` ones: forward from the main landmark it wraps to the navigation (an addition:
/// upstream checks `aria-hidden` only, and its modals hide with `aria-hidden`).
#[browser_test]
pub async fn skips_inert_landmarks(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element("#test-lm-inert-toggle").await?.click().await?;
    page.element("#test-lm-inert").await?;
    let nav = page.element("#test-lm-nav").await?;
    let name = page.element("#test-lm-name").await?;
    name.focus().await?;
    page.wait_for_focus(&name).await?;
    // Forward from the main landmark, past the inert region, wrapping to the navigation.
    page.send_keys(Key::F6).await?;
    page.wait_for_focus(&nav).await?;
    // Backward from the navigation, wrapping past the inert region to the main landmark (its
    // field, focused there last).
    page.send_keys(Key::Shift + Key::F6).await?;
    page.wait_for_focus(&name).await?;
    Ok(())
}

/// Records the directions of the landmark navigation's wrap events and cancels them (no wrap).
async fn record_wrap_events(page: &Page<'_>) -> Result<(), Report> {
    page.low_level()
        .eval::<()>(
            "window.__landmarkEvents = [];
         window.addEventListener('react-aria-landmark-navigation', e => {
             e.preventDefault();
             window.__landmarkEvents.push(e.detail.direction);
         });",
            vec![],
        )
        .await
}

/// The directions [`record_wrap_events`] recorded.
async fn wrap_events(page: &Page<'_>) -> Result<Vec<String>, Report> {
    page.low_level()
        .eval("return window.__landmarkEvents;", vec![])
        .await
}

/// Shift+F6 wrapping backward fires a cancelable event first: cancelled, the focus stays
/// ("landmark navigation fires custom event when wrapping backward").
#[browser_test]
pub async fn wrap_event_backward(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let nav = page.element("#test-lm-nav").await?;
    record_wrap_events(page).await?;
    page.send_keys(Key::F6).await?;
    page.wait_for_focus(&nav).await?;
    page.send_keys(Key::Shift + Key::F6).await?;
    page.focus_stays(&nav, std::time::Duration::from_millis(100))
        .await?;
    assert_that!(wrap_events(page).await?).contains_exactly(["backward"]);
    Ok(())
}

/// Shift+F6 from outside every landmark moves to the last landmark ("can shift+F6 to a landmark
/// region").
#[browser_test]
pub async fn shift_f6_from_outside(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element("#test-lm-toggle").await?.focus().await?;
    page.send_keys(Key::Shift + Key::F6).await?;
    page.wait_for_focus(&page.element("#test-lm-main").await?)
        .await?;
    Ok(())
}

/// F6 into a landmark focuses the element focused in it last instead of the landmark ("F6 should
/// focus the last focused element in a landmark region").
#[browser_test]
pub async fn restores_last_focused(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element("#test-lm-home").await?.focus().await?;
    page.send_keys(Key::Tab).await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&page.element("#test-lm-contact").await?)
        .await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&page.element("#test-lm-name").await?)
        .await?;
    page.send_keys(Key::F6).await?;
    page.wait_for_focus(&page.element("#test-lm-contact").await?)
        .await?;
    Ok(())
}

/// Alt+F6 moves the focus to the main landmark ("can alt+F6 to main landmark").
#[browser_test]
pub async fn alt_f6_to_main(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element("#test-lm-home").await?.focus().await?;
    page.send_keys(Key::Alt + Key::F6).await?;
    page.wait_for_focus(&page.element("#test-lm-main").await?)
        .await?;
    Ok(())
}

/// F6 reaches a landmark added inside the main landmark and skips it again once it is removed
/// ("Should navigate to a landmark that has been added as a child to an existing landmark.",
/// "Should not navigate to a landmark that has been removed from the DOM").
#[browser_test]
pub async fn added_and_removed(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element("#test-lm-toggle").await?.click().await?;
    page.element("#test-lm-extra").await?;
    page.send_keys(Key::F6).await?;
    page.wait_for_focus(&page.element("#test-lm-nav").await?)
        .await?;
    page.send_keys(Key::F6).await?;
    page.wait_for_focus(&page.element("#test-lm-main").await?)
        .await?;
    page.send_keys(Key::F6).await?;
    page.wait_for_focus(&page.element("#test-lm-extra").await?)
        .await?;

    page.element("#test-lm-toggle").await?.click().await?;
    page.wait_for_count("#test-lm-extra", 0).await?;
    page.send_keys(Key::F6).await?;
    page.wait_for_focus(&page.element("#test-lm-nav").await?)
        .await?;
    page.send_keys(Key::F6).await?;
    page.wait_for_focus(&page.element("#test-lm-main").await?)
        .await?;
    page.send_keys(Key::F6).await?;
    page.wait_for_focus(&page.element("#test-lm-nav").await?)
        .await?;
    Ok(())
}

/// Wrapping forward fires one cancelable navigation event, and a listener preventing it keeps the
/// focus on the last landmark ("landmark navigation fires custom event when wrapping forward").
#[browser_test]
pub async fn wrap_event(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let nav = page.element("#test-lm-nav").await?;
    let main = page.element("#test-lm-main").await?;
    record_wrap_events(page).await?;
    page.send_keys(Key::F6).await?;
    page.wait_for_focus(&nav).await?;
    page.send_keys(Key::F6).await?;
    page.wait_for_focus(&main).await?;
    page.send_keys(Key::F6).await?;
    page.focus_stays(&main, std::time::Duration::from_millis(100))
        .await?;
    assert_that!(wrap_events(page).await?).contains_exactly(["forward"]);
    Ok(())
}

/// Changing a landmark's label updates its `aria-label` ("updates the landmark if the label
/// changes").
#[browser_test]
pub async fn label_updates(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let main = page.element("#test-lm-main").await?;
    assert_that!(main)
        .has_attribute("aria-label")
        .await
        .is_equal_to("Content");
    page.element("#test-lm-rename").await?.click().await?;
    main.wait_for_attr("aria-label", Some("Article")).await?;
    Ok(())
}

/// F6 and Shift+F6 visit nested landmarks in document order, the outer one before the inner ones
/// ("goes in dom order with two nested landmarks", "can F6 to a nested landmark region that is
/// first").
#[browser_test]
pub async fn nested_order(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(NESTED).await?;
    for landmark in [
        "#test-lmn-main",
        "#test-lmn-region-1",
        "#test-lmn-region-2",
        "#test-lmn-main",
    ] {
        page.send_keys(Key::F6).await?;
        page.wait_for_focus(&page.element(landmark).await?).await?;
    }
    for landmark in ["#test-lmn-region-2", "#test-lmn-region-1", "#test-lmn-main"] {
        page.send_keys(Key::Shift + Key::F6).await?;
        page.wait_for_focus(&page.element(landmark).await?).await?;
    }
    Ok(())
}

/// Calls a controller method of the fixture through the button `selector` (a virtual click
/// doesn't move the focus).
async fn call_controller(page: &Page<'_>, selector: &str) -> Result<(), Report> {
    page.element(selector).await?.virtual_click().await
}

/// A `LandmarkController` moves the focus to the next and previous landmark and to the main
/// landmark ("should navigate forward", "should navigate backward", "should focus main").
#[browser_test]
pub async fn controller(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(NESTED).await?;
    page.element("#test-lmn-first").await?.click().await?;
    call_controller(page, "#test-lmn-next").await?;
    page.wait_for_focus(&page.element("#test-lmn-region-1").await?)
        .await?;
    call_controller(page, "#test-lmn-forward").await?;
    page.wait_for_focus(&page.element("#test-lmn-region-2").await?)
        .await?;
    call_controller(page, "#test-lmn-previous").await?;
    page.wait_for_focus(&page.element("#test-lmn-region-1").await?)
        .await?;
    call_controller(page, "#test-lmn-main-button").await?;
    page.wait_for_focus(&page.element("#test-lmn-main").await?)
        .await?;
    Ok(())
}

/// The page's `console.warn` messages.
async fn warnings(page: &Page<'_>) -> Result<Vec<String>, Report> {
    Ok(crate::pages::health::diagnostics(page.low_level().driver())
        .await?
        .console_warnings)
}

/// Landmarks sharing a role log a warning when unlabelled or equally labelled, but not when
/// labelled distinctly ("Should warn if 2+ landmarks with same role are used but not labelled.",
/// "Should warn if 2+ landmarks with same role and same label", "Should allow 2+ landmarks with
/// same role if they are labelled.").
#[browser_test]
pub async fn duplicate_role_warnings(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(NESTED).await?;
    // The two distinctly labelled regions don't warn.
    page.settle().await?;
    assert_that!(|| warnings(page))
        .consistently_ok()
        .for_at_least(std::time::Duration::from_millis(100))
        .matches(eq(Vec::<String>::new()))
        .await;

    page.element("#test-lmn-add-unlabelled")
        .await?
        .click()
        .await?;
    page.element("#test-lmn-nav-2").await?;
    // A warning about unlabelled navigation landmarks.
    assert_that!(|| warnings(page))
        .eventually_ok()
        .satisfies(|warnings| {
            warnings.contains_matching(satisfying(|warning: AssertThat<String, Capture>| {
                warning
                    .contains("more than one landmark with the role Navigation")
                    .contains("label each");
            }));
        })
        .await;
    // The warning was expected: the page check must not fail on it.
    crate::fixtures::take_warnings(page, "more than one landmark with the role Navigation", 2)
        .await?;

    page.goto_path(NESTED).await?;
    page.element("#test-lmn-add-same-label")
        .await?
        .click()
        .await?;
    page.element("#test-lmn-same-2").await?;
    // A warning about equally labelled landmarks.
    assert_that!(|| warnings(page))
        .eventually_ok()
        .satisfies(|warnings| {
            warnings.contains_matching(satisfying(|warning: AssertThat<String, Capture>| {
                warning.contains("label them uniquely");
            }));
        })
        .await;
    crate::fixtures::take_warnings(page, "label them uniquely", 2).await?;
    Ok(())
}
