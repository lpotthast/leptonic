// Upstream: react-aria/test/interactions/useHover.test.js @ 99e6102368
use std::{borrow::Cow, time::Duration};

use assertr::prelude::*;
use browser_test::{
    BrowserTest, async_trait,
    thirtyfour::{WebDriver, WebElement},
};
use rootcause::Report;

use crate::pages::{BaseActions, Page};

const START: &str = "start:mouse:test-hover-target,change:true";
const END: &str = "end:mouse:test-hover-target,change:false";

/// `use_hover`: hover start/end with the hooked element as target (also over inner elements),
/// no hover by touch, hover ends when disabled and when the hovered child is removed.
pub struct HoverTests {}

#[async_trait]
impl BrowserTest<str> for HoverTests {
    fn name(&self) -> Cow<'_, str> {
        "hover_tests".into()
    }

    async fn run(&self, driver: &WebDriver, base_url: &str) -> Result<(), Report> {
        let page = Page { driver, base_url };
        page.goto_path("/hooks/hover").await?;
        let away = page.element("test-hover-away").await?;
        let target = page.element("test-hover-target").await?;

        // "hover event target should be the same element we attached listeners to even if we
        // hover over inner elements".
        hover(driver, &away).await?;
        hover(driver, &page.element("test-hover-inner").await?).await?;
        page.wait_for_text("test-hover-log", START).await?;
        page.wait_for_attr(&target, "data-hovered", Some("true"))
            .await?;
        hover(driver, &away).await?;
        page.wait_for_text("test-hover-log", &format!("{START},{END}"))
            .await?;
        page.wait_for_attr(&target, "data-hovered", None).await?;
        reset(&page).await?;

        // "should not fire hover events when pointerType is touch".
        driver
            .execute(
                "const el = document.getElementById('test-hover-target');
                 for (const type of ['pointerenter', 'pointerleave']) {
                     el.dispatchEvent(new PointerEvent(type, { pointerType: 'touch' }));
                 }",
                vec![],
            )
            .await?;
        tokio::time::sleep(Duration::from_millis(100)).await;
        assert_that!(page.element("test-hover-log").await?.text().await?)
            .is_equal_to(String::new());

        // "should end hover when disabled": disabled by a script, the pointer stays.
        hover(driver, &page.element("test-hover-inner").await?).await?;
        page.wait_for_text("test-hover-log", START).await?;
        js_click(driver, "test-hover-disable").await?;
        page.wait_for_text("test-hover-log", &format!("{START},{END}"))
            .await?;
        page.wait_for_attr(&target, "data-hovered", None).await?;
        // "does not handle hover events if disabled".
        hover(driver, &away).await?;
        hover(driver, &page.element("test-hover-inner").await?).await?;
        tokio::time::sleep(Duration::from_millis(100)).await;
        assert_that!(page.element("test-hover-log").await?.text().await?)
            .is_equal_to(format!("{START},{END}"));
        js_click(driver, "test-hover-disable").await?;
        hover(driver, &away).await?;
        reset(&page).await?;

        // "should trigger onHoverEnd after an element is removed": the target shrinks, the
        // browser fires no `pointerleave`, only `pointerover` on what is under the pointer now.
        let remove = page.element("test-hover-remove").await?;
        hover(driver, &remove).await?;
        page.wait_for_text("test-hover-log", START).await?;
        remove.click().await?;
        page.wait_for_no_selector("#test-hover-remove").await?;
        driver.action_chain().move_by_offset(1, 0).perform().await?;
        page.wait_for_text("test-hover-log", &format!("{START},{END}"))
            .await?;
        page.wait_for_attr(&target, "data-hovered", None).await?;

        // The `Hoverable` atom sets `data-hovered` on its child.
        let hoverable = page.element("test-hoverable").await?;
        hover(driver, &hoverable).await?;
        page.wait_for_attr(&hoverable, "data-hovered", Some("true"))
            .await?;
        hover(driver, &away).await?;
        page.wait_for_attr(&hoverable, "data-hovered", None).await?;

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

/// Clicks a button by script, without moving the pointer.
async fn js_click(driver: &WebDriver, id: &str) -> Result<(), Report> {
    driver
        .execute(&format!("document.getElementById('{id}').click();"), vec![])
        .await?;
    Ok(())
}

async fn reset(page: &Page<'_>) -> Result<(), Report> {
    js_click(page.driver, "test-hover-reset").await?;
    page.wait_for_text("test-hover-log", "").await
}
