// Upstream: react-aria/test/landmark/useLandmark.test.tsx @ 99e6102368
use std::borrow::Cow;

use assertr::prelude::*;
use browser_test::{
    BrowserTest, async_trait,
    thirtyfour::{Key, WebDriver},
};
use rootcause::Report;

use crate::pages::{BaseActions, Page};

/// `use_landmark`: F6/Shift+F6 move between landmarks in document order and wrap; Alt+F6 goes to
/// the main landmark; a landmark regains the element focused in it last; `aria-hidden` landmarks
/// are skipped; added and removed landmarks are followed; a cancelable event fires before
/// wrapping; the focused landmark is `tabindex="-1"` until the focus leaves it; labels update;
/// nested landmarks go in document order; a
/// `LandmarkController` moves between them; landmarks sharing a role without distinct labels are
/// reported.
pub struct LandmarkTests {}

#[async_trait]
impl BrowserTest<str> for LandmarkTests {
    fn name(&self) -> Cow<'_, str> {
        "landmark_tests".into()
    }

    async fn run(&self, driver: &WebDriver, base_url: &str) -> Result<(), Report> {
        let page = Page { driver, base_url };
        navigation_order(&page).await?;
        restores_last_focused(&page).await?;
        alt_f6_to_main(&page).await?;
        added_and_removed(&page).await?;
        wrap_event(&page).await?;
        label_updates(&page).await?;
        nested_order(&page).await?;
        controller(&page).await?;
        duplicate_role_warnings(&page).await?;
        page.expect_no_page_errors().await
    }
}

/// "can F6 to a landmark region", "can F6 to the next landmark region", "landmark navigation
/// forward wraps", "can shift+F6 to the previous landmark region", "landmark navigation backward
/// wraps", "skips over aria-hidden landmarks", "landmark has tabIndex="-1" when focused", "loses
/// the tabIndex=-1 if something else is focused".
async fn navigation_order(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/hooks/landmark").await?;
    page.send_keys_to_active(Key::F6).await?;
    page.wait_for_active_id("test-lm-nav").await?;
    let nav = page.element("test-lm-nav").await?;
    assert_that!(nav.attr("tabindex").await?).is_equal_to(Some("-1".to_owned()));
    page.send_keys_to_active(Key::F6).await?;
    page.wait_for_active_id("test-lm-main").await?;
    // The region inside `aria-hidden` is skipped; forward wraps.
    page.send_keys_to_active(Key::F6).await?;
    page.wait_for_active_id("test-lm-nav").await?;
    // Backward wraps too.
    page.send_keys_to_active(Key::Shift + Key::F6).await?;
    page.wait_for_active_id("test-lm-main").await?;
    page.send_keys_to_active(Key::Shift + Key::F6).await?;
    page.wait_for_active_id("test-lm-nav").await?;
    // Focusing something else drops the landmark's tabindex.
    page.click_element_with_id("test-lm-name").await?;
    page.wait_for_attr(&nav, "tabindex", None).await
}

/// "F6 should focus the last focused element in a landmark region".
async fn restores_last_focused(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/hooks/landmark").await?;
    page.driver
        .execute("document.getElementById('test-lm-home').focus()", vec![])
        .await?;
    page.press_tab().await?;
    page.press_tab().await?;
    page.wait_for_active_id("test-lm-contact").await?;
    page.press_tab().await?;
    page.wait_for_active_id("test-lm-name").await?;
    page.send_keys_to_active(Key::F6).await?;
    page.wait_for_active_id("test-lm-contact").await
}

/// "can alt+F6 to main landmark".
async fn alt_f6_to_main(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/hooks/landmark").await?;
    page.driver
        .execute("document.getElementById('test-lm-home').focus()", vec![])
        .await?;
    page.send_keys_to_active(Key::Alt + Key::F6).await?;
    page.wait_for_active_id("test-lm-main").await
}

/// "Should navigate to a landmark that has been added to the DOM" (as a child of an existing
/// landmark), "Should not navigate to a landmark that has been removed from the DOM".
async fn added_and_removed(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/hooks/landmark").await?;
    page.click_element_with_id("test-lm-toggle").await?;
    page.wait_for_selector("#test-lm-extra").await?;
    page.send_keys_to_active(Key::F6).await?;
    page.wait_for_active_id("test-lm-nav").await?;
    page.send_keys_to_active(Key::F6).await?;
    page.wait_for_active_id("test-lm-main").await?;
    page.send_keys_to_active(Key::F6).await?;
    page.wait_for_active_id("test-lm-extra").await?;

    page.click_element_with_id("test-lm-toggle").await?;
    page.wait_for_no_selector("#test-lm-extra").await?;
    page.send_keys_to_active(Key::F6).await?;
    page.wait_for_active_id("test-lm-nav").await?;
    page.send_keys_to_active(Key::F6).await?;
    page.wait_for_active_id("test-lm-main").await?;
    page.send_keys_to_active(Key::F6).await?;
    page.wait_for_active_id("test-lm-nav").await
}

/// "landmark navigation fires custom event when wrapping forward": a listener preventing it keeps
/// the focus where it is.
async fn wrap_event(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/hooks/landmark").await?;
    page.driver
        .execute(
            "window.__landmarkEvents = []; window.addEventListener('react-aria-landmark-navigation', e => { e.preventDefault(); window.__landmarkEvents.push(e.detail.direction); });",
            vec![],
        )
        .await?;
    page.send_keys_to_active(Key::F6).await?;
    page.wait_for_active_id("test-lm-nav").await?;
    page.send_keys_to_active(Key::F6).await?;
    page.wait_for_active_id("test-lm-main").await?;
    page.send_keys_to_active(Key::F6).await?;
    tokio::time::sleep(std::time::Duration::from_millis(200)).await;
    page.wait_for_active_id("test-lm-main").await?;
    let events = page
        .driver
        .execute("return window.__landmarkEvents", vec![])
        .await?
        .convert::<Vec<String>>()?;
    assert_that!(events).is_equal_to(vec!["forward".to_owned()]);
    Ok(())
}

/// "updates the landmark if the label changes".
async fn label_updates(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/hooks/landmark").await?;
    let main = page.element("test-lm-main").await?;
    assert_that!(main.attr("aria-label").await?).is_equal_to(Some("Content".to_owned()));
    page.click_element_with_id("test-lm-rename").await?;
    page.wait_for_attr(&main, "aria-label", Some("Article"))
        .await
}

/// "goes in dom order with two nested landmarks", "can F6 to a nested landmark region that is
/// first".
async fn nested_order(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/hooks/landmark-nested").await?;
    for id in [
        "test-lmn-main",
        "test-lmn-region-1",
        "test-lmn-region-2",
        "test-lmn-main",
    ] {
        page.send_keys_to_active(Key::F6).await?;
        page.wait_for_active_id(id).await?;
    }
    for id in ["test-lmn-region-2", "test-lmn-region-1", "test-lmn-main"] {
        page.send_keys_to_active(Key::Shift + Key::F6).await?;
        page.wait_for_active_id(id).await?;
    }
    Ok(())
}

/// Calls a controller method of the fixture (a script click doesn't move the focus).
async fn call_controller(page: &Page<'_>, button: &str) -> Result<(), Report> {
    page.driver
        .execute(
            &format!("document.getElementById('{button}').click()"),
            vec![],
        )
        .await?;
    Ok(())
}

/// `LandmarkController`: "should navigate forward", "should navigate backward", "should focus
/// main", from the focused element.
async fn controller(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/hooks/landmark-nested").await?;
    page.click_element_with_id("test-lmn-first").await?;
    call_controller(page, "test-lmn-next").await?;
    page.wait_for_active_id("test-lmn-region-1").await?;
    call_controller(page, "test-lmn-forward").await?;
    page.wait_for_active_id("test-lmn-region-2").await?;
    call_controller(page, "test-lmn-previous").await?;
    page.wait_for_active_id("test-lmn-region-1").await?;
    call_controller(page, "test-lmn-main-button").await?;
    page.wait_for_active_id("test-lmn-main").await
}

/// "Should warn if 2+ landmarks with same role are used but not labelled.", "Should warn if 2+
/// landmarks with same role and same label", "Should allow 2+ landmarks with same role if they are
/// labelled." (the two regions of the page).
async fn duplicate_role_warnings(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/hooks/landmark-nested").await?;
    page.driver
        .execute(
            "window.__warnings = [];
            const warn = console.warn;
            console.warn = (...args) => { window.__warnings.push(args.join(' ')); warn(...args); };",
            vec![],
        )
        .await?;
    let warnings = async || -> Result<Vec<String>, Report> {
        Ok(page
            .driver
            .execute("return window.__warnings;", vec![])
            .await?
            .convert()?)
    };
    // The two distinctly labelled regions don't warn.
    tokio::time::sleep(std::time::Duration::from_millis(200)).await;
    assert_that!(warnings().await?).is_empty();

    page.click_element_with_id("test-lmn-add-unlabelled")
        .await?;
    page.wait_for_selector("#test-lmn-nav-2").await?;
    tokio::time::sleep(std::time::Duration::from_millis(200)).await;
    let unlabelled = warnings().await?;
    assert_that!(unlabelled.iter().any(|warning| {
        warning.contains("more than one landmark with the role Navigation")
            && warning.contains("label each")
    }))
    .with_detail_message(format!("warnings: {unlabelled:?}"))
    .is_true();

    page.goto_path("/hooks/landmark-nested").await?;
    page.driver
        .execute(
            "window.__warnings = [];
            const warn = console.warn;
            console.warn = (...args) => { window.__warnings.push(args.join(' ')); warn(...args); };",
            vec![],
        )
        .await?;
    page.click_element_with_id("test-lmn-add-same-label")
        .await?;
    page.wait_for_selector("#test-lmn-same-2").await?;
    tokio::time::sleep(std::time::Duration::from_millis(200)).await;
    let same = warnings().await?;
    assert_that!(
        same.iter()
            .any(|warning| warning.contains("label them uniquely"))
    )
    .with_detail_message(format!("warnings: {same:?}"))
    .is_true();
    Ok(())
}
