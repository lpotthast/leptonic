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
/// wrapping; the focused landmark is `tabindex="-1"` until the focus leaves it; labels update.
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
