// Upstream: react-aria/test/interactions/Focusable.test.js @ 99e6102368
// Upstream: react-aria-components/test/Tooltip.test.js @ 99e6102368
use std::borrow::Cow;

use assertr::prelude::*;
use browser_test::{
    BrowserTest, async_trait,
    thirtyfour::{WebDriver, WebElement},
};
use rootcause::Report;

use crate::pages::{BaseActions, Page};

/// The `Focusable` atom: its focus handling goes onto the child, which gets a `tabindex` unless it
/// has one; `Focusable` and `Pressable` children are custom tooltip triggers.
pub struct FocusableAtomTests {}

#[async_trait]
impl BrowserTest<str> for FocusableAtomTests {
    fn name(&self) -> Cow<'_, str> {
        "focusable_atom_tests".into()
    }

    async fn run(&self, driver: &WebDriver, base_url: &str) -> Result<(), Report> {
        let page = Page { driver, base_url };
        page.goto_path("/atoms/focusable").await?;

        // "should apply focusable props to child element": focusable by Tab, focus handlers.
        let focusable = page.element("test-focusable").await?;
        page.wait_for_attr(&focusable, "tabindex", Some("0"))
            .await?;
        page.element("test-focusable-before").await?.focus().await?;
        page.press_tab().await?;
        page.wait_for_active_id("test-focusable").await?;
        page.wait_for_text("test-focusable-log", "focus").await?;

        // "supports isDisabled", "supports excludeFromTabOrder".
        let disabled = page.element("test-focusable-disabled").await?;
        assert_that!(disabled.attr("tabindex").await?).is_none();
        let excluded = page.element("test-focusable-excluded").await?;
        page.wait_for_attr(&excluded, "tabindex", Some("-1"))
            .await?;

        // Tab skips the disabled, the excluded and the merged one (`tabindex="-1"` of its own) onto
        // the trigger: "should support custom Focusable trigger on focus".
        page.press_tab().await?;
        page.wait_for_active_id("test-focusable-trigger").await?;
        let trigger = page.element("test-focusable-trigger").await?;
        page.wait_for_selector("[role=tooltip]").await?;
        // Visible once it faded in.
        page.wait_for_selector_text("[role=tooltip]", "Focusable tooltip")
            .await?;
        let tooltip = page.css("[role=tooltip]").await?;
        let tooltip_id = tooltip.attr("id").await?;
        assert_that!(trigger.attr("aria-describedby").await?).is_equal_to(tooltip_id);

        // "should should merge with existing props": the child's own `tabindex` stays, both focus
        // handlers run.
        let merged = page.element("test-focusable-merged").await?;
        assert_that!(merged.attr("tabindex").await?).is_equal_to(Some("-1".to_owned()));
        merged.focus().await?;
        page.wait_for_text("test-focusable-log", "focus, blur, own focus, merged focus")
            .await?;

        // "should support custom Pressable trigger": on hover.
        hover(driver, &page.element("test-focusable-log").await?).await?;
        page.element("test-focusable-before").await?.click().await?;
        page.wait_for_no_selector("[role=tooltip]").await?;
        let pressable = page.element("test-pressable-trigger").await?;
        hover(driver, &pressable).await?;
        page.wait_for_selector("[role=tooltip]").await?;
        let tooltip = page.css("[role=tooltip]").await?;
        assert_that!(tooltip.text().await?).is_equal_to("Pressable tooltip".to_owned());
        let tooltip_id = tooltip.attr("id").await?;
        assert_that!(pressable.attr("aria-describedby").await?).is_equal_to(tooltip_id);

        // "supports autoFocus".
        page.element("test-focusable-mount-auto")
            .await?
            .click()
            .await?;
        page.wait_for_active_id("test-focusable-auto").await?;

        page.expect_no_page_errors().await
    }
}

async fn hover(driver: &WebDriver, element: &WebElement) -> Result<(), Report> {
    driver
        .action_chain()
        .move_to_element_center(element)
        .perform()
        .await?;
    Ok(())
}
