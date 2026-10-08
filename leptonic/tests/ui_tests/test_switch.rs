// Upstream: react-aria-components/test/Switch.test.js @ 99e6102368
use std::borrow::Cow;

use assertr::prelude::*;
use browser_test::{BrowserTest, async_trait, thirtyfour::prelude::*};
use rootcause::Report;

use crate::pages::{ElementActions, Page, PageActions, css};

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

        cases!(
            selected_state(&page),
            keyboard(&page),
            virtual_label_click(&page),
            disabled_state(&page),
            read_only_state(&page),
            bound_state(&page),
            bound_read_only(&page),
        );

        Ok(())
    }
}

/// The `<label>` of the switch with the visible text `text`.
async fn label(page: &Page<'_>, text: &str) -> Result<WebElement, Report> {
    page.element(css("label").text(text)).await
}

/// The fixture's mirror of the basic switch's state.
async fn value(page: &Page<'_>) -> Result<WebElement, Report> {
    page.element("#test-sw-value").await
}

/// The native checkbox inside `label`.
async fn input(label: &WebElement) -> Result<WebElement, Report> {
    label.element("input").await
}

async fn selected_state(page: &Page<'_>) -> Result<(), Report> {
    let label = label(page, "Basic").await?;
    let input = input(&label).await?;
    assert_that!(input.attr("role").await?)
        .get_some()
        .is_equal_to("switch");
    assert_that!(input.attr("type").await?)
        .get_some()
        .is_equal_to("checkbox");
    label.click().await?;
    label.wait_for_attr("data-selected", Some("true")).await?;
    value(page).await?.wait_for_inner_text("true").await?;
    assert_that!(input.is_selected().await?).is_true();
    label.click().await?;
    label.wait_for_attr("data-selected", None).await?;
    value(page).await?.wait_for_inner_text("false").await?;
    Ok(())
}

async fn keyboard(page: &Page<'_>) -> Result<(), Report> {
    let label = label(page, "Basic").await?;
    let input = input(&label).await?;
    page.element("#test-sw-before").await?.click().await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&input).await?;
    label
        .wait_for_attr("data-focus-visible", Some("true"))
        .await?;
    page.send_keys(Key::Space).await?;
    value(page).await?.wait_for_inner_text("true").await?;
    page.send_keys(Key::Space).await?;
    value(page).await?.wait_for_inner_text("false").await?;
    Ok(())
}

/// A virtual click on the label toggles, as with a native label.
async fn virtual_label_click(page: &Page<'_>) -> Result<(), Report> {
    let label = label(page, "Basic").await?;
    label.virtual_click().await?;
    value(page).await?.wait_for_inner_text("true").await?;
    label.virtual_click().await?;
    value(page).await?.wait_for_inner_text("false").await?;
    Ok(())
}

async fn disabled_state(page: &Page<'_>) -> Result<(), Report> {
    let label = label(page, "Disabled").await?;
    assert_that!(label.attr("data-disabled").await?)
        .get_some()
        .is_equal_to("true");
    assert_that!(input(&label).await?.is_enabled().await?).is_false();
    label.click().await?;
    label.attr_stays("data-selected", None).await?;
    Ok(())
}

async fn read_only_state(page: &Page<'_>) -> Result<(), Report> {
    let label = label(page, "Read only").await?;
    let input = input(&label).await?;
    assert_that!(label.attr("data-readonly").await?)
        .get_some()
        .is_equal_to("true");
    assert_that!(input.attr("aria-readonly").await?)
        .get_some()
        .is_equal_to("true");
    label.click().await?;
    label.attr_stays("data-selected", Some("true")).await?;
    assert_that!(input.is_selected().await?).is_true();
    Ok(())
}

async fn bound_state(page: &Page<'_>) -> Result<(), Report> {
    let label = label(page, "Bound").await?;
    page.element("#test-sw-bound-flip").await?.click().await?;
    label.wait_for_attr("data-selected", Some("true")).await?;
    label.click().await?;
    label.wait_for_attr("data-selected", None).await?;
    page.element("#test-sw-bound-value")
        .await?
        .wait_for_inner_text("false")
        .await?;
    Ok(())
}

/// A bound switch stays read-only, also for Space on the focused input.
async fn bound_read_only(page: &Page<'_>) -> Result<(), Report> {
    let label = label(page, "Bound read only").await?;
    let input = input(&label).await?;
    label.click().await?;
    input.focus().await?;
    page.send_keys(Key::Space).await?;
    page.element("#test-sw-bound-read-only-value")
        .await?
        .inner_text_stays("false")
        .await?;
    assert_that!(input.is_selected().await?).is_false();
    Ok(())
}
