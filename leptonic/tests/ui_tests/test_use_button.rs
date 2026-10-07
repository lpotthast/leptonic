// Upstream: react-aria/test/button/useButton.test.js @ 99e6102368
use std::{borrow::Cow, time::Duration};

use assertr::prelude::*;
use browser_test::{
    BrowserTest, async_trait,
    thirtyfour::{Key, WebDriver},
};
use rootcause::Report;

use crate::pages::{BaseActions, Page};

pub struct UseButtonTests {}

#[async_trait]
impl BrowserTest<str> for UseButtonTests {
    fn name(&self) -> Cow<'_, str> {
        "use_button_tests".into()
    }

    async fn run(&self, driver: &WebDriver, base_url: &str) -> Result<(), Report> {
        let page = Page { driver, base_url };
        page.goto_path("/hooks/button").await?;

        attributes_depend_on_element_type(&page).await?;
        native_and_custom_elements_press(&page).await?;
        tab_order(&page).await?;
        disabled_buttons(&page).await?;
        form_submission(&page).await?;
        hover_and_focus_visible(&page).await?;

        Ok(())
    }
}

async fn attributes_depend_on_element_type(page: &Page<'_>) -> Result<(), Report> {
    // Native buttons need no role and default to `type="button"` (not `submit`).
    assert_that!(page.attr_of("test-btn-native", "role").await?).is_none();
    assert_that!(page.attr_of("test-btn-native", "type").await?)
        .is_equal_to(Some("button".to_owned()));
    // Other elements are announced as buttons and are focusable.
    assert_that!(page.attr_of("test-btn-div", "role").await?)
        .is_equal_to(Some("button".to_owned()));
    assert_that!(page.attr_of("test-btn-div", "tabindex").await?).is_equal_to(Some("0".to_owned()));
    assert_that!(page.attr_of("test-btn-div", "type").await?).is_none();
    assert_that!(page.attr_of("test-btn-anchor", "role").await?)
        .is_equal_to(Some("button".to_owned()));
    assert_that!(page.attr_of("test-btn-anchor", "href").await?)
        .is_equal_to(Some("#anchor-target".to_owned()));
    // "handles input elements": `type="button"`, `role="button"`.
    assert_that!(page.attr_of("test-btn-input", "type").await?)
        .is_equal_to(Some("button".to_owned()));
    assert_that!(page.attr_of("test-btn-input", "role").await?)
        .is_equal_to(Some("button".to_owned()));
    // "handles target and rel": a new tab also gets `noopener` (a leptonic addition).
    assert_that!(page.attr_of("test-btn-blank", "target").await?)
        .is_equal_to(Some("_blank".to_owned()));
    assert_that!(page.attr_of("test-btn-blank", "rel").await?)
        .is_equal_to(Some("nofollow noopener".to_owned()));
    // RAC Button.test.js "removes href attribute from anchor element when isPending is true".
    assert_that!(page.attr_of("test-btn-pending-anchor", "href").await?).is_none();
    assert_that!(
        page.attr_of("test-btn-pending-anchor", "aria-disabled")
            .await?
    )
    .is_equal_to(Some("true".to_owned()));
    Ok(())
}

async fn native_and_custom_elements_press(page: &Page<'_>) -> Result<(), Report> {
    page.click_element_with_id("test-btn-native").await?;
    page.wait_for_text("test-btn-presses", "1").await?;
    page.click_element_with_id("test-btn-div").await?;
    page.wait_for_text("test-btn-presses", "2").await?;
    // Keyboard activation of the custom element (it has focus after the click).
    page.send_keys_to_active(Key::Enter).await?;
    page.wait_for_text("test-btn-presses", "3").await?;
    page.send_keys_to_active(" ").await?;
    page.wait_for_text("test-btn-presses", "4").await?;
    Ok(())
}

async fn tab_order(page: &Page<'_>) -> Result<(), Report> {
    page.click_element_with_id("test-btn-before").await?;
    let mut order = Vec::new();
    for _ in 0..4 {
        page.press_tab().await?;
        order.push(page.active_element_id().await?.unwrap_or_default());
    }
    // `exclude_from_tab_order` skips the "Excluded" button.
    assert_that!(order).is_equal_to(vec![
        "test-btn-native".to_owned(),
        "test-btn-div".to_owned(),
        "test-btn-anchor".to_owned(),
        "test-btn-after".to_owned(),
    ]);
    Ok(())
}

async fn disabled_buttons(page: &Page<'_>) -> Result<(), Report> {
    page.click_element_with_id("test-btn-toggle-disabled")
        .await?;
    page.wait_for_selector("#test-btn-native[disabled]").await?;

    // Native buttons and inputs use the `disabled` attribute, everything else `aria-disabled`.
    page.wait_for_selector("#test-btn-input[disabled]").await?;
    assert_that!(page.attr_of("test-btn-div", "aria-disabled").await?)
        .is_equal_to(Some("true".to_owned()));
    assert_that!(page.attr_of("test-btn-div", "disabled").await?).is_none();
    assert_that!(page.attr_of("test-btn-native", "aria-disabled").await?).is_none();
    // Disabled anchors lose their link and their place in the tab order.
    assert_that!(page.attr_of("test-btn-anchor", "href").await?).is_none();
    assert_that!(page.attr_of("test-btn-div", "tabindex").await?).is_none();

    let before = page.read_text_of("test-btn-presses").await?;
    page.click_element_with_id("test-btn-div").await?;
    tokio::time::sleep(Duration::from_millis(200)).await;
    assert_that!(page.read_text_of("test-btn-presses").await?).is_equal_to(before);

    page.click_element_with_id("test-btn-toggle-disabled")
        .await?;
    Ok(())
}

async fn form_submission(page: &Page<'_>) -> Result<(), Report> {
    // A button defaults to `type="button"` and does not submit its form.
    page.click_element_with_id("test-btn-in-form").await?;
    tokio::time::sleep(Duration::from_millis(200)).await;
    assert_that!(page.read_text_of("test-btn-submits").await?).is_equal_to("0".to_owned());

    page.click_element_with_id("test-btn-submit").await?;
    page.wait_for_text("test-btn-submits", "1").await?;
    Ok(())
}

async fn hover_and_focus_visible(page: &Page<'_>) -> Result<(), Report> {
    let state = page.element("test-btn-state").await?;
    page.driver
        .action_chain()
        .move_to_element_center(&state)
        .perform()
        .await?;
    page.wait_for_text("test-btn-is-hovered", "true").await?;

    // Pointer focus does not show a focus ring, keyboard focus does.
    state.click().await?;
    assert_that!(page.read_bool("test-btn-is-focus-visible").await?).is_false();
    page.press_shift_tab().await?;
    page.press_tab().await?;
    page.wait_for_text("test-btn-is-focus-visible", "true")
        .await?;
    Ok(())
}
