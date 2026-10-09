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
use browser_test::thirtyfour::prelude::*;
use rootcause::Report;

use crate::pages::{ElementActions, Page, PageActions};

/// "can F6 to a landmark region", "can F6 to the next landmark region", "landmark navigation
/// forward wraps", "can shift+F6 to the previous landmark region", "landmark navigation backward
/// wraps", "skips over aria-hidden landmarks", "landmark has tabIndex="-1" when focused", "loses
/// the tabIndex=-1 if something else is focused".
pub async fn navigation_order(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/hooks/landmark").await?;
    let nav = page.element("#test-lm-nav").await?;
    let main = page.element("#test-lm-main").await?;
    page.send_keys(Key::F6).await?;
    page.wait_for_focus(&nav).await?;
    assert_that!(nav.attr("tabindex").await?)
        .get_some()
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

/// "F6 should focus the last focused element in a landmark region".
pub async fn restores_last_focused(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/hooks/landmark").await?;
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

/// "can alt+F6 to main landmark".
pub async fn alt_f6_to_main(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/hooks/landmark").await?;
    page.element("#test-lm-home").await?.focus().await?;
    page.send_keys(Key::Alt + Key::F6).await?;
    page.wait_for_focus(&page.element("#test-lm-main").await?)
        .await?;
    Ok(())
}

/// "Should navigate to a landmark that has been added to the DOM" (as a child of an existing
/// landmark), "Should not navigate to a landmark that has been removed from the DOM".
pub async fn added_and_removed(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/hooks/landmark").await?;
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

/// "landmark navigation fires custom event when wrapping forward": a listener preventing it keeps
/// the focus where it is.
pub async fn wrap_event(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/hooks/landmark").await?;
    let nav = page.element("#test-lm-nav").await?;
    let main = page.element("#test-lm-main").await?;
    page.eval::<()>(
        "window.__landmarkEvents = [];
         window.addEventListener('react-aria-landmark-navigation', e => {
             e.preventDefault();
             window.__landmarkEvents.push(e.detail.direction);
         });",
        vec![],
    )
    .await?;
    page.send_keys(Key::F6).await?;
    page.wait_for_focus(&nav).await?;
    page.send_keys(Key::F6).await?;
    page.wait_for_focus(&main).await?;
    page.send_keys(Key::F6).await?;
    page.focus_stays(&main).await?;
    let events: Vec<String> = page.eval("return window.__landmarkEvents;", vec![]).await?;
    assert_that!(events).contains_exactly(["forward"]);
    Ok(())
}

/// "updates the landmark if the label changes".
pub async fn label_updates(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/hooks/landmark").await?;
    let main = page.element("#test-lm-main").await?;
    assert_that!(main.attr("aria-label").await?)
        .get_some()
        .is_equal_to("Content");
    page.element("#test-lm-rename").await?.click().await?;
    main.wait_for_attr("aria-label", Some("Article")).await?;
    Ok(())
}

/// "goes in dom order with two nested landmarks", "can F6 to a nested landmark region that is
/// first".
pub async fn nested_order(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/hooks/landmark-nested").await?;
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

/// `LandmarkController`: "should navigate forward", "should navigate backward", "should focus
/// main", from the focused element.
pub async fn controller(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/hooks/landmark-nested").await?;
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
    Ok(page.diagnostics().await?.console_warnings)
}

/// "Should warn if 2+ landmarks with same role are used but not labelled.", "Should warn if 2+
/// landmarks with same role and same label", "Should allow 2+ landmarks with same role if they are
/// labelled." (the two regions of the page).
pub async fn duplicate_role_warnings(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/hooks/landmark-nested").await?;
    // The two distinctly labelled regions don't warn.
    page.settle().await?;
    assert_that!(|| warnings(page))
        .consistently_ok()
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

    page.goto_path("/hooks/landmark-nested").await?;
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
    Ok(())
}
