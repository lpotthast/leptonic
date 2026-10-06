// Upstream: react-aria-components/test/Switch.test.js @ 99e6102368
use std::borrow::Cow;

use assertr::prelude::*;
use browser_test::{
    BrowserTest, async_trait,
    thirtyfour::{By, Key, WebDriver, WebElement},
};
use rootcause::Report;

use crate::pages::{BaseActions, Page};

/// Behavior of the switch hooks (through the `Switch` atom): a native `role="switch"` checkbox
/// inside a label, toggled by press and Space, focus ring, disabled and read-only states, and a
/// switch bound to a signal. Spec: react-aria-components `Switch.test.js`.
pub struct SwitchTests {}

#[async_trait]
impl BrowserTest<str> for SwitchTests {
    fn name(&self) -> Cow<'_, str> {
        "switch_tests".into()
    }

    async fn run(&self, driver: &WebDriver, base_url: &str) -> Result<(), Report> {
        let page = Page { driver, base_url };
        page.goto_path("/atoms/switch").await?;

        selected_state(&page).await?;
        keyboard(&page).await?;
        virtual_label_click(&page).await?;
        disabled_state(&page).await?;
        read_only_state(&page).await?;
        bound_state(&page).await?;
        bound_read_only(&page).await?;

        Ok(())
    }
}

async fn label(page: &Page<'_>, text: &str) -> Result<WebElement, Report> {
    Ok(page
        .driver
        .find(By::XPath(format!("//label[normalize-space(.)='{text}']")))
        .await?)
}

async fn input(label: &WebElement) -> Result<WebElement, Report> {
    Ok(label.find(By::Css("input")).await?)
}

async fn attr(element: &WebElement, name: &str) -> Result<Option<String>, Report> {
    Ok(element.attr(name).await?)
}

async fn selected_state(page: &Page<'_>) -> Result<(), Report> {
    let label = label(page, "Basic").await?;
    let input = input(&label).await?;
    assert_that!(attr(&input, "role").await?).is_equal_to(Some("switch".to_owned()));
    assert_that!(attr(&input, "type").await?).is_equal_to(Some("checkbox".to_owned()));
    label.click().await?;
    page.wait_for_attr(&label, "data-selected", Some("true"))
        .await?;
    page.wait_for_text("test-sw-value", "true").await?;
    assert_that!(input.prop("checked").await?).is_equal_to(Some("true".to_owned()));
    label.click().await?;
    page.wait_for_attr(&label, "data-selected", None).await?;
    page.wait_for_text("test-sw-value", "false").await
}

async fn keyboard(page: &Page<'_>) -> Result<(), Report> {
    let label = label(page, "Basic").await?;
    let input = input(&label).await?;
    page.click_element_with_id("test-sw-before").await?;
    page.press_tab().await?;
    page.wait_for_focus_on(&input, "the switch").await?;
    page.wait_for_attr(&label, "data-focus-visible", Some("true"))
        .await?;
    page.send_keys_to_active(Key::Space).await?;
    page.wait_for_text("test-sw-value", "true").await?;
    page.send_keys_to_active(Key::Space).await?;
    page.wait_for_text("test-sw-value", "false").await
}

async fn disabled_state(page: &Page<'_>) -> Result<(), Report> {
    let label = label(page, "Disabled").await?;
    assert_that!(attr(&label, "data-disabled").await?).is_equal_to(Some("true".to_owned()));
    assert_that!(attr(&input(&label).await?, "disabled").await?).is_some();
    label.click().await?;
    tokio::time::sleep(std::time::Duration::from_millis(200)).await;
    assert_that!(attr(&label, "data-selected").await?).is_none();
    Ok(())
}

async fn read_only_state(page: &Page<'_>) -> Result<(), Report> {
    let label = label(page, "Read only").await?;
    let input = input(&label).await?;
    assert_that!(attr(&label, "data-readonly").await?).is_equal_to(Some("true".to_owned()));
    assert_that!(attr(&input, "aria-readonly").await?).is_equal_to(Some("true".to_owned()));
    label.click().await?;
    tokio::time::sleep(std::time::Duration::from_millis(200)).await;
    assert_that!(attr(&label, "data-selected").await?).is_equal_to(Some("true".to_owned()));
    assert_that!(input.prop("checked").await?).is_equal_to(Some("true".to_owned()));
    Ok(())
}

async fn bound_state(page: &Page<'_>) -> Result<(), Report> {
    let label = label(page, "Bound").await?;
    page.click_element_with_id("test-sw-bound-flip").await?;
    page.wait_for_attr(&label, "data-selected", Some("true"))
        .await?;
    label.click().await?;
    page.wait_for_attr(&label, "data-selected", None).await?;
    page.wait_for_text("test-sw-bound-value", "false").await
}

/// A bound switch stays read-only, also for Space on the focused input.
async fn bound_read_only(page: &Page<'_>) -> Result<(), Report> {
    let label = label(page, "Bound read only").await?;
    let input = input(&label).await?;
    label.click().await?;
    input.focus().await?;
    page.send_keys_to_active(Key::Space).await?;
    tokio::time::sleep(std::time::Duration::from_millis(200)).await;
    assert_that!(page.read_text_of("test-sw-bound-read-only-value").await?)
        .is_equal_to("false".to_owned());
    assert_that!(input.prop("checked").await?).is_equal_to(Some("false".to_owned()));
    Ok(())
}

/// A virtual click on the label toggles, as with a native label.
async fn virtual_label_click(page: &Page<'_>) -> Result<(), Report> {
    let label = label(page, "Basic").await?;
    virtual_click(page, &label).await?;
    page.wait_for_text("test-sw-value", "true").await?;
    virtual_click(page, &label).await?;
    page.wait_for_text("test-sw-value", "false").await
}

/// `element.click()` from script: a virtual click, as assistive technology sends.
async fn virtual_click(page: &Page<'_>, element: &WebElement) -> Result<(), Report> {
    page.driver
        .execute("arguments[0].click()", vec![element.to_json()?])
        .await?;
    Ok(())
}
